# RA-FSP-SYS

This is a basic wrapper over the Flexible Software Package - the bare minimum to correctly compile and run a Rust program on RA microcontrollers.

This crate should be used *instead* of `cortex-m-rt`, and it makes a best effort to mimic its interface, so you can use Embassy and RTIC without any problems. RA is special: `cortex-m-rt` does technically run on these MCUs, but it does not handle the manufacturer linker script (and you can permanently lock yourself out of the MCU via ID code protection), nor does it call the C `SystemInit` that the chip requires. This crate takes care of both.

## Features

- `device`: Provides the `cortex-m-rt`-compatible runtime described below. Without it, the crate only generates raw bindings to FSP, which you can call from your own Rust code - useful when you already have a C codebase and just want to add Rust on top.
- `log`: Enables logging through the [`log`] crate. FSP logs are forwarded to the Rust logging system.
- `mod-r_<peripheral>`: Generates bindings for an individual FSP driver. Available modules include `r_ioport`, `r_gpt`, `r_icu`, `r_elc`, `r_dmac`, `r_flash_hp`, `r_glcdc`, and `r_ether` / `r_ether_phy`. Each enables compilation of the corresponding FSP source.
- MCU selection (`ra6m3`, `ra4m1`, ...): Selects the target MCU. Exactly one must be enabled. `ra6m3` and `ra4m1` are currently supported; the remaining RA targets are reserved but not yet tested.

[`log`]: https://crates.io/crates/log

## Runtime with the `device` feature

- The basic vector table is provided by FSP, including `Reset_Handler`, which calls `SystemInit` (to configure the C runtime, clocks, etc.) and then `main`.
- An application-specific vector table may be provided by the user via the `.application_vectors` section. The associated PACs provide it.
- The linker script that the manufacturer ships for your MCU is applied to support features like ID code protection. You supply it yourself (rather than it being vendored) so you can patch it easily.

## What you must provide

- `FSP_CFG` environment variable with header files to configure FSP.
  - Contains all `r_.*_cfg.h` configuration files.
  - Contains `/bsp` directory with bsp configurations, like `bsp_cfg.h`, `board_cfg.h`, `bsp_clock_cfg.h` etc.
- `FSP_PATH` environment variable provides the source code for FSP.
- `CMSIS_PATH` environment variable provides the source code for CMSIS.

If you enable the `device` feature, you must also provide:

- `memory.x`.
- `fsp_base.ld`, the linker script provided by FSP for your MCU.
- The [PAC] crate, which provides `device.x` and exports the interrupt vectors.
- `-C link-arg=-Tra-fsp-sys.x` (the equivalent of `-Tlink.x` with `cortex-m-rt`).

[PAC]: https://github.com/Ddystopia/ra/tree/main/pac

## Footgun

Renesas devices support ID code protection via the OSIS register:

* The OSIS register stores a 128-bit ID code used for authentication when connecting a debugger or serial programmer.
* If the ID codes do not match, the device remains locked and prohibits external access.
* A special linker script and startup procedure are required to configure the OSIS register safely and avoid accidental locking of the MCU.

The manufacturer-provided FSP handles ID code setup in `SystemInit` and in the linker script. **Do not** rewrite or bypass this logic unless you fully understand the implications. The author of this crate killed several MCUs to figure it out.

## License

By using this software, you agree to the additional terms and conditions found at: http://www.renesas.com/disclaimer.

All Rust source code except `generated` module is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
