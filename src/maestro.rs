/* external uses */
use std::io::{
    Error,
    ErrorKind,
};
use std::time::Duration;
use std::boxed::Box;
use rppal::{
    uart::{
        Parity,
        Uart,
        Result as RppalResult,
        Error as UartError,
    },
    gpio::Error as GpioError,
};

/* internal mods */

/* internal uses */
use crate::utils::*;
use crate::maestro_constants::*;
use crate::maestro_commands::*;

const DATA_BITS: u8 = 8u8;
const STOP_BITS: u8 = 1u8;
const BUFFER_SIZE: usize = 6usize;

pub struct Maestro {
    uart: Option<Box<Uart>>,
    read_buf: Option<Box<[u8; 6usize]>>,
    write_buf: Option<Box<[u8; 6usize]>>,
}

impl Maestro {
    pub fn new() -> Self {
        return Maestro {
            uart: None,
            read_buf: None,
            write_buf: None,
        };
    }

    pub fn start(self: &mut Self, baud_rate: BaudRates) -> Result<(), Error> {
        let uart_result: RppalResult<Uart> = Uart::new(
            baud_rate as u32,
            Parity::None,
            DATA_BITS,
            STOP_BITS,
        );

        const ERR_FUNC: fn(UartError) -> Error = |rppal_err| Maestro::deconstruct_error(rppal_err);

        return uart_result
            .map_err(ERR_FUNC)
            .and_then(|uart| {
                const RESPONSE_SIZE: u8 = 2u8;
                const BLOCK_DURATION: u64 = 2u64;

                self.uart = Some(Box::new(uart));
                self.read_buf = Some(Box::new([0u8; BUFFER_SIZE]));
                self.write_buf = Some(Box::new([0u8; BUFFER_SIZE]));

                return self.uart
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .set_read_mode(RESPONSE_SIZE, Duration::from_secs(BLOCK_DURATION))
                    .map_err(ERR_FUNC);
            })
            .map(|_| {
                let buf = self.write_buf
                    .as_mut()
                    .unwrap()
                    .as_mut();
                
                buf[0usize] = ProtocolMetadata::SYNC as u8;
                buf[1usize] = ProtocolMetadata::DEVICE_NUMBER as u8;
            });
    }

    pub fn close(self: &mut Self) -> () {
        self.uart = None;
        self.read_buf = None;
        self.write_buf = None;
    }

    pub fn set_block_duration(self: &mut Self, duration: Duration) -> Result<(), Error> {
        return self.uart
            .as_mut()
            .ok_or(Error::new(ErrorKind::NotConnected, "maestro not initialized; consider calling .start on the maestro instance"))
            .and_then(|uart| {
                const RESPONSE_SIZE: u8 = 2u8;

                return uart
                    .set_read_mode(RESPONSE_SIZE, duration)
                    .map_err(|rppal_err| Maestro::deconstruct_error(rppal_err));
            })
            .map(|_| ());
    }

    fn read(self: &mut Self, length: usize) -> Result<usize, Error> {
        if BUFFER_SIZE < length {
            panic!();
        }
        
        let slice = &mut self.read_buf
            .as_mut()
            .unwrap()
            .as_mut()[0usize..length];

        return self.uart
            .as_mut()
            .unwrap()
            .read(slice)
            .map_err(|rppal_err| Maestro::deconstruct_error(rppal_err));
    }

    fn write(self: &mut Self, length: usize) -> Result<usize, Error> {
        if (length <= 2usize) || (BUFFER_SIZE < length)  {
            panic!();
        }

        let slice = &self.write_buf
            .as_mut()
            .unwrap()
            .as_mut()[0usize..length];

        return self.uart
            .as_mut()
            .unwrap()
            .write(slice)
            .map_err(|rppal_err| Maestro::deconstruct_error(rppal_err));
    }

    #[inline]
    fn write_channel_and_payload(
        self: &mut Self,
        command_flag: CommandFlags,
        channel: Channels,
        microsec: u16,
    ) -> UnitResultType {
        return if self.write_buf
            .as_mut()
            .is_some() {
                let command = mask_byte(command_flag as u8);
                let (lower, upper) = microsec_to_target(microsec);

                let buffer = self.write_buf
                    .as_mut()
                    .unwrap()
                    .as_mut();

                buffer[2usize] = command;
                buffer[3usize] = channel as u8;
                buffer[4usize] = lower;
                buffer[5usize] = upper;

                self
                    .write(6usize)
                    .map(|_| ())
        } else {
            let err_type = ErrorKind::NotConnected;
            let err_msg = "maestro not initialized; consider calling .start on the maestro instance";

            Err(Error::new(err_type, err_msg))
        };
    }

