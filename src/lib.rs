#![allow(non_snake_case)]
#![allow(unused)]

mod gpio;
mod maestro;
mod uart_metadata;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_UART_TX: u8 = 14u8;
// const GPIO_UART_RX: u8 = 15u8;
// const BAUD_RATE: u32 = 50u32;
// const DATA_BITS: u8 = 8u8;
// const STOP_BITS: u8 = 1u8;

#[cfg(test)]
mod tests {
    use crate::maestro::Maestro;
    use crate::uart_metadata::{
        BaudRate,
        Channel,
    };
    use std::error::Error;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn maestro_initialization() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.initialize(BaudRate::BR_115200);

        let small: u16 = 500u16;
        let big: u16 = 2500u16;
        let sleep_time: u64 = 1000u64;

        loop {
            maestro.set_target(Channel::C_0, small);
            thread::sleep(Duration::from_millis(sleep_time));

            maestro.set_target(Channel::C_0, big);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}
