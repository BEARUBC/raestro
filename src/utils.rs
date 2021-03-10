/* external crates */

/* external uses */

/* internal mods */

/* internal uses */

#[allow(unused)]
#[inline]
pub(crate) fn mask_byte(byte: u8) -> u8 {
    let top_mask: u8 = 0x7fu8;
    return byte & top_mask;
}

#[allow(unused)]
pub(crate) fn microsec_to_target(mut microsec: u16) -> (u8, u8) {
    let multiplier: u8 = 2u8;
    let down_shift: u8 = 7u8;

    microsec <<= multiplier;

    let lower: u8 = mask_byte(microsec as u8);
    let upper: u8 = mask_byte((microsec >> down_shift) as u8);

    return (lower, upper);
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn simple_mask_byte_test() -> () {
        let byte: u8 = 0x00u8;
        let expected_byte: u8 = 0x00u8;

        assert_eq!(mask_byte(byte), expected_byte);
    }

    #[test]
    fn medium_mask_byte_test() -> () {
        let byte: u8 = 0xffu8;
        let expected_byte: u8 = 0x7fu8;

        assert_eq!(mask_byte(byte), expected_byte);
    }

    #[test]
    fn complex_mask_byte_test() -> () {
        let byte: u8 = 0xa5u8;
        let expected_byte: u8 = 0x25u8;

        assert_eq!(mask_byte(byte), expected_byte);
    }

    #[test]
    fn simple_short_to_target_test() -> () {
        let short: u16 = 1500u16;
        let expected: (u8, u8) = (0x70u8, 0x2eu8);

        assert_eq!(microsec_to_target(short), expected);
    }
}
