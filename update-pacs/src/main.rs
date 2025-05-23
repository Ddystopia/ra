use std::{
    fs::{self},
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use svd2rust::config::IdentFormatsTheme;

const MANIFEST_TEMPLATE: &str = r#"
[package]
name = "@name@-fsp-pac"
description = "Peripheral access API for @NAME@ microcontrollers (generated using svd2rust)"
version = { workspace = true }
authors = [
    "Nathan Larsen <n8tlarsen@gmail.com>",
    "Addison Heavner <addisonheavner@gmail.com>",
    "Oleksandr Babak <alexanderbabak@proton.me>",
]
keywords = ["no-std", "arm", "cortex-m", "renesas", "fsp"]
categories = ["embedded", "hardware-support", "no-std"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/ra-rs/ra"
readme = "README.md"
edition = "2021"

[dependencies]
critical-section = { version = "1.0", optional = true }
cortex-m = "0.7.6"
cortex-m-rt = { version = "0.7.5", optional = true }
vcell = "0.1.2"
portable-atomic = { version = "0.3.16", default-features = false, optional = true }

[features]
rt = []
fsp = ["rt"]
cortex-m-rt-device = ["cortex-m-rt/device", "rt"]
atomics = ["dep:portable-atomic"]
critical-section = ["dep:critical-section"]

[lints]
rust.unreachable_patterns = "allow"
"#;

const README_TEMPLATE: &str = r#"
# `%NAME%` Peripheral Access Crate

A Peripheral Access API (PAC) for `%NAME%` microcontrollers, generated using [`svd2rust`](https://github.com/rust-embedded/svd2rust).

This crate provides low‑level register and bitfield definitions for all on‑chip peripherals. \
It is designed to preserve the familiar `cortex-m-rt` interface while handling `%NAME%`‑specific initialization requirements.

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
%name%_pac = "0.*"
```

To enable the FSP‑based runtime:

```toml
[dependencies]
%name%_pac = { version = "0.*", features = ["rt", "fsp"] }
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
use %name%_pac::Peripherals;

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
"#;

fn main() -> Result<()> {
    let pac_dir = Path::new("pac");
    let patch_dir = Path::new("svd/patch");
    if patch_dir.exists() {
        fs::remove_dir_all(patch_dir)?;
    }
    fs::create_dir_all(patch_dir)?;

    eprintln!("Generating PACs");

    // Copy lowercase-named SVDs to patch dir
    for entry in fs::read_dir("svd/vendor")? {
        let entry = entry?;
        let src = entry.path();
        let dst = patch_dir.join(entry.file_name().to_string_lossy().to_lowercase());
        fs::copy(src, dst)?;
    }

    eprintln!("Patching SVDs");

    // Replace `A-B` with `A,B`
    let path = patch_dir.join("r7fa6m5bh.svd");
    fs::write(&path, fs::read_to_string(&path)?.replace("A-B", "A,B"))?;

    // Patch SVDs

    let mut thread_handles: Vec<std::thread::JoinHandle<Result<()>>> = Vec::new();
    for entry in fs::read_dir("svd/device")? {
        let entry = entry?;
        let patch_dir = patch_dir.to_path_buf();
        let svdtools_config = svdtools::patch::Config::default();

        let handle = std::thread::spawn(move || {
            if entry.file_type()?.is_file() && entry.path().extension() == Some("yaml".as_ref()) {
                let mut out_path = patch_dir.join(entry.file_name());
                out_path.set_extension("patched");
                svdtools::patch::process_file(
                    &entry.path(),
                    Some(&out_path),
                    None,
                    &svdtools_config,
                )
                .with_context(|| format!("Failed to patch SVD file: {}", entry.path().display()))?;
            }
            Ok(())
        });

        thread_handles.push(handle)
    }

    for handle in thread_handles {
        handle
            .join()
            .map_err(|_| anyhow::anyhow!("Thread panicked while processing SVD files"))?
            .context("Failed to patch SVD file")?;
    }

    eprintln!("Generating PACs");

    let mut handles: Vec<std::thread::JoinHandle<Result<()>>> = Vec::new();

    // Generate PACs
    for entry in fs::read_dir(patch_dir)? {
        let entry = entry?;
        let svd_file = entry.path();

        if svd_file.extension().is_none_or(|ext| ext != "patched") {
            continue;
        }

        let name = extract_pac_name(&svd_file)
            .with_context(|| format!("Failed to extract PAC name from: {}", svd_file.display()))?;
        let pac_dir = pac_dir.join(&name);

        if name != "ra6m3" {
            // fixme: when running in debug mode, we see that `svd2rust` panics on debug assertion
            //        due to bad SVD files. We need to patch those errors.
            println!("Skipping {name} as it is not supported yet");
            continue;
        }

        handles.push(std::thread::spawn(move || {
            generate_pac(&pac_dir, &name, &svd_file)
                .with_context(|| format!("Failed to generate PAC for: {}", svd_file.display()))
        }));
    }

    for handle in handles {
        handle
            .join()
            .map_err(|_| anyhow::anyhow!("Thread panicked while generating PACs"))?
            .context("Failed to generate PAC")?;
    }

    fs::remove_dir_all(patch_dir)?;

    Ok(())
}

fn extract_pac_name(svd_file: &Path) -> Result<String> {
    assert_eq!(svd_file.extension(), Some("patched".as_ref()));

    Ok(svd_file
        .file_stem()
        .context("Invalid SVD file name")?
        .to_str()
        .context("Invalid SVD file name")?
        .to_lowercase()
        .replace("7f", "")[..5]
        .to_string())
}

fn generate_pac(pac_dir: &Path, name: &str, svd_file: &Path) -> Result<()> {
    let name_upper = name.to_uppercase();

    if pac_dir.exists() {
        fs::remove_dir_all(&pac_dir).context("Failed to remove existing PAC directory")?;
    }
    fs::create_dir_all(&pac_dir).context("Failed to create PAC directory")?;

    println!("Found device family {name_upper}");

    // read to string `svd_file`
    let svd = fs::read_to_string(&svd_file).context("Failed to read SVD file")?;

    let mut svd2rust_config = svd2rust::Config::default();
    svd2rust_config.atomics = true;
    svd2rust_config.atomics_feature = Some("atomics".to_string());
    svd2rust_config.target = svd2rust::Target::CortexM;
    svd2rust_config.output_dir = Some(pac_dir.to_path_buf());
    svd2rust_config.ident_formats_theme = Some(IdentFormatsTheme::Legacy);
    svd2rust_config.interrupt_link_section = Some(".CHANGE_ME".into());

    let res = svd2rust::generate(&svd, &svd2rust_config).context("Failed to run svd2rust")?;
    let specific = res.device_specific.context("No device-scecific files")?;

    let lib_rs = res.lib_rs;
    let lib_rs = lib_rs + "\n";
    let lib_rs = lib_rs
        + "
#[cfg(feature = \"rt\")]
pub use self::Interrupt as interrupt;

#[cfg(all(feature = \"fsp\", feature = \"cortex-m-rt-device\"))]
compile_error!(\"Cannot enable both `fsp` and `cortex-m-rt-device` features at the same time.\");

#[cfg(feature = \"rt\")]
impl Interrupt {
    pub const fn try_from_u16(n: u16) -> Option<Self> {
        assert!(__INTERRUPTS.len() < u16::MAX as usize);

        if n >= __INTERRUPTS.len() as u16 {
            return None;
        }

        Some(unsafe { ::core::mem::transmute(n as u16) })
    }
}

#[cfg(feature = \"rt\")]
#[derive(Debug)]
pub struct InvalidInterruptNumber;
#[cfg(feature = \"rt\")]
impl TryFrom<u16> for Interrupt {
    type Error = InvalidInterruptNumber;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Interrupt::try_from_u16(value).ok_or(InvalidInterruptNumber)
    }
}
#[cfg(feature = \"rt\")]
impl core::error::Error for InvalidInterruptNumber {}
#[cfg(feature = \"rt\")]
impl core::fmt::Display for InvalidInterruptNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, \"Invalid interrupt number\")
    }
}
\n";
    let lib_rs = lib_rs.replace(
        "# [link_section = \".CHANGE_ME\"]",
        "
#[cfg_attr(feature = \"fsp\", link_section = \".application_vectors\")]
#[cfg_attr(feature = \"cortex-m-rt-device\", link_section = \".vector_table.interrupts\")]",
    );
    assert!(
        !lib_rs.contains("CHANGE_ME"),
        "Failed to replace CHANGE_ME in lib.rs"
    );
    let build_rs = specific.build_rs;
    let device_x = specific.device_x;

    fs::write(pac_dir.join("build.rs"), build_rs).context("Failed to write build.rs")?;
    fs::write(pac_dir.join("device.x"), device_x).context("Failed to write device.x")?;

    let output = pac_dir.join("src");

    form::create_directory_structure(output, &lib_rs, true)
        .context("Failed to create directory structure ")?;

    let manifest = MANIFEST_TEMPLATE
        .replace("@name@", &name)
        .replace("@NAME@", &name_upper);

    fs::write(pac_dir.join("Cargo.toml"), manifest).context("Failed to write Cargo.toml")?;

    fs::write(
        pac_dir.join("README.md"),
        README_TEMPLATE
            .replace("%NAME%", &name_upper)
            .replace("%name%", &name),
    )?;

    Command::new("cargo")
        .args(["fmt", "--manifest-path"])
        .arg(pac_dir.join("Cargo.toml"))
        .status()
        .context("Failed to run `cargo fmt`")?;

    let _ = Command::new("rustfmt")
        .arg(pac_dir.join("build.rs"))
        .stderr(Stdio::null())
        .status();

    Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(pac_dir.join("Cargo.toml"))
        .args(["--features", "atomics"])
        .status()
        .context("Failed to run `cargo build`")?;

    Ok(())
}
