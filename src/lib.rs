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
    }
}
