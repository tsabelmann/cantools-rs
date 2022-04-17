use std::cmp::min;
use std::ops::{Div};
use crate::utils::{Mask, Endian};
use crate::data::{CANData, CANWrite};
use crate::decode::{TryDecode, DefaultDecode, Decode, DecodeError};
use crate::encode::{TryEncode, Encode, EncodeError};


#[derive(Debug,PartialEq)]
pub enum LengthError {
    LengthZero,
    LengthGreater64
}

///
pub trait Min {
    type Item;

    ///
    fn min(&self) -> Self::Item;
}

///
pub trait Max {
    ///
    type Item;

    ///
    fn max(&self) -> Self::Item;
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

impl DefaultDecode<bool> for Bit {}
impl Decode<bool> for Bit {}

impl TryEncode<bool> for Bit {
    type Error = EncodeError;

    fn try_encode<D: CANWrite>(&self, data: &mut D, value: bool) -> Result<(), Self::Error> {
        let start_bit_in_byte = self.start % 8;
        let start_byte = self.start.div(8);

        if start_byte as usize >= data.dlc() {
            return Err(EncodeError::NotEnoughData);
        }

        match value {
            true => {
                let mask_byte = u8::mask(1, start_bit_in_byte);
                data.mut_data()[start_byte as usize] |= mask_byte;
            },
            false => {
                let mask_byte = !u8::mask(1, start_bit_in_byte);
                data.mut_data()[start_byte as usize] &= mask_byte;
            }
        }

        Ok(())
    }
}

impl Encode<bool> for Bit {}

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
    /// use cantools::signals::Unsigned;
    /// use cantools::utils::Endian;
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

impl Min for Unsigned {
    type Item = f64;

    fn min(&self) -> Self::Item {
        self.offset
    }
}

impl Max for Unsigned {
    type Item = f64;

    fn max(&self) -> Self::Item {
        let mut base = u64::mask(self.length, 0) as f64;
        base *= self.factor;
        base += self.offset;
        base
    }
}

impl TryDecode<f64> for Unsigned {
    type Error = DecodeError;

    fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    return Err(DecodeError::NotEnoughData);
                }

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
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    return Err(DecodeError::NotEnoughData);
                }

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

impl DefaultDecode<f64> for Unsigned {}
impl Decode<f64> for Unsigned {}

impl TryEncode<f64> for Unsigned {
    type Error = EncodeError;

    fn try_encode<D: CANWrite>(&self, data: &mut D, value: f64) -> Result<(), Self::Error> {
        if value < self.min() {
            return Err(EncodeError::MinError);
        }

        if value > self.max() {
            return Err(EncodeError::MaxError);
        }

        match self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    return Err(EncodeError::NotEnoughData);
                }

                // compute integer value to be set
                let value = value - self.offset;
                let value = value / self.factor;
                let value = value.trunc() as u64;
                let mut value = value & u64::mask(self.length, 0);

                // set data by setting the corresponding data bits
                for i in 0..self.length {
                    let bit = Bit::new(self.start + i);

                    if value & 1 == 0 {
                        bit.try_encode(data, false).unwrap();
                    } else {
                        bit.try_encode(data, true).unwrap();
                    }

                    value >>= 1;
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    return Err(EncodeError::NotEnoughData);
                }

                // compute integer value to be set
                let value = value - self.offset;
                let value = value / self.factor;
                let mut value = value.trunc() as u64;

                value &= u64::mask(self.length, 0);

                // set data by setting the corresponding data bits
                let mut start = self.start;
                for i in 0..self.length {
                    let bit = Bit::new(start);

                    if (value >> (self.length - 1 - i)) & 1 == 0 {
                        bit.try_encode(data, false).unwrap();
                    } else {
                        bit.try_encode(data, true).unwrap();
                    }

                    // update start to be the next bit to set
                    start = if start % 8 == 0 {
                        (start.div(8) + 1) * 8 + 7
                    } else {
                        start - 1
                    };
                }
            }
        }

        Ok(())
    }
}

impl Encode<f64> for Unsigned {}


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
    /// use cantools::utils::Endian;
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

impl Min for Signed {
    type Item = f64;

