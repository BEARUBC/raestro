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

    let positions: [u16; 2usize] = [992u16, 2000u16];
    let sleep_time: u64 = 1000u64;

    #[allow(unused_assignments)]
    let mut arr: Option<u16> = None;

    loop {
        for (index, position) in positions.iter().enumerate() {
	        maestro.set_target(Channels::C_0, *position).unwrap();
	        arr = Some(maestro.get_position(Channels::C_0).unwrap());
	
	        println!("position_{}: {:?}", index, arr.unwrap());
	        thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}
