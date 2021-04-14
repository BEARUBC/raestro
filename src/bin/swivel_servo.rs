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

    let mut position = 5_000u16;
    let sleep_time: u64 = 200u64;

    loop {
        println!("{}", position);
        maestro.set_target(Channels::C_0, position).unwrap();
        thread::sleep(Duration::from_millis(sleep_time));

        position += 100u16;
        if position == 8_000u16 {
            position = 5_000u16;
        }
    }
}
