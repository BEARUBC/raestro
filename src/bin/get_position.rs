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

    let small: u16 = 500u16;
    let big: u16 = 60000u16;
    let sleep_time: u64 = 1000u64;

    loop {
        maestro.set_target(Channels::C_0, small).unwrap();
        let arr = maestro.get_position(Channels::C_0).unwrap();

        println!("position: {:?}", arr);

        thread::sleep(Duration::from_millis(sleep_time));

        maestro.set_target(Channels::C_0, big).unwrap();
        let arr = maestro.get_position(Channels::C_0).unwrap();

        println!("position: {:?}", arr);

        thread::sleep(Duration::from_millis(sleep_time));
    }
}
