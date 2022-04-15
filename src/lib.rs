#![allow(dead_code)]

//! If you are looking for CAN-bus analysis software written in another programming language, have
//! a look at the following repositories:
//! - [cantools](https://github.com/cantools/cantools) CAN-bus analysis and hardware interface
//!     software written in the Python programming language
//! - [CANalyze.jl](https://github.com/tsabelmann/CANalyze.jl) CAN-bus analysis software written in
//!     the Julia programming language

pub mod data;
pub mod utils;

#[cfg(feature = "decode")]
pub mod decode;

#[cfg(feature = "prelude")]
pub mod prelude;

#[cfg(feature = "signals")]
pub mod signals;

pub mod logging;