    fn min(&self) -> Self::Item {
        let mut base = i64::mask(64 - self.length + 1 , self.length - 1) as f64;
        base *= self.factor;
        base += self.offset;
        base
    }
}

impl Max for Signed {
    type Item = f64;

    fn max(&self) -> Self::Item {
        let mut base = u64::mask(self.length - 1, 0) as f64;
        base *= self.factor;
        base += self.offset;
        base
    }
}

impl TryDecode<f64> for Signed {
    type Error = DecodeError;

    fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    return Err(DecodeError::NotEnoughData);
                }

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
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    return Err(DecodeError::NotEnoughData);
                }

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

impl DefaultDecode<f64> for Signed {}
impl Decode<f64> for Signed {}

impl TryEncode<f64> for Signed {
    type Error = EncodeError;

    fn try_encode<D: CANWrite>(&self, data: &mut D, value: f64) -> Result<(), Self::Error> {
        if value < self.min() {
            return Err(EncodeError::MinError);
        }

        if value > self.max() {
            return Err(EncodeError::MaxError);
        }

        match self.endian {
            Endian::Little => {
                if self.start + self.length > (8 * data.dlc() as u16) {
                    return Err(EncodeError::NotEnoughData);
                }

                // compute integer value to be set
                let value = value - self.offset;
                let value = value / self.factor;
                let mut value = value.trunc() as i64;

                if value < 0 {
                    value -= !i64::mask(self.length, 0);
                };

                value &= i64::mask(self.length, 0);

                // set data by setting the corresponding data bits
                for i in 0..self.length {
                    let bit = Bit::new(self.start + i);

                    if value & 1 == 0 {
                        bit.try_encode(data, false).unwrap();
                    } else {
                        bit.try_encode(data, true).unwrap();
                    }

                    value >>= 1;
                }
            },
            Endian::Big => {
                let shift = (7 - self.start % 8) + 8 * self.start.div(8);
                let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
                if shift < 0 {
                    return Err(EncodeError::NotEnoughData);
                }

                // compute integer value to be set
                let value = value - self.offset;
                let value = value / self.factor;
                let mut value = value.trunc() as i64;

                if value < 0 {
                    value -= !i64::mask(self.length, 0);
                };

                value &= i64::mask(self.length, 0);

                // set data by setting the corresponding data bits
                let mut start = self.start;
                for i in 0..self.length {
                    let bit = Bit::new(start);

                    if (value >> (self.length - 1 - i)) & 1 == 0 {
                        bit.try_encode(data, false).unwrap();
                    } else {
                        bit.try_encode(data, true).unwrap();
                    }

                    // update start to be the next bit to set
                    start = if start % 8 == 0 {
                        (start.div(8) + 1) * 8 + 7
                    } else {
                        start - 1
                    };
                }
            }
        }

        Ok(())
    }
}

impl Encode<f64> for Signed {}


