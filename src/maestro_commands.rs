// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */
use std::io::Error;

/* internal mods */

/* internal uses */
use crate::maestro_constants::Channels;

pub type UnitResultType = Result<(), Error>;
pub type DataResultType = Result<u16, Error>;

pub trait MaestroCommands {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> UnitResultType;
    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> UnitResultType;
    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> UnitResultType;

    fn go_home(self: &mut Self) -> UnitResultType;
    fn stop_script(self: &mut Self) -> UnitResultType;

    fn get_position(self: &mut Self, channel: Channels) -> DataResultType;
    fn get_errors(self: &mut Self) -> DataResultType;
}
