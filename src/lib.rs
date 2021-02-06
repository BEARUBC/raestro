#![allow(non_snake_case)]
#![allow(unused)]

use std::error::Error;
use std::thread;
use std::time::Duration;

mod gpio;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_UART_TX: u8 = 14u8;
// const GPIO_UART_RX: u8 = 15u8;
// const BAUD_RATE: u32 = 50u32;
// const DATA_BITS: u8 = 8u8;
// const STOP_BITS: u8 = 1u8;

#[cfg(test)]
mod tests {
    use rppal::{
        uart::{
            Parity,
            Uart,
            Result,
        },
        system::DeviceInfo,
    };
    use super::gpio::{
        UartMetaData,
        BUFFER,
    };
 
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn lights() -> () {
        // println!("Blinking an LED on a {}.", super::DeviceInfo::new().unwrap().model());
        let mut uart: Uart = Uart::new(
            UartMetaData::BAUDRATE as u32,
            Parity::None,
            UartMetaData::DATABITS as u8,
            UartMetaData::STOPBITS as u8
        )
            .unwrap();

        uart.send_start();

        loop {
            uart.write(&BUFFER);
        }

        // Blink the LED by setting the pin's logic level high for 500 ms.
        // uart.set_high();
        // thread::sleep(Duration::from_millis(500));
        // uart.set_low();

        // Ok(())
    }
}
