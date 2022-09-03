// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::convert::TryInto;
use std::thread;
use std::time::Duration;

use raestro::maestro;

fn main() -> ! {
    let mut maestro: maestro::Maestro = maestro::builder::Builder::default()
        .baudrate(maestro::constants::Baudrate::Baudrate11520)
        .block_duration(Duration::from_millis(100))
        .try_into()
        .unwrap();
    let channel = maestro::constants::Channels::Channel0;
    let pos_min = maestro::constants::MIN_QTR_PWM;
    let pos_max = maestro::constants::MAX_QTR_PWM;
    let speed0 = 10u16;
    let speed1 = 140u16;
    let sleep_duration = Duration::from_secs(1);
    loop {
        maestro.set_speed(channel, speed0).unwrap();
        maestro.set_target(channel, pos_min).unwrap();
        thread::sleep(sleep_duration);
        maestro.set_speed(channel, speed1).unwrap();
        maestro.set_target(channel, pos_max).unwrap();
        thread::sleep(sleep_duration);
    }
}
