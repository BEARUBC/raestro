// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

// external crates

// external uses
use std::thread;
use std::time::Duration;

use raestro::prelude::*;

// internal mods

// internal uses

fn main() -> () {
    let mut maestro: Maestro = Maestro::new();
    maestro.start(BaudRates::BR_115200).unwrap();

    let channel0: Channels = Channels::C_0;
    let channel1: Channels = Channels::C_1;
    let channel2: Channels = Channels::C_2;

    let pos_min = 3968u16;
    let pos_max = 8000u16;

    let sleep_time = Duration::from_millis(1000u64);

    loop {
        maestro.set_target(channel0, pos_min).unwrap();
        maestro.set_target(channel1, pos_min).unwrap();
        maestro.set_target(channel2, pos_min).unwrap();
        thread::sleep(sleep_time);

        maestro.set_target(channel0, pos_max).unwrap();
        maestro.set_target(channel1, pos_max).unwrap();
        maestro.set_target(channel2, pos_max).unwrap();
        thread::sleep(sleep_time);
    }
}
