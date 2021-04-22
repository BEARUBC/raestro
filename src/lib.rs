// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external crates */

/* external uses */

/* internal mods */
pub mod commands;
pub mod constants;
pub mod maestro;
pub mod prelude;
mod utils;
#[cfg(test)]
mod tests {
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

/* internal uses */
