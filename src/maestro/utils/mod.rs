// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

#[cfg(test)]
mod tests;

/// Given a `u8`, clears the top bit by applying a
/// mask to it.
pub(super) fn mask_byte(byte: u8) -> u8 {
    let top_mask: u8 = 0x7fu8;
    byte & top_mask
}

/// The Pololu-Protocol requires that the `u16` be
/// formatted in a specific manner before
/// embedding it in the protocol message and
/// sending it over `UART`.
///
/// Given a 16-bit integer, execute the following:
/// 1. take low order bits 0 to 6, pad with a 0 in
/// the 7th position. This is the lower byte. 2.
/// take upper order bits 7 to 13, shift it down 7
/// bits, pad with a 0 in the 7th position. This
/// is the higher byte.
///
/// # Note
/// This leaves the top 2 bits unused. This is as
/// is required by the Pololu-Protocol.
pub(super) fn microsec_to_target(microsec: u16) -> (u8, u8) {
    let down_shift = 7usize;

    let lower = mask_byte(microsec as u8);
    let upper = mask_byte((microsec >> down_shift) as u8);

    (lower, upper)
}
