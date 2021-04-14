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

    // this is the maximum range, apparently...?
    //      (i.e., 3968 - 8000)
    //      (which is equal to 992us - 2000us)
    //      (most servos operate in this microsecond range,
    //      with some minor overextensions)
    let positions: [u16; 2usize] = [3968u16, 8000u16];
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
