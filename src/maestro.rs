// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::{
    boxed::Box,
    result::Result as StdResult,
    time::Duration,
    vec::Vec,
};

use rppal::uart::{
    Parity,
    Result as RppalResult,
    Uart,
};

use crate::{
    constants::*,
    errors::*,
    utils::*,
};

/// Public result type.
///
/// Expands to `std::result::Result<T,
/// raestro::error::Error>`.
pub type Result<T> = StdResult<T, Error>;

const DEFAULT_BLOCKING_DURATION: Duration = Duration::from_secs(2u64);
const BUFFER_SIZE: usize = 6usize;

/// The main wrapper around the Maestro
/// communications interface.
///
/// The `impl` blocks for this struct are split
/// into three sections, two of which are included
/// below, with the remaining being private
/// and hidden from documentation: 1. Basic public
/// APIs; contains the standard APIs to create,
/// initailize, and close a Maestro instance
/// 2. Pololu Micro Maestro Protocols; all the
/// protocols supported by the Maestro, sendable
/// over the `UART` pins on the Raspberry Pi
#[derive(Debug)]
pub struct Maestro {
    uart: Option<Box<Uart>>,
    read_buf: Option<Box<[u8; BUFFER_SIZE]>>,
    write_buf: Option<Box<[u8; BUFFER_SIZE]>>,
}

/// # Basic public APIs
/// This section contains all the APIs required to
/// get a `maestro` instance up and running.
///
/// ## Definitions
/// An `initialized` `maestro` instance is defined
/// as an instance in which struct fields are the
/// `Some(_)` variant.
///
/// An `uninitialized` `maestro` instance is an
/// instance in which all fields are the `None`
/// variant.
///
/// An `invalid` `maestro` instance is an instance
/// which does not fit the above definitions.
///
/// All valid `maestro` instances must either
/// `uninitialized` or `initialized`. An `invalid`
/// `maestro` instance will result in
/// undefined behaviour. All methods below are
/// proved to either only transition a `maestro`
/// instance to `uninitialized` and `initialized`
/// states only. Therefore, `invalid` states are
/// impossible to reach. This is an invariant
/// which must (and will) be guaranteed for all
/// provided APIs.
impl Maestro {
    /// Creates a new `maestro` instance in the
    /// `uninitialized` state.
    pub fn new() -> Self {
        Maestro {
            uart: None,
            read_buf: None,
            write_buf: None,
        }
    }

    /// Transitions the given `maestro` instance
    /// to the `initialized` state with
    /// default configuration values.
    ///
    /// Default block duration is set to
    /// `2seconds`.
    ///
    /// Returns an error if any fields were unable
    /// to be initialized. In the case of this
    /// failure, this instance will be closed and
    /// will be transitioned *back* into the
    /// `uninitialized` state. This is done to
    /// prevent any leakage of `maestro` instances
    /// into the `invalid` state.
    pub fn start(&mut self, baud_rate: BaudRates) -> Result<()> {
        let uart_result: RppalResult<Uart> =
            Uart::new(baud_rate as u32, Parity::None, DATA_BITS, STOP_BITS);

        uart_result
            .and_then(|uart| {
                self.uart = Some(Box::new(uart));
                self.read_buf = Some(Box::new([0u8; BUFFER_SIZE]));
                self.write_buf = Some(Box::new([0u8; BUFFER_SIZE]));

                self.uart
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .set_read_mode(RESPONSE_SIZE, DEFAULT_BLOCKING_DURATION)
            })
            .map(|()| {
                let buf = self.write_buf.as_mut().unwrap().as_mut();

                buf[0usize] = SYNC as u8;
                buf[1usize] = DEVICE_NUMBER as u8;
            })
            .map_err(|rppal_err| {
                self.close();

                Error::from(rppal_err)
            })
    }

    /// Closes the `maestro` instance (i.e.,
    /// transitions the `maestro`
    /// instance *back* into the `uninitialized`
    /// state).
    ///
    /// This instance is no longer usable to
    /// communicate with the Maestro, unless
    /// until `Maestro::start()` is called again.
    pub fn close(&mut self) -> () {
        self.uart = None;
        self.read_buf = None;
        self.write_buf = None;
    }

