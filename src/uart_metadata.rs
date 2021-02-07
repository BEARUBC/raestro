#![allow(non_camel_case_types)]

#[repr(u32)]
pub enum BaudRate {
    BR_50 = 50u32,
    BR_115200 = 115200u32,
}

#[repr(u8)]
pub enum Channel {
    C_0 = 0x0u8,
    C_1 = 0x1u8,
    C_2 = 0x2u8,
    C_3 = 0x3u8,
    C_4 = 0x4u8,
    C_5 = 0x5u8,
}
