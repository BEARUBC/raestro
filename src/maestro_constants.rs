/* external uses */

/* internal mods */

/* internal uses */

#[allow(non_camel_case_types)]
#[repr(u8)]
pub(crate) enum ProtocolMetadata {
    SYNC = 0xaau8,
    DEVICE_NUMBER = 0x0cu8,
}

#[allow(non_camel_case_types, unused)]
#[repr(u8)]
#[derive(Clone)]
pub(crate) enum CommandFlags {
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

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum Channels {
    C_0 = 0x0u8,
    C_1 = 0x1u8,
    C_2 = 0x2u8,
    C_3 = 0x3u8,
    C_4 = 0x4u8,
    C_5 = 0x5u8,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum BaudRates {
    BR_50 = 50u32,
    BR_115200 = 115200u32,
}

#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum Errors {
    SER_SIGNAL_ERR = 0u16,
    SER_OVERRUN_ERR = 1u16,
    SER_BUFFER_FULL = 2u16,
    SER_CRC_ERR = 3u16,
    SER_PROTOCOL_ERR = 4u16,
    SER_TIMEOUT = 5u16,
    SCRIPT_STACK_ERR = 6u16,
    SCRIPT_CALL_STACK_ERR = 7u16,
    SCRIPT_PC_ERR = 8u16,
}