/* external crates */

/* external uses */
#[allow(unused_imports)]
use std::io::Error;
use std::io::ErrorKind;
use std::boxed::Box;
use rppal::{
    uart::{
        Parity,
        Uart,
        Result as RppalResult,
        Error as RppalError,
        Error::{
            Io,
            Gpio,
            InvalidValue
        }
    },
};
#[allow(unused_imports)]
use std::sync::{
    Arc,
    Mutex,
};

/* internal mods */

/* internal uses */
#[allow(unused_imports)]
use crate::utils::*;
use crate::maestro_constants::*;
use crate::maestro_commands::MaestroCommands;

// const W_BUFFER_3: Arc<Mutex<[u8; 6usize]>> = Arc::new(Mutex::new([0u8; 6usize]));
// const W_BUFFER_4: [u8; 6usize] = [0u8] * 6usize;
// const W_BUFFER_6: [u8; 6usize] = [0u8] * 6usize;

// const R_BUFFER: [U8, 256usize] = [0u8] * 256usize;

pub struct Maestro {
    uart: Option<Box<Uart>>,
}

impl Maestro {
    pub fn new() -> Self {
        return Maestro {
            uart: None,
        };
    }
    
    // pub fn deconstructError(rppal_err: RppalError) -> Error {
    //     // err_msg is of type RppalError
    //     return match rppal_err {
    //         Io(std_err) => Err(std_err),
    //         Gpio(gpio_err) => {
    //             return match gpio_err {
    //                 rppal::gpio::Error::UnknownModel => Err(Error::new(ErrorKind::Other, "unknown model")),
    //                 rppal::gpio::Error::PinNotAvailable(pin) => Err(Error::new(ErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin))),
    //                 rppal::gpio::Error::PermissionDenied(err_string) => Err(Error::new(ErrorKind::PermissionDenied, format!("Permission denied: {} ", err_string))),
    //                 rppal::gpio::Error::Io(error) => Err(error),
    //                 rppal::gpio::Error::ThreadPanic => Err(Error::new(ErrorKind::Other, "Thread panic")),
    //             }
    //         },
    //         InvalidValue => Err(Error::new(ErrorKind::Other, "Invalid Value")),
    //     }
    // }

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
            Ok(uart) => {
                self.uart = Some(Box::new(uart));
                Ok(())
            },
            // Err(err_msg) => Err(err_msg),
            Err(rppal_err) => {
                // err_msg is of type RppalError
                return match rppal_err {
                    Io(std_err) => Err(std_err),
                    Gpio(gpio_err) => {
                        return match gpio_err {
                            rppal::gpio::Error::UnknownModel => Err(Error::new(ErrorKind::Other, "unknown model")),
                            rppal::gpio::Error::PinNotAvailable(pin) => Err(Error::new(ErrorKind::AddrNotAvailable, format!("pin number {} is not available", pin))),
                            rppal::gpio::Error::PermissionDenied(err_string) => Err(Error::new(ErrorKind::PermissionDenied, format!("Permission denied: {} ", err_string))),
                            rppal::gpio::Error::Io(error) => Err(error),
                            rppal::gpio::Error::ThreadPanic => Err(Error::new(ErrorKind::Other, "Thread panic")),
                        }
                    },
                    InvalidValue => Err(Error::new(ErrorKind::Other, "Invalid Value")),
                };
            },
        };
    }

    pub fn close(self: &mut Self) -> () {
        match &self.uart {
            Some(_) => self.uart = None,
            None => (),
        };
    }

    #[allow(unused)]
    fn write(self: &mut Self, buffer: &[u8]) -> Result<usize, RppalError> {
        if let Some(boxed_uart) = &mut self.uart {
            let result: RppalResult<usize> = (*boxed_uart).write(buffer);
            // let result: Result<usize> = (*boxed_uart).write(&BUFFER);

            return match result {
                Ok(bits_read) => Ok(bits_read),
                Err(err_msg) => Err(err_msg),
            };
        } else {
            // return Ok(0usize);
            // return Error::SER_SIGNAL_ERR;
            todo!();
        }
    }

    #[allow(unused)]
    fn read(self: &mut Self) -> Result<usize, RppalError> {
        if let Some(boxed_uart) = &mut self.uart {
            let mut buffer: [u8;2] = [0,0];
            let result: RppalResult<usize> = (*boxed_uart).read(&mut buffer);
            // let result: Result<usize> = (*boxed_uart).write(&BUFFER);

            return match result {
                Ok(bits_read) => Ok(result.unwrap()),
                Err(err_msg) => Err(err_msg),
            };
        } else {
            return Ok(0usize);
        }
    }

    #[allow(unused)]
    #[inline]
    fn write_two(self: &mut Self, command: u8, channel: Channels, payload_0: u8, payload_1: u8) -> Result<usize, RppalError> {
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
    fn write_one_channel(self: &mut Self, command: u8, channel: Channels) -> Result<usize, RppalError> {
        let buffer: [u8; 4usize] = [
            ProtocolMetadata::SYNC as u8,
            ProtocolMetadata::DEVICE_NUMBER as u8,
            command,
            channel as u8
        ];

        return self.write(&buffer);
    }

    #[allow(unused)]
    #[inline]
    fn write_one(self: &mut Self, command: u8) -> Result<usize, RppalError> {
        let buffer: [u8; 3usize] = [
            ProtocolMetadata::SYNC as u8,
            ProtocolMetadata::DEVICE_NUMBER as u8,
            command
        ];

        return self.write(&buffer);
    }

    #[allow(unused)]
    fn dispatcher(self: &mut Self, command: CommandFlags, channel: Channels, payload_0: u8, payload_1: u8, microsec: u16) -> Result<usize, RppalError> {
        // let commandCopy: crate::maestro_constants::CommandFlags = command.clone();
        // let masked_command: u8 = mask_byte(command as u8);
        // let (lower, upper): (u8, u8) = microsec_to_target(microsec);
        

        // match command {
        //     CommandFlags::SET_TARGET => { return self.write_two(masked_command, channel, lower, upper); },
        //     CommandFlags::SET_SPEED => { return self.write_two(masked_command, channel, lower, upper); },
        //     CommandFlags::SET_ACCELERATION => { return self.write_two(masked_command, channel, lower, upper); },
        //     CommandFlags::GET_POSITION => { 
        //         self.write_one_channel(masked_command, channel).unwrap();
        //         return self.read(); 
        //     },
        //     CommandFlags::GET_ERRORS => {
        //         self.write_one(masked_command);

        //         match self.read() {
        //             Err(e) => Err(e),
        //             Ok(0) => Ok(Errors::SER_SIGNAL_ERR as usize),
        //             Ok(1) => Ok(Errors::SER_OVERRUN_ERR as usize),
        //             Ok(2) => Ok(Errors::SER_BUFFER_FULL as usize),
        //             Ok(3) => Ok(Errors::SER_CRC_ERR as usize),
        //             Ok(4) => Ok(Errors::SER_PROTOCOL_ERR as usize),
        //             Ok(5) => Ok(Errors::SER_TIMEOUT as usize),
        //             Ok(6) => Ok(Errors::SCRIPT_STACK_ERR as usize),
        //             Ok(7) => Ok(Errors::SCRIPT_CALL_STACK_ERR as usize),
        //             Ok(8) => Ok(Errors::SCRIPT_PC_ERR as usize),
        //             Ok(_) => std::io::Error{_},
        //             _ => {},
        //         }
        //     },
        //     CommandFlags::GO_HOME => { return self.write_one(masked_command); },
        //     CommandFlags::STOP_SCRIPT => { return self.write_one(masked_command); },
        //     _ => Ok(1),
        // }

        todo!();
    }
}

