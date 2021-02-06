# A Rust-flavoured Pololu Mini Maestro API Interface
Written and developed by UBC Bionics, Ltd.

## Getting Started
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
The link is [here][1].

[1]: https://docs.rs/rppal/0.11.3/rppal/uart/index.html
