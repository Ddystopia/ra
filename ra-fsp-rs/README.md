# RA-FSP-RS

This is a Rust HAL for Renesas RA microcontrollers, built on FSP (Flexible Software Package). To use this crate you must fulfill the requirements of [`ra-fsp-sys`]. It makes a best effort to be compatible with [`cortex-m-rt`], so its ecosystem - such as Embassy and RTIC - should be reusable.

[`ra-fsp-sys`]: https://docs.rs/ra-fsp-sys
[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

## Features

- `device`: Forwards to the `ra-fsp-sys/device` feature, which provides the correct runtime setup, leaving you to define your `main` function in Rust. The crate can still be used without this feature, providing safe driver wrappers and integrations with the Rust ecosystem.
- `log`: Enables logging through the [`log`] crate. FSP logs are forwarded to the Rust logging system.
- `mod-r_<peripheral>`: Enables an individual FSP driver and forwards to the matching `ra-fsp-sys` feature. Safe wrappers currently exist, at varying levels of completeness, for `r_ioport` (GPIO), `r_gpt` (general PWM timer), `r_icu` (interrupt controller), `r_glcdc` (graphics LCD controller), and `r_ether` / `r_ether_phy` (Ethernet MAC and PHY). The GLCDC and GPT wrappers are proven in production firmware, and the Ethernet path powers the smoltcp integration in the example project.
- MCU selection (`ra6m3`, `ra4m1`, ...): Selects the target MCU, pulling in the matching PAC and `ra-fsp-sys` target. Exactly one must be enabled. `ra6m3` and `ra4m1` are currently wired up; the remaining RA targets are reserved but not yet tested.
- Integration features: Opt into the ecosystem adapters listed below.

[`log`]: https://crates.io/crates/log

## Integrations

This HAL provides adapters to following crates:
- [`smoltcp`] via `smoltcp-ether` feature.
- [`embassy-time`] via `embassy-time-gpt` feature.
- [`rtic-monotonics`] via `rtic-monotonics` and `rtic-monotonics-gpt` features.
- [`embedded_graphics`] and [`buoyant`] via `embedded-graphics-glcdc` feature.

[`smoltcp`]: https://crates.io/crates/smoltcp
[`embassy-time`]: https://crates.io/crates/embassy-time
[`rtic-monotonics`]: https://crates.io/crates/rtic-monotonics
[`embedded_graphics`]: https://crates.io/crates/embedded-graphics
[`buoyant`]: https://crates.io/crates/buoyant

## License

By using this software, you agree to the additional terms and conditions found at: http://www.renesas.com/disclaimer.

All Rust source code is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
