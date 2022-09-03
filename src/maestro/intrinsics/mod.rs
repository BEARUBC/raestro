// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

//! Available public constants for the operation of the [`crate::maestro::Maestro`] struct.

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

/// The minimum PWM (in quarter us) that can be
/// sent to any channel by the Maestro.
///
/// All values below `MIN_QTR_PWM` being used as
/// parameters to `set_target` will result in an
/// error.
pub const MIN_QTR_PWM: u16 = 3968u16;

/// The maximum PWM (in quarter us) that can be
/// sent to any channel by the Maestro.
///
/// All values above `MAX_QTR_PWM` being used as
/// parameters to `set_target` will result in an
/// error.
pub const MAX_QTR_PWM: u16 = 8000u16;

/// ### Purpose:
/// All available channels to send commands to.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Channels {
    #[allow(missing_docs)]
    Channel0 = 0x0u8,

    #[allow(missing_docs)]
    Channel1 = 0x1u8,

    #[allow(missing_docs)]
    Channel2 = 0x2u8,

    #[allow(missing_docs)]
    Channel3 = 0x3u8,

    #[allow(missing_docs)]
    Channel4 = 0x4u8,

    #[allow(missing_docs)]
    Channel5 = 0x5u8,
}

/// ### Purpose:
/// Available baudrates supported by the Maestro.
///
/// ### Notes:
/// Note that not all baudrates have been specified. They will be added in a
/// future release.
#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[repr(u32)]
pub enum Baudrate {
    #[allow(missing_docs)]
    Baudrate50 = 50u32,

    #[allow(missing_docs)]
    Baudrate11520 = 115200u32,
}

/// ### Purpose:
/// All available errors throwable by the Maestro board.
///
/// ### Notes:
/// For each [`Errors`] variant down below, the
/// documentation provided was taken directly
/// from [Section 4.e of the Pololu Micro Maestro
/// manual](https://www.pololu.com/docs/pdf/0J40/maestro.pdf).
#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[repr(u16)]
pub enum MaestroError {
    /// A hardware-level error that occurs when a byte’s stop bit is not
    /// detected at the expected place. This can occur if you are communicating
    /// at a baud rate that differs from the Maestro’s baud rate.
    SerSignalError = 0u16,

    /// A hardware-level error that occurs when the UART’s internal buffer
    /// fills up. This should not occur during normal operation.
    SerOverrunError = 1u16,

    /// A firmware-level error that occurs when
    /// the firmware’s buffer for bytes received
    /// on the RX line is full and a byte from RX
    /// has been lost as a result. This error
    /// should not occur during normal operation.
    SerBufferFull = 2u16,

    /// This error occurs when the Maestro is
    /// running in CRC-enabled mode and the cyclic
    /// redundancy check (CRC) byte at the end of
    /// the command packet does not match what the
    /// Maestro has computed as that packet’s CRC
    /// (Section 5.d). In such a case, the Maestro
    /// ignores the command packet and generates a
    /// CRC error.
    SerCrcError = 3u16,

    /// This error occurs when the Maestro
    /// receives an incorrectly formatted or
    /// nonsensical command packet. For example,
    /// if the command byte does not match a known
    /// command or an unfinished command packet is
    /// interrupted by another command packet,
    /// this error occurs.
    SerProtocolError = 4u16,

    /// When the serial timeout is enabled, this
    /// error occurs whenever the timeout period
    /// has elapsed without the Maestro receiving
    /// any valid serial commands. This timeout
    /// error can be used to make the servos
    /// return to their home positions in the
    /// event that serial communication between
    /// the Maestro and its controller is
    /// disrupted.
    SerTimeout = 5u16,

    /// This error occurs when a bug in the user
    /// script has caused the stack to overflow or
    /// underflow. Any script command that
    /// modifies the stack has the potential to
    /// cause this error. The stack depth is 32 on
    /// the Micro Maestro and 126 on the Mini
    /// Maestros.
    ScriptStackError = 6u16,

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
    ScriptCallStackError = 7u16,

    /// This error occurs when a bug in the user
    /// script has caused the program counter (the
    /// address of the next instruction to be
    /// executed) to go out of bounds. This can
    /// happen if your program is not terminated
    /// by a quit, return, or infinite loop.
    ScriptPcError = 8u16,
}

impl MaestroError {
    /// ### Purpose:
    /// Converts the [`u16`] returned from the
    /// Maestro that represents an error into a
    /// vector of Errors.
    ///
    /// ### Notes:
    /// Each bit-position in the [`u16`] represents a specific error, and
    /// whether or not the bit is set or cleared represents whether or not
    /// that specific error was encountered. There exist only 9 possible
    /// Maestro errors, and as such, only the first 9 bits (bit-0 to bit-8)
    /// can be set in the [`u16`]. All other bits are ignored.
    pub fn from_data(data: u16) -> Vec<MaestroError> {
        const MASK: u16 = 0x0001u16;
        let (_, errors) = (0u16..=8u16).into_iter().fold(
            (data, vec![]),
            |(mut data, mut errors), index| {
                let masked_data = data & MASK;
                let comparison = masked_data.cmp(&MASK);
                if let Ordering::Equal = comparison {
                    let error = index.into();
                    errors.push(error);
                };
                data >>= 1;
                (data, errors)
            },
        );
        errors
    }
}

impl From<u16> for MaestroError {
    /// ### Purpose:
    /// Converts a raw [`u16`] into an [`Errors`] type.
    ///
    /// ### Notes:
    /// Given that the underlying [`Errors`] types
    /// are represented by `u16s`, the
    /// conversion should not result in any
    /// undefined or erroneous behaviour.
    fn from(data: u16) -> Self {
        let contained = ((MaestroError::SerSignalError as u16)
            ..=(MaestroError::ScriptPcError as u16))
            .contains(&data);
        match contained {
            true => unsafe { std::mem::transmute(data) },
            false => unreachable!(
                "The data should always be contained within the above"
            ),
        }
    }
}
