# RA-FSP-SYS

This is a basic wrapper over Flexible Software Package, a bare minimum to correctly compile and run basic Rust program.

This crate should be used *instead* of `cortex-m-rt`, but it makes a best effort to mimic it's interface. This way, you can use Embassy and RTIC wihout any problems.

## Runtime

- Applies a manufacturer‑provided linker script to support features like ID code protection.
- Basic vector table is provided by FSP, including `Reset_Handler`, which will call `SystemInit` (to configure C runtime, clocks etc) and then `main`.
- Application specific vector table may be provided by the user using `.application_vectors` section. Associated PACs do provide it.

### What you must provide

- `FSP_CFG` env variable with header files to configure FSP.
  - Contains all `r_.*_cfg.h` coniguration files.
  - Containst `/bsp` directory witb bsp configurations, like `bsp_cfg.h`, `board_cfg.h`, `bsp_clock_cfg.h` etc.
- `memory.x`.
- Use [PAC](https://github.com/Ddystopia/ra/tree/main/pac) crate, which will have `device.x` and export IV.
- Add `-C link-arg=-Tra-fsp-sys.x` (as `-Tlink.x` with `cortex-m-rt`).
- `FSP_PATH` env variable provides the source code for FSP.
- `CMSIS_PATH` env variable provides the source code for CMSIS.

## Security Considerations

Renesas devices support ID code protection via the OSIS register:

* The OSIS register stores a 128-bit ID code used for authentication when connecting a debugger or serial programmer.
* If the ID codes do not match, the device remains locked and prohibits external access.
* A special linker script and startup procedure are required to configure the OSIS register safely and avoid accidental locking of the MCU.

Manufacturer‑provided FSP to handles ID code setup in `SystemInit` and linker script. **Do not** rewrite or bypass this logic unless you fully understand the implications. Authour of this crate killed several MCUs to figure it out.

## License

By using this software, you agree to the additonal terms and conditions found at: http://www.renesas.com/disclaimer.

`script/fsp_base.ld` is licensed under Renesas license.

All Rust source code exept `generated` module is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
