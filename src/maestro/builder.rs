// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

//! Implementation of the builder pattern to build an instance of
//! [`crate::maestro::Maestro`].
//!
//! ### Examples:
//! ```rust
//! let builder = Builder::default()
//!     .baudrate(Baudrate::Baudrate50)
//!     .block_duration(std::time::Duration::from_secs(10));
//!
//! // build into a `maestro` instance by doing the following:
//! let maestro: Maestro = builder.try_into()?;
//! ```
//!
//! The internals of the [`Builder`] struct are also public, meaning that they
//! can easily be modified manually.

use std::time::Duration;

use rppal::uart::Parity;
use rppal::uart::Uart;

use crate::errors::Error;
use crate::maestro::constants::Baudrate;
use crate::maestro::internals;
use crate::maestro::Maestro;

#[derive(Default)]
/// ### Purpose:
/// A builder into which configurations for the [`super::maestro::Maestro`] can
/// be set.
pub struct Builder {
    /// ### Purpose:
    /// The baudrate setting.
    pub baudrate: Option<Baudrate>,

    /// ### Purpose:
    /// How long to wait for a response before quitting and returning.
    pub block_duration: Option<Duration>,
}

impl Builder {
    /// ### Purpose:
    /// Convenience function to configure the baudrate for this builder.
    pub fn baudrate(self, baudrate: Baudrate) -> Self {
        let baudrate = Some(baudrate);
        Self { baudrate, ..self }
    }

    /// ### Purpose:
    /// Convenience function to configure the block-duration for this builder.
    pub fn block_duration(self, block_duration: Duration) -> Self {
        let block_duration = Some(block_duration);
        Self {
            block_duration,
            ..self
        }
    }
}

impl TryFrom<Builder> for Maestro {
    type Error = crate::errors::Error;

    fn try_from(
        Builder {
            baudrate,
            block_duration,
        }: Builder,
    ) -> Result<Self, Self::Error> {
        let baudrate = baudrate.ok_or_else(|| Error::Uninitialized)? as u32;
        let mut uart = Uart::new(
            baudrate,
            Parity::None,
            internals::DATA_BITS,
            internals::STOP_BITS,
        )?;
        let block_duration = block_duration.unwrap_or_default();
        uart.set_read_mode(0u8, block_duration)?;
        let read_buf = [0u8; internals::BUFFER_SIZE];
        let write_buf = [0u8; internals::BUFFER_SIZE];
        let maestro = Self {
            uart,
            read_buf,
            write_buf,
        };
        Ok(maestro)
    }
}