    /// Configures the block duration to the
    /// passed in value.
    ///
    /// Default block duration is set to
    /// `2seconds`.
    ///
    /// # Note
    /// Reading from the Maestro is implemented as
    /// a *blocking* read. Given that the
    /// Maestro only writes back when requested to
    /// do so, and that writes coming from the
    /// Maestro happen immediately
    /// after requests sent to it, (implying that
    /// waiting times are minimal), a blocking
    /// read is sufficient (and probably more
    /// efficient than implementing any sort of
    /// asynchronous functionality).
    ///
    /// Returns an error if the `maestro` instance
    /// has not been initialized by calling
    /// `Maestro::start()`.
    pub fn set_block_duration(&mut self, duration: Duration) -> Result<()> {
        self.uart
            .as_mut()
            .ok_or(Error::Uninitialized)
            .and_then(|uart| {
                uart.set_read_mode(RESPONSE_SIZE, duration)
                    .map_err(Error::from)
            })
    }
}

/// # Pololu Micro Maestro Protocols
///
/// These protocols are officially supported by
/// the Pololu Micro Maestro 6-Channel.
///
/// For interacting with the Pololu, the official
/// "Pololu-Protocol" is being utilized.
///
/// More information on the Pololu-Protocol can be
/// found at the official Pololu Micro Maestro
/// documentation pages, available
/// [here](https://www.pololu.com/docs/pdf/0J40/maestro.pdf).
/// Information on the available serial commands,
/// as well as the specific protocols officially
/// supported (for each type of Maestro), is
/// available in section 5.e.
impl Maestro {
    /// Sets the target of the servo motor at the
    /// given channel with
    /// the given microseconds.
    ///
    /// Microsecond ranges can only be between
    /// `992microsecs` and `2000microsecs`.
    /// Any values outside of this range will
    /// return an error.
    ///
    /// # Example Usage
    /// ```
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let channel: Channels = Channels::C_0; // can be any arbitrary channel in the Channels enum
    /// let microsec = 1234u16; // can be any value between 992u16 and 2000u16
    ///
    /// m.set_target(channel, microsec);
    /// ```
    pub fn set_target(&mut self, channel: Channels, microsec: u16) -> Result<()> {
        if MIN_PWM <= microsec && microsec <= MAX_PWM {
            Ok(microsec << DATA_MULTIPLIER)
        } else {
            Err(Error::InvalidValue(microsec))
        }
        .and_then(move |payload| {
            self.write_channel_and_payload(CommandFlags::SET_TARGET, channel, payload)
        })
    }

    /// Sets the rotational speed of the servo
    /// motor at the given channel with the
    /// given speed.
    ///
    /// # Example Usage
    /// ```
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let channel: Channels = Channels::C_0; // can be any arbitrary channel in the Channels enum
    /// let speed = 10u16;
    ///
    /// m.set_speed(channel, speed);
    /// ```
    ///
    /// # TODO
    /// Search up the max speed value allowable.
    pub fn set_speed(&mut self, channel: Channels, speed: u16) -> Result<()> {
        self.write_channel_and_payload(CommandFlags::SET_SPEED, channel, speed)
    }

    /// Sets the rotational acceleration of the
    /// servo motor at the given channel with
    /// the given value.
    ///
    /// The acceleration can be any usigned 8-bit
    /// integer from `1u8` to `255u8`. An
    /// acceleration of `0u8` will command the
    /// Maestro to reject the request.
    ///
    /// # Example Usage
    /// ```
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let channel: Channels = Channels::C_0; // can be any arbitrary channel in the Channels enum
    /// let acceleration = 10u8;
    ///
    /// m.set_acceleration(channel, acceleration);
    /// ```
    ///
    /// # TODO:
    /// Check if the Maestro actually rejects the
    /// request or just doesn't move if `0u8`
    /// is sent as the acceleration.
    pub fn set_acceleration(&mut self, channel: Channels, acceleration: u8) -> Result<()> {
        self.write_channel_and_payload(CommandFlags::SET_ACCELERATION, channel, acceleration as u16)
    }

