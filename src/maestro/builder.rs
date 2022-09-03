// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

//! Implementation of the builder pattern to build an instance of [`crate::maestro::Maestro`].

use std::time::Duration;

use crate::maestro::intrinsics::Baudrate;

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
