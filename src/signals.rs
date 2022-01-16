use std::ops::Div;
use crate::mask::Mask;
use crate::data::CANData;
use crate::endian::Endian;
use crate::decode::{TryDecode};

#[derive(Debug)]
pub struct Bit {
    pub start: u16
}

impl Bit {
    fn new(start: u16) -> Result<Bit, ()>{
        let var = Bit{start};
        Ok(var)
    }
}

impl Default for Bit {
    fn default() -> Self {
        Bit {
            start: 0
        }
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
            byte = byte >> bit_in_start_byte;
            byte = byte & 0x01;
            if byte != 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Unsigned {
    start: u16,
    length: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Unsigned {
    fn new(start: u16, length: u16, factor: f64, offset: f64, endian: Endian) -> Result<Unsigned, ()> {
        if length == 0 {
            Err(())
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

    // fn new_2<const N: usize>(start: u16, length: u16, factor: f64, offset: f64, endian: Endian) -> Self {
    //     match endian {
    //         Endian::Little => {
    //             if (start + length - 1) as usize >= 8*N {
    //                 panic!("Not enough data available")
    //             } else {
    //                 Unsigned {
    //                     start,
    //                     length,
    //                     factor,
    //                     offset,
    //                     endian
    //                 }
    //             }
    //         },
    //         Endian::Big => {
    //             let bit_in_byte = start % 8;
    //             let byte = start.div(8);
    //             let new_shift = 8 * byte + (7 - bit_in_byte);
    //             if ((8 * N - new_shift as usize) as isize) < 0 {
    //                 panic!("Not enough data available")
    //             } else {
    //                 Unsigned {
    //                     start,
    //                     length,
    //                     factor,
    //                     offset,
    //                     endian
    //                 }
    //             }
    //         }
    //     }
    // }
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
    type Error = ();

    fn try_decode<D: CANData>(&self, data: &D) -> Result<f64, Self::Error> {
        match &self.endian {
            Endian::Little => {
                if self.start >= (8 * data.dlc() as u16) {
                    Err(())
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
                Ok(20f64)
            }
        }

        // if self.start >= (8 * data.dlc()) as u16 {
        //     Err(())
        // } else {
        //     let start_byte = self.start.div(8);
        //     let bit_in_byte = self.start % 8;
        //
        //     let mut byte = data.data()[start_byte as usize];
        //     byte = byte >> bit_in_byte;
        //     byte = byte & 0x01;
        //     if byte != 0 {
        //         Ok(true)
        //     } else {
        //         Ok(false)
        //     }
        // }
    }
}



#[derive(Debug, PartialEq)]
struct Signed {
    start: u16,
    length: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Signed {
    fn new(start: u16, length: u16, factor: f64, offset: f64, endian: Endian) -> Result<Signed, ()> {
        if length == 0 {
            Err(())
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

#[derive(Debug)]
struct Float32 {
    start: u16,
    factor: f32,
    offset: f32,
    endian: Endian
}

impl Float32 {
    fn new(start: u16, factor: f32, offset: f32, endian: Endian) -> Self {
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

#[derive(Debug)]
struct Float64 {
    start: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Float64 {
    fn new(start: u16, factor: f64, offset: f64, endian: Endian) -> Self {
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

#[derive(Debug)]
struct Raw {
    start: u16,
    length: u16,
    endian: Endian
}

impl Raw {
    fn new(start: u16, length: u16, endian: Endian) -> Result<Raw, ()> {
        if length == 0 {
            Err(())
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

#[cfg(test)]
mod tests {
    use crate::decode::TryDecode;
    use crate::endian::Endian;
    use crate::signals::{Bit, Unsigned};

    // #[test]
    // #[should_panic]
    // fn test_unsigned_001() {
    //     let _var = Unsigned::new_2::<8>(1, 64, 1.0, 0.0, Endian::Little);
    // }

    // #[test]
    // #[should_panic]
    // fn test_unsigned_002() {
    //     let _var = Unsigned::new_2::<8>(6, 64, 1.0, 0.0, Endian::Big);
    // }

    #[test]
    fn test_unsigned_003() {
        let var1 = Unsigned::new(6, 64, 1.0, 0.0, Endian::Big).unwrap();
        let var2 = Unsigned::new(6, 64, 1.0, 0.0, Endian::Big).unwrap();
        assert_eq!(var1, var2);
    }

    #[test]
    fn test_unsigned_004() {
        let var1 = Unsigned::new(5, 64, 1.0, 0.0, Endian::Big).unwrap();
        let var2 = Unsigned::new(6, 64, 1.0, 0.0, Endian::Big).unwrap();
        assert_ne!(var1, var2);
    }


    #[test]
    fn test_decode_bit_001() {
        let bit = Bit::new(1).unwrap();
        let data = [0b1111_0010u8];

        let decode = bit.try_decode(&data);
        assert_eq!(decode, Result::Ok(true));
    }

    #[test]
    fn test_decode_bit_002() {
        let bit = Bit::new(0).unwrap();
        let data = [0b1111_0010u8];

        let decode = bit.try_decode(&data);
        assert_eq!(decode, Result::Ok(false));
    }

    #[test]
    fn test_decode_unsigned_001() {
        let bit = Unsigned::new(0, 8, 1.0, 0.0,Endian::Little).unwrap();
        let data = [255u8, 0, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{}", &decode.unwrap());
        assert_eq!(decode, Result::Ok(255f64));
    }

    #[test]
    fn test_decode_unsigned_002() {
        let bit = Unsigned::new(4, 8, 1.0, 0.0,Endian::Little).unwrap();
        let data = [0b11110000, 0b00001111, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{}", &decode.unwrap());
        assert_eq!(decode, Result::Ok(255f64));
    }

    #[test]
    fn test_decode_unsigned_003() {
        let bit = Unsigned::new(4, 8, 2.0, 1337.0,Endian::Little).unwrap();
        let data = [0b11110000, 0b00001111, 0, 0, 0, 0,0, 0];

        let decode = bit.try_decode(&data);
        println!("{}", &decode.unwrap());
        assert_eq!(decode, Result::Ok(255f64 * 2.0 + 1337.0));
    }
}