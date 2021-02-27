/* external crates */

/* external uses */

/* internal mods */
pub mod maestro_commands;
pub mod maestro;
pub mod maestro_constants;
mod utils;

/* internal uses */

#[cfg(test)]
mod tests {
    // External Uses

    // Internal Uses
    use crate::maestro::Maestro;
    use crate::maestro_constants::BaudRates;

    #[test]
    fn init_and_close() -> () {
        let mut maestro: Maestro = Maestro::new();
        let result = maestro.start(BaudRates::BR_115200);

        match result {
            Ok(_) => (),
            Err(_) => panic!(),
        };

        maestro.close();
    }
}
