// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external uses */
use std::{
    time::Duration,
    boxed::Box,
    result::Result as StdResult,
};
use rppal::{
    uart::{
        Parity,
        Uart,
        Result as RppalResult,
    },
};

/* internal mods */

/* internal uses */
use crate::{
    utils::*,
    constants::*,
    errors::*,
};

pub type Result<T> = StdResult<T, Error>;

const BUFFER_SIZE: usize = 6usize;

pub struct Maestro {
    uart: Option<Box<Uart>>,
    read_buf: Option<Box<[u8; BUFFER_SIZE]>>,
    write_buf: Option<Box<[u8; BUFFER_SIZE]>>,
}

// basic public APIs
impl Maestro {
    pub fn new() -> Self {
        return Maestro {
            uart: None,
            read_buf: None,
            write_buf: None,
        };
    }

    pub fn start(self: &mut Self, baud_rate: BaudRates) -> Result<()> {
        let uart_result: RppalResult<Uart> = Uart::new(
            baud_rate as u32,
            Parity::None,
            DATA_BITS,
            STOP_BITS,
        );

        return uart_result
            .and_then(|uart| {
                let block_duration = 2u64;

                self.uart = Some(Box::new(uart));
                self.read_buf = Some(Box::new([0u8; BUFFER_SIZE]));
                self.write_buf = Some(Box::new([0u8; BUFFER_SIZE]));

                return self.uart
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .set_read_mode(RESPONSE_SIZE, Duration::from_secs(block_duration));
            })
            .map(|()| {
                let buf = self.write_buf
                    .as_mut()
                    .unwrap()
                    .as_mut();
                
                buf[0usize] = SYNC as u8;
                buf[1usize] = DEVICE_NUMBER as u8;
            })
            .map_err(|rppal_err| Error::from(rppal_err));
    }

    pub fn close(self: &mut Self) -> () {
        self.uart = None;
        self.read_buf = None;
        self.write_buf = None;
    }

    pub fn set_block_duration(self: &mut Self, duration: Duration) -> Result<()> {
        return self.uart
            .as_mut()
            .ok_or(Error::Uninitialized)
            .and_then(|uart| {
                const RESPONSE_SIZE: u8 = 2u8;

                return uart
                    .set_read_mode(RESPONSE_SIZE, duration)
                    .map_err(|rppal_err| Error::from(rppal_err));
            });
    }
}

// public maestro commands
impl Maestro {
    pub fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> Result<()> {
        return if MIN_PWM < microsec && microsec < MAX_PWM {
            Ok(microsec << DATA_MULTIPLIER)
        } else {
            Err(Error::InvalidValue(microsec))
        }
            .and_then(move |payload| {
                self.write_channel_and_payload(CommandFlags::SET_TARGET, channel, payload)
            });
    }

    pub fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> Result<()> {
        return self.write_channel_and_payload(CommandFlags::SET_SPEED, channel, microsec);
    }

    pub fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> Result<()> {
        return self.write_channel_and_payload(CommandFlags::SET_ACCELERATION, channel, value as u16);
    }

    pub fn go_home(self: &mut Self) -> Result<()> {
        return self.write_command(CommandFlags::GO_HOME);
    }

    pub fn stop_script(self: &mut Self) -> Result<()> {
        return self.write_command(CommandFlags::STOP_SCRIPT);
    }

    pub fn get_position(self: &mut Self, channel: Channels) -> Result<u16> {
        let write_result = self.write_channel(CommandFlags::GET_POSITION, channel);

        return self
            .read_after_writing(write_result)
            .map(move |result| result >> DATA_MULTIPLIER);
    }

    pub fn get_errors(self: &mut Self) -> Result<u16> {
        let write_result = self.write_command(CommandFlags::GET_ERRORS);

        return self.read_after_writing(write_result);
    }
}

// private utility methods
impl Maestro {
    fn read(self: &mut Self, length: usize) -> Result<()> {
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
            .map_err(|rppal_err| Error::from(rppal_err))
            .and_then(|bytes_read|
                if bytes_read == length {
                    Ok(())
                } else {
                    Err(Error::FaultyRead {
                        actual_count: bytes_read,
                        expected_count: length
                    })
                }
            );
    }

    fn write(self: &mut Self, length: usize) -> Result<()> {
        if (length < MIN_WRITE_LENGTH) || (BUFFER_SIZE < length)  {
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
            .map_err(|rppal_err| Error::from(rppal_err))
            .and_then(|bytes_written|
                if bytes_written == length {
                    Ok(())
                } else {
                    Err(Error::FaultyWrite {
                        actual_count: bytes_written,
                        expected_count: length,
                    })
                }
            );
    }

    #[inline]
    fn write_channel_and_payload(
        self: &mut Self,
        command_flag: CommandFlags,
        channel: Channels,
        microsec: u16,
    ) -> Result<()> {
        let length_to_write = 6usize;

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

        return self.write(length_to_write);
    }

    #[inline]
    fn write_channel(
        self: &mut Self,
        command_flag: CommandFlags,
        channel: Channels,
    ) -> Result<()> {
        let length_to_write = 4usize;

        let command = mask_byte(command_flag as u8);

        let buffer = self.write_buf
            .as_mut()
            .unwrap()
            .as_mut();

        buffer[2usize] = command;
        buffer[3usize] = channel as u8;

        return self.write(length_to_write);
    }

    #[inline]
    fn write_command(
        self: &mut Self,
        command_flag: CommandFlags,
    ) -> Result<()> {
        let length_to_write = 3usize;

        let command = mask_byte(command_flag as u8);

        let buffer = self.write_buf
            .as_mut()
            .unwrap()
            .as_mut();

        buffer[2usize] = command;

        return self.write(length_to_write);
    }

    fn prepare_data_from_buffer(self: &mut Self) -> u16 {
        let buf = self.read_buf
            .as_mut()
            .unwrap()
            .as_mut();

        let data: u16 = ((buf[1usize] as u16) << 8usize) | (buf[0usize] as u16);

        return data;
    }

    fn read_after_writing(self: &mut Self, write_result: Result<()>) -> Result<u16> {
        return write_result
            .and_then(|()| self.read(RESPONSE_SIZE as usize))
            .map(|()| self.prepare_data_from_buffer())
    }
}
