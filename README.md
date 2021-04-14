# Raestro
[![crates.io](https://meritbadge.herokuapp.com/raestro)](https://crates.io/crates/raestro)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.txt)

Raestro is a Rust-flavoured API Interface for the Pololu Micro-Maestro (6-Channel) Servo Controller Board. This library is developed specifically for the Raspberry Pi, which acts as the main computer system on our bionic arm.

Raestro is developed and maintained by [UBC Bionics, Ltd.](https://ubcbionics.com/), a design team based in the University of British Columbia, Vancouver.

## Table of Contents
- [Documentation](#Documentation)
- [Getting Started](#Getting-Started)
	- [Hardware Setup](#Hardware-Setup)
	- [Software Setup](#Software-Setup)
	- [Trouble-Shooting](#Trouble-Shooting)
- [Usage](#Usage)

## Documentation
As of current, documentation for this library and all of its APIs is still being worked on. However, design decisions on Raestro are now finally starting to stabilize, and as such, documentation will soon be available as part of the next incremental release.

## Getting Started

### Hardware Setup
1. Connect the power+ground lines from the RPi to the Maestro.
2. Connect the RPI's TX and RX pins to the Maestro's RX and TX pins, respectively. Please note the order in which the pins need to be connected (RPi TX connected to Maestro RX; RPi RX connected to Maestro TX).
3. Connect the power lines for the servos (holding the board such that the pins are facing you and are on the right side of the board,these are the 2 pins on the top right). The left one of the pair is the power; the right one is ground.
4. Connect up to 6 servos on one of the pin-triples available (the backside of the board has more info on each pin-type).

### Software Setup
The Rust crate "rppal" provides user-level APIs for protocols such as PWM, I2C, and UART.
In order to configure UART for the Raspberry Pi, do the following:
1. Remove "console=serial0,11520" from /boot/cmdline.txt
2. Disable the Bluetooth by:
	* Adding "dtoverlay=pi3-disable-bt" to /boot/config.txt (without the quotation marks)
		* For the RPi4 models, add "dtoverlay=disable-bt" instead
		* Once this is done, reboot the Pi (by powering it off and then on again)
		* Connecting GPIO Pin-14 (physical pin 08) to an LED, this LED should be LIT
	* Running the command "sudo systemctl disable hciuart"

### Trouble-shooting
If "cargo build" or "cargo test" do not work because of the rppal dependency, check the rppal documentations on how to set up UART.
The link is [here](https://docs.rs/rppal/0.11.3/rppal/uart/index.html).

## Usage
Add the following to your Cargo.toml file:
```toml
[dependencies]
raestro = "0.2.0"
```
In the main function, create a new Maestro instance and initialize it by calling start.
This initialized struct can now be utilized to perform writes and reads to and from the Maestro.
```rust
use raestro::prelude::*;

fn main() -> () {
	let mut maestro: Maestro = Maestro::new();
	maestro.start(BaudRates::BR_115200).unwrap();
    
	let channel = Channels::C_0;
	let position = 5_000u16;

	maestro.set_target(channel, position).unwrap();

	let actual_position = maestro.get_position(channel).unwrap();
	
	assert!(position, actual_position);
}
```
