# Board Support Package for the Nordic nRF52840-DK

This crate is a Board Support Package (BSP). It wraps the HAL crate (nrf52840-hal) for the on-board nRF52840, and provides high level wrappers for the
onboard features:

- 4 on-board LEDs
- 4 on-board buttons

This BSP assumes you are not using a bootloader running in non-secure mode.

## Usage

You will require the `thumbv7em-none-eabihf` target installed. To build one of these examples:

```console
$ rustup target add thumbv7em-none-eabihf
$ git clone https://github.com/nrf-rs/nrf52840-DK.git
$ cd nrf52840-DK
$ cargo build --target=thumbv7em-none-eabihf --example blinky
```

To use in your own application, add as a dependency and call the
`Board::init()` function.

## Documentation

The docs for this crate can be found at https://docs.rs/nrf52840-dk-bsp. The
manufacturer's documentation is available from
https://infocenter.nordicsemi.com/pdf/nRF52840_DK_User_Guide_v1.4.1.pdf.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
