// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */
use std::{
    error::Error as StdError,
    io::{
        Error as IoError,
        ErrorKind as IoErrorKind,
    },
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
};
use rppal::{
    uart::Error as UartError,
    gpio::Error as GpioError,
};

/* internal mods */

/* internal uses */

/// The custom `raestro` error type.
///
/// Contains all custom error variants, as well as a wrapper for the `std::io::Error` enum.
/// All `rppal` errors are unwrapped and converted into their underlying `std::io::Error` instances,
/// and then wrapped in the `Error::Io` variant.
#[derive(Debug)]
pub enum Error {

    /// The `maestro` instance is in an `uninitialized` state.
    ///
    /// Consider calling `Maestro::start` with a corresponding baudrate to transition this instance
    /// into the `initialized` state.
    Uninitialized,

    /// An invalid value was passed in as a parameter.
    ///
    /// Mainly used when an incorrect microsecond target was passed into `set_target`.
    InvalidValue(u16),

    /// Occurs when the expected number of bytes received from the Maestro do not equal `RESPONSE_SIZE`.
    ///
    /// `FaultyRead.0` is the number of bytes actually read.
    FaultyRead(usize),

    /// Occurs when the expected number of bytes written to the Maestro were incorrect (according to the protocol being sent).
    FaultyWrite {
        actual_count: usize,
        expected_count: usize,
    },

    /// Remaining IO errors as encountered by the `rppal` library, or something else.
    Io(IoError),
}

#[doc(hidden)]
impl Error {

    /// Constructs a `std::io::Error` from the given parameters
    pub(crate) fn new_io_error<E>(err_kind: IoErrorKind, err_msg: E) -> Self
    where
    E: Into<Box<dyn StdError + Send + Sync>>, {
        return Error::Io(IoError::new(err_kind, err_msg));
    }
}

impl StdError for Error {}

impl Display for Error {

    /// Formatting for `raestro::Error`.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        return match self {
            Error::Uninitialized => write!(f, "maestro struct is uninitialized; please consider calling .start() on the instance first"),
            Error::InvalidValue(value) => write!(f, "microsec values must be between 992us and 2000us but {} was used", value),
            Error::FaultyRead { actual_count, expected_count } => write!(f, "{} were expected to be read, but only {} were actually read", expected_count, actual_count),
            Error::FaultyWrite { actual_count, expected_count } => write!(f, "{} were expected to be written, but only {} were actually written", expected_count, actual_count),
            Error::Io(io_error) => io_error.fmt(f),
        };
    }
}

impl From<IoError> for Error {

    /// Wraps a `std::io::Error` in the `raestro::Error::Io` variant.
    fn from(io_error: IoError) -> Self {
        return Self::Io(io_error);
    }
}

impl From<UartError> for Error {

    /// Used to convert from an `rppal::uart::Error` type into the `raestro::Error`.
    fn from(uart_error: UartError) -> Self {
        return match uart_error {
            UartError::Io(std_err) => Error::from(std_err),
            UartError::Gpio(gpio_err) => match gpio_err {
                GpioError::UnknownModel => Error::new_io_error(IoErrorKind::Other, "unknown model"),
                GpioError::PinNotAvailable(pin) => Error::new_io_error(IoErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin)),
                GpioError::PermissionDenied(err_string) => Error::new_io_error(IoErrorKind::PermissionDenied, format!("permission denied: {} ", err_string)),
                GpioError::Io(error) => Error::from(error),
                GpioError::ThreadPanic => Error::new_io_error(IoErrorKind::Other, "thread panic"),
            },
            UartError::InvalidValue => Error::new_io_error(IoErrorKind::Other, "invalid value"),
        };
    }
}
