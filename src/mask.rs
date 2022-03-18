/// Trait provides utility functions that enable a convenient
/// construction of bit masks.
pub trait Mask {
    /// Creates a mask where the least-significant `length` number of bits
    /// are set to 1 and left-shifted by `shift.`
    ///
    /// # Example
    /// ```
    /// use cantools::mask::Mask;
    /// let value: u8 = Mask::mask(4, 4);
    /// assert_eq!(value, 0xF0);
    /// ```
    fn mask(length: u16, shift: u16) -> Self;

    /// Sets the bits inside of `bits` to 1.
    ///
    /// # Example
    /// ```
    /// use cantools::mask::Mask;
    /// let value: u8 = Mask::bit_mask(&[7,4,3,0]);
    /// assert_eq!(value, 0b10011001);
    /// ```
    fn bit_mask(bits: &[u16]) -> Self;
    /// Sets every bit to 1.
    ///
    /// # Example
    /// ```
    /// use cantools::mask::Mask;
    /// let value: u8 = Mask::full_mask();
    /// assert_eq!(value, 0b1111_1111);
    /// ```
    fn full_mask() -> Self;
}

impl Mask for u8 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        0xFFu8
    }
}

impl Mask for u16 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        0xFF_FFu16
    }
}

impl Mask for u32 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        0xFF_FF_FF_FFu32
    }
}

impl Mask for u64 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        0xFF_FF_FF_FF_FF_FF_FF_FFu64
    }
}

impl Mask for i8 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        -1i8
    }
}

impl Mask for i16 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        -1i16
    }
}

impl Mask for i32 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        -1i32
    }
}

impl Mask for i64 {
    fn mask(length: u16, shift: u16) -> Self {
        let mut result = 0;
        for _ in 0..(length-1) {
            result += 1;
            result <<= 1;
        }
        result += 1;
        result <<= shift;
        result
    }

    fn bit_mask(bits: &[u16]) -> Self {
        let mut result = 0;
        for bit in bits {
            result |= 1 << bit;
        }
        result
    }

    fn full_mask() -> Self {
        -1i64
    }
}


#[cfg(test)]
mod tests {
    use super::Mask;

    #[test]
    fn test_mask_u8_001() {
        let value: u8 = Mask::mask(4, 0);
        assert_eq!(value, 0x0Fu8);
    }

    #[test]
    fn test_mask_u8_002() {
        let value: u8 = Mask::mask(4, 4);
        assert_eq!(value, 0xF0u8);
    }

    #[test]
    fn test_mask_u8_003() {
        let value: u8 = Mask::mask(1, 7);
        assert_eq!(value, 0b1000_0000u8);
    }

    #[test]
    fn test_mask_u8_004() {
        let value: u8 = Mask::bit_mask(&[1,2,3,4]);
        assert_eq!(value, 0b0001_1110u8);
    }

    #[test]
    fn test_mask_u8_005() {
        let value: u16 = Mask::full_mask();
        assert_eq!(value, 0xFFFF);
    }

    // u16
    #[test]
    fn test_mask_u16_001() {
        let value: u16 = Mask::mask(8,8);
        assert_eq!(value, 0xFF00);
    }

    #[test]
    fn test_mask_u16_002() {
        let value: u16 = Mask::mask(8,4);
        assert_eq!(value, 0x0FF0);
    }

    #[test]
    fn test_mask_u16_003() {
        let value: u16 = Mask::mask(1,15);
        assert_eq!(value, 0b1000_0000_0000_0000u16);
    }

    #[test]
    fn test_mask_u16_004() {
        let value: u16 = Mask::full_mask();
        assert_eq!(value, 0xFFFF);
    }

    #[test]
    fn test_mask_u16_005() {
        let value1: u16 = Mask::mask(4, 12);
        let value2: u16 = Mask::bit_mask(&[15,14,13,12]);
        assert_eq!(&value1, &value2);
        assert_eq!(value1, 0xF0_00);
        assert_eq!(value2, 0xF0_00);
    }

     // u32
    #[test]
    fn test_mask_u32_001() {
        let value: u32 = Mask::mask(16,8);
        assert_eq!(value, 0x00_FF_FF_00);
    }

    #[test]
    fn test_mask_u32_002() {
        let value: u32 = Mask::mask(16,4);
        assert_eq!(value, 0x00_0F_FF_F0);
    }

    #[test]
    fn test_mask_u32_003() {
        let value: u32 = Mask::mask(1,15);
        assert_eq!(value, 0b1000_0000_0000_0000u32);
    }

    #[test]
    fn test_mask_u32_004() {
        let value: u32 = Mask::full_mask();
        assert_eq!(value, 0xFF_FF_FF_FF);
    }

    #[test]
    fn test_mask_u32_005() {
        let value1: u32 = Mask::mask(32,0);
        let value2: u32 = Mask::full_mask();
        assert_eq!(value1, value2);
    }

    #[test]
    fn test_mask_u32_006() {
        let value1: u32 = Mask::mask(4, 28);
        let value2: u32 = Mask::bit_mask(&[31,30,29,28]);
        assert_eq!(&value1, &value2);
        assert_eq!(value1, 0xF0_00_00_00);
        assert_eq!(value2, 0xF0_00_00_00);
    }

