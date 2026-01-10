# RA-FSP-RS

This is a Rust HAL for Renesas RA Microcontrollers that is using FSP (Flexible Software Package). You should fulfill the requirements of [`ra-fsp-sys`] to use this crate. It makes a best effort to be compatible with [`cortex-m-rt`], thus it's ecosystem should be reusable, such as Embassy or RTIC.

[`ra-fsp-sys`]: https://docs.rs/ra-fsp-sys
[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

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

By using this software, you agree to the additonal terms and conditions found at: http://www.renesas.com/disclaimer.

All Rust source code is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
