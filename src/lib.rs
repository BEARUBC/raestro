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

    use crate::utils::{
        mask_byte,
        short_to_target,
    };

    #[test]

    fn init_and_close() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.start(BaudRates::BR_115200);
        maestro.close();
    }

    #[test]
    fn mask_byte_test_0() -> () {
        assert_eq!(mask_byte(5),5); 
    }

    #[test]
    fn mask_byte_test_1() -> () {
        assert_eq!(mask_byte(255),127)
    }

    #[test]
    fn short_to_target_test_0() -> () {
        assert_eq!(short_to_target(0), (0,0));
    }
}