// #[derive(Debug,PartialEq)]
// pub struct Float32 {
//     start: u16,
//     factor: f32,
//     offset: f32,
//     endian: Endian
// }
//
// impl Float32 {
//     /// Constructs a new Float32 (f32) signal.
//     ///
//     /// # Example
//     /// ```
//     /// use cantools::signals::Float32;
//     /// use cantools::utils::Endian;
//     /// let sig = Float32::new(0, 42.0, 1337.0, Endian::Little);
//     /// ```
//     pub fn new(start: u16, factor: f32, offset: f32, endian: Endian) -> Self {
//         Float32 {
//             start,
//             factor,
//             offset,
//             endian
//         }
//     }
// }
//
// impl Default for Float32 {
//     fn default() -> Self {
//         Float32 {
//             start: 0,
//             factor: 1f32,
//             offset: 0f32,
//             endian: Endian::Little
//         }
//     }
// }
//
// impl TryDecode<f32> for Float32 {
//     type Error = DataError;
//     fn try_decode<D: CANData>(&self, data: &D) -> Result<f32, Self::Error> {
//         match &self.endian {
//             Endian::Little => {
//                 if self.start + 32 > (8 * data.dlc() as u16) {
//                     return Err(DataError::NotEnoughData);
//                 }
//
//                 let start_byte = self.start.div(8);
//                 let bit_in_start_byte = self.start % 8;
//                 let end_byte = (self.start + 32 - 1).div(8);
//
//                 let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
//                 let s = start_byte..=end_byte;
//                 for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
//                     match data.data().get(byte_index as usize){
//                         None => {
//                             slice[i] = 0;
//                         },
//                         Some(value) => {
//                             slice[i] = *value;
//                         }
//                     }
//                 }
//
//                 let mut converted = u64::from_le_bytes(slice);
//                 converted >>= bit_in_start_byte;
//                 converted &= u64::mask(32, 0);
//                 let converted = (converted as u32).to_le_bytes();
//
//                 let mut result = f32::from_le_bytes(converted);
//                 result *= &self.factor;
//                 result += &self.offset;
//                 Ok(result)
//             },
//             Endian::Big => {
//                 let shift = (7 - self.start % 8) + 8 * self.start.div(8);
//                 let shift = (8 * data.dlc()) as isize - (shift as isize) - 32;
//                 if shift < 0 {
//                     return Err(DataError::NotEnoughData);
//                 }
//
//                 let start_byte = self.start.div(8);
//                 let end_byte = 7 - (self.start % 8) + 8 * self.start.div(8);
//                 let end_byte = (end_byte + 32 - 1).div(8);
//
//                 let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
//                 let s = start_byte..=end_byte;
//
//                 let min_data = min(8, s.len());
//                 for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
//                     match data.data().get(byte_index as usize){
//                         None => {
//                             slice[min_data-i-1] = 0;
//                         },
//                         Some(value) => {
//                             slice[min_data-i-1] = *value;
//                         }
//                     }
//                 }
//
//                 let mut converted = u64::from_le_bytes(slice);
//                 converted >>= 7 - (self.start % 8);
//                 converted &= u64::mask(32, 0);
//                 let converted = converted as u32;
//                 let converted = converted.to_le_bytes();
//
//                 let mut result = f32::from_le_bytes(converted);
//                 result *= &self.factor;
//                 result += &self.offset;
//                 Ok(result)
//             }
//         }
//     }
// }
//
// #[derive(Debug,PartialEq)]
// pub struct Float64 {
//     start: u16,
//     factor: f64,
//     offset: f64,
//     endian: Endian
// }
//
// impl Float64 {
//     /// Constructs a new Float64 (f64) signal.
//     ///
//     /// # Example
//     /// ```
//     /// use cantools::signals::Float64;
//     /// use cantools::utils::Endian;
//     /// let sig = Float64::new(0, 42.0, 1337.0, Endian::Little);
//     /// ```
//     pub fn new(start: u16, factor: f64, offset: f64, endian: Endian) -> Self {
//         Float64 {
//             start,
//             factor,
//             offset,
//             endian
//         }
//     }
// }
//
// impl Default for Float64 {
//     fn default() -> Self {
//         Float64 {
//             start: 0,
//             factor: 1f64,
//             offset: 0f64,
//             endian: Endian::Little
//         }
//     }
// }
//
// impl TryDecode<f64> for Float64 {
//     type Error = DataError;
//     fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
//         match &self.endian {
//             Endian::Little => {
//                 if self.start + 64 > (8 * data.dlc() as u16) {
//                     return Err(DataError::NotEnoughData);
//                 }
//
//                 let start_byte = self.start.div(8);
//                 let bit_in_start_byte = self.start % 8;
//                 let end_byte = (self.start + 64 - 1).div(8);
//
//                 let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
//                 let s = start_byte..=end_byte;
//
//                 for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
//                     match data.data().get(byte_index as usize){
//                         None => {
//                             slice[i] = 0;
//                         },
//                         Some(value) => {
//                             slice[i] = *value;
//                         }
//                     }
//                 }
//
//                 let mut converted = u64::from_le_bytes(slice);
//                 converted >>= bit_in_start_byte;
//                 converted &= u64::mask(64, 0);
//                 let converted = converted.to_le_bytes();
//
//                 let mut result = f64::from_le_bytes(converted);
//                 result *= &self.factor;
//                 result += &self.offset;
//                 Ok(result)
//             },
//             Endian::Big => {
//                 let shift = (7 - self.start % 8) + 8 * self.start.div(8);
//                 let shift = (8 * data.dlc()) as isize - (shift as isize) - 64;
//                 if shift < 0 {
//                     return Err(DataError::NotEnoughData);
//                 }
//
//                 let start_byte = self.start.div(8);
//                 let end_byte = (7 - self.start % 8) + 8 * self.start.div(8);
//                 let end_byte = (end_byte + 64 - 1).div(8);
//
//                 let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
//                 let s = start_byte..=end_byte;
//
//                 let min_data = min(8, s.len());
//                 for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
//                     match data.data().get(byte_index as usize){
//                         None => {
//                             slice[min_data-i-1] = 0;
//                         },
//                         Some(value) => {
//                             slice[min_data-i-1] = *value;
//                         }
//                     }
//                 }
//
//                 let mut converted = u64::from_le_bytes(slice);
//                 converted >>= 7 - self.start % 8;
//                 converted &= u64::mask(64, 0);
//                 let converted = converted.to_le_bytes();
//
//                 let mut result = f64::from_le_bytes(converted);
//                 result *= &self.factor;
//                 result += &self.offset;
//                 Ok(result)
//             }
//         }
//     }
// }
//
// #[derive(Debug,PartialEq)]
// pub struct Raw {
//     start: u16,
//     length: u16,
//     endian: Endian
// }
//
// impl Raw {
//     /// Constructs a new Raw signal.
//     ///
//     /// # Example
//     /// ```
//     /// use cantools::signals::Raw;
//     /// use cantools::utils::Endian;
//     /// let sig = Raw::new(42, 8, Endian::Little).unwrap();
//     /// ```
//     pub fn new(start: u16, length: u16, endian: Endian) -> Result<Raw, LengthError> {
//         if length == 0 {
//             Err(LengthError::LengthZero)
//         } else if length > 64 {
//             Err(LengthError::LengthGreater64)
//         } else {
//             let var = Raw {
//                 start,
//                 length,
//                 endian
//             };
//             Ok(var)
//         }
//     }
// }
//
// impl Default for Raw {
//     fn default() -> Self {
//         Raw {
//             start: 0,
//             length: 1,
//             endian: Endian::Little
//         }
//     }
// }
//
// impl TryDecode<u64> for Raw {
//     type Error = DataError;
//
//     fn try_decode<D: CANData>(&self, data: &D) -> Result<u64, Self::Error> {
//         match &self.endian {
//             Endian::Little => {
//                 if self.start + self.length > (8 * data.dlc() as u16) {
//                     return Err(DataError::NotEnoughData);
//                 }
//
//                 let start_byte = self.start.div(8);
//                 let bit_in_start_byte = self.start % 8;
//                 let end_byte = (self.start + self.length - 1).div(8);
//
//                 let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
//                 let s = start_byte..=end_byte;
//
//                 for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < 8) {
//                     match data.data().get(byte_index as usize){
//                         None => {
//                             slice[i] = 0;
//                         },
//                         Some(value) => {
//                             slice[i] = *value;
//                         }
//                     }
//                 }
//
//                 let mut converted = u64::from_le_bytes(slice);
//                 converted >>= bit_in_start_byte;
//                 converted &= u64::mask(self.length, 0);
//                 Ok(converted)
//             },
//             Endian::Big => {
//                 let shift = (7 - self.start % 8) + 8 * self.start.div(8);
//                 let shift = (8 * data.dlc()) as isize - (shift as isize) - (self.length as isize);
//                 if shift < 0 {
//                     return Err(DataError::NotEnoughData);
//                 }
//
//                 let start_byte = self.start.div(8);
//                 let end_byte = (7 - self.start % 8) + 8 * self.start.div(8);
//                 let end_byte = (end_byte + self.length - 1).div(8);
//
//                 let mut slice = [0u8,0u8,0u8,0u8,0u8,0u8,0u8,0u8];
//                 let s = start_byte..=end_byte;
//
//                 let min_data = min(8, s.len());
//                 for (i, byte_index) in s.into_iter().enumerate().filter(|(i,_)| *i < min_data) {
//                     match data.data().get(byte_index as usize){
//                         None => {
//                             slice[min_data-i-1] = 0;
//                         },
//                         Some(value) => {
//                             slice[min_data-i-1] = *value;
//                         }
//                     }
//                 }
//
//                 let mut converted = u64::from_le_bytes(slice);
//                 converted >>= 7 - self.start % 8;
//                 converted &= u64::mask(self.length, 0);
//                 Ok(converted)
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::decode::TryDecode;
    use crate::encode::{EncodeError, TryEncode, Encode};
    use crate::utils::{Endian, Mask};
    // use crate::signals::{Bit, Unsigned, Raw, DataError, Float32, Signed};
    use crate::signals::{Bit, Unsigned, DecodeError, Signed, Min, Max};

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

    /* TEST ENCODE BIT */

    #[test]
    fn test_encode_bit_001() {
        for i in 0..8 {
            let bit = Bit::new(i);
            let data = [u8::mask(1, i)];
            let mut data_to_encode = [0u8];
            let result = bit.try_encode(&mut data_to_encode, true);

            assert_eq!(result, Ok(()));
            assert_eq!(data, data_to_encode);
        }
    }

    #[test]
    fn test_encode_bit_002() {
        for i in 0..8 {
            let bit = Bit::new(i);
            let data = [u8::mask(1, i)];
            let mut data_to_encode = [0u8];
            Encode::encode(&bit,&mut data_to_encode, true);

            assert_eq!(data, data_to_encode);
        }
    }

    #[test]
    fn test_encode_bit_003() {
        for i in 0..8 {
            let bit = Bit::new(i);
            let data = [!u8::mask(1, i)];
            let mut data_to_encode = [0xFFu8];
            let result = bit.try_encode(&mut data_to_encode, false);

            assert_eq!(result, Ok(()));
            assert_eq!(data, data_to_encode);
        }
    }

    #[test]
    fn test_encode_bit_004() {
        for i in 0..8 {
            let bit = Bit::new(i);
            let data = [!u8::mask(1, i)];
            let mut data_to_encode = [0xFFu8];
            let result = bit.try_encode(&mut data_to_encode, false);

            assert_eq!(result, Ok(()));
            assert_eq!(data, data_to_encode);
        }
    }

    #[test]
    fn test_encode_bit_005() {
        for i in 8..64 {
            let bit = Bit::new(i);
            let mut data_to_encode = [0xFFu8];
            let result = bit.try_encode(&mut data_to_encode, false);

            assert!(result.is_err());
            assert_eq!(result, Err(EncodeError::NotEnoughData))
        }
    }

    #[test]
    #[should_panic]
    fn test_encode_bit_006() {
        for i in 8..64 {
            let bit = Bit::new(i);
            let mut data_to_encode = [0xFFu8];
            bit.encode(&mut data_to_encode, false);
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
        assert_eq!(decode, Result::Err(DecodeError::NotEnoughData));
    }

    #[test]
    fn test_decode_unsigned_008() {
        let sig = Unsigned::new(6, 8, 2.0, 1337.0,Endian::Big).unwrap();
        let data = [0b0000_1111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DecodeError::NotEnoughData));
    }

    #[test]
    fn test_decode_unsigned_min_max_001() {
        let sig = Unsigned::new(6, 8, 2.0, 1337.0,Endian::Little).unwrap();
        assert_eq!(sig.min(), 1337.0);
        assert_eq!(sig.max(), (255.0 * 2.0) + 1337.0);
    }

    #[test]
    fn test_decode_unsigned_min_max_002() {
        let sig = Unsigned::new(6, 8, 1.0, 0.0,Endian::Little).unwrap();
        assert_eq!(sig.min(), 0.0);
        assert_eq!(sig.max(), 255.0);
    }

    #[test]
    fn test_decode_unsigned_min_max_003() {
        let sig = Unsigned::new(6, 16, 1.0, 0.0,Endian::Little).unwrap();
        assert_eq!(sig.min(), 0.0);
        assert_eq!(sig.max(), 256.0 * 256.0 - 1.0);
    }

    #[test]
    fn test_encode_unsigned_001() {
        let unsigned = Unsigned::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        for i in 0..=255u8 {
            let mut data = [0u8];

            let result = unsigned.try_encode(&mut data, i as f64);
            assert!(result.is_ok());
            assert_eq!(data, [i])
        }
    }

    #[test]
    fn test_encode_unsigned_002() {
        let unsigned = Unsigned::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0u8, 0u8];

        let result = unsigned.try_encode(&mut data, 255.0);
        assert!(result.is_ok());
        assert_eq!(data, [0x0Fu8, 0xF0u8]);
    }

    #[test]
    fn test_encode_unsigned_004() {
        let unsigned = Unsigned::new(2, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0u8, 0u8];

        let result = unsigned.try_encode(&mut data, 255.0);
        assert!(result.is_ok());
        assert_eq!(data, [0b0000_0111u8, 0b1111_1000u8]);
    }

    #[test]
    fn test_encode_unsigned_005() {
        let unsigned = Unsigned::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0xA0u8, 0x0Bu8];

        let result = unsigned.try_encode(&mut data, 255.0);
        assert!(result.is_ok());
        assert_eq!(data, [0xAFu8, 0xFBu8]);
    }

    #[test]
    fn test_encode_unsigned_006() {
        let unsigned = Unsigned::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0xA0u8, 0x0Bu8];

        let result = unsigned.try_encode(&mut data, 254_f64);
        assert!(result.is_ok());
        assert_eq!(data, [0xAFu8, 0xEBu8]);
    }

    /* TEST SIGNED */

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
        assert_eq!(decode, Result::Err(DecodeError::NotEnoughData));
    }

    #[test]
    fn test_decode_signed_008() {
        let sig = Signed::new(6, 8, 42.0, 1337.0,Endian::Big).unwrap();
        let data = [0b0000_0111];

        let decode = sig.try_decode(&data);
        println!("{:?}", &decode);
        assert_eq!(decode, Result::Err(DecodeError::NotEnoughData));
    }

        #[test]
    fn test_decode_signed_min_max_001() {
        let sig = Signed::new(6, 8, 2.0, 1337.0,Endian::Little).unwrap();
        assert_eq!(sig.min(), -128.0 * 2.0 + 1337.0);
        assert_eq!(sig.max(), 127.0 * 2.0 + 1337.0);
    }

    #[test]
    fn test_encode_signed_001() {
        let unsigned = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        for i in 0..=127u8 {
            let mut data = [0u8];

            let result = unsigned.try_encode(&mut data, i as f64);
            assert!(result.is_ok());
            assert_eq!(data, [i])
        }
    }

    #[test]
    fn test_encode_signed_002() {
        let unsigned = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let mut data = [0u8];

        let result = unsigned.try_encode(&mut data, -128_f64);
        assert!(result.is_ok());
        assert_eq!(data, [0b1000_0000u8])
    }

    #[test]
    fn test_encode_signed_003() {
        let unsigned = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let mut data = [0u8];

        let result = unsigned.try_encode(&mut data, -1_f64);
        assert!(result.is_ok());
        assert_eq!(data, [0b1111_1111])
    }

    #[test]
    fn test_encode_signed_004() {
        let unsigned = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let mut data = [0u8];

        let result = unsigned.try_encode(&mut data, 128_f64);
        assert!(result.is_err());
        assert_eq!(result, Err(EncodeError::MaxError));
    }

    #[test]
    fn test_encode_signed_005() {
        let unsigned = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let mut data = [0u8];

        let result = unsigned.try_encode(&mut data, -129_f64);
        assert!(result.is_err());
        assert_eq!(result, Err(EncodeError::MinError));
    }

    #[test]
    fn test_encode_signed_006() {
        let unsigned = Signed::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let mut data = [];

        let result = unsigned.try_encode(&mut data, -128_f64);
        assert!(result.is_err());
        assert_eq!(result, Err(EncodeError::NotEnoughData));
    }

    #[test]
    fn test_encode_signed_007() {
        let unsigned = Signed::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0u8, 0u8];

        let result = unsigned.try_encode(&mut data, 127_f64);
        assert!(result.is_ok());
        assert_eq!(data, [0b0000_0111u8, 0b1111_0000u8]);
    }

    #[test]
    fn test_encode_signed_008() {
        let signed = Signed::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0b0000_0000u8, 0b0000_0000u8];

        let result = signed.try_encode(&mut data, -128_f64);
        assert!(result.is_ok());
        assert_eq!(data, [0b0000_1000u8, 0b0000_0000u8]);
    }

    #[test]
    fn test_encode_signed_009() {
        let signed = Signed::new(3, 8, 1.0, 0.0,Endian::Big).unwrap();
        let mut data = [0b1000_0000u8, 0b0000_0001u8];

        let result = signed.try_encode(&mut data, -1_f64);
        assert!(result.is_ok());
        assert_eq!(data, [0b1000_1111u8, 0b1111_0001u8]);
    }


    // #[test]
    // fn test_decode_signed_min_max_002() {
    //     let sig = Signed::new(6, 8, 1.0, 0.0,Endian::Little).unwrap();
    //     assert_eq!(sig.min(), 0.0);
    //     assert_eq!(sig.max(), 255.0);
    // }
    //
    // #[test]
    // fn test_decode_signed_min_max_003() {
    //     let sig = Signed::new(6, 16, 1.0, 0.0,Endian::Little).unwrap();
    //     assert_eq!(sig.min(), 0.0);
    //     assert_eq!(sig.max(), 256.0 * 256.0 - 1.0);
    // }

    // #[test]
    // fn test_decode_raw_001() {
    //     let sig = Raw::new(0, 8, Endian::Little).unwrap();
    //     let data = [0b0000_1111];
    //
    //     let decode = sig.try_decode(&data);
    //     println!("{:?}", &decode);
    //     assert_eq!(decode.unwrap(), 0b0000_1111);
    // }
    //
    // #[test]
    // fn test_decode_raw_002() {
    //     let sig = Raw::new(4, 8, Endian::Little).unwrap();
    //     let data = [0b0000_1111, 0b0000_1111];
    //
    //     let decode = sig.try_decode(&data);
    //     println!("{:?}", &decode);
    //     assert_eq!(decode.unwrap(), 0b1111_0000);
    // }
    //
    // #[test]
    // fn test_decode_raw_003() {
    //     let sig = Raw::new(7, 8, Endian::Big).unwrap();
    //     let data = [0b0000_1111];
    //
    //     let decode = sig.try_decode(&data);
    //     println!("{:?}", &decode);
    //     assert_eq!(decode, Result::Ok(0b0000_1111));
    // }
    //
    // #[test]
    // fn test_decode_raw_004() {
    //     let sig = Raw::new(3, 8, Endian::Big).unwrap();
    //     let data = [0b0000_1111, 0b0000_1111];
    //
    //     let decode = sig.try_decode(&data);
    //     println!("{:?}", &decode);
    //     assert_eq!(decode, Result::Ok(0b1111_0000));
    // }
    //
    // #[test]
    // fn test_decode_raw_005() {
    //     let sig = Raw::new(1, 8, Endian::Little).unwrap();
    //     let data = [0b0000_1111];
    //
    //     let decode = sig.try_decode(&data);
    //     println!("{:?}", &decode);
    //     assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    // }
    //
    // #[test]
    // fn test_decode_raw_006() {
    //     let sig = Raw::new(6, 8, Endian::Big).unwrap();
    //     let data = [0b0000_1111];
    //
    //     let decode = sig.try_decode(&data);
    //     println!("{:?}", &decode);
    //     assert_eq!(decode, Result::Err(DataError::NotEnoughData));
    // }
    //
    // #[test]
    // fn test_decode_float32_001() {
    //     let sig = Float32::new(0, 1.0, 0.0,Endian::Little);
    //     let data = [1, 2, 3, 4, 5, 6, 7, 8];
    //
    //     let decode = sig.try_decode(&data);
    //     assert_eq!(decode, Result::Ok(f32::from_le_bytes([1, 2, 3, 4])));
    // }
    //
    // #[test]
    // fn test_decode_float32_002() {
    //     let sig = Float32::new(7, 1.0, 0.0,Endian::Big);
    //     let data = [1, 2, 3, 4, 5, 6, 7, 8];
    //
    //     let decode = sig.try_decode(&data);
    //     assert_eq!(decode, Result::Ok(f32::from_be_bytes([1, 2, 3, 4])));
    // }
}