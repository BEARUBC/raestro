# v0.2.0

## Changes:
Note that all dates are in DD/MM/YYYY form.

### 04/17/2021
* added legal copyright notations to the top of every source code file

### 04/13/2021
* improved API design:
	* relocated read/write buffers in Maestro struct instead of localized buffers in each API
	* all requests supported by Polulu Micro Maestro 6-Channel (i.e., set\_target, get\_position, etc.) implemented
	* get requests (i.e., get\_position, get\_errors) return a u16 instead of a reference to the internal read buffer

### 05/01/2021
* finished documentation for all public modules and exports
* formatted library using rustfmt (all formatting rules can be found in rustfmt.toml)
* fixed implementation for raestro::constants::Errors
	* when an error or errors are encountered, the Maestro returns a 2-byte (`u16`) integer in which each of the first 9 positions (i.e., bit 0 to bit 8) represent an error
	* if the bit in position i (where i = `0..=8`) is set, then the according error was thrown by the Maestro
	* previous implementation of `constants::Errors` assumed that each error had a specific number attached to it (i.e., `SER\_SIGNAL\_ERR` was `0`, `SER\_BUFFER\_ERR` was `1`, etc.), which is incorrect

### 05/06/2021
* the underlying `uart` instance was incorrectly configured to wait indefinitely if no bytes were read on the `UART` lines
* this was updated so that `Maestro::read` would only block for 2 seconds, max  (default configuration)
* `Maestro::set_target` was updated to take in values in units of quarter-us, not us, since all other `raestro` APIs expect values in units of quarter-us
	* docs were updated to reflect this change
