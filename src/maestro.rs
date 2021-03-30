/* external crates */

/* external uses */
use std::io::{
    Error,
    ErrorKind,
};
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
use crate::maestro_commands::MaestroCommands;

pub struct Maestro {
    uart: Option<Box<Uart>>,
}

impl Maestro {
    pub fn new() -> Self {
        return Maestro {
            uart: None,
        };
    }

    pub fn start(self: &mut Self, baud_rate: BaudRates) -> Result<(), Error> {
        let data_bits: u8 = 8u8;
        let stop_bits: u8 = 1u8;

        let uart_result: RppalResult<Uart> = Uart::new(
            baud_rate as u32,
            Parity::None,
            data_bits,
            stop_bits
        );

        

        return match uart_result {
            Ok(uart) => Ok(self.uart = Some(Box::new(uart))),
            Err(rppal_err) => Err(Maestro::deconstruct_error(rppal_err)),
        };
    }

    pub fn close(self: &mut Self) -> () {
        match &self.uart {
            Some(_) => self.uart = None,
            None => (),
        };
    }

    fn write(self: &mut Self, buffer: &[u8]) -> Result<usize, Error> {
        if let Some(boxed_uart) = &mut self.uart {
            let result: RppalResult<usize> = (*boxed_uart).write(buffer);

            return match result {
                Ok(bits_read) => Ok(bits_read),
                Err(rppal_err) => Err(Maestro::deconstruct_error(rppal_err)),
            };
        } else {
            return Err(Error::new(ErrorKind::NotConnected, "Maestro not initialized. Consider calling .start()"));
        }
    }

    #[allow(unused)]
    fn read(self: &mut Self, buffer: &mut [u8]) -> Result<usize, Error> {
        if let Some(boxed_uart) = &mut self.uart {
            let result: RppalResult<usize> = (*boxed_uart).read(buffer);

            return match result {
                Ok(bits_read) => Ok(bits_read),
                Err(rppal_err) => Err(Maestro::deconstruct_error(rppal_err)),
            };
        } else {
            return Err(Error::new(ErrorKind::NotConnected, "Maestro not initialized. Consider calling .start()"));
        }
    }

    #[inline]
    fn write_channel_payload(self: &mut Self, command: u8, channel: Channels, payload_0: u8, payload_1: u8) -> Result<usize, Error> {
        let buffer: [u8; 6usize] = [
            ProtocolMetadata::SYNC as u8,
            ProtocolMetadata::DEVICE_NUMBER as u8,
            command,
            channel as u8,
            payload_0,
            payload_1,
        ];

        return self.write(&buffer);
    }

    #[allow(unused)]
    #[inline]
    fn write_channel(self: &mut Self, command: u8, channel: Channels) -> Result<usize, Error> {
        let buffer: [u8; 4usize] = [
            ProtocolMetadata::SYNC as u8,
            ProtocolMetadata::DEVICE_NUMBER as u8,
            command,
            channel as u8,
        ];

        return self.write(&buffer);
    }

    #[inline]
    fn write_command(self: &mut Self, command: u8) -> Result<usize, Error> {
        let buffer: [u8; 3usize] = [
            ProtocolMetadata::SYNC as u8,
            ProtocolMetadata::DEVICE_NUMBER as u8,
            command,
        ];

        return self.write(&buffer);
    }

    fn deconstruct_error(rppal_err: UartError) -> Error {
        return match rppal_err {
            UartError::Io(std_err) => std_err,
            UartError::Gpio(gpio_err) => match gpio_err {
                GpioError::UnknownModel => Error::new(ErrorKind::Other, "unknown model"),
                GpioError::PinNotAvailable(pin) => Error::new(ErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin)),
                GpioError::PermissionDenied(err_string) => Error::new(ErrorKind::PermissionDenied, format!("Permission denied: {} ", err_string)),
                GpioError::Io(error) => error,
                GpioError::ThreadPanic => Error::new(ErrorKind::Other, "Thread panic"),
            },
            UartError::InvalidValue => Error::new(ErrorKind::Other, "Invalid Value"),
        };
    }
}

impl MaestroCommands for Maestro {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> Result<usize, Error> {
        let command = mask_byte(CommandFlags::SET_TARGET as u8);
        let (lower, upper) = microsec_to_target(microsec);

        return self.write_channel_payload(command, channel, lower, upper);
    }

    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> Result<usize, Error> {
        let command = mask_byte(CommandFlags::SET_SPEED as u8);
        let (lower, upper) = microsec_to_target(microsec);

        return self.write_channel_payload(command, channel, lower, upper);
    }

    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> Result<usize, Error> {
        let command = mask_byte(CommandFlags::SET_ACCELERATION as u8);
        let (lower, upper) = microsec_to_target(value as u16);

        return self.write_channel_payload(command, channel, lower, upper);
    }

    #[allow(unused)]
    fn get_position(self: &mut Self, channel: Channels) -> Result<usize, Error> {
        let command = mask_byte(CommandFlags::GET_POSITION as u8);
        match self.write_channel(command, channel) {
            Err(err) => Err(err),
            Ok(bits_read) => {
                let mut buffer: [u8; 2usize] = [0,0]; 
                match self.read(&mut buffer) {
                    Err(err) => Err(err),
                    Ok(bits_read) => Ok( (buffer[1] << 8 + buffer[0]).into() ),   
                }
            }
        }
        
    }

    #[allow(unused)]
    fn get_errors(self: &mut Self) -> Result<usize, Error> {
        todo!();
        // let command = mask_byte(CommandFlags::GET_POSITION as u8);
        // return self.write_command(command);
    }

    fn go_home(self: &mut Self) -> Result<usize, Error> {
        let command = mask_byte(CommandFlags::GO_HOME as u8);
        return self.write_command(command);
    }

    fn stop_script(self: &mut Self) -> Result<usize, Error> {
        let command = mask_byte(CommandFlags::STOP_SCRIPT as u8);
        return self.write_command(command);
    }
}
