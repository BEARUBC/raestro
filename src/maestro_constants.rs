#![allow(non_camel_case_types)]

#[repr(u8)]
pub(crate) enum ProtocolMetaData {
    SYNC = 0xaau8,
    DEVICE_NUMBER = 0x0cu8,
}

#[repr(u8)]
pub(crate) enum Commands {
    SET_TARGET = 0x84u8,
    SET_SPEED = 0x87u8,
    SET_ACCELERATION = 0x89u8,
    GET_POSITION = 0x90u8,
    GET_ERRORS = 0xA1u8,
    GO_HOME = 0xA2u8,
    STOP_SCRIPT = 0xA4u8,
    RESTART_SCRIPT_AT_SUBROUTINE = 0xA7u8,
    RESTART_SCRIPT_AT_SUBROUTINE_WITH_PARAMETER = 0xA8u8,
    GET_SCRIPT_STATUS = 0xAEu8,
}

#[repr(u8)]
pub enum Channels {
    C_0 = 0x0u8,
    C_1 = 0x1u8,
    C_2 = 0x2u8,
    C_3 = 0x3u8,
    C_4 = 0x4u8,
    C_5 = 0x5u8,
}

#[repr(u32)]
pub enum BaudRates {
    BR_50 = 50u32,
    BR_115200 = 115200u32,
}
