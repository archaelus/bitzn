# Using Zinc Externally with a 1bitsy

This project is an example of how you can setup an external project
using [Zinc](https://www.github.com/hackndev/zinc) to target
the [1bitsy](http://1bitsy.org).  The actual example used here is one
adapter from the Zinc examples, but with this infrastructure in place,
you can create whatever you want.

![1bitsy running bitzn](doc/1bitsy_blink.gif)

(This gif is poorly edited - it really does blink at 1Hz I swear :)

## Pre-requisites

### Hardware

You'll need:
* A [1bitsy](http://1bitsy.org) microcontroller board.
* A [Black Magic Probe v2](https://1bitsquared.com/collections/frontpage/products/black-magic-probe) to load/debug your firmware.

### OS X

* [Install rustup](https://www.rustup.rs)
  * `$ rustup update nightly-2016-09-17`
  * In your project dir: `$ rustup override set nightly-2016-09-17`
* [Install xargo](https://github.com/japaric/xargo)
  * `$ cargo install xargo`
* Install the Rust source so xargo can build libcore
  * `$ rustup component add --toolchain nightly-2016-09-17 rust-src`
* Install `arm-none-eabi` gcc suite (for cortex-m4)
  * `$ brew cask install gcc-arm-embedded`
* Setup the `thumbv7em-none-eabi` target
  * `$ echo -e '[target.thumbv7em-none-eabi]\nlinker = "arm-none-eabi-gcc"\nar = "arm-none-eabi-ar"\n' >> ~/.cargo/config`

## Building the Example

The code can be built with xargo.

```
$ xargo build
```

When xargo builds a libcore for your target (thumbv7em-none-eabi), it may complain that it can't build libc - that seems to be the way xargo/libcore builds right now. It doesn't seem to break the final firmware.

If you get an error along the lines of:
```
error: error recursively walking the sysroot
caused by: IO error for operation on $HOME/.rustup/toolchains/nightly-$ARCH/lib/rustlib/src: No such file or directory (os error 2)
caused by: No such file or directory (os error 2)
note: run with `RUST_BACKTRACE=1` for a backtrace
```
that usually means you need to do the `$ rustup component add --toolchain nightly-2016-09-17 rust-src` step mentioned earlier.

This will eventually generate an ELF binary of your program, which can be loaded onto the 1bitsy with GDB:

```
$ gdb ./target/thumbv7em-none-eabi/debug/bitzn
GNU gdb (GNU Tools for ARM Embedded Processors) 7.10.1.20160210-cvs
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "--host=x86_64-apple-darwin10 --target=arm-none-eabi".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/thumbv7em-none-eabi/debug/bitzn...(no debugging symbols found)...done.
```

Now, connect to the bmpm2, and connect it to the 1bitsy (on my Mac, the bmpm2 shows up as `/dev/cu.usbmodemD5DCDBE1`):
```
(gdb) target extended-remote /dev/cu.usbmodemD5DCDBE1
Remote debugging using /dev/cu.usbmodemD5DCDBE1
```
Enable power to the 1bitsy with `monitor tpwr enable`
```
(gdb) monitor tpwr enable
```
Have the bmpm find the 1bitsy:
```
(gdb) monitor jtag_scan
Target voltage: 3.2V
Available Targets:
No. Att Driver
 1      STM32F4xx
```
Have GDB attach to the 1bitsy:
```
(gdb) attach 1
Attaching to program: /Users/nem/projects/arm/bitzn/target/thumbv7em-none-eabi/release/bitzn, Remote target
0x08000332 in ?? ()
```

Now you're debugging the 1bitsy!

### Flashing the Teensy

Continuing in the GDB session above, you can run the `load` command to flash the 1bitsy with whatever file gdb has loaded (in our case, "target/thumbv7em-none-eabi/debug/bitzn"):
```
(gdb) load
Loading section .vector, size 0x40 lma 0x8000000
Loading section .text, size 0xe8 lma 0x8000040
Loading section .init, size 0x4 lma 0x8000128
Loading section .fini, size 0x4 lma 0x800012c
Start address 0x8000040, load size 304
Transfer rate: 723 bytes/sec, 76 bytes/write.
```

### Resetting the 1bitsy

In GDB, use the `start` command to restart the 1bitsy without disconnecting the bmpm2:
```
(gdb) start
The program being debugged has been started already.
Start it from the beginning? (y or n) y
Temporary breakpoint 1 at 0x800004e
Starting program: /Users/nem/projects/arm/bitzn/target/thumbv7em-none-eabi/release/bitzn 
Note: automatically using hardware breakpoints for read-only addresses.

Temporary breakpoint 1, 0x0800004e in main ()
```


You can reset the 1bitsy from GDB with the `start` command:
```
(gdb) start
The program being debugged has been started already.
Start it from the beginning? (y or n) y
Temporary breakpoint 1 at 0x800038e
Starting program: /Users/nem/projects/arm/bitzn/target/thumbv7em-none-eabi/debug/bitzn 
Note: automatically using hardware breakpoints for read-only addresses.

Temporary breakpoint 1, 0x0800038e in main ()
```

### Running the firmware

GDB automatically set a breakpoint on the main function. You can let the firmware run with the `continue` command, and `^C` to return to the debugger:
```
(gdb) c
Continuing.
^C
Program received signal SIGINT, Interrupt.
0x080000fc in main ()
(gdb) 
```


## Creating Your Own Poject Using Zinc

### Step 1: Create a new rust project

```
$ cargo new --bin --vcs git rust-1bitsy-blink
$ cd rust-1bitsy-blink
```

### Step 2: Set up your Cargo.toml

Add the following to Cargo.toml, replacing the information preset here
with information that makes sense for your MCU, binary, etc.

```toml
[package]
name = "rust-1bitsy-blink"
version = "0.1.0"
authors = [YOU, "Geoff Cant <nem@erlang.geek.nz>", "Paul Osborne <osbpau@gmail.com>"]

[dependencies.zinc]
git = "https://github.com/hackndev/zinc.git"
branch = "master"
features = ["mcu_stm32f4"]

[dependencies.macro_zinc]
git = "https://github.com/hackndev/zinc.git"
branch = "master"
```

### Step 3: Grab a target specification

Grab a suitable target specification from those available in the root
of the Zinc repository (`thumbv7em-none-eabi.json` for the 1bitsy).

### Step 4: Tell rust about your toolchain

This usually requires putting a couple lines like this in your
`.cargo/config` so that Rust knows to use a proper cross-linker for
your target:

```toml
[target.thumbv7em-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"

[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
```

### Step 5: Create!

Write your code using Zinc!
