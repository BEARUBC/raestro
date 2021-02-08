#![allow(unused)]

// External Uses
use std::boxed::Box;
use rppal::{
    uart::{
        Parity,
        Uart,
        Result,
        Error,
    },
    system::DeviceInfo,
};

// Internal Uses
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

pub struct Maestro {
    uart: Option<Box<Uart>>,
}

impl Maestro {
    pub fn new() -> Self {
        return Maestro {
            uart: None,
        };
    }

    pub fn initialize(self: &mut Self, baud_rate: BaudRates) -> std::result::Result<(), Error> {
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
            Err(err_msg) => Err(err_msg)
        };
    }

    pub fn close(self: &mut Self) -> () {
        match &self.uart {
            Some(boxed_uart) => self.uart = None,
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

    pub fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> std::result::Result<usize, Error> {
        let command: u8 = mask_byte(Commands::SET_TARGET as u8);
        let (lower, upper): (u8, u8) = microsec_to_target(microsec);

        let buffer: [u8; 6usize] = [
            ProtocolMetaData::SYNC as u8,
            ProtocolMetaData::DEVICE_NUMBER as u8,
            command,
            channel as u8,
            lower,
            upper
        ];

        return self.write(&buffer);
    }

    pub fn read(self: &Self) -> Result<usize> {
        todo!();
    }
}
