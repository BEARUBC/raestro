/* external uses */
use std::io::Error;

/* internal mods */

/* internal uses */
use crate::maestro_constants::Channels;

pub type ResultType = Result<(), Error>;

pub trait MaestroCommands {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> ResultType;
    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> ResultType;
    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> ResultType;

    fn go_home(self: &mut Self) -> ResultType;
    fn stop_script(self: &mut Self) -> ResultType;

    fn get_position(self: &mut Self, channel: Channels) -> ResultType;
    fn get_errors(self: &mut Self) -> ResultType;
}
