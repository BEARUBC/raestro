// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! An interface for the Pololu Micro Maestro 6-Channel Servo Controller Board.
//!
//! `raestro` provides an easy-to-use interface to communicate with the 6-Channel Maestro.
//!
//! # Prelude
//! Before continuing, please take note of the following points:
//! * This library is developed specifically for the Raspberry Pi. Builds on different architectures will not be guaranteed to work.
//! * Please take caution in wiring the Pololu Micro Maestro to the Raspberry Pi. Incorrect wiring may lead to permanent hardware damage.
//!
//! # Getting Started
//! Below are the hardware and software setup processes that must be followed before successfully interacting with the Maestro, as well as trouble-shooting tips.
//!
//! ### Hardware Setup
//! 1. Connect the power + ground lines from the Raspberry Pi to the Maestro.
//! 2. Connect the Raspberry Pi's TX and RX pins to the Maestro's RX and TX pins, respectively. Please note the order in which the pins need to be connected (Raspberry Pi's TX connected to Maestro's RX; Raspberry Pi's RX connected to Maestro's TX).
//! 3. Connect the power lines for the servos (holding the board such that the pins are facing you and are on the right side of the board, these are the 2 pins on the top right). The left one of the pair is the power; the right one is ground.
//! 4. Connect up to 6 servos on one of the pin-triples available (the backside of the board has more info on each pin-type).
//!
//! ### Software Setup
//! The Rust crate [rppal](https://crates.io/crates/rppal) provides user-level APIs for protocols such as `PWM`, `I2C`, and `UART`. In order to configure `UART` for the Raspberry Pi, do the following:
//! 1. Remove `console=serial0,11520` from `/boot/cmdline.txt`
//! 2. Disable the Bluetooth by:
//!     * Adding `dtoverlay=pi3-disable-bt` to `/boot/config.txt`
//!         * For the Raspberry Pi 4 models, add `dtoverlay=disable-bt` instead
//!         * Once this is done, reboot the Raspberry Pi (by powering it off and then on again)
//!     * Running the command `sudo systemctl disable hciuart`
//!
//! ### Trouble-shooting
//! If permission denied errors are being observed, please inspect your user's permissions. More specifically, your user must be added to group `dialout`.
//! If `cargo build` or `cargo test` do not work because of the `rppal` dependency, check the `rppal` documentations on how to set up `UART`. The link is [here](https://docs.rs/rppal/0.11.3/rppal/uart/index.html).
//!
//! # Example Usage
//! Below are included some simple examples on how to instantiate and start a ['maestro'] instance, as well as how to send some commands to the Micro Maestro 6-Channel Servo Board.
//!
//! ```
//! use std::{
//!     thread,
//!     time::Duration,
//! };
//! use raestro::prelude::*;
//!
//! let mut m: Maestro = Maestro::new();
//! m.start(BaudRates::BR_115200).unwrap();
//!
//! let channel: Channels = Channels::C_0;
//!
//! let pos_min = 992u16;
//! let pos_max = 2000u16;
//!
//! let sleep_time = Duration::from_millis(1000u64);
//!
//! loop {
//!     maestro.set_target(channel, pos_min).unwrap();
//!     thread::sleep(sleep_time);
//!
//!     maestro.set_target(channel, pos_max).unwrap();
//!     thread::sleep(sleep_time);
//! }
//! ```

// external crates

// external uses

// internal mods
pub mod constants;
mod errors;
mod maestro;
pub mod prelude;
mod utils;
#[cfg(test)]
mod tests {
    // external uses

    // internal mods

    // internal uses
    use crate::{constants::*, maestro::*};

    #[test]
    fn init_and_close() -> () {
        let mut maestro: Maestro = Maestro::new();
        maestro.start(BaudRates::BR_115200).unwrap();
        maestro.close();
    }
}

// internal uses
pub use errors::*;
pub use maestro::*;
