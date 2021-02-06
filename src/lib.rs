#![allow(non_snake_case)]
#![allow(unused)]

use std::error::Error;
use std::thread;
use std::time::Duration;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_UART_TX: u8 = 14u8;

const BAUD_RATE: u32 = 50u32;
const DATA_BITS: u8 = 8u8;
const STOP_BITS: u8 = 1u8;
const BUFFER: &[u8; 1usize] = &[0x01u8];

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
 
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn lights() -> () {
        // println!("Blinking an LED on a {}.", super::DeviceInfo::new().unwrap().model());
        let mut uart: Uart = Uart::new(BAUD_RATE, Parity::None, DATA_BITS, STOP_BITS).unwrap();
        // uart.set_baud_rate(115_200u32);
        // uart.set_baud_rate(50u32);
        // uart.set_data_bits(8u8);
        // uart.set_stop_bits(1u8);
        uart.send_start();

        loop {
            uart.write(BUFFER);
        }

        // Blink the LED by setting the pin's logic level high for 500 ms.
        // uart.set_high();
        // thread::sleep(Duration::from_millis(500));
        // uart.set_low();

        // Ok(())
    }
}
