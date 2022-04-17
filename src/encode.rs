//!
//!
//!

use crate::data::CANWrite;

///
#[derive(Debug, PartialEq)]
pub enum EncodeError {
    ///
    NotEnoughData,
    ///
    MinError,
    ///
    MaxError
}

///
pub trait TryEncode<T> {
    ///
    type Error;
    ///
    fn try_encode<D: CANWrite>(&self, data: &mut D, value: T) -> Result<(), Self::Error>;
}

///
pub trait Encode<T>: TryEncode<T> {
    ///
    fn encode<D: CANWrite>(&self, data: &mut D, value: T) {
        match self.try_encode(data, value) {
            Ok(_) => (),
            Err(_) => panic!("cannot encode data")
        }
    }
}
