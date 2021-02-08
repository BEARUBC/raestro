#![allow(non_snake_case)]
#![allow(unused)]

mod maestro;
mod maestro_constants;
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
    fn maestro_initialization() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.initialize(BaudRates::BR_115200);

        let small: u16 = 500u16;
        let big: u16 = 60000u16;
        let sleep_time: u64 = 1000u64;

        loop {
            maestro.set_target(Channels::C_0, small);
            thread::sleep(Duration::from_millis(sleep_time));

            maestro.set_target(Channels::C_0, big);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}
