// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */

/* internal mods */
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

/* internal uses */

/// Given a `u8`, clears the top bit by applying a mask to it.
#[inline]
pub(crate) fn mask_byte(byte: u8) -> u8 {
    let top_mask: u8 = 0x7fu8;
    return byte & top_mask;
}

/// The Pololu-Protocol requires that the `u16` be formatted in a specific manner before embedding it
/// in the protocol message and sending it over `UART`.
///
/// Given a 16-bit integer, execute the following:
/// 1. take low order bits 0 to 6, pad with a 0 in the 7th position. This is the lower byte.
/// 2. take upper order bits 7 to 13, shift it down 7 bits, pad with a 0 in the 7th position. This is the higher byte.
///
/// # Note
/// This leaves the top 2 bits unused. This is as is required by the Pololu-Protocol.
pub(crate) fn microsec_to_target(microsec: u16) -> (u8, u8) {
    let down_shift = 7usize;

    let lower = mask_byte(microsec as u8);
    let upper = mask_byte((microsec >> down_shift) as u8);

    return (lower, upper);
}
