/* external crates */

/* external uses */

/* internal mods */
pub mod maestro_commands;
pub mod maestro;
pub mod maestro_constants;
mod utils;

/* internal uses */

#[cfg(test)]
mod lib_tests {
    /* external crates */

    /* external uses */

    /* internal mods */

    /* internal uses */
    use crate::maestro::Maestro;
    use crate::maestro_constants::BaudRates;

    use crate::utils::{
        mask_byte,
        short_to_target,
    };

    #[test]
    fn init_and_close() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.start(BaudRates::BR_115200).unwrap();
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