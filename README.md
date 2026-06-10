# RA

Rust support for [Renesas RA][ra] microcontrollers, built on top of Renesas FSP (Flexible Software Package).

This repository works towards providing everything needed to write firmware for RA MCUs in Rust: safe HAL bindings to FSP, the underlying raw bindings and runtime, and the Peripheral Access Crates (PACs) together with the tooling to regenerate them.

[ra]: https://www.renesas.com/en/products/microcontrollers-microprocessors/ra-cortex-m-mcus

## Crates

| Crate          | Description                                                                                                       |
| -------------- | ----------------------------------------------------------------------------------------------------------------- |
| [`ra‑fsp‑rs`]  | Safe Rust HAL over FSP, with integrations for the embedded ecosystem (smoltcp, Embassy, RTIC, embedded-graphics). |
| [`ra‑fsp‑sys`] | Raw bindings to FSP and a `cortex-m-rt`-compatible runtime. The foundation `ra-fsp-rs` builds on.                 |
| [`pac`]        | Generated Peripheral Access Crates for individual MCUs (`ra6m3`, `ra4m1`, ...).                                   |
| [`update‑pacs`]| Tooling that generates the PACs from SVD files.                                                                   |
| [`svd`]        | SVD files the PACs are generated from.                                                                            |

[`ra‑fsp‑rs`]: ra-fsp-rs
[`ra‑fsp‑sys`]: ra-fsp-sys
[`pac`]: pac
[`update‑pacs`]: update-pacs
[`svd`]: svd

## Status

This stack is **used in production on the RA6M3**, where the GLCDC (graphics) and GPT (timer) drivers run in shipped firmware. The crate as a whole builds on FSP, which is generic over the MCU family, so other drivers and RA targets are expected to work - `ra6m3` and `ra4m1` are wired up today, and adding a new target is mostly a matter of enabling the corresponding feature and PAC.

## Example project

[`ra6m3-rtic`] is a complete project built with this stack (originally a bachelor's thesis) - a good starting point to see how the crates fit together, including the Ethernet/`smoltcp` integration.

[`ra6m3-rtic`]: https://github.com/Ddystopia/ra6m3-rtic

## Getting started

`ra-fsp-rs` is the entry point for most users. See its [README][ra-fsp-rs readme] for features and ecosystem integrations, and the [`ra-fsp-sys` README][ra-fsp-sys readme] for the build-time requirements (FSP sources, configuration headers, and linker setup).

[ra-fsp-rs readme]: ra-fsp-rs/README.md
[ra-fsp-sys readme]: ra-fsp-sys/README.md

## Contributing

Contributions are very welcome! This stack covers what its users have needed so far, which means there is plenty of room to grow - and most of the groundwork is already in place.

Good ways to help:

- **Bring up a new MCU.** Most of FSP is generic over the family, so enabling another RA target is often just wiring up the corresponding feature and PAC. If you have the hardware, reports that an existing target works (or doesn't) are valuable too.
- **Add a safe driver wrapper.** Several FSP modules currently expose only the raw bindings; wrapping one in a safe, ergonomic API is a self-contained and high-impact contribution.
- **Add ecosystem integrations**, improve docs, or file issues for rough edges and missing features.

No contribution is too small. If you're unsure where to start or want to discuss an idea before diving in, open an issue - questions are welcome.

## License

By using this software, you agree to the additional terms and conditions found at: http://www.renesas.com/disclaimer.

All Rust source code (except generated bindings and the `pac/` directory) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
