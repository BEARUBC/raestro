/* external crates */

/* external uses */
use std::boxed::Box;
use rppal::{
    uart::{
        Parity,
        Uart,
        Result,
        Error,
    },
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

    pub fn start(self: &mut Self, baud_rate: BaudRates) -> std::result::Result<(), Error> {
        let data_bits: u8 = 8u8;
        let stop_bits: u8 = 1u8;

        let uart_result: Result<Uart> = Uart::new(
            baud_rate as u32,
            Parity::None,
            data_bits,
            stop_bits
        );

        return match uart_result {
            Ok(uart) => {
                self.uart = Some(Box::new(uart));
                Ok(())
            },
            Err(err_msg) => Err(err_msg),
        };
    }

    pub fn close(self: &mut Self) -> () {
        match &self.uart {
            Some(_) => self.uart = None,
            None => (),
        };
    }

    fn write(self: &mut Self, buffer: &[u8]) -> std::result::Result<usize, Error> {
        if let Some(boxed_uart) = &mut self.uart {
            let result: Result<usize> = (*boxed_uart).write(buffer);
            // let result: Result<usize> = (*boxed_uart).write(&BUFFER);

            return match result {
                Ok(bits_read) => Ok(bits_read),
                Err(err_msg) => Err(err_msg),
            };
        } else {
            return Ok(0usize);
        }
    }

    #[allow(unused)]
    fn read(self: &mut Self) -> std::result::Result<usize, Error> {
        if let Some(boxed_uart) = &mut self.uart {
            let mut buffer: [u8;2] = [0,0];
            let result: Result<usize> = (*boxed_uart).read(&mut buffer);
            // let result: Result<usize> = (*boxed_uart).write(&BUFFER);

            return match result {
                Ok(bits_read) => Ok(result.unwrap()),
                Err(err_msg) => Err(err_msg),
            };
        } else {
            return Ok(0usize);
        }
    }

    #[inline]
    fn write_two(self: &mut Self, command: u8, channel: Channels, payload_0: u8, payload_1: u8) -> std::result::Result<usize, Error> {
        let buffer: [u8; 6usize] = [
            ProtocolMetaData::SYNC as u8,
            ProtocolMetaData::DEVICE_NUMBER as u8,
            command,
            channel as u8,
            payload_0,
            payload_1,
        ];

        return self.write(&buffer);
    }

    fn write_one_channel(self: &mut Self, command: u8, channel: Channels) -> std::result::Result<usize, Error> {
        let buffer: [u8; 4usize] = [
            ProtocolMetaData::SYNC as u8,
            ProtocolMetaData::DEVICE_NUMBER as u8,
            command,
            channel as u8
        ];

        return self.write(&buffer);
    }

    fn write_one(self: &mut Self, command: u8) -> std::result::Result<usize, Error> {
        let buffer: [u8; 3usize] = [
            ProtocolMetaData::SYNC as u8,
            ProtocolMetaData::DEVICE_NUMBER as u8,
            command
        ];

        return self.write(&buffer);
    }
}

impl MaestroCommands for Maestro {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::SET_TARGET as u8);
        let (lower, upper): (u8, u8) = microsec_to_target(microsec);

        return self.write_two(command, channel, lower, upper);
    }

    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::SET_SPEED as u8);
        let (lower, upper): (u8, u8) = microsec_to_target(microsec);

        return self.write_two(command, channel, lower, upper);

        // let buffer: [u8; 6usize] = [
        //     ProtocolMetaData::SYNC as u8,
        //     ProtocolMetaData::DEVICE_NUMBER as u8,
        //     command,
        //     channel as u8,
        //     lower,
        //     upper
        // ];

        // return self.write(&buffer);
    }

    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::SET_ACCELERATION as u8);
        let (lower, upper): (u8, u8) = microsec_to_target(value as u16);

        return self.write_two(command, channel, lower, upper);

        // let buffer: [u8; 6usize] = [
        //     ProtocolMetaData::SYNC as u8,
        //     ProtocolMetaData::DEVICE_NUMBER as u8,
        //     command,
        //     channel as u8,
        //     lower,
        //     upper
        // ];

        // return self.write(&buffer);
    }

    fn get_position(self: &mut Self, channel: Channels) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::GET_POSITION as u8);
        
        self.write_one_channel(command, channel).unwrap();

        return self.read();
    }

    #[allow(unused)]
    fn get_errors(self: &mut Self) -> std::result::Result<u16, Error> {
        let command: u8 = mask_byte(Commands::GET_ERRORS as u8);

        self.write_one(command);

        match self.read() {
            Err(e) => Err(e),
            Ok(0) => Ok(ERRORS::SER_SIGNAL_ERR as u16),
            Ok(1) => Ok(ERRORS::SER_OVERRUN_ERR as u16),
            Ok(2) => Ok(ERRORS::SER_BUFFER_FULL as u16),
            Ok(3) => Ok(ERRORS::SER_CRC_ERR as u16),
            Ok(4) => Ok(ERRORS::SER_PROTOCOL_ERR as u16),
            Ok(5) => Ok(ERRORS::SER_TIMEOUT as u16),
            Ok(6) => Ok(ERRORS::SCRIPT_STACK_ERR as u16),
            Ok(7) => Ok(ERRORS::SCRIPT_CALL_STACK_ERR as u16),
            Ok(8) => Ok(ERRORS::SCRIPT_PC_ERR as u16),
            Ok(_) => Err(rppal::uart::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "unkonwn error type"))),
        }
    }

    #[allow(unused)]
    fn go_home(self: &mut Self) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::GO_HOME as u8);
        
        return self.write_one(command);
    }

    #[allow(unused)]
    fn stop_script(self: &mut Self) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::STOP_SCRIPT as u8);
        
        return self.write_one(command);
    }
}
