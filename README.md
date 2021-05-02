# `raestro` - A Rust-flavoured API Interface for the Pololu Micro-Maestro (6-Channel) Servo Controller Board
[![Build Status](https://travis-ci.com/raunakab/raestro.svg?branch=master)](https://travis-ci.com/github/raunakab/raestro)
[![crates.io](https://meritbadge.herokuapp.com/raestro)](https://crates.io/crates/raestro)
[![docs](https://docs.rs/raestro/badge.svg)](https://docs.rs/crate/raestro)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

`raestro` is developed and maintained by [UBC Bionics, Ltd.](https://ubcbionics.com/), a design team based in the University of British Columbia, Vancouver, Canada.

## Table of Contents
- [Prelude](#Prelude)
- [Documentation](#Documentation)
- [Getting Started](#Getting-Started)
	- [Hardware Setup](#Hardware-Setup)
	- [Software Setup](#Software-Setup)
	- [Trouble-Shooting](#Trouble-Shooting)
- [Usage](#Usage)

## Prelude
This library is developed specifically for the Raspberry Pi, which acts as the main computer system on our bionic arm. Builds on different architectures will not be guaranteed to work.

## Documentation
All public exports have been properly documented with examples for usage of critical APIs.
A complete version of the documentation can be found [here](https://docs.rs/raestro).
Included below is a minimal example of how to setup your environment and build a project using `raestro`.

## Getting Started

### Hardware Setup
1. Connect the power and ground lines from the Raspberry Pi to the Maestro.
2. Connect the Raspberry Pi's TX and RX pins to the Maestro's RX and TX pins, respectively. Please note the order in which the pins need to be connected (the Pi's TX connected to the Maestro's RX and the Pi's RX connected to the Maestro's TX).
3. Connect the power lines for the servos. Documentation on which line is which is available readily online.
4. Connect up to 6 servos to one of the pin-triples available (the backside of the board has more info on each pin-type).

### Software Setup
The Rust crate `rppal` provides user-level APIs for protocols such as `PWM`, `I2C`, and `UART`.
In order to configure `UART` for the Raspberry Pi, do the following:
1. Remove `console=serial0,11520` from `/boot/cmdline.txt`
2. Disable the Bluetooth by:
	* Adding `dtoverlay=pi3-disable-bt` to `/boot/config.txt`
		* For the RPi4 models, do this by adding `dtoverlay=disable-bt` instead
		* Rebooting the Pi (by powering it off and then on again)
	* Running the command `sudo systemctl disable hciuart`

### Trouble-shooting
If `cargo build` or `cargo test` do not work because of the `rppal` dependency, check the `rppal` documentations on how to set up `UART`.
The link is [here](https://docs.rs/rppal/0.11.3/rppal/uart/index.html).

## Usage
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
raestro = "0.2.0"
```
Create a new `maestro` instance and initialize it by calling `Maestro::start`.
This initialized struct can now be utilized to perform reads and writes to and from the Micro-Maestro 6-Channel.
```rust
use raestro::prelude::*;

fn main() -> () {
	let mut maestro: Maestro = Maestro::new();
	maestro.start(BaudRates::BR_115200).unwrap();
    
	let channel = Channels::C_0;

	// the position is in microseconds and can only be between 992 and 2000
	// (specifically for the Pololu Micro-Maestro 6-Channel Board)
	let position = 992u16;

	maestro.set_target(channel, position).unwrap();

	let actual_position = maestro.get_position(channel).unwrap();
	
	assert_eq!(position, actual_position);
}
```
More examples of API usage are provided in the `examples` folder.
