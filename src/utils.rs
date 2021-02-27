#[inline]
pub(crate) fn mask_byte(mut byte: u8) -> u8 {
    byte &= 0x7fu8;
    return byte;
}

pub(crate) fn short_to_target(mut microsec: u16) -> (u8, u8) {
    let multiplier: u8 = 2u8;
    let down_shift: u8 = 7u8;

    microsec <<= multiplier;

    let lower: u8 = mask_byte(microsec as u8);
    let upper: u8 = mask_byte((microsec >> down_shift) as u8);

    return (lower, upper);
}
