//! Module providing utility traits for encoding CAN-bus data.
//!
//! The module provides two traits: [TryEncode], and [Encode]. [try_encode](TryEncode::try_encode)
//! models the possibility that the encoding fails whereas [encode](Encode::encode) models the not
//! failable encoding. If [encode](Encode::encode) fails internally, it panics.
//!
//! # Example
//! ```
//! use cantools::data::CANRead;
//! use cantools::signals::Bit;
//! use cantools::encode::{TryEncode, Encode};
//!
//! let bit = Bit::new(20);
//! let mut data = [0u8, 0u8, 0u8, 0u8];
//!
//! let result = bit.try_encode(&mut data, true);
//! bit.encode(&mut data, false);
//! ```

use crate::data::CANWrite;

/// Type representing possible encoding errors.
#[derive(Debug, PartialEq)]
pub enum EncodeError {
    /// There is not enough byte data available to encode a value.
    NotEnoughData,
    /// The value to encode is smaller than the minimum value encodable.
    MinError,
    /// The value to encode is greater than the maximum value encodable.
    MaxError,
}

/// A trait modeling the failable encoding of data.
pub trait TryEncode<T> {
    /// A type modelling the different possible failures of the encoding.
    type Error;

    /// Tries to encode `value` into `data`.
    fn try_encode<D: CANWrite>(&self, data: &mut D, value: T) -> Result<(), Self::Error>;
}

/// A trait modeling the not failable encoding of data.
///
/// [Encode] is sub-trait of [TryEncode]. [encode](Encode::encode) is implemented using
/// [try_encode](TryEncode::try_encode). If [try_encode](TryEncode::try_encode) succeeds,
/// [encode](Encode::encode) returns. Otherwise, [encode](Encode::encode) panics.
pub trait Encode<T>: TryEncode<T> {
    /// Encodes `value` into the CAN-bus data `data`.
    ///
    /// # Panics
    /// [encode](Encode::encode) panics if the call to [try_encode](TryEncode::try_encode) in the
    /// implementation returns the error variant.
    fn encode<D: CANWrite>(&self, data: &mut D, value: T) {
        match self.try_encode(data, value) {
            Ok(_) => (),
            Err(_) => panic!("cannot encode data"),
        }
    }
}
