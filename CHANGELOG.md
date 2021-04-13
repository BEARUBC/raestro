# v0.0.2
## Changes:
* improved API design:
	* relocated read/write buffers in Maestro struct instead of localized buffers in each API
	* all requests supported by Polulu Micro Maestro 6-Channel (i.e., set_target, get_position, etc.) implemented
	* get requests (i.e., get_position, get_errors) return a u16 instead of a reference to the internal read buffer

## Future Goals:
* finish documentation for all functions
