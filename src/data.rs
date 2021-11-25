//! DATA MODULE


/// TRAIT
pub trait CANData {
    /// Return data
    fn data(&self) -> &[u8];

    /// Returns the length of the slize
    fn dlc(&self) -> usize;
}

impl CANData for Vec<u8> {
    ///
    /// # Example
    /// ```
    /// use cantools::prelude::CANData;
    /// let v = Vec::<u8>::new();
    /// assert_eq!(CANData::data(&v), &[]);
    /// ```
    ///
    fn data(&self) -> &[u8] {
        self.as_slice()
    }

    ///
    /// # Example
    /// ```
    /// use cantools::prelude::CANData;
    /// let v = Vec::<u8>::new();
    /// assert_eq!(CANData::dlc(&v), 0);
    /// ```
    ///
    fn dlc(&self) -> usize {
        self.len()
    }
}

impl CANData for &[u8] {
    /// ```
    /// use cantools::prelude::CANData;
    /// let v : [u8; 0] = [];
    /// assert_eq!(CANData::data(&v), &[]);
    /// ```
    fn data(&self) -> &[u8] {
        self
    }

    /// # Example
    /// ```
    /// use cantools::prelude::CANData;
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