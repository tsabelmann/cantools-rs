use std::cmp::min;
use std::ops::Div;
use crate::mask::Mask;
use crate::data::CANData;
use crate::endian::Endian;
use crate::decode::{TryDecode};

#[derive(Debug,PartialEq)]
pub enum LengthError {
    LengthZero,
    LengthGreater64
}

#[derive(Debug,PartialEq)]
pub enum DataError {
    NotEnoughData
}

#[derive(Debug,Default,PartialEq)]
pub struct Bit {
    start: u16
}

impl Bit {
    /// Constructs a new Bit signal.
    ///
    /// # Example
    /// ```
    /// use cantools::signals::Bit;
    /// let sig = Bit::new(42);
    /// assert_eq!(sig.start(), 42);
    /// ```
    pub fn new(start: u16) -> Bit {
        Bit{ start }
    }

    pub fn start(&self) -> u16 {
        self.start
    }
}

impl TryDecode<bool> for Bit {
    type Error = ();

    fn try_decode<D: CANData>(&self, data: &D) -> Result<bool, Self::Error> {
        if self.start >= (8 * data.dlc()) as u16 {
            Err(())
        } else {
            let start_byte = self.start.div(8);
            let bit_in_start_byte = self.start % 8;

            let mut byte = data.data()[start_byte as usize];
            byte >>= bit_in_start_byte;
            byte &= 0x01;
            if byte != 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Unsigned {
    start: u16,
    length: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Unsigned {
    /// Constructs a new Unsigned signal.
    ///
    /// # Example
    /// ```
    /// use cantools::signals::{Unsigned};
    /// use cantools::endian::Endian;
    /// let sig = Unsigned::new(0, 8, 42.0, 1337.0, Endian::Little).unwrap();
    /// ```
    pub fn new(start: u16, length: u16,
               factor: f64, offset: f64, endian: Endian) -> Result<Unsigned, LengthError> {
        if length == 0 {
            Err(LengthError::LengthZero)
        } else if length > 64 {
            Err(LengthError::LengthGreater64)
        } else {
            let var = Unsigned {
                start,
                length,
                factor,
                offset,
                endian
            };
            Ok(var)
        }
    }
}

impl Default for Unsigned {
    fn default() -> Self {
        Unsigned {
            start: 0,
            length: 1,
            factor: 1.0,
            offset: 0.0,
            endian: Endian::Little
        }
    }
}

impl TryDecode<f64> for Unsigned {
    type Error = DataError;

    fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let bit_in_start_byte = self.start % 8;
                    let end_byte = (self.start + self.length - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[i] = 0;
                            },
                            Some(value) => {
                                slice[i] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= bit_in_start_byte;
                    converted &= u64::mask(self.length, 0);

                    let mut result = converted as f64;
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let end_byte = (7 - self.start % 8) + 8 * self.start.div(8);
                    let end_byte = (end_byte + self.length - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    let min_data = min(8, s.len());
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[min_data-i-1] = 0;
                            },
                            Some(value) => {
                                slice[min_data-i-1] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= 7 - (self.start % 8);
                    converted &= u64::mask(self.length, 0);
                    let mut result = converted as f64;
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Signed {
    start: u16,
    length: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Signed {
    /// Constructs a new Signed signal.
    ///
    /// # Example
    /// ```
    /// use cantools::signals::Signed;
    /// use cantools::endian::Endian;
    /// let sig = Signed::new(0, 8, 42.0, 1337.0, Endian::Little).unwrap();
    /// ```
    pub fn new(start: u16, length: u16,
               factor: f64, offset: f64, endian: Endian) -> Result<Signed, LengthError> {
        if length == 0 {
            Err(LengthError::LengthZero)
        } else if length > 64 {
            Err(LengthError::LengthGreater64)
        } else {
            let var = Signed {
                start,
                length,
                factor,
                offset,
                endian
            };
            Ok(var)
        }
    }
}

impl Default for Signed {
    fn default() -> Self {
        Signed {
            start: 0,
            length: 1,
            factor: 1.0,
            offset: 0.0,
            endian: Endian::Little
        }
    }
}

impl TryDecode<f64> for Signed {
    type Error = DataError;

    fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let bit_in_start_byte = self.start % 8;
                    let end_byte = (self.start + self.length - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[i] = 0;
                            },
                            Some(value) => {
                                slice[i] = *value;
                            }
                        }
                    }

                    let mut converted = i64::from_le_bytes(slice);
                    converted >>= bit_in_start_byte;
                    converted &= i64::mask(self.length, 0);

                    if converted & i64::mask(1, self.length - 1) != 0 {
                        converted += !i64::mask(self.length, 0);
                    }

                    let mut result = converted as f64;
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let end_byte = (7 - self.start % 8) + 8 * self.start.div(8);
                    let end_byte = (end_byte + self.length - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    let min_data = min(8, s.len());
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[min_data-i-1] = 0;
                            },
                            Some(value) => {
                                slice[min_data-i-1] = *value;
                            }
                        }
                    }

                    let mut converted = i64::from_le_bytes(slice);
                    converted >>= 7 - self.start % 8;
                    converted &= i64::mask(self.length, 0);

                    if converted & i64::mask(1, self.length - 1) != 0 {
                        converted += !i64::mask(self.length, 0);
                    }

                    let mut result = converted as f64;
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Float32 {
    start: u16,
    factor: f32,
    offset: f32,
    endian: Endian
}

impl Float32 {
    /// Constructs a new Float32 (f32) signal.
    ///
    /// # Example
    /// ```
    /// use cantools::signals::Float32;
    /// use cantools::endian::Endian;
    /// let sig = Float32::new(0, 42.0, 1337.0, Endian::Little);
    /// ```
    pub fn new(start: u16, factor: f32, offset: f32, endian: Endian) -> Self {
        Float32 {
            start,
            factor,
            offset,
            endian
        }
    }
}

impl Default for Float32 {
    fn default() -> Self {
        Float32 {
            start: 0,
            factor: 1f32,
            offset: 0f32,
            endian: Endian::Little
        }
    }
}

impl TryDecode<f32> for Float32 {
    type Error = DataError;
    fn try_decode<D: CANData>(&self, data: &D) -> Result<f32, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + 32 > (8 * data.dlc() as u16) {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let bit_in_start_byte = self.start % 8;
                    let end_byte = (self.start + 32 - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[i] = 0;
                            },
                            Some(value) => {
                                slice[i] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= bit_in_start_byte;
                    converted &= u64::mask(32, 0);
                    let converted = (converted as u32).to_le_bytes();

                    let mut result = f32::from_le_bytes(converted);
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - 32;
                if shift < 0 {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let end_byte = 7 - (self.start % 8) + 8 * self.start.div(8);
                    let end_byte = (end_byte + 32 - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    let min_data = min(8, s.len());
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[min_data-i-1] = 0;
                            },
                            Some(value) => {
                                slice[min_data-i-1] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= 7 - (self.start % 8);
                    converted &= u64::mask(32, 0);
                    let converted = converted as u32;
                    let converted = converted.to_le_bytes();

                    let mut result = f32::from_le_bytes(converted);
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Float64 {
    start: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Float64 {
    /// Constructs a new Float64 (f64) signal.
    ///
    /// # Example
    /// ```
    /// use cantools::signals::Float64;
    /// use cantools::endian::Endian;
    /// let sig = Float64::new(0, 42.0, 1337.0, Endian::Little);
    /// ```
    pub fn new(start: u16, factor: f64, offset: f64, endian: Endian) -> Self {
        Float64 {
            start,
            factor,
            offset,
            endian
        }
    }
}

impl Default for Float64 {
    fn default() -> Self {
        Float64 {
            start: 0,
            factor: 1f64,
            offset: 0f64,
            endian: Endian::Little
        }
    }
}

impl TryDecode<f64> for Float64 {
    type Error = DataError;
    fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + 64 > (8 * data.dlc() as u16) {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let bit_in_start_byte = self.start % 8;
                    let end_byte = (self.start + 64 - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[i] = 0;
                            },
                            Some(value) => {
                                slice[i] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= bit_in_start_byte;
                    converted &= u64::mask(64, 0);
                    let converted = converted.to_le_bytes();

                    let mut result = f64::from_le_bytes(converted);
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - 64;
                if shift < 0 {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let end_byte = (7 - self.start % 8) + 8 * self.start.div(8);
                    let end_byte = (end_byte + 64 - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    let min_data = min(8, s.len());
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[min_data-i-1] = 0;
                            },
                            Some(value) => {
                                slice[min_data-i-1] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= 7 - self.start % 8;
                    converted &= u64::mask(64, 0);
                    let converted = converted.to_le_bytes();

                    let mut result = f64::from_le_bytes(converted);
                    result *= &self.factor;
                    result += &self.offset;
                    Ok(result)
                }
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Raw {
    start: u16,
    length: u16,
    endian: Endian
}

impl Raw {
    /// Constructs a new Raw signal.
    ///
    /// # Example
    /// ```
    /// use cantools::signals::Raw;
    /// use cantools::endian::Endian;
    /// let sig = Raw::new(42, 8, Endian::Little).unwrap();
    /// ```
    pub fn new(start: u16, length: u16, endian: Endian) -> Result<Raw, LengthError> {
        if length == 0 {
            Err(LengthError::LengthZero)
        } else if length > 64 {
            Err(LengthError::LengthGreater64)
        } else {
            let var = Raw {
                start,
                length,
                endian
            };
            Ok(var)
        }
    }
}

impl Default for Raw {
    fn default() -> Self {
        Raw {
            start: 0,
            length: 1,
            endian: Endian::Little
        }
    }
}

impl TryDecode<u64> for Raw {
    type Error = DataError;

    fn try_decode<D: CANData>(&self, data: &D) -> Result<u64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let bit_in_start_byte = self.start % 8;
                    let end_byte = (self.start + self.length - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[i] = 0;
                            },
                            Some(value) => {
                                slice[i] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= bit_in_start_byte;
                    converted &= u64::mask(self.length, 0);
                    Ok(converted)
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    Err(DataError::NotEnoughData)
                } else {
                    let start_byte = self.start.div(8);
                    let end_byte = (7 - self.start % 8) + 8 * self.start.div(8);
                    let end_byte = (end_byte + self.length - 1).div(8);

                    let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
                    let s = start_byte..=end_byte;

                    let min_data = min(8, s.len());
                    for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
                        match data.data().get(byte_index as usize){
                            None => {
                                slice[min_data-i-1] = 0;
                            },
                            Some(value) => {
                                slice[min_data-i-1] = *value;
                            }
                        }
                    }

                    let mut converted = u64::from_le_bytes(slice);
                    converted >>= 7 - self.start % 8;
                    converted &= u64::mask(self.length, 0);
                    Ok(converted)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::TryDecode;
    use crate::endian::Endian;
    use crate::mask::Mask;
    use crate::signals::{Bit, Unsigned, Raw, DataError, Float32, Signed};

    #[test]
    fn test_unsigned_001() {
        let sig = Unsigned::new(1, 64, 1.0, 0.0, Endian::Little);
        assert!(sig.is_ok())
    }

    #[test]
    fn test_unsigned_002() {
        let sig = Unsigned::new(1, 65, 1.0, 0.0, Endian::Little);
        assert!(sig.is_err())
    }

    #[test]
    fn test_unsigned_003() {
        let sig = Unsigned::new(1, 0, 1.0, 0.0, Endian::Little);
        assert!(sig.is_err())
    }

    #[test]
    fn test_decode_bit_001() {
        let bit = Bit::new(1);
        let data = [0b1111_0010u8];

        let decode = bit.try_decode(&data);
        assert_eq!(decode, Result::Ok(true));
    }

    #[test]
    fn test_decode_bit_002() {
        let bit = Bit::new(0);
        let data = [0b1111_0010u8];

        let decode = bit.try_decode(&data);
        assert_eq!(decode, Result::Ok(false));
    }

    #[test]
    fn test_decode_bit_003() {

        for i in 0..8 {
            let bit = Bit::new(i);
            let data = [u8::mask(1, i)];
            let decode = bit.try_decode(&data);
            assert_eq!(decode, Result::Ok(true));
        }
    }

    #[test]
    fn test_decode_unsigned_001() {
        let bit = Unsigned::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let data = [255u8, 0, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(255f64));
    }

    #[test]
    fn test_decode_unsigned_002() {
        let bit = Unsigned::new(7, 8, 1.0, 0.0,Endian::Big).unwrap();
        let data = [255u8, 0, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(255f64));
    }

    #[test]
    fn test_decode_unsigned_004() {
        let bit = Unsigned::new(4, 8, 1.0, 0.0,Endian::Little).unwrap();
        let data = [0b11110000, 0b00001111, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(255f64));
    }

    #[test]
    fn test_decode_unsigned_005() {
        let bit = Unsigned::new(4, 8, 2.0, 1337.0,Endian::Little).unwrap();
        let data = [0b11110000, 0b00001111, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(255f64 * 2.0 + 1337.0));
    }

    #[test]
    fn test_decode_unsigned_006() {
        let sig = Unsigned::new(3, 8, 2.0, 1337.0,Endian::Big).unwrap();
        let data = [0b0000_1111, 0b00001111, 0, 0, 0, 0,0, 0];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok((0b1111_0000 as f64 * 2.0) + 1337.0));
    }

    #[test]
    fn test_decode_unsigned_007() {
        let sig = Unsigned::new(0, 9, 2.0, 1337.0,Endian::Little).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    }

    #[test]
    fn test_decode_unsigned_008() {
        let sig = Unsigned::new(6, 8, 2.0, 1337.0,Endian::Big).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    }

    #[test]
    fn test_decode_signed_001() {
        let sig = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let data = [0b0111_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(127.0));
    }

    #[test]
    fn test_decode_signed_002() {
        let sig = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let data = [0b1000_0000];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(-128.0));
    }

    #[test]
    fn test_decode_signed_003() {
        let sig = Signed::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let data = [0b0000_0111, 0b1111_0000];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(127.0));
    }

    #[test]
    fn test_decode_signed_004() {
        let sig = Signed::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let data = [0b0000_1000, 0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(-128.0));
    }

    #[test]
    fn test_decode_signed_005() {
        let sig = Signed::new(0, 8, 42.0, 1337.0,Endian::Little).unwrap();
        let data = [0b0111_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(127.0 * 42.0 + 1337.0));
    }

    #[test]
    fn test_decode_signed_006() {
        let sig = Signed::new(3, 8, 42.0, 1337.0,Endian::Big).unwrap();
        let data = [0b0000_0111, 0b1111_0000];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(127.0 * 42.0 + 1337.0));
    }

    #[test]
    fn test_decode_signed_007() {
        let sig = Signed::new(1, 8, 42.0, 1337.0,Endian::Little).unwrap();
        let data = [0b0000_0111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    }

    #[test]
    fn test_decode_signed_008() {
        let sig = Signed::new(6, 8, 42.0, 1337.0,Endian::Big).unwrap();
        let data = [0b0000_0111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    }

    #[test]
    fn test_decode_raw_001() {
        let sig = Raw::new(0, 8, Endian::Little).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode.unwrap(), 0b0000_1111);
    }

    #[test]
    fn test_decode_raw_002() {
        let sig = Raw::new(4, 8, Endian::Little).unwrap();
        let data = [0b0000_1111, 0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode.unwrap(), 0b1111_0000);
    }

    #[test]
    fn test_decode_raw_003() {
        let sig = Raw::new(7, 8, Endian::Big).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(0b0000_1111));
    }

    #[test]
    fn test_decode_raw_004() {
        let sig = Raw::new(3, 8, Endian::Big).unwrap();
        let data = [0b0000_1111, 0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Ok(0b1111_0000));
    }

    #[test]
    fn test_decode_raw_005() {
        let sig = Raw::new(1, 8, Endian::Little).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    }

    #[test]
    fn test_decode_raw_006() {
        let sig = Raw::new(6, 8, Endian::Big).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    }

    #[test]
    fn test_decode_float32_001() {
        let sig = Float32::new(0, 1.0, 0.0,Endian::Little);
        let data = [1, 2, 3, 4, 5, 6, 7, 8];

        let decode = sig.try_decode(&data);
        assert_eq!(decode, Result::Ok(f32::from_le_bytes([1, 2, 3, 4])));
    }

    #[test]
    fn test_decode_float32_002() {
        let sig = Float32::new(7, 1.0, 0.0,Endian::Big);
        let data = [1, 2, 3, 4, 5, 6, 7, 8];

        let decode = sig.try_decode(&data);
        assert_eq!(decode, Result::Ok(f32::from_be_bytes([1, 2, 3, 4])));
    }
}