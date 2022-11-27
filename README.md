# `raestro` - A Rust-flavoured API Interface for the Pololu Micro-Maestro (6-Channel) Servo Controller Board
[![build\_status](https://github.com/raunakab/raestro/actions/workflows/main.yml/badge.svg)](https://github.com/raunakab/raestro/actions/workflows/main.yml)
[![crates.io](https://meritbadge.herokuapp.com/raestro)](https://crates.io/crates/raestro)
[![docs](https://docs.rs/raestro/badge.svg)](https://docs.rs/crate/raestro)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

`raestro` provides an easy-to-use interface to communicate with the 6-Channel Maestro.
It is developed and maintained by [UBC Bionics, Ltd.](https://ubcbionics.com/), a design team based in the University of British Columbia, Vancouver, Canada.

## Table of Contents
- [Prelude](#Prelude)
- [Documentation](#Documentation)
- [Getting Started](#Getting-Started)
	- [Hardware Setup](#Hardware-Setup)
	- [Software Setup](#Software-Setup)
	- [Trouble-Shooting](#Trouble-Shooting)
- [Usage](#Usage)

## Prelude
Before continuing, please take note of the following points:

-
	This library is developed specifically for the Raspberry Pi.
-
	Please take caution in wiring the Pololu Micro Maestro to the Raspberry Pi.
	Incorrect wiring may lead to permanent hardware damage.

## Documentation
All public exports have been properly documented with examples for usage of critical APIs.
A complete version of the documentation can be found [here](https://docs.rs/raestro).
Included below is a minimal example of how to setup your environment and build a project using `raestro`.

## Getting Started

### Hardware Setup
1.
	Connect the power and ground lines from the Raspberry Pi to the Maestro.
2.
	Connect the Raspberry Pi's TX and RX pins to the Maestro's RX and TX pins, respectively.
	Please note the order in which the pins need to be connected (the Pi's TX connected to the Maestro's RX and the Pi's RX connected to the Maestro's TX).
3.
	Connect the power lines for the servos.
	Documentation on which line is which is available readily online.
4.
	Connect up to 6 servos to one of the pin-triples available (the backside of the board has more info on each pin-type).

### Software Setup
The Rust crate `rppal` provides user-level APIs for protocols such as `PWM`, `I2C`, and `UART`.
In order to configure `UART` for the Raspberry Pi, do the following:

1.
	Remove `console=serial0,11520` from `/boot/cmdline.txt`
2.
	Disable the Bluetooth by:
	* Adding `dtoverlay=pi3-disable-bt` to `/boot/config.txt`
		* For the RPi4 models, do this by adding `dtoverlay=disable-bt` instead
		* Rebooting the Pi (by powering it off and then on again)
	* Running the command `sudo systemctl disable hciuart`

### Trouble-shooting
If permission denied errors are being observed, please inspect your user's permissions.
More specifically, your user must be added to group `dialout`.

If `cargo build` or `cargo test` do not work because of the `rppal` dependency, check the `rppal` documentations on how to set up `UART`.
The link is [here](https://docs.rs/rppal/0.11.3/rppal/uart/index.html).

## Usage
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
raestro = "0.3.0"
```

Finally, create a new `maestro` instance and initialize it by calling `Maestro::start`.
This initialized struct can now be utilized to perform reads and writes to and from the Micro-Maestro 6-Channel.
```rust
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
	// Create a new `Maestro` instance by configuring a `Builder`.
    let mut maestro: Maestro = Builder::default()
        .baudrate(Baudrate::Baudrate11520)
        .block_duration(Duration::from_millis(100))
        .try_into()
        .expect("Failed to build a `maestro` instance.");

    let channel = Channels::Channel0;
    let pos_min = MIN_QTR_PWM;
    let pos_max = MAX_QTR_PWM;
    let sleep_duration = Duration::from_secs(1);

	// Set the initial position of the servo at the specified channel to the specified location!
	maestro.set_target(channel, pos_min).unwrap();
	let position = maestro.get_position(channel).unwrap();

	// Assert that the requested position is truly being broadcast on the requested channel.
	assert_eq!(position, pos_min);
	thread::sleep(sleep_duration);

	// Move the servo back!
	maestro.set_target(channel, pos_max).unwrap();
	let position = maestro.get_position(channel).unwrap();

	// Once again, assert that the requested position is truly being broadcast on the requested channel.
	assert_eq!(position, pos_max);
	thread::sleep(sleep_duration);
}

```
More examples of API usage are provided in the `examples` folder.
