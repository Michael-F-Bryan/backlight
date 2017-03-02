# Linux Backlight Utility

A simple Linux utility for changing your laptop screen's brightness.

The utility works by talking to the `sysfs` pseudo file system provided by the
Linux kernel for interacting with a computer's hardware. In particular, the
`/sys/class/backlight/` directory. 


## Building

First get the source code:

```bash
$ git clone git@github.com:Michael-F-Bryan/backlight.git
```

Then install with `Cargo`:

```bash
$ cargo install
```


## Caveats

If you haven't already guessed, this is a platform-specific library. It 
*should* work for any Linux laptop but your mileage may vary. I may eventually
go on to support Windows and Mac, but using the Windows API usually means
calling some pretty gnarly C functions.
