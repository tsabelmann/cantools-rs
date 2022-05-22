#![allow(dead_code)]

//! [cantools](crate) provides types and traits useful if analyzing CAN-bus data.
//! This includes read and write access to the CAN-bus data (see [data](crate::data)), decoding (see
//! [decode](crate::decode)), encoding (see [encode](crate::encode)), and signals (see
//! [signals](crate::signals)) that combine these aspects to extract or set data.
//!
//! If you are looking for CAN-bus analysis software written in another programming language, have
//! a look at the following repositories:
//! - [cantools](https://github.com/cantools/cantools) CAN-bus analysis and hardware interface
//!     software written in the Python programming language
//! - [CANalyze.jl](https://github.com/tsabelmann/CANalyze.jl) CAN-bus analysis software written in
//!     the Julia programming language
//!
//! New features are planed. The following selection shows a non-exhaustive list of future features:
//! - Signal overlap check: Checks whether two or more signals overlap. This is important because
//! otherwise one signal encoding corrupts data set from another signal.
//! - Messages: Grouping of multiple signals into one message such that mass decoding or encoding
//! becomes possible. Messages do have an elaborate interface that I cannot explain here.
//! - Database: Same idea as before. Grouping of messages into one database.
//! - Logging: Implementation of popular logging formats, e.g., **candump** or **Peak**. The formats
//! should work in both directions, either read or write.
//! - Formats: Reading of popular file formats describing the decoding or encoding, e.g., **SYM**,
//! **DBC** or a self conceived **JSON** format.

pub mod data;
pub use data::{CANRead, CANWrite};

pub mod utils;
pub use utils::{Endian, Mask};

pub mod decode;
pub use decode::{Decode, DefaultDecode, TryDecode};

pub mod encode;
pub use encode::{Encode, TryEncode};

pub mod signals;
pub use signals::{Bit, LengthError, Signed, Unsigned};

pub mod formats;
pub mod logging;
