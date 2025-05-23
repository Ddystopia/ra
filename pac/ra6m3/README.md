
# `RA6M3` Peripheral Access Crate

A Peripheral Access API (PAC) for `RA6M3` microcontrollers, generated using [`svd2rust`](https://github.com/rust-embedded/svd2rust).

This crate provides low‑level register and bitfield definitions for all on‑chip peripherals. \
It is designed to preserve the familiar `cortex-m-rt` interface while handling `RA6M3`‑specific initialization requirements.

[`ra-fsp-rs`]: https://docs.rs/ra-fsp-rs/

---

## Features

- **`rt`**: Includes IV in ".application_vectors" section. Does not enable any runtime. Either use `ra-fsp-sys` or `cortex-m-rt/device`.
- **`fsp`**: places IV in ".application_vectors" section. You still need to add `ra-fsp-sys/@name@` or `ra-fsp-rs/@name@` crate to your dependencies.
- **`cortex-m-rt-device`**: places IV in ".vector_table.interrupts" section and enables `cortex-m-rt/device` feature.

[`ra-fsp-sys`]: https://docs.rs/ra-fsp-sys/

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ra6m3_pac = "0.*"
```

To enable the FSP‑based runtime:

```toml
[dependencies]
ra6m3_pac = { version = "0.*", features = ["rt", "fsp"] }
ra_fsp-sys = { version = "0.*", features = ["@name@"] }
```

## Usage

[`RTIC`]: https://rtic.rs/2/book/en/

This PAC has equivalent interface to any `cortex-m-rt/device`-compatible PAC, thus
the code is looking exactly the same. You can use an OS like [`RTIC`] or just:

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use ra6m3_pac::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    // Configure and use peripherals...
    loop {}
}
```

> **Note**: FSP manages interrupt vectors, invokes `SystemInit`, and \
then calls `main`. `cortex-m-rt::entry` can still be used, but `cortex-m-rt/device` \
feature cannot be enabled, as `FSP` is responsible for the vector table and `Reset`.

## Contributing

Contributions are welcome! Please open an issue or pull request in this repository. Follow these guidelines:

1. Synchronize register definitions from the latest SVD.
2. Keep changes minimal—only update what’s necessary.
3. Always test on real hardware.

## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license (LICENSE-MIT or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
