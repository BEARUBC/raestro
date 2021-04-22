// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */

/* internal mods */

/* internal uses */

#[inline]
pub(crate) fn mask_byte(byte: u8) -> u8 {
    let top_mask: u8 = 0x7fu8;
    return byte & top_mask;
}

pub(crate) fn microsec_to_target(microsec: u16) -> (u8, u8) {
    let down_shift = 7usize;

    let lower = mask_byte(microsec as u8);
    let upper = mask_byte((microsec >> down_shift) as u8);

    return (lower, upper);
}

#[cfg(test)]
mod tests {
    /* external uses */

    /* internal mods */

    /* internal uses */
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
        let target: u16 = 6000u16;
        let expected: (u8, u8) = (0x70u8, 0x2eu8);

        assert_eq!(microsec_to_target(target), expected);
    }
}