impl MaestroCommands for Maestro {
    #[allow(unused)]
    fn set_target(self: &mut Self, channel: Channels, microsec: u16) -> Result<usize, RppalError> {
        todo!();
        // let (lower, upper): (u8, u8) = microsec_to_target(microsec);
        // return self.dispatcher(CommandFlags::SET_TARGET, channel, lower, upper, microsec);
    }

    #[allow(unused)]
    fn set_speed(self: &mut Self, channel: Channels, microsec: u16) -> Result<usize, RppalError> {
        todo!();
        // let (lower, upper): (u8, u8) = microsec_to_target(microsec);
        // return self.dispatcher(CommandFlags::SET_SPEED, channel, lower, upper, microsec);
    }

    #[allow(unused)]
    fn set_acceleration(self: &mut Self, channel: Channels, value: u8) -> Result<usize, RppalError> {
        todo!();
        // let (lower, upper): (u8, u8) = microsec_to_target(value.into());
        // return self.dispatcher(CommandFlags::SET_ACCELERATION, channel, lower, upper, value.into());
    }

    #[allow(unused)]
    fn get_position(self: &mut Self, channel: Channels) -> Result<usize, RppalError> {
        todo!();
        // return self.dispatcher(CommandFlags::GET_POSITION, channel, 0, 0, 0);
    }

    #[allow(unused)]
    fn get_errors(self: &mut Self) -> Result<usize, RppalError> {
        todo!();
        // return self.dispatcher(CommandFlags::GET_ERRORS, Channels::C_0, 0, 0, 0);
    }

    #[allow(unused)]
    fn go_home(self: &mut Self) -> Result<usize, RppalError> {
        todo!();
        // return self.dispatcher(CommandFlags::GO_HOME, Channels::C_0, 0, 0, 0);
    }

    #[allow(unused)]
    fn stop_script(self: &mut Self) -> Result<usize, RppalError> {
        todo!();
        // return self.dispatcher(CommandFlags::STOP_SCRIPT, Channels::C_0, 0, 0, 0);
    }
}
