/* external crates */

/* external uses */
use std::thread;
use std::time::Duration;
use raestro::prelude::*;

/* internal mods */

/* internal uses */

fn main() -> () {
    let mut maestro: Maestro = Maestro::new();
    maestro.start(BaudRates::BR_115200).unwrap();

    let slow: u8 = 10u8;
    let fast: u8 = 200u8;
    let small: u16 = 500u16;
    let big: u16 = 60000u16;
    let sleep_time: u64 = 1000u64;

    loop {
        maestro.set_acceleration(Channels::C_0, slow).unwrap();
        maestro.set_target(Channels::C_0, small).unwrap();

        thread::sleep(Duration::from_millis(sleep_time));

        maestro.set_acceleration(Channels::C_0, fast).unwrap();
        maestro.set_target(Channels::C_0, big).unwrap();

        thread::sleep(Duration::from_millis(sleep_time));
    }
}
