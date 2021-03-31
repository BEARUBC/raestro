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
use crate::maestro_commands::*;

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
        const DATA_BITS: u8 = 8u8;
        const STOP_BITS: u8 = 1u8;
        const BUFFER_SIZE: usize = 6usize;

        let uart_result: RppalResult<Uart> = Uart::new(
            baud_rate as u32,
            Parity::None,
            DATA_BITS,
            STOP_BITS,
        );

        return match uart_result {
            Ok(uart) => {
                self.uart = Some(Box::new(uart));
                self.read_buf = Some(Box::new([0u8; BUFFER_SIZE]));
                self.write_buf = Some(Box::new([0u8; BUFFER_SIZE]));

                let buf = self.write_buf.as_mut().unwrap().as_mut();

                buf[0usize] = ProtocolMetadata::SYNC as u8;
                buf[1usize] = ProtocolMetadata::DEVICE_NUMBER as u8;

                Ok(())
            },
            Err(rppal_err) => Err(Maestro::deconstruct_error(rppal_err)),
        };
    }

    pub fn close(self: &mut Self) -> () {
        self.uart = None;
        self.read_buf = None;
        self.write_buf = None;
    }

    pub fn get_read_buffer(self: &Self) -> Option<Box<[u8; 6usize]>> {
        return self.read_buf.clone();
    }

    fn read(self: &mut Self, length: usize) -> Result<usize, Error> {
        if length <= 2usize {
            panic!();
        }
        
        let slice = &mut self.write_buf
            .as_mut()
            .unwrap()
            .as_mut()[0usize..length];

        return self.uart.as_mut().unwrap().read(slice)
            .map(|bytes_written| bytes_written)
            .map_err(|rppal_err| Maestro::deconstruct_error(rppal_err));
    }

    fn write(self: &mut Self, length: usize) -> Result<usize, Error> {
        if length <= 2usize {
            panic!();
        }

        let slice = &self.write_buf
            .as_mut()
            .unwrap()
            .as_mut()[0usize..length];

        return self.uart.as_mut().unwrap().write(slice)
            .map(|bytes_written| bytes_written)
            .map_err(|rppal_err| Maestro::deconstruct_error(rppal_err));
    }

    #[inline]
    fn write_channel_and_payload(
        self: &mut Self,
        command: u8,
        channel: Channels,
        payload_0: u8,
        payload_1: u8
    ) -> UnitResultType {
        let buffer = self.write_buf
            .as_mut()
            .unwrap()
            .as_mut();

        buffer[2usize] = command;
        buffer[3usize] = channel as u8;
        buffer[4usize] = payload_0;
        buffer[5usize] = payload_1;

        return self.write(6usize)
            .map(|_| ());

        // let buffer_size = 6usize;

        // let buffer: [u8; 6usize] = [
        //     ProtocolMetadata::SYNC as u8,
        //     ProtocolMetadata::DEVICE_NUMBER as u8,
        //     command,
        //     channel as u8,
        //     payload_0,
        //     payload_1,
        // ];

        // return match self.write_buf(&buffer) {
        //     Ok(bytes_written) => match bytes_written == buffer_size {
        //         true => Ok(()),
        //         _ => {
        //             let err_type = ErrorKind::ConnectionAborted;
        //             let err_msg = "maestro protocol could not be sent";

        //             Err(Error::new(err_type, err_msg))
        //         },
        //     },
        //     Err(err) => Err(err),
        // };

        // return match &mut self.write_buf {
        //     Some(buffer) => {
        //         buffer[2usize] = command;
        //         buffer[3usize] = channel as u8;
        //         buffer[4usize] = lower;
        //         buffer[5usize] = upper;

        //         return self.write(6usize)
        //             .map(|_| ());
        //     },
        //     _ => {
        //         let err_type = ErrorKind::NotConnected;
        //         let err_msg = "Maestro not initialized. Consider calling .start()";

        //         Err(Error::new(err_type, err_msg))
        //     },
        // };
    }

    #[inline]
    fn write_channel(self: &mut Self, command: u8, channel: Channels) -> UnitResultType {
        let buffer = self.write_buf
            .as_mut()
            .unwrap()
            .as_mut();

        buffer[2usize] = command;
        buffer[3usize] = channel as u8;

        return self.write(4usize)
            .map(|_| ());
    }

    #[inline]
    fn write_command(self: &mut Self, command: u8) -> UnitResultType {
        let buffer = self.write_buf
            .as_mut()
            .unwrap()
            .as_mut();

        buffer[2usize] = command;

        return self.write(3usize)
            .map(|_| ());
    }

    fn deconstruct_error(rppal_err: UartError) -> Error {
        return match rppal_err {
            UartError::Io(std_err) => std_err,
            UartError::Gpio(gpio_err) => match gpio_err {
                GpioError::UnknownModel => Error::new(ErrorKind::Other, "unknown model"),
                GpioError::PinNotAvailable(pin) => Error::new(ErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin)),
                GpioError::PermissionDenied(err_string) => Error::new(ErrorKind::PermissionDenied, format!("permission denied: {} ", err_string)),
                GpioError::Io(error) => error,
                GpioError::ThreadPanic => Error::new(ErrorKind::Other, "thread_buf panic"),
            },
            UartError::InvalidValue => Error::new(ErrorKind::Other, "invalid value"),
        };
    }
}

