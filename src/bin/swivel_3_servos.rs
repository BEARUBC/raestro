use std::thread;
use std::time::Duration;

use rustro::maestro::*;
use rustro::maestro_constants::*;

fn main() -> () {
    let mut maestro: Maestro = Maestro::new();
    maestro.start(BaudRates::BR_115200).unwrap();

    let small: u16 = 500u16;
    let big: u16 = 60000u16;
    let sleep_time: u64 = 1000u64;

    loop {
        maestro.set_target(Channels::C_0, small).unwrap();
        maestro.set_target(Channels::C_2, small).unwrap();
        maestro.set_target(Channels::C_4, small).unwrap();

        thread::sleep(Duration::from_millis(sleep_time));

        maestro.set_target(Channels::C_0, big).unwrap();
        maestro.set_target(Channels::C_2, big).unwrap();
        maestro.set_target(Channels::C_4, big).unwrap();

        thread::sleep(Duration::from_millis(sleep_time));
    }
}