     // u64
    #[test]
    fn test_mask_u64_001() {
        let value: u64 = Mask::mask(16,8);
        assert_eq!(value, 0x00_FF_FF_00);
    }

    #[test]
    fn test_mask_u64_002() {
        let value: u64 = Mask::mask(16,4);
        assert_eq!(value, 0x00_0F_FF_F0);
    }

    #[test]
    fn test_mask_u64_003() {
        let value: u64 = Mask::mask(1,15);
        assert_eq!(value, 0b1000_0000_0000_0000);
    }

    #[test]
    fn test_mask_u64_004() {
        let value: u64 = Mask::full_mask();
        assert_eq!(value, 0xFF_FF_FF_FF_FF_FF_FF_FF);
    }

    #[test]
    fn test_mask_u64_005() {
        let value1: u64 = Mask::mask(64,0);
        let value2: u64 = Mask::full_mask();
        assert_eq!(value1, value2);
    }

    #[test]
    fn test_mask_u64_006() {
        let value1: u64 = Mask::mask(4, 60);
        let value2: u64 = Mask::bit_mask(&[63,62,61,60]);
        assert_eq!(&value1, &value2);
        assert_eq!(value1, 0xF0_00_00_00_00_00_00_00);
        assert_eq!(value2, 0xF0_00_00_00_00_00_00_00);
    }

    // i8
    #[test]
    fn test_mask_i8_001() {
        let value: i8 = Mask::mask(4, 0);
        assert_eq!(value, 0x0Fi8);
    }

    #[test]
    fn test_mask_i8_002() {
        let value: i8 = Mask::mask(1, 7);
        assert_eq!(value, -128);
    }

    #[test]
    fn test_mask_i8_003() {
        let value: i8 = Mask::bit_mask(&[1,2,3,4]);
        assert_eq!(value, 0b0001_1110i8);
    }

    #[test]
    fn test_mask_i8_004() {
        let value: i8 = Mask::full_mask();
        assert_eq!(value, -1);
    }

    // i16
    #[test]
    fn test_mask_i16_001() {
        let value: i16 = Mask::mask(4, 0);
        assert_eq!(value, 0x0F);
    }

    #[test]
    fn test_mask_i16_002() {
        let value1: i16 = Mask::mask(4,12);
        let value2: i16 = Mask::bit_mask(&[15,14,13,12]);
        assert_eq!(&value1, &value2);
    }

    #[test]
    fn test_mask_i16_003() {
        let value: i8 = Mask::full_mask();
        assert_eq!(value, -1);
    }

    #[test]
    fn test_mask_i16_004() {
        let value1: i16 = Mask::mask(16,0);
        let value2: i16 = Mask::full_mask();
        assert_eq!(&value1, &value2);
        assert_eq!(value1, -1);
        assert_eq!(value2, -1);
    }

    #[test]
    fn test_mask_i16_005() {
        let value: i16 = Mask::mask(1, 15);
        assert_eq!(value, i16::MIN);
    }

    // i32
    #[test]
    fn test_mask_i32_001() {
        let value: i32 = Mask::mask(4, 0);
        assert_eq!(value, 0x0F);
    }

    #[test]
    fn test_mask_i32_002() {
        let value1: i32 = Mask::mask(4,28);
        let value2: i32 = Mask::bit_mask(&[31,30,29,28]);
        assert_eq!(&value1, &value2);
    }

    #[test]
    fn test_mask_i32_003() {
        let value: i32 = Mask::full_mask();
        assert_eq!(value, -1);
    }

    #[test]
    fn test_mask_i32_004() {
        let value1: i32 = Mask::mask(32,0);
        let value2: i32 = Mask::full_mask();
        assert_eq!(&value1, &value2);
        assert_eq!(value1, -1);
        assert_eq!(value2, -1);
    }

    #[test]
    fn test_mask_i32_005() {
        let value: i32 = Mask::mask(1, 31);
        assert_eq!(value, i32::MIN);
    }

    // i64
    #[test]
    fn test_mask_i64_001() {
        let value: i64 = Mask::mask(4, 0);
        assert_eq!(value, 0x0F);
    }

    #[test]
    fn test_mask_i64_002() {
        let value1: i64 = Mask::mask(4,60);
        let value2: i64 = Mask::bit_mask(&[63,62,61,60]);
        assert_eq!(&value1, &value2);
    }

    #[test]
    fn test_mask_i64_003() {
        let value: i64 = Mask::full_mask();
        assert_eq!(value, -1);
    }

    #[test]
    fn test_mask_i64_004() {
        let value1: i64 = Mask::mask(64,0);
        let value2: i64 = Mask::full_mask();
        assert_eq!(&value1, &value2);
        assert_eq!(value1, -1);
        assert_eq!(value2, -1);
    }

    #[test]
    fn test_mask_i64_005() {
        let value: i64 = Mask::mask(1, 63);
        assert_eq!(value, i64::MIN);
    }
}