    #[inline]
    fn write_channel(
        self: &mut Self,
        command_flag: CommandFlags,
        channel: Channels,
    ) -> UnitResultType {
        return if self.write_buf
            .as_mut()
            .is_some() {
                let command = mask_byte(command_flag as u8);

                let buffer = self.write_buf
                    .as_mut()
                    .unwrap()
                    .as_mut();

                buffer[2usize] = command;
                buffer[3usize] = channel as u8;
        
                self
                    .write(4usize)
                    .map(|_| ())
        } else {
            let err_type = ErrorKind::NotConnected;
            let err_msg = "maestro not initialized; consider calling .start on the maestro instance";

            Err(Error::new(err_type, err_msg))
        };
    }

    #[inline]
    fn write_command(
        self: &mut Self,
        command_flag: CommandFlags,
    ) -> UnitResultType {
        return if self.write_buf
            .as_mut()
            .is_some() {
                let command = mask_byte(command_flag as u8);

                let buffer = self.write_buf
                    .as_mut()
                    .unwrap()
                    .as_mut();

                buffer[2usize] = command;

                self
                    .write(3usize)
                    .map(|_| ())
        } else {
            let err_type = ErrorKind::NotConnected;
            let err_msg = "maestro not initialized; consider calling .start on the maestro instance";

            Err(Error::new(err_type, err_msg))
        };
    }

    fn deconstruct_error(rppal_err: UartError) -> Error {
        return match rppal_err {
            UartError::Io(std_err) => std_err,
            UartError::Gpio(gpio_err) => match gpio_err {
                GpioError::UnknownModel => Error::new(ErrorKind::Other, "unknown model"),
                GpioError::PinNotAvailable(pin) => Error::new(ErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin)),
                GpioError::PermissionDenied(err_string) => Error::new(ErrorKind::PermissionDenied, format!("permission denied: {} ", err_string)),
                GpioError::Io(error) => error,
                GpioError::ThreadPanic => Error::new(ErrorKind::Other, "thread panic"),
            },
            UartError::InvalidValue => Error::new(ErrorKind::Other, "invalid value"),
        };
    }

    fn prepare_data_from_buffer(self: &mut Self) -> u16 {
        let buf = self.read_buf
            .as_mut()
            .unwrap()
            .as_mut();

        let data: u16 = ((buf[1usize] as u16) << 8usize) | (buf[0usize] as u16);

        return data;
    }

    fn read_after_writing(self: &mut Self, write_result: UnitResultType) -> DataResultType {
        const RESPONSE_SIZE: usize = 2usize;

        return write_result
            .and_then(|()| self.read(RESPONSE_SIZE))
            .and_then(move |bytes_read| {
                return if bytes_read == RESPONSE_SIZE {
                        Ok(self.prepare_data_from_buffer())
                } else {
                    let err_type = ErrorKind::ConnectionAborted;
                    let err_msg = "maestro message could not be read";

                    Err(Error::new(err_type, err_msg))
                };
            });
    }
}

impl MaestroCommands for Maestro {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> UnitResultType {
        return self.write_channel_and_payload(CommandFlags::SET_TARGET, channel, microsec);
    }

    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> UnitResultType {
        return self.write_channel_and_payload(CommandFlags::SET_SPEED, channel, microsec);
    }

    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> UnitResultType {
        return self.write_channel_and_payload(CommandFlags::SET_ACCELERATION, channel, value as u16);
    }

    fn go_home(self: &mut Self) -> UnitResultType {
        return self.write_command(CommandFlags::GO_HOME);
    }

    fn stop_script(self: &mut Self) -> UnitResultType {
        return self.write_command(CommandFlags::STOP_SCRIPT);
    }

    fn get_position(self: &mut Self, channel: Channels) -> DataResultType {
        let write_result = self.write_channel(CommandFlags::GET_POSITION, channel);

        return self.read_after_writing(write_result);
    }

    fn get_errors(self: &mut Self) -> DataResultType {
        let write_result = self.write_command(CommandFlags::GET_ERRORS);

        return self.read_after_writing(write_result);
    }
}
