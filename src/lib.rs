/* external crates */

/* external uses */

/* internal mods */
pub mod maestro_commands;
pub mod maestro;
pub mod maestro_constants;
mod utils;
pub mod prelude;

/* internal uses */

#[cfg(test)]
mod lib_tests {
    /* external crates */

    /* external uses */

    /* internal mods */

    /* internal uses */
    use crate::maestro::Maestro;
    use crate::maestro_constants::BaudRates;

    #[test]
    fn init_and_close() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.start(BaudRates::BR_115200).unwrap();
        maestro.close();
    }
}
