//! Module providing utility traits for extracting, i.e., decoding CAN-bus data.
//!
//! The module provides three traits: [TryDecode], [DefaultDecode], and [Decode].

use crate::data::CANData;

///
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    ///
    NotEnoughData
}

///
pub trait TryDecode<T> {
    ///
    type Error;

    ///
    fn try_decode<D: CANData>(&self, data: &D) -> Result<T, Self::Error>;
}

pub trait DefaultDecode<T: Default>: TryDecode<T> {
    fn default_decode<D: CANData>(&self, data: &D) -> T {
        match self.try_decode(data) {
            Ok(value) => value,
            Err(_) => T::default()
        }
    }
}

pub trait Decode<T> : TryDecode<T> {
    fn decode<D: CANData>(&self, data: &D) -> T {
        match self.try_decode(data) {
            Ok(value) => value,
            Err(_) => panic!("cannot decode data")
        }
    }
}
