// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

//! All constants defined and used by the Maestro
//! for `UART` communication through the
//! Pololu-Protocol.
//!
//! The specifics on the Pololu-Protoco, as well
//! as the overall serial communication with the
//! Maestro, can be found in the Pololu
//! Micro-Maestro manual, located
//! [here](https://www.pololu.com/docs/pdf/0J40/maestro.pdf)
//! in section 5.e.

use std::vec::Vec;

pub(crate) const SYNC: u8 = 0xaau8;
pub(crate) const DEVICE_NUMBER: u8 = 0x0cu8;
pub(crate) const DATA_BITS: u8 = 8u8;
pub(crate) const STOP_BITS: u8 = 1u8;
pub(crate) const MIN_WRITE_LENGTH: usize = 3usize;
pub(crate) const RESPONSE_SIZE: u8 = 2u8;
pub(crate) const DATA_MULTIPLIER: usize = 2usize;

/// The minimum PWM that can be sent to any
/// channel by the Maestro.
///
/// All values below `MIN_PWM` being used as
/// parameters to `set_target` will result in an
/// error.
pub const MIN_PWM: u16 = 992u16;

/// The maximum PWM that can be sent to any
/// channel by the Maestro.
///
/// All values above `MAX_PWM` being used as
/// parameters to `set_target` will result in an
/// error.
pub const MAX_PWM: u16 = 2000u16;

/// All available command flags supported by the
/// Pololu-Protocol.
///
/// # TODO
/// Review existing docs for this enum and add
/// more iff necessary.
#[allow(non_camel_case_types, clippy::upper_case_acronym)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub(crate) enum CommandFlags {
    SET_TARGET = 0x84u8,
    SET_SPEED = 0x87u8,
    SET_ACCELERATION = 0x89u8,
    GET_POSITION = 0x90u8,
    GET_ERRORS = 0xA1u8,
    GO_HOME = 0xA2u8,
    STOP_SCRIPT = 0xA4u8,

    #[allow(unused)]
    RESTART_SCRIPT_AT_SUBROUTINE = 0xA7u8,

    #[allow(unused)]
    RESTART_SCRIPT_AT_SUBROUTINE_WITH_PARAMETER = 0xA8u8,

    #[allow(unused)]
    GET_SCRIPT_STATUS = 0xAEu8,
}

/// All available channels to send commands to.
///
/// # TODO
/// Review existing docs for this enum and add
/// more iff necessary.
#[allow(non_camel_case_types, clippy::upper_case_acronym)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
/// Note that not all baudrates have been
/// specified.
///
/// # TODO
/// Add all remaining baudrates to the enum
/// below.
///
/// Review existing docs for this enum and add
/// more iff necessary.
#[allow(non_camel_case_types, clippy::upper_case_acronym)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum BaudRates {
    #[allow(missing_docs)]
    BR_50 = 50u32,

    #[allow(missing_docs)]
    BR_115200 = 115200u32,
}

/// All available errors throwable by the Maestro.
///
/// For each Errors variant down below, the
/// documentation provided was taken directly
/// from section 4.e of the Pololu Micro Maestro
/// manual, which can be found
/// [here](https://www.pololu.com/docs/pdf/0J40/maestro.pdf).
#[allow(non_camel_case_types, clippy::upper_case_acronym)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum Errors {
    /// A hardware-level error that occurs when a
    /// byte’s stop bit is not detected at the
    /// expected place. This can occur if you are
    /// communicating at a baud rate that differs
    /// from the Maestro’s baud rate.
    SER_SIGNAL_ERR = 0u16,

    /// A hardware-level error that occurs when
    /// the UART’s internal buffer fills up. This
    /// should not occur during normal operation.
    SER_OVERRUN_ERR = 1u16,

    /// A firmware-level error that occurs when
    /// the firmware’s buffer for bytes received
    /// on the RX line is full and a byte from RX
    /// has been lost as a result. This error
    /// should not occur during normal operation.
    SER_BUFFER_FULL = 2u16,

    /// This error occurs when the Maestro is
    /// running in CRC-enabled mode and the cyclic
    /// redundancy check (CRC) byte at the end of
    /// the command packet does not match what the
    /// Maestro has computed as that packet’s CRC
    /// (Section 5.d). In such a case, the Maestro
    /// ignores the command packet and generates a
    /// CRC error.
    SER_CRC_ERR = 3u16,

    /// This error occurs when the Maestro
    /// receives an incorrectly formatted or
    /// nonsensical command packet. For example,
    /// if the command byte does not match a known
    /// command or an unfinished command packet is
    /// interrupted by another command packet,
    /// this error occurs.
    SER_PROTOCOL_ERR = 4u16,

    /// When the serial timeout is enabled, this
    /// error occurs whenever the timeout period
    /// has elapsed without the Maestro receiving
    /// any valid serial commands. This timeout
    /// error can be used to make the servos
    /// return to their home positions in the
    /// event that serial communication between
    /// the Maestro and its controller is
    /// disrupted.
    SER_TIMEOUT = 5u16,

    /// This error occurs when a bug in the user
    /// script has caused the stack to overflow or
    /// underflow. Any script command that
    /// modifies the stack has the potential to
    /// cause this error. The stack depth is 32 on
    /// the Micro Maestro and 126 on the Mini
    /// Maestros.
    SCRIPT_STACK_ERR = 6u16,

    /// This error occurs when a bug in the user
    /// script has caused the call stack to
    /// overflow or underflow. An overflow can
    /// occur if there are too many levels of
    /// nested subroutines, or a subroutine calls
    /// itself too many times. The call stack
    /// depth is 10 on the Micro Maestro and 126
    /// on the Mini Maestros. An underflow can
    /// occur when there is a return without a
    /// corresponding subroutine call. An
    /// underflow will occur if you run a
    /// subroutine using the “Restart Script at
    /// Subroutine” serial command and the
    /// subroutine terminates with a return
    /// command rather than a quit command or an
    /// infinite loop.
    SCRIPT_CALL_STACK_ERR = 7u16,

    /// This error occurs when a bug in the user
    /// script has caused the program counter (the
    /// address of the next instruction to be
    /// executed) to go out of bounds. This can
    /// happen if your program is not terminated
    /// by a quit, return, or infinite loop.
    SCRIPT_PC_ERR = 8u16,
}

