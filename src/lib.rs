use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::uart::Parity;
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
    fn lights() {
        println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

        let uart = Uart::new(115_200, Parity::None, 8, 1)?;

        // Blink the LED by setting the pin's logic level high for 500 ms.
        uart.set_high();
        thread::sleep(Duration::from_millis(500));
        uart.set_low();

        Ok(())
    }
}
