// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */
use std::{
    error::{
        Error as StdError,
    },
    io::{
        Error as IoError,
        ErrorKind as IoErrorKind,
    },
    fmt::{
        Display,
        Formatter,
        Result,
    },
};
use rppal::uart::Error as UartError;

/* internal mods */

/* internal uses */

#[derive(Debug)]
pub enum Error {
    Uninitialized,
    InvalidValue(u16),
    FaultyRead {
        actual_count: usize,
        expected_count: usize,
    },
    FaultyWrite {
        actual_count: usize,
        expected_count: usize,
    },
    Io(IoError),
}

impl Error {
    pub fn new_io_error() -> Self {
        todo!();
    }
}

impl StdError for Error {}

impl Display for Error {
    #[allow(unused)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!();
    }
}

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Self {
        return Self::Io(io_error);
    }
}

impl From<UartError> for Error {
    #[allow(unused)]
    fn from(uart_error: UartError) -> Self {
        // return Self::Uart(uart_error);
        unimplemented!();
        // return match rppal_err {
        //     UartError::Io(std_err) => std_err,
        //     UartError::Gpio(gpio_err) => match gpio_err {
        //         GpioError::UnknownModel => Error::new(ErrorKind::Other, "unknown model"),
        //         GpioError::PinNotAvailable(pin) => Error::new(ErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin)),
        //         GpioError::PermissionDenied(err_string) => Error::new(ErrorKind::PermissionDenied, format!("permission denied: {} ", err_string)),
        //         GpioError::Io(error) => error,
        //         GpioError::ThreadPanic => Error::new(ErrorKind::Other, "thread panic"),
        //     },
        //     UartError::InvalidValue => Error::new(ErrorKind::Other, "invalid value"),
        // };
    }
}
