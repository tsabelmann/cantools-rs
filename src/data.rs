//! Module providing utility traits for accessing and mutating CAN-bus data.
//!
//! The [CANRead] trait provides methods and retrieve the [DLC](CANRead::dlc), the number of
//! available bytes, and a [data](CANRead::data) slice that is read-only. In theory, only the
//! [data](CANRead::data) slice is needed since one can retrieve the DLC from the slice as well.
//!
//! The [CANWrite] trait provides one additional methods. The [mut_data](CANWrite::mut_data) method
//! allows for mutating the slice.

/// A trait providing methods for accessing the underlying bytes of some CAN-bus data.
pub trait CANRead {
    /// Returns a slice representing the accessible bytes.
    fn data(&self) -> &[u8];

    /// Returns the number of accessible bytes.
    fn dlc(&self) -> usize;
}

/// A trait providing methods for accessing the underlying data in a mutable fashion.
pub trait CANWrite: CANRead {
    /// Returns a mutable slice representing the mutable data.
    fn mut_data(&mut self) -> &mut [u8];
}

impl CANRead for Vec<u8> {
    /// # Example
    /// ```
    /// use cantools::data::CANRead;
    /// let v = Vec::<u8>::new();
    /// assert_eq!(CANRead::data(&v), &[]);
    /// ```
    fn data(&self) -> &[u8] {
        self.as_slice()
    }

    /// # Example
    /// ```
    /// use cantools::data::CANRead;
    /// let v = Vec::<u8>::new();
    /// assert_eq!(CANRead::dlc(&v), 0);
    /// ```
    fn dlc(&self) -> usize {
        self.len()
    }
}

impl CANWrite for Vec<u8> {
    fn mut_data(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl CANRead for &[u8] {
    /// ```
    /// use cantools::data::CANRead;
    /// let v : [u8; 0] = [];
    /// assert_eq!(CANRead::data(&v), &[]);
    /// ```
    fn data(&self) -> &[u8] {
        self
    }

    /// # Example
    /// ```
    /// use cantools::data::CANRead;
    /// let v : [u8; 0] = [];
    /// assert_eq!(CANRead::dlc(&v), 0);
    /// ```
    fn dlc(&self) -> usize {
        self.len()
    }
}

impl CANRead for &mut [u8] {
    fn data(&self) -> &[u8] {
        self
    }

    fn dlc(&self) -> usize {
        self.len()
    }
}

impl CANWrite for &mut [u8] {
    fn mut_data(&mut self) -> &mut [u8] {
        self
    }
}

impl<const N: usize> CANRead for [u8; N] {
    fn data(&self) -> &[u8] {
        self.as_ref()
    }

    fn dlc(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> CANWrite for [u8; N] {
    fn mut_data(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::CANRead;

    #[test]
    fn test_001() {
        for i in 0..8 {
            let mut v = Vec::new();
            for j in 0..i {
                v.push(j);
            }
            assert_eq!(CANRead::dlc(&v), i as usize);
        }
    }

    #[test]
    fn test_002() {
        for i in 0..8 {
            let mut v = Vec::new();
            for j in 0..i {
                v.push(j);
            }
            assert_eq!(CANRead::data(&v), v.as_slice());
        }
    }
}
