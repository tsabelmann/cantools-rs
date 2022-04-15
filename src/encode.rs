//!
//!
//!

use crate::data::CANWrite;

///
#[derive(Debug, PartialEq)]
pub enum EncodeError {
    ///
    UnavailableByte(u8)
}

///
pub trait TryEncode<T> {
    ///
    fn try_encode<D: CANWrite>(&self, data: &mut D, value: T) -> Result<(), EncodeError>;
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
