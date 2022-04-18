//! Module providing utility traits for decoding CAN-bus data.
//!
//! The module provides three traits: [TryDecode], [DefaultDecode], and [Decode].
//! [try_decode](TryDecode::try_decode) models the possibility of the decoding to fail.
//! [default_decode](DefaultDecode::default_decode) returns a default value if the decoding fails.
//! Finally, [decode](Decode::decode) panics if the internal decoding fails. Otherwise, it returns
//! the decoded value.
//!
//! # Example
//! ```
//! use cantools::data::CANRead;
//! use cantools::signals::Bit;
//! use cantools::decode::{TryDecode, DefaultDecode, Decode};
//!
//! let bit = Bit::new(20);
//! let mut data = [1u8, 2u8, 3u8, 4u8];
//!
//! let result_1 = bit.try_decode(&data);
//! let result_2 = bit.default_decode(&data);
//! let result_3 = bit.decode(&data);
//! ```

use crate::data::CANRead;

/// Type representing possible decoding errors.
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    /// There is not enough byte data available from which one can decode a value.
    NotEnoughData,
}

/// A trait modeling the failable decoding of data.
pub trait TryDecode<T> {
    /// A type modeling the different possible failures of the decoding.
    type Error;

    /// Tries to decode a value.
    fn try_decode<D: CANRead>(&self, data: &D) -> Result<T, Self::Error>;
}

/// A trait modeling the failable decoding of data.
pub trait DefaultDecode<T: Default>: TryDecode<T> {
    /// Tries to decode a value. If the data is not decodable, a default value is returned.
    /// Otherwise, the decoded value is returned.
    fn default_decode<D: CANRead>(&self, data: &D) -> T {
        match self.try_decode(data) {
            Ok(value) => value,
            Err(_) => T::default(),
        }
    }
}

/// A trait modeling the not failable decoding of data.
pub trait Decode<T>: TryDecode<T> {
    /// Decodes the data.
    ///
    /// # Panics
    /// If the data is not decodable internally, the call to [decode](Decode::decode) panics.
    fn decode<D: CANRead>(&self, data: &D) -> T {
        match self.try_decode(data) {
            Ok(value) => value,
            Err(_) => panic!("cannot decode data"),
        }
    }
}