impl Errors {
    /// Converts the `u16` returned from the
    /// Maestro that represents an error into a
    /// vector of Errors. Each bit-position in
    /// the `u16` represents a specific error, and
    /// whether or not the bit is set or cleared
    /// represents whether or not that specific
    /// error was encounted. There exist only
    /// 9 possible Maestro error, and as such,
    /// only the first 9 bits (bit 0 to bit 8) can
    /// be set in the `u16`. All other bits
    /// are ignored.
    pub fn into_errors(mut data: u16) -> Vec<Errors> {
        #[allow(unused)]
        const MASK: u16 = 0x0001u16;

        let mut vec = Vec::with_capacity(9usize);

        for i in 0u16..=8u16 {
            if (data & MASK) == MASK {
                vec.push(i.into());
            };

            data >>= 1usize;
        }

        vec
    }
}

impl From<u16> for Errors {
    /// Converts a raw `u16` into an `Errors`
    /// type.
    ///
    /// Given that the underlying `Errors` types
    /// are represented by `u16s`, the
    /// conversion should not result in any
    /// undefined or erroneous behaviour.
    fn from(data: u16) -> Self {
        if (data >= (Errors::SER_SIGNAL_ERR as u16))
            && (data <= (Errors::SCRIPT_PC_ERR as u16))
        {
            unsafe { std::mem::transmute(data) }
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod errors_test {
    use super::*;

    #[test]
    fn no_errors() -> () {
        let err = 0u16;
        let actual_vec = Errors::into_errors(err);

        assert_eq!(actual_vec.len(), 0usize);
    }

    #[test]
    fn ser_signal_error() -> () {
        let err = 1u16;
        let actual_vec = Errors::into_errors(err);

        assert_eq!(actual_vec.len(), 1usize);
        assert_eq!(actual_vec[0usize], Errors::SER_SIGNAL_ERR);
    }

    #[test]
    fn ser_overrun_error() -> () {
        let err = 2u16;
        let actual_vec = Errors::into_errors(err);

        assert_eq!(actual_vec.len(), 1usize);
        assert_eq!(actual_vec[0usize], Errors::SER_OVERRUN_ERR);
    }

    #[test]
    fn two_errors() -> () {
        let err = 3u16;
        let actual_vec = Errors::into_errors(err);

        assert_eq!(actual_vec.len(), 2usize);
        assert_eq!(actual_vec[0usize], Errors::SER_SIGNAL_ERR);
        assert_eq!(actual_vec[1usize], Errors::SER_OVERRUN_ERR);
    }

    #[test]
    fn invalid_err() -> () {
        let err = 0x0200u16;
        let actual_vec = Errors::into_errors(err);

        assert_eq!(actual_vec.len(), 0usize);
    }

    #[test]
    fn all_errors() -> () {
        let err = 0x01ffu16;
        let actual_vec = Errors::into_errors(err);

        assert_eq!(actual_vec.len(), 9usize);
        assert_eq!(actual_vec[0usize], Errors::SER_SIGNAL_ERR);
        assert_eq!(actual_vec[1usize], Errors::SER_OVERRUN_ERR);
        assert_eq!(actual_vec[2usize], Errors::SER_BUFFER_FULL);
        assert_eq!(actual_vec[3usize], Errors::SER_CRC_ERR);
        assert_eq!(actual_vec[4usize], Errors::SER_PROTOCOL_ERR);
        assert_eq!(actual_vec[5usize], Errors::SER_TIMEOUT);
        assert_eq!(actual_vec[6usize], Errors::SCRIPT_STACK_ERR);
        assert_eq!(actual_vec[7usize], Errors::SCRIPT_CALL_STACK_ERR);
        assert_eq!(actual_vec[8usize], Errors::SCRIPT_PC_ERR);
    }
}
