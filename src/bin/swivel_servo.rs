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

    // 0x0090u16 seems to be the minimum with 4x multiplier
    // 0x0200u16 seems to be the maximum with 4x multiplier

    // let p0: u16 = 1500u16;
    // let p1: u16 = 6000u16;
    // let p2: u16 = 8000u16;
    // let p3: u16 = 6000u16;
    // let sleep_time: u64 = 1000u64;

    // loop {
    //     println!("{}", p0);
    //     maestro.set_target(Channels::C_0, p0).unwrap();
    //     thread::sleep(Duration::from_millis(sleep_time));
    //     println!();

    //     println!("{}", p1);
    //     maestro.set_target(Channels::C_0, p1).unwrap();
    //     thread::sleep(Duration::from_millis(sleep_time));
    //     println!();

    //     println!("{}", p2);
    //     maestro.set_target(Channels::C_0, p2).unwrap();
    //     thread::sleep(Duration::from_millis(sleep_time));
    //     println!();

    //     println!("{}", p3);
    //     maestro.set_target(Channels::C_0, p3).unwrap();
    //     thread::sleep(Duration::from_millis(sleep_time));
    //     println!();
    // }

    let mut position = 5_000u16;
    let a = 5_000u16;
    let b = 8_000u16;
    let sleep_time: u64 = 200u64;

    loop {
        println!("{}", position);
        maestro.set_target(Channels::C_0, position).unwrap();
        thread::sleep(Duration::from_millis(sleep_time));

        // position += 100u16;
        // if position == 8_000u16 {
        //     position = 5_000u16;
        // }

        if position == a {
            position = b;
        } else if position == b {
            position = a;
        }
    }
}
