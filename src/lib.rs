#![allow(non_snake_case)]
#![allow(unused)]

pub mod maestro;
pub mod maestro_constants;
mod utils;

#[cfg(test)]
mod tests {
    // External Uses
    use std::error::Error;
    use std::thread;
    use std::time::Duration;

    // Internal Uses
    use crate::maestro::Maestro;
    use crate::maestro_constants::{
        Channels,
        BaudRates,
    };

    #[test]

    fn init_and_close() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.start(BaudRates::BR_115200);
        maestro.close();
// =======
//     fn lights() -> () {
//         // println!("Blinking an LED on a {}.", super::DeviceInfo::new().unwrap().model());
//         let mut uart: Uart = Uart::new(115_200, Parity::None, 8u8, 1u8).unwrap();
//         // uart.set_baud_rate(115_200u32);
//         uart.set_baud_rate(50u32);
//         uart.send_start();

//         let buffer: &[u8; 1usize] = &[0x01u8];

//         loop {
//             uart.write(buffer);
//         }

//         // Blink the LED by setting the pin's logic level high for 500 ms.
//         // uart.set_high();
//         // thread::sleep(Duration::from_millis(500));
//         // uart.set_low();

//         // Ok(())  
// >>>>>>> master
    }
}
