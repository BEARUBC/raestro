// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::convert::TryInto;
use std::thread;
use std::time::Duration;

use raestro::maestro::builder::Builder;
use raestro::maestro::constants::Baudrate;
use raestro::maestro::constants::Channels;
use raestro::maestro::constants::MAX_QTR_PWM;
use raestro::maestro::constants::MIN_QTR_PWM;
use raestro::maestro::Maestro;

fn main() -> ! {
    let mut maestro: Maestro = Builder::default()
        .baudrate(Baudrate::Baudrate11520)
        .block_duration(Duration::from_millis(100))
        .try_into()
        .unwrap();
    let channel = Channels::Channel0;
    let pos_min = MIN_QTR_PWM;
    let pos_max = MAX_QTR_PWM;
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
