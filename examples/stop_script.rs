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
    let sleep_duration = Duration::from_secs(1);
    loop {
        maestro.set_target(channel, pos_min).unwrap();
        maestro.stop_script().unwrap();
        thread::sleep(sleep_duration);
        maestro.set_target(channel, pos_max).unwrap();
        maestro.stop_script().unwrap();
        thread::sleep(sleep_duration);
    }
}
