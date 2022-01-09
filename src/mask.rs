

pub trait Mask {
    fn mask(length: u16, shift: u16) -> Self;
    fn bit_mask(bits: &[u16]) -> Self;
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
        let value: u8 = Mask::full_mask();
        assert_eq!(value, 0xFF);
    }
}
