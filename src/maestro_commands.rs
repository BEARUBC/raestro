/* external crates */
use std::io::Error;
/* external uses */
// use rppal::uart::Error as RppalError;

/* internal mods */

/* internal uses */
#[allow(unused_imports)]
use crate::maestro_constants::{
    Channels,
    CommandFlags,
};

type ResultType = Result<usize, Error>;

pub trait MaestroCommands {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> ResultType;
    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> ResultType;
    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> ResultType;
    fn get_position(self: &mut Self, channel: Channels) -> ResultType;
    fn get_errors(self: &mut Self) -> ResultType;
    fn go_home(self: &mut Self) -> ResultType;
    fn stop_script(self: &mut Self) -> ResultType;
    // fn dispatcher(self: &mut Self, command: CommandFlags, channel: Channels, payload_0: u8, payload_1: u8, microsec: u16) -> ResultType;
}
