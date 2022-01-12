use crate::data::CANData;

pub trait TryDecode<T> {
    type Error;

    fn try_decode<D: CANData>(&self, _data: &D) -> Result<T, Self::Error>;
}


