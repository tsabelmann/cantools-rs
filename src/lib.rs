#![allow(dead_code)]

//! If you are looking for CAN-bus analysis software written in another programming language, have
//! a look at the following repositories:
//! - [cantools](https://github.com/cantools/cantools) CAN-bus analysis and hardware interface
//!     software written in the Python programming language
//! - [CANalyze.jl](https://github.com/tsabelmann/CANalyze.jl) CAN-bus analysis software written in
//!     the Julia programming language

pub mod data;
pub use data::{CANRead, CANWrite};

pub mod utils;
pub use utils::{Mask, Endian};

#[cfg(feature = "decode")]
pub mod decode;

#[cfg(feature = "encode")]
pub mod encode;

#[cfg(feature = "signals")]
pub mod signals;

