# v0.2.0
## Changes:
* improved API design:
	* relocated read/write buffers in Maestro struct instead of localized buffers in each API
	* all requests supported by Polulu Micro Maestro 6-Channel (i.e., set\_target, get\_position, etc.) implemented
	* get requests (i.e., get\_position, get\_errors) return a u16 instead of a reference to the internal read buffer
* updated the repo name

## Future Goals:
* finish documentation for all functions
