use std::collections::HashSet;
use core::fmt::{Debug};
use crate::endian::Endian;

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

#[derive(Debug)]
pub struct Bits {
    pub bits: HashSet<u16>
}

impl Bits {
    fn new(bits: &[u16]) -> Result<Bits, ()> {
        if bits.len() == 0 {
            Err(())
        } else {
            Bits {
                bits: bits.collect()
            }
        }
    }
}

impl Default for Bits {
    fn default() -> Self {
        Bits {
            bits: [0].collect()
        }
    }
}

#[derive(Debug)]
pub struct Unsigned {
    start: u16,
    length: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Unsigned {
    fn new(start: u16, length: u16, factor: f64, offset: f64, endian: Endian) -> Result<Unsigned, ()> {
        if &length == 0 {
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

#[derive(Debug)]
struct Signed {
    start: u16,
    length: u16,
    factor: f64,
    offset: f64,
    endian: Endian
}

impl Signed {
    fn new(start: u16, length: u16, factor: f64, offset: f64, endian: Endian) -> Result<Signed, ()> {
        if &length == 0 {
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
