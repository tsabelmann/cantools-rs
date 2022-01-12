/// Defines the two endiannesses that are considered for decoding and encoding.
#[derive(Debug,PartialEq)]
pub enum Endian {
    /// Defines data to be in little-endian byte-order.
    Little,
    /// Defines data to be in big-endian byte-order.
    Big
}

impl Default for Endian {
    fn default() -> Self {
        Endian::Little
    }
}