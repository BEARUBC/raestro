// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

//! The main source module for the [`Maestro`] struct, as well as all related
//! definitions.

pub mod builder;
pub mod constants;
mod internals;
mod utils;

use std::cmp::Ordering;

use rppal::uart::Uart;

use crate::errors::Error;
use crate::maestro::constants::ErrorValues;
use crate::maestro::utils::mask_byte;
use crate::maestro::utils::microsec_to_target;

/// ### Purpose:
/// The main wrapper around the Maestro
/// communications interface.
#[cfg_attr(test, derive(Debug))]
pub struct Maestro {
    uart: Uart,
    read_buf: [u8; internals::BUFFER_SIZE],
    write_buf: [u8; internals::BUFFER_SIZE],
}

impl Maestro {
    /// Sets the target of the servo motor at the
    /// given channel with
    /// the given microseconds.
    ///
    /// Microsecond ranges can only be between
    /// `992us` and `2000us`.
    /// However, the input to `set_target` is in
    /// quarter microseconds. Thus, the accepted
    /// range to `set_target` is between `3968`
    /// and `8000`.
    /// Any values outside of this range will
    /// return an error.
    ///
    /// The units to `set_target` are in:
    /// `target * (0.25) [us]`
    ///
    /// # Example Usage
    /// ```
    /// let maestro: Maestro = Builder::default()
    ///     .baudrate(Baudrate::Baudrate50)
    ///     .block_duration(Duration::from_secs(10))
    ///     .try_into()?;
    ///
    /// // can be any arbitrary channel in the Channel enum
    /// let channel: Channel = Channel::C_0;
    ///
    /// // can be any value between 3968 and 8000
    /// // 4000 quarter microsecs would be 1000us, thus this example sets a target of 1000us
    /// let qtr_microsec = 4000u16;
    ///
    /// maestro.set_target(channel, qtr_microsec);
    /// ```
    pub fn set_target(
        &mut self,
        channel: constants::Channel,
        target: u16,
    ) -> crate::Result<()> {
        (constants::MIN_QTR_PWM..=constants::MAX_QTR_PWM)
            .contains(&target)
            .then(|| ())
            .ok_or_else(|| Error::InvalidValue(target))?;
        self.write_channel_and_payload(
            internals::CommandFlags::SetTarget,
            channel,
            target,
        )
    }

    /// Sets the rotational speed of the servo
    /// motor at the given channel with the
    /// given speed.
    ///
    /// The units to `set_speed` are in:
    /// `speed * (0.025) [us / ms]`
    ///
    /// # Example Usage
    /// ```
    /// let maestro: Maestro = Builder::default()
    ///     .baudrate(Baudrate::Baudrate50)
    ///     .block_duration(Duration::from_secs(10))
    ///     .try_into()?;
    ///
    /// let channel: Channel = Channel::C_0; // can be any arbitrary channel in the Channel enum
    /// let speed = 10u16;
    ///
    /// m.set_speed(channel, speed);
    /// ```
    pub fn set_speed(
        &mut self,
        channel: constants::Channel,
        speed: u16,
    ) -> crate::Result<()> {
        self.write_channel_and_payload(
            internals::CommandFlags::SetSpeed,
            channel,
            speed,
        )
    }

    /// Sets the rotational acceleration limit of
    /// the servo motor at the given channel with
    /// the given value.
    ///
    /// The acceleration can be any usigned 8-bit
    /// integer from `1u8` to `255u8`. An
    /// acceleration of `0u8` will command the
    /// Maestro to *not* set any acceleration
    /// limit.
    ///
    /// Note that an acceleration limit causes
    /// the servo to speed up and the slow down
    /// as it approaches the target. By having no
    /// acceleration limit, this behaviour is
    /// disabled.
    ///
    /// The units to `set_acceleration` are in:
    /// `acceleration * 0.0003125 [us / ((ms)^2)]`
    ///
    /// # Example Usage
    /// ```
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let channel: Channel = Channel::C_0; // can be any arbitrary channel in the Channel enum
    /// let acceleration = 10u8;
    ///
    /// m.set_acceleration(channel, acceleration);
    /// ```
    pub fn set_acceleration(
        &mut self,
        channel: constants::Channel,
        acceleration: u8,
    ) -> crate::Result<()> {
        let acceleration = acceleration as u16;
        self.write_channel_and_payload(
            internals::CommandFlags::SetAcceleration,
            channel,
            acceleration,
        )
    }

    /// Sends all servos to home position.
    ///
    /// Home position is defined as
    /// `992us`.
    ///
    /// # Example Usage
    /// ```
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// m.go_home();
    /// ```
    pub fn go_home(&mut self) -> crate::Result<()> {
        self.write_command(internals::CommandFlags::GoHome)
    }

    /// Stops all requested actions sent to the
    /// Maestro to be stopped immediately.
    ///
    /// # Example Usage
    /// ```
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// m.stop_script();
    /// ```
    pub fn stop_script(&mut self) -> crate::Result<()> {
        self.write_command(internals::CommandFlags::StopScript)
    }

    /// Gets the `PWM` signal being broadcasted to
    /// the servo at the given channel.
    ///
    /// # Important
    /// In order to rotate the servos, the Maestro
    /// sends a PWM signal
    /// over the corresponding channel. This is,
    /// in essence, what is happening when
    /// `set_target` is called. However, this
    /// signal can still be sent even if a servo
    /// motor is not connected to the pins;
    /// the only difference here being that no
    /// servo is connected to execute the
    /// rotation, but the signal
    /// is *still sent*, regardless.
    ///
    /// The `get_position` request will only
    /// return the `PWM` that is
    /// being broadcasted on the channel. Using
    /// this method will NOT help you in
    /// determining servo failures, incorrect
    /// servo positions, etc. This method will
    /// *only* return the `PWM` that is being
    /// broadcasted on the given channel.
    ///
    /// The Maestro, in and of itself, cannot
    /// possibly know if a servo is or is not
    /// at the location that was encoded
    /// in the request (i.e., if the servo failed
    /// half-way through exectution). As such,
    /// `raestro` cannot support this
    /// functionality either. If this
    /// functionality is required
    /// for your project, you will need to develop
    /// additional hardware.
    ///
    /// `get_position` returns the `PWM` being
    /// broadcasted in quarter microsec.
    /// If `get_position` returns `4000`, the
    /// servo is currently broadcasting `1000us`
    /// to the respective channel.
    ///
    /// # Example Usage
    /// ```ignore
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let channel: Channel = Channel::C_0; // can be any arbitrary channel in the Channel enum
    /// let target = 4000u16; // can be any value between 3968u16 and 8000u16
    ///
    /// m.set_target(channel, target);
    ///
    /// let actual_position = m.get_position(channel).unwrap();
    ///
    /// assert_eq!(target, actual_position);
    /// ```
    pub fn get_position(
        &mut self,
        channel: constants::Channel,
    ) -> crate::Result<u16> {
        self.write_channel(internals::CommandFlags::GetPosition, channel)?;
        self.read(internals::RESPONSE_SIZE as usize)?;
        let pos = self.prepare_data_from_buffer();
        Ok(pos)
    }

    /// Gets any errors encountered by the Maestro
    /// during execution.
    ///
    /// # Important
    /// This method will *not* inform you of any
    /// failures with the servo hardware. The
    /// Maestro, in and of itself, is not
    /// capable of determining external hardware
    /// malfunctions. If your project requires
    /// this feature, you will need to develop
    /// additional hardware to implement it.
    ///
    /// # Example Usage
    /// ```ignore
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let errors = m.get_errors().unwrap();
    /// ```
    pub fn get_errors(&mut self) -> crate::Result<Vec<ErrorValues>> {
        self.write_command(internals::CommandFlags::GetErrors)?;
        self.read(internals::RESPONSE_SIZE as usize)?;
        let data = self.prepare_data_from_buffer();
        let errors = ErrorValues::from_data(data);
        Ok(errors)
    }

    /// ### Purpose:
    /// Reads the given number of bytes into
    /// `self.read_buf`.
    ///
    /// ### Notes:
    /// Please note that the `self.uart.read`
    /// method is being utilized to send the
    /// commands over `UART`. This command
    /// operates on a blocking read. Blocking
    /// duration is default set to
    /// `DEFAULT_BLOCKING_DURATION`.
    fn read(&mut self, length: usize) -> crate::Result<()> {
        let Self { uart, read_buf, .. } = self;
        let mut slice = &mut read_buf[0..length];
        let bytes_read = uart.read(&mut slice)?;
        let comparison = bytes_read.cmp(&length);
        match comparison {
            Ordering::Equal => Ok(()),
            _ => Err(Error::FaultyRead {
                actual_count: bytes_read,
            }),
        }
    }

    /// ### Purpose:
    /// Writes the given number of bytes over to
    /// the Maestro.
    ///
    /// ### Notes:
    /// The bytes that are being written are
    /// located in the `self.write_buf` array.
    /// This is the method that actually calls
    /// `self.uart.write`. Other methods in this
    /// `impl` block just write to
    /// `self.write_buf`, but do not actually send
    /// data over the `UART` pins.
    fn write(&mut self, length: usize) -> crate::Result<()> {
        let Self {
            uart, write_buf, ..
        } = self;
        let mut slice = &mut write_buf[0..length];
        let bytes_written = uart.write(&mut slice)?;
        let comparison = bytes_written.cmp(&length);
        match comparison {
            Ordering::Equal => Ok(()),
            _ => Err(Error::FaultyWrite {
                actual_count: bytes_written,
                expected_count: length,
            }),
        }
    }

    /// ### Purpose:
    /// Writes the given arguments into the
    /// appropriate place in `self.write_buf`.
    ///
    /// ### Notes:
    /// This method does not actually send the
    /// bytes over the `UART` pins. It just
    /// writes them into the correct place in the
    /// buffer and then calls `self.write`
    /// while passing in the desired length.
    fn write_channel_and_payload(
        &mut self,
        command_flag: internals::CommandFlags,
        channel: constants::Channel,
        microsec: u16,
    ) -> crate::Result<()> {
        let Self { write_buf, .. } = self;
        let command = mask_byte(command_flag as u8);
        let (lower, upper) = microsec_to_target(microsec);
        write_buf[2usize] = command;
        write_buf[3usize] = channel as u8;
        write_buf[4usize] = lower;
        write_buf[5usize] = upper;
        self.write(internals::WRITE_CHANNEL_AND_PAYLOAD_SIZE)
    }

    /// ### Purpose:
    /// Writes the given arguments into the
    /// appropriate place in `self.write_buf`.
    ///
    /// ### Notes:
    /// This method does not actually send the
    /// bytes over the `UART` pins. It just
    /// writes them into the correct place in the
    /// buffer and then calls `self.write`
    /// while passing in the desired length.
    #[inline]
    fn write_channel(
        &mut self,
        command_flag: internals::CommandFlags,
        channel: constants::Channel,
    ) -> crate::Result<()> {
        let Self { write_buf, .. } = self;
        let command = mask_byte(command_flag as u8);
        write_buf[2usize] = command;
        write_buf[3usize] = channel as u8;
        self.write(internals::WRITE_CHANNEL_SIZE)
    }

    /// ### Purpose:
    /// Writes the given arguments into the
    /// appropriate place in `self.write_buf`.
    ///
    /// ### Notes:
    /// This method does not actually send the
    /// bytes over the `UART` pins. It just
    /// writes them into the correct place in the
    /// buffer and then calls `self.write`
    /// while passing in the desired length.
    fn write_command(
        &mut self,
        command_flag: internals::CommandFlags,
    ) -> crate::Result<()> {
        let Self { write_buf, .. } = self;
        let command = mask_byte(command_flag as u8);
        write_buf[2usize] = command;
        self.write(internals::WRITE_COMMAND_SIZE)
    }

    /// ### Purpose:
    /// Utility function to take the first two bytes in [`Self::read_buf`] and
    /// convert them from the Pololu standardized-return-form to [`u16`].
    fn prepare_data_from_buffer(&self) -> u16 {
        let Self { read_buf: buf, .. } = self;
        let top = (buf[1usize] as u16) << 8usize;
        let bottom = buf[0usize] as u16;
        top | bottom
    }
}
