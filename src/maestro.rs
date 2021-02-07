#![allow(unused)]

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

use crate::gpio::{
    UartMetaData,
};
use crate::uart_metadata::{
    BaudRate,
    Channel,
};

pub(crate) const BUFFER: [u8; 4usize] = [0x84u8, 0x00u8, 0x70u8, 0x2eu8];

pub struct Maestro {
    uart: Option<Box<Uart>>,
}

impl Maestro {
    pub fn new() -> Self {
        return Maestro {
            uart: None,
        };
    }

    pub fn initialize(self: &mut Self, baud_rate: BaudRate) -> std::result::Result<(), Error> {
        let uart_result: Result<Uart> = Uart::new(
            baud_rate as u32,
            Parity::None,
            8u8,
            1u8
        );

        let buffer: [u8; 1usize] = [0xaau8];
        self.write(&buffer);

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

    fn microsec_to_target(mut microsec: u16) -> (u8, u8) {
        let multiplier: u8 = 2u8;
        let mask: u16 = 0x7fu16;
        let down_shift: u8 = 7u8;

        microsec <<= multiplier;

        let lower: u8 = (microsec & mask) as u8;
        let upper: u8 = ((microsec >> down_shift) & mask) as u8;

        return (lower, upper);
    }

    fn write(self: &mut Self, buffer: &[u8]) -> std::result::Result<usize, Error> {
        if let Some(boxed_uart) = &mut self.uart {
            // let result: Result<usize> = (*boxed_uart).write(buffer);
            let result: Result<usize> = (*boxed_uart).write(&BUFFER);

            return match result {
                Ok(bits_read) => Ok(bits_read),
                Err(err_msg) => Err(err_msg),
            };
        } else {
            return Ok(0usize);
        }
    }

    pub fn set_target(self: &mut Self, channel: Channel, microsec: u16) -> std::result::Result<usize, Error> {
        let command: u8 = 0x84u8;
        let (lower, upper): (u8, u8) = Maestro::microsec_to_target(microsec);

        let buffer: [u8; 4usize] = [command, channel as u8, lower, upper];
        return self.write(&buffer);
    }

    pub fn read(self: &Self) -> Result<usize> {
        todo!();
    }
}
