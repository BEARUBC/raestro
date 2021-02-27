/* external crates */

/* external uses */
use rppal::{
    uart::{
        Parity,
        Uart,
        Result,
        Error,
    },
    system::DeviceInfo,
};

/* internal mods */

/* internal uses */
use crate::utils::{
    mask_byte,
    microsec_to_target,
};
use crate::maestro_constants::{
    ProtocolMetaData,
    Commands,
    Channels,
    BaudRates,
};

pub trait Functions {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error>;
    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error>;
    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> std::result::Result<usize, Error>;
}
