/* external crates */

/* external uses */
use rppal::uart::Error;

/* internal mods */

/* internal uses */
use crate::maestro_constants::Channels;

pub trait MaestroCommands {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error>;
    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error>;
    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> std::result::Result<usize, Error>;
    fn get_position(self: &mut Self, channel: Channels) -> std::result::Result<usize, Error>;
    fn get_errors(self: &mut Self) -> std::result::Result<u16, Error>;
    fn go_home(self: &mut Self) -> std::result::Result<usize, Error>;
    fn stop_script(self: &mut Self) -> std::result::Result<usize, Error>;
}
