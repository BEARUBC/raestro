#![allow(non_snake_case)]
#![allow(unused)]

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::uart::{
    Parity,
    Uart,
};
use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_UART_TX: u8 = 14;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn lights() -> () {
        println!("Blinking an LED on a {}.", super::DeviceInfo::new().unwrap().model());
        let uart = super::Uart::new(115_200, super::Parity::None, 8, 1);
        
        match uart {
            Ok(_) => println!("uart init work"),
            Err(_) => println!("uart init failed"),
        };
        // Blink the LED by setting the pin's logic level high for 500 ms.
        // uart.set_high();
        // thread::sleep(Duration::from_millis(500));
        // uart.set_low();

        // Ok(())
    }
}
