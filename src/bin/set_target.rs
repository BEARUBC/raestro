// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/* external crates */

/* external uses */
use std::{
    thread,
    time::Duration,
};
use raestro::prelude::*;

/* internal mods */

/* internal uses */

fn main() -> () {
    let mut maestro: Maestro = Maestro::new();
    maestro.start(BaudRates::BR_115200).unwrap();

    let pos_min = 992u16;
    let pos_max = 2000u16;
    let sleep_time: u64 = 1000u64;

    loop {
        maestro.set_target(Channels::C_0, pos_min).unwrap();
        thread::sleep(Duration::from_millis(sleep_time));

        maestro.set_target(Channels::C_0, pos_max).unwrap();
        thread::sleep(Duration::from_millis(sleep_time));
    }
}