    /// Sends all servos to home position.
    ///
    /// Home position is defined as
    /// `992microsecs`.
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
    pub fn go_home(&mut self) -> Result<()> { self.write_command(CommandFlags::GO_HOME) }

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
    ///
    /// # TODO
    /// Find out how the Maestro implements
    /// `stop_script`.
    pub fn stop_script(&mut self) -> Result<()> { self.write_command(CommandFlags::STOP_SCRIPT) }

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
    /// # Example Usage
    /// ```ignore
    /// use raestro::prelude::*;
    ///
    /// let mut m = Maestro::new();
    /// m.start(BaudRates::BR_115200).unwrap();
    ///
    /// let channel: Channels = Channels::C_0; // can be any arbitrary channel in the Channels enum
    /// let position = 1234u16; // can be any value between 992u16 and 2000u16
    ///
    /// m.set_target(channel, position);
    ///
    /// let actual_position = m.get_position(channel).unwrap();
    ///
    /// assert_eq!(position, actual_position);
    /// ```
    pub fn get_position(self: &mut Self, channel: Channels) -> Result<u16> {
        let write_result = self.write_channel(CommandFlags::GET_POSITION, channel);

        self.read_after_writing(write_result)
            .map(move |result| result >> DATA_MULTIPLIER)
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
    pub fn get_errors(&mut self) -> Result<Vec<Errors>> {
        let write_result = self.write_command(CommandFlags::GET_ERRORS);

        self.read_after_writing(write_result)
            .map(Errors::into_errors)
    }
}

/// Private utility methods
///
/// All hidden from public documentation.
///
/// Provide basic utilities and abstracted
/// functionality to the rest of the program.
///
/// Please note that all methods in this `impl`
/// block operate on the assumption that
/// 'self.start()' has been called. Since these
/// are private methods, calls to these methods
/// can only be made by `Maestro` methods, *NOT*
/// public callers. Therefore, operating under the
/// assumptions that `self.start()` has been
/// called and panicking otherwise is appropriate.
/// Before calling these methods, please ensure
/// that `self.start()` has been called. This can
/// easily be checked by ensuring that
/// `self.uart`, `read_buf`, and `write_buf` are
/// the Some(_) variants.
impl Maestro {
    /// Reads the given number of bytes into
    /// `self.read_buf`.
    ///
    /// Please note that the `self.uart.read`
    /// method is being utilized to send the
    /// commands over `UART`. This command
    /// operates on a blocking read. Blocking
    /// duration is default set to
    /// `DEFAULT_BLOCKING_DURATION`.
    ///
    /// # Panics
    /// Panics if:
    /// * `length` is strictly greater than the
    ///   `BUFFER_SIZE`
    /// * `self.read_buf` array is the `None`
    ///   variant; in this case, the `self`
    ///   instance has NOT been initialized.
    /// * `self.uart` array is the `None` variant;
    ///   in this case, the `self` instance has
    ///   NOT been initialized.
    fn read(&mut self, length: usize) -> Result<()> {
        if length > BUFFER_SIZE {
            panic!();
        }

        let slice = &mut self.read_buf.as_mut().unwrap().as_mut()[0usize..length];

        self.uart
            .as_mut()
            .unwrap()
            .read(slice)
            .map_err(Error::from)
            .and_then(|bytes_read| {
                if bytes_read == length {
                    Ok(())
                } else {
                    Err(Error::FaultyRead {
                        actual_count: bytes_read,
                    })
                }
            })
    }

    /// Writes the given number of bytes over to
    /// the Maestro.
    ///
    /// The bytes that are being written are
    /// located in the `self.write_buf` array.
    /// This is the method that actually calls
    /// `self.uart.write`. Other methods in this
    /// `impl` block just write to
    /// `self.write_buf`, but do not actually send
    /// data over the `UART` pins.
    ///
    /// # Panics
    /// Panics if:
    /// * `length` is strictly greater than the
    ///   `BUFFER_SIZE`
    /// * `self.write_buf` array is the `None`
    ///   variant; in this case, the `self`
    ///   instance has NOT been initialized.
    /// * `self.uart` array is the `None` variant;
    ///   in this case, the `self` instance has
    ///   NOT been initialized.
    fn write(&mut self, length: usize) -> Result<()> {
        if !(MIN_WRITE_LENGTH..=BUFFER_SIZE).contains(&length) {
            panic!();
        }

        let slice = &self.write_buf.as_mut().unwrap().as_mut()[0usize..length];

        self.uart
            .as_mut()
            .unwrap()
            .write(slice)
            .map_err(Error::from)
            .and_then(|bytes_written| {
                if bytes_written == length {
                    Ok(())
                } else {
                    Err(Error::FaultyWrite {
                        actual_count: bytes_written,
                        expected_count: length,
                    })
                }
            })
    }