impl MaestroCommands for Maestro {
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> UnitResultType {
        let command = mask_byte(CommandFlags::SET_TARGET as u8);
        let (lower, upper) = microsec_to_target(microsec);

        return match self.write_buf {
            Some(_) => self.write_channel_and_payload(command, channel, lower, upper),
            _ => todo!(),
        };
    }

    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> UnitResultType {
        let command = mask_byte(CommandFlags::SET_SPEED as u8);
        let (lower, upper) = microsec_to_target(microsec);

        return match self.write_buf {
            Some(_) => self.write_channel_and_payload(command, channel, lower, upper),
            _ => todo!(),
        };
    }

    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> UnitResultType {
        let command = mask_byte(CommandFlags::SET_ACCELERATION as u8);
        let (lower, upper) = microsec_to_target(value as u16);

        return match self.write_buf {
            Some(_) => self.write_channel_and_payload(command, channel, lower, upper),
            _ => todo!(),
        };
    }

    fn go_home(self: &mut Self) -> UnitResultType {
        let command = mask_byte(CommandFlags::GO_HOME as u8);

        return match self.write_buf {
            Some(_) => self.write_command(command),
            _ => todo!(),
        }
    }

    fn stop_script(self: &mut Self) -> UnitResultType {
        let command = mask_byte(CommandFlags::STOP_SCRIPT as u8);

        return match self.write_buf {
            Some(_) => self.write_command(command),
            _ => todo!(),
        }
    }

    // #[allow(unused)]
    fn get_position(self: &mut Self, channel: Channels) -> SliceResultType {
        const RESPONSE_SIZE: usize = 2usize;
        let command = mask_byte(CommandFlags::GET_POSITION as u8);

        return match self.write_buf {
            Some(_) => match self.write_channel(command, channel) {
                Ok(()) => match self.read(RESPONSE_SIZE) {
                    Ok(bytes_read) => {
                        match bytes_read == RESPONSE_SIZE {
                            true => {
                                let slice = &self.read_buf
                                    .as_mut()
                                    .unwrap()
                                    .as_mut()[0usize..(bytes_read - 1usize)];

                                Ok(slice)
                            },
                            _ => {
                                let err_type = ErrorKind::ConnectionAborted;
                                let err_msg = "maestro message could not be read";

                                Err(Error::new(err_type, err_msg))
                            },
                        }
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            _ => todo!(),
        };
    }

    fn get_errors(self: &mut Self) -> SliceResultType {
        const RESPONSE_SIZE: usize = 2usize;
        let command = mask_byte(CommandFlags::GET_POSITION as u8);

        return match self.write_buf {
            Some(_) => match self.write_command(command) {
                Ok(()) => match self.read(RESPONSE_SIZE) {
                    Ok(bytes_read) => {
                        match bytes_read == RESPONSE_SIZE {
                            true => {
                                let slice = &self.read_buf
                                    .as_mut()
                                    .unwrap()
                                    .as_mut()[0usize..(bytes_read - 1usize)];

                                Ok(slice)
                            },
                            _ => {
                                let err_type = ErrorKind::ConnectionAborted;
                                let err_msg = "maestro message could not be read";

                                Err(Error::new(err_type, err_msg))
                            },
                        }
                    },
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            _ => todo!(),
        };
    }
}
