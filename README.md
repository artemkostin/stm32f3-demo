# Rust on STM32F3 Quick Demo

This repository contains everything needed to flash an LED on an STM32F3.

It shows how to set up a project to build for the Cortex-M4 core, how to set
interrupt handlers, and how to use the register access crates to control a GPIO.

This demo uses an STM32F303 specifically, but should be easy to port to another
STM32F0, STM32F1, STM32F3 or STM32F4. See the Porting section below for information.

## Trying It Out

First make sure your Rust is set up.
[Install rustup](http://rustup.rs/) if you haven't already, and install
the latest stable Rust. Then ensure you have added the required thumbv7 target:
```
$ rustup target add thumbv7em-none-eabihf
```
Or according to platform support page: https://doc.rust-lang.org/nightly/rustc/platform-support.html

Now you can download this repository and build it:
```
$ git clone https://github.com/artemkostin/stm32f3-demo.git
$ cd stm32f3-demo
$ cargo build --release
```

The ELF binary file is in `target/thumbv7em-none-eabihf/release/stm32f3-demo`.
For information on how to program it on your target, consult the
[embedded Rust docs](https://rust-embedded.github.io/bookshelf/).

## Porting to another STM32

First edit `Cargo.toml`, replacing `stm32f303` on the bottom line with the
required module name from [the stm32-rs README].

[the stm32-rs README]: https://github.com/stm32-rs/stm32-rs#supported-device-families

Then edit `src/main.rs` and perform the same change everywhere `stm32f303` is
used. Change registers to a correct one.

Finally check if you need to change `memory.x`: it uses conservative values
by default but might not be appropriate for your target. You'll need to update
the amount of flash and RAM on your actual chip.

Done!

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
