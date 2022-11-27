// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

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
