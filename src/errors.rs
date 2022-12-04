// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

//! The crate-wide errors definition.

use std::io;

use derive_more::Display;
use rppal::gpio;
use rppal::uart;

/// The custom `raestro` error type.
///
/// Contains all custom error variants, as well as
/// a wrapper for the `std::io::Error` enum. All
/// `rppal` errors are unwrapped and
/// converted into their underlying
/// `std::io::Error` instances, and then wrapped
/// in the `Error::Io` variant.
#[derive(Display, Debug)]
pub enum Error {
    /// ### Purpose:
    /// The `maestro` instance is in an
    /// `uninitialized` state.
    #[display(fmt = "Maestro struct is uninitialized.")]
    Uninitialized,

    /// ### Purpose:
    /// An invalid value was passed in as a parameter. Mainly used when an
    /// incorrect microsecond target was passed into `set_target`.
    #[display(
        fmt = "Target must be between 3968 quarter-us (992us) and 8000 quarter-us (2000us) but {} quarter-us was used.",
        _0
    )]
    InvalidValue(u16),

    /// ### Purpose:
    /// Occurs when the expected number of bytes
    /// received from the Maestro board does not
    /// equal [`crate::maestro::internals::RESPONSE_SIZE`].
    #[display(
        fmt = "2 bytes were expected to be read but only {} byte(s) were actually read.",
        actual_count
    )]
    FaultyRead {
        /// ### Purpose:
        /// The number of bytes actually read.
        actual_count: usize,
    },

    /// ### Purpose:
    /// Occurs when the expected number of bytes
    /// written to the Maestro were incorrect
    /// (according to the protocol being sent).
    #[display(
        fmt = "{} bytes were expected to be written, but only {} byte(s) were actually written.",
        actual_count,
        expected_count
    )]
    FaultyWrite {
        /// ### Purpose:
        /// Number of bytes actually written.
        actual_count: usize,

        /// ### Purpose:
        /// Number of bytes expected to be written.
        expected_count: usize,
    },

    /// ### Purpose:
    /// Any [`std::io::Error`] encountered.
    #[display(fmt = "{}", _0)]
    Io(io::Error),
}

#[doc(hidden)]
impl Error {
    /// ### Purpose:
    /// Constructs a [`std::io::Error`] from the given parameters.
    fn new_io_error<E>(err_kind: io::ErrorKind, err_msg: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::Io(io::Error::new(err_kind, err_msg))
    }
}

impl std::error::Error for Error {}

impl From<uart::Error> for Error {
    fn from(uart_error: uart::Error) -> Self {
        match uart_error {
            uart::Error::Io(err) => Self::Io(err),
            uart::Error::Gpio(gpio_err) => match gpio_err {
                gpio::Error::UnknownModel => {
                    Self::new_io_error(io::ErrorKind::Other, "Unknown model.")
                },
                gpio::Error::PinNotAvailable(pin) => Self::new_io_error(
                    io::ErrorKind::AddrNotAvailable,
                    format!("Pin number {} is not available.", pin),
                ),
                gpio::Error::PermissionDenied(err_string) => {
                    Self::new_io_error(
                        io::ErrorKind::PermissionDenied,
                        format!("Permission denied: {}.", err_string),
                    )
                },
                gpio::Error::Io(err) => Self::Io(err),
                gpio::Error::ThreadPanic => {
                    Self::new_io_error(io::ErrorKind::Other, "Thread panic.")
                },
                gpio::Error::PinUsed(_) => Self::new_io_error(
                    io::ErrorKind::AddrInUse,
                    "That pin is already being used.",
                ),
            },
            uart::Error::InvalidValue => {
                Self::new_io_error(io::ErrorKind::Other, "Invalid value.")
            },
        }
    }
}