    /// Writes the given arguments into the
    /// appropriate place in `self.write_buf`.
    ///
    /// This method does not actually send the
    /// bytes over the `UART` pins. It just
    /// writes them into the correct place in the
    /// buffer and then calls `self.write`
    /// while passing in the desired length.
    ///
    /// # Panics
    /// Panics if:
    /// * `self.write_buf` array is the `None`
    ///   variant; in this case, the `self`
    ///   instance has NOT been initialized.
    #[inline]
    fn write_channel_and_payload(
        &mut self,
        command_flag: CommandFlags,
        channel: Channels,
        microsec: u16,
    ) -> Result<()> {
        let length_to_write = 6usize;

        let command = mask_byte(command_flag as u8);
        let (lower, upper) = microsec_to_target(microsec);

        let buffer = self.write_buf.as_mut().unwrap().as_mut();

        buffer[2usize] = command;
        buffer[3usize] = channel as u8;
        buffer[4usize] = lower;
        buffer[5usize] = upper;

        self.write(length_to_write)
    }

    /// Writes the given arguments into the
    /// appropriate place in `self.write_buf`.
    ///
    /// This method does not actually send the
    /// bytes over the `UART` pins. It just
    /// writes them into the correct place in the
    /// buffer and then calls `self.write`
    /// while passing in the desired length.
    ///
    /// # Panics
    /// Panics if:
    /// * `self.write_buf` array is the `None`
    ///   variant; in this case, the `self`
    ///   instance has NOT been initialized.
    #[inline]
    fn write_channel(&mut self, command_flag: CommandFlags, channel: Channels) -> Result<()> {
        let length_to_write = 4usize;

        let command = mask_byte(command_flag as u8);

        let buffer = self.write_buf.as_mut().unwrap().as_mut();

        buffer[2usize] = command;
        buffer[3usize] = channel as u8;

        self.write(length_to_write)
    }

    /// Writes the given arguments into the
    /// appropriate place in `self.write_buf`.
    ///
    /// This method does not actually send the
    /// bytes over the `UART` pins. It just
    /// writes them into the correct place in the
    /// buffer and then calls `self.write`
    /// while passing in the desired length.
    ///
    /// # Panics
    /// Panics if:
    /// * `self.write_buf` array is the `None`
    ///   variant; in this case, the `self`
    ///   instance has NOT been initialized.
    #[inline]
    fn write_command(&mut self, command_flag: CommandFlags) -> Result<()> {
        let length_to_write = 3usize;

        let command = mask_byte(command_flag as u8);

        let buffer = self.write_buf.as_mut().unwrap().as_mut();

        buffer[2usize] = command;

        self.write(length_to_write)
    }

    /// Utility function to take the first two
    /// bytes in `self.read_buf` and convert
    /// them from Pololu
    /// standardized-return-form to u16.
    ///
    /// # Panics
    /// Panics if:
    /// * `self.read_buf` array is the `None`
    ///   variant; in this case, the `self`
    ///   instance has NOT been initialized.
    #[inline]
    fn prepare_data_from_buffer(&mut self) -> u16 {
        let buf = self.read_buf.as_mut().unwrap().as_mut();

        let data: u16 = ((buf[1usize] as u16) << 8usize) | (buf[0usize] as u16);

        data
    }

    /// Takes the write result and applies
    /// immediately calls for a read after.
    ///
    /// Useful abstraction over Pololu protocols
    /// that send data back right after a
    /// request. For example, a request to
    /// `get_position` will require first a
    /// `write`, and then immediately a
    /// `read_after_writing`. Therefore, for those
    /// types of situations, use this method.
    fn read_after_writing(&mut self, write_result: Result<()>) -> Result<u16> {
        write_result
            .and_then(|()| self.read(RESPONSE_SIZE as usize))
            .map(|()| self.prepare_data_from_buffer())
    }
}
