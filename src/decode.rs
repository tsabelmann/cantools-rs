use crate::data::CANData;

pub trait TryDecode<T> {
    type Error;

    fn try_decode<D: CANData>(&self, data: &D) -> Result<T, Self::Error>;
}

pub trait Decode<T> : TryDecode<T> {
    type Error;

    fn decode<D: CANData>(&self, data: &D) -> T {
        match self.try_decode(data) {
            Ok(value) => {
                value
            },
            Err(_) => {
                panic!("cannot decode data using");
            }
        }
    }
}


