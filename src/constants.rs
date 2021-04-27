// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! All constants defined and used by the Maestro for `UART` communication
//! through the Pololu-Protocol.
//!
//! The specifics on the Pololu-Protoco, as well as the overall serial communication
//! with the Maestro, can be found in the Pololu Micro-Maestro manual, located [here](https://www.pololu.com/docs/pdf/0J40/maestro.pdf)
//! in section 5.e.

/* external uses */

/* internal mods */

/* internal uses */

pub(crate) const SYNC: u8 = 0xaau8;
pub(crate) const DEVICE_NUMBER: u8 = 0x0cu8;
pub(crate) const DATA_BITS: u8 = 8u8;
pub(crate) const STOP_BITS: u8 = 1u8;
pub(crate) const MIN_WRITE_LENGTH: usize = 3usize;
pub(crate) const RESPONSE_SIZE: u8 = 2u8;
pub(crate) const DATA_MULTIPLIER: usize = 2usize;

/// The minimum PWM that can be sent to any channel by the Maestro.
///
/// All values below `MIN_PWM` being used as parameters to `set_target` will result in an error.
pub const MIN_PWM: u16 = 992u16;

/// The maximum PWM that can be sent to any channel by the Maestro.
///
/// All values above `MAX_PWM` being used as parameters to `set_target` will result in an error.
pub const MAX_PWM: u16 = 2000u16;

/// All available command flags supported by the Pololu-Protocol.
///
/// # TODO
/// Review existing docs for this enum and add more iff necessary.
#[allow(non_camel_case_types, unused)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
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

/// All available channels to send commands to.
///
/// # TODO
/// Review existing docs for this enum and add more iff necessary.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Channels {
    #[allow(missing_docs)]
    C_0 = 0x0u8,

    #[allow(missing_docs)]
    C_1 = 0x1u8,

    #[allow(missing_docs)]
    C_2 = 0x2u8,

    #[allow(missing_docs)]
    C_3 = 0x3u8,

    #[allow(missing_docs)]
    C_4 = 0x4u8,

    #[allow(missing_docs)]
    C_5 = 0x5u8,
}

/// Available baudrates supported by the Maestro.
///
/// Note that not all baudrates have been specified.
/// # TODO
/// Add all remaining baudrates to the enum below.
///
/// Review existing docs for this enum and add more iff necessary.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum BaudRates {
    #[allow(missing_docs)]
    BR_50 = 50u32,

    #[allow(missing_docs)]
    BR_115200 = 115200u32,
}

/// All available errors throwable by the Maestro.
///
/// # TODO
/// Review existing docs for this enum and add more iff necessary.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
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

impl From<u16> for Errors {

    /// Converts a raw `u16` into an `Errors` type.
    ///
    /// Given that the underlying `Errors` types are represented by `u16s`, the conversion should not
    /// result in any undefined or erroneous behaviour.
    fn from(data: u16) -> Self {
        return if (data >= (Errors::SER_SIGNAL_ERR as u16)) && (data <= (Errors::SCRIPT_PC_ERR as u16)) {
            unsafe { std::mem::transmute(data) }
        } else {
            panic!()
        };
    }
}
