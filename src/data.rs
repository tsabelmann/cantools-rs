//! Module providing convenient access to the underlying CAN-bus data, either read-only
//! (see [CANData]), or read and write (see [CANWrite]).
//!
//! The [CANData] trait provides methods and retrieve the [DLC](CANData::dlc), the number of
//! available bytes, and a [data](CANData::data) slice that is read-only. In theory, only the
//! [data](CANData::data) slice is needed since one can retrieve the DLC from the slice as well.
//!
//! The [CANWrite] trait provides additional methods. The [mut_data](CANWrite::mut_data) method
//! allows for mutating the slice. [set_and_exchange](CANWrite::set_and_exchange) and
//! [set](CANWrite::set) allow for safe and unsafe ways to change specific bytes.

/// A trait providing methods for accessing the underlying bytes of some CAN-bus data.
pub trait CANData {
    /// Returns a slice representing the accessible bytes.
    fn data(&self) -> &[u8];

    /// Returns the number of accessible bytes.
    fn dlc(&self) -> usize;
}

/// Type representing failure in the process of writing CAN-bus data.
#[derive(Debug, PartialEq)]
pub enum CANWriteError {
    /// The byte at some index is not available or not existent.
    UnavailableByte(u8)
}

/// A trait providing methods for accessing the underlying data in a mutable fashion.
pub trait CANWrite: CANData {
    /// Returns a mutable slice representing the mutable data.
    fn mut_data(&mut self) -> &mut [u8];

    /// Retrieved the byte and `index` ans sets it to the value `value`. If the byte at `index` is
    /// unavailable, a [CANWriteError] is returned.
    fn set_and_exchange(&mut self, value: u8, index: u8) -> Result<u8, CANWriteError>;

    /// Retrieved the byte and `index` and sets it to the value `value`. Uses
    /// [set_and_exchange](CANWrite::set_and_exchange) underneath.
    ///
    /// # Panics
    /// If the byte at `index` is unavailable or non existent, the method panics.
    fn set(&mut self, value: u8, index: u8) -> u8 {
        self.set_and_exchange(value, index).unwrap()
    }
}

impl CANData for Vec<u8> {
    /// # Example
    /// ```
    /// use cantools::data::CANData;
    /// let v = Vec::<u8>::new();
    /// assert_eq!(CANData::data(&v), &[]);
    /// ```
    fn data(&self) -> &[u8] {
        self.as_slice()
    }

    /// # Example
    /// ```
    /// use cantools::data::CANData;
    /// let v = Vec::<u8>::new();
    /// assert_eq!(CANData::dlc(&v), 0);
    /// ```
    fn dlc(&self) -> usize {
        self.len()
    }
}

impl CANData for &[u8] {
    /// ```
    /// use cantools::data::CANData;
    /// let v : [u8; 0] = [];
    /// assert_eq!(CANData::data(&v), &[]);
    /// ```
    fn data(&self) -> &[u8] {
        self
    }

    /// # Example
    /// ```
    /// use cantools::data::CANData;
    /// let v : [u8; 0] = [];
    /// assert_eq!(CANData::dlc(&v), 0);
    /// ```
    fn dlc(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> CANData for [u8; N] {
    fn data(&self) -> &[u8] {
        self.as_ref()
    }

    fn dlc(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::CANData;

    #[test]
    fn test_001() {
        for i in 0..8 {
            let mut v = Vec::new();
            for j in 0..i {
                v.push(j);
            }
            assert_eq!(CANData::dlc(&v), i as usize);
        }
    }

    #[test]
    fn test_002() {
        for i in 0..8 {
            let mut v = Vec::new();
            for j in 0..i {
                v.push(j);
            }
            assert_eq!(CANData::data(&v), v.as_slice());
        }
    }
}