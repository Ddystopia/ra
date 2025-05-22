# RA-FSP-SYS

This is a basic wrapper over Flexible Software Package, a bare minimum to correctly compile and run basic Rust program.

This crate should be used *instead* of `cortex-m-rt`, but it makes a best effort to mimic it's interface. This way, you can use Embassy and RTIC wihout any problems.

### What you must provide

- `FSP_CFG` env variable with header files to configure FSP.
  - Contains all `r_.*_cfg.h` coniguration files.
  - Containst `/bsp` directory witb bsp configurations, like `bsp_cfg.h`, `board_cfg.h`, `bsp_clock_cfg.h` etc.
- `memory.x`.
- `device.x` provided by [PAC](https://github.com/Ddystopia/ra).
- Add `-C link-arg=-Tra-fsp-sys.x` (as `-Tlink.x` with `cortex-m-rt`).
- Optional: `FSP_OVERWRITE` env variable provides an alternative source code for FSP. The default is https://github.com/renesas/fsp.
- Optional: `CMSIS_OVERWRITE` env variable provides an alternative source code for CMSIS. The default is https://github.com/ARM-software/CMSIS_5.

## License

By using this software, you agree to the additonal terms and conditions found at: http://www.renesas.com/disclaimer.

`script/fsp_base.ld`, `ra-fsp` are licensed under Renesas license.

All Rust source code exept `generated` module is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
