use crate::data::CANData;

pub trait Decode {
    type Output;

    fn decode<D: CANData>(&self, data: &D) -> Self::Output {
        todo!()
    }
}

pub enum DecodeError {
    NotEnoughData
}

pub trait TryDecode {
    type Output;

    fn decode<D: CANData>(&self, data: &D) -> Result<Self::Output, ()> {
        todo!()
    }
}