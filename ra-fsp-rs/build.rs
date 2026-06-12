use std::{env, fs, path::PathBuf};

/// Generate the GPT `Channel` impl list for the active PAC.
///
/// The set of GPT channels (and their channel numbers) differs per MCU, so the
/// list cannot be hardcoded if the crate is to support more than one device.
/// Instead we read the active PAC's already-generated `lib.rs` and grep it for
/// GPT timer channels, emitting `ch!(NAME, N);` calls that `gpt.rs` includes.
///
/// GPT timer channels appear in the PAC as `pub type GPT<...> = Periph<_, ADDR>`
/// and are laid out at `base + 0x100 * channel`. Their names never contain an
/// underscore, which sets them apart from control blocks like `GPT_OPS`/`GPT_ODC`.
fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=build.rs");

    let mut code = String::new();
    match gpt_channels() {
        Some((mcu, channels)) => {
            sanity_check(&mcu, &channels);
            for (name, n) in channels {
                code.push_str(&format!("ch!({name}, {n});\n"));
            }
        }
        // The `gpt` module is only built with `mod-r_gpt`; if it is on but we
        // could not locate the PAC's generated `lib.rs`, fail with a clear
        // message instead of silently emitting no `Channel` impls.
        None if env::var_os("CARGO_FEATURE_MOD_R_GPT").is_some() => {
            code.push_str(
                "compile_error!(\"could not locate the active PAC's generated `lib.rs` \
                 to derive GPT channels; expected it next to this crate under `../pac/<mcu>`\");\n",
            );
        }
        None => {}
    }

    fs::write(out.join("gpt_channels.rs"), code).expect("failed to write gpt_channels.rs");
}

/// A known-good channel set used to detect PAC codegen format changes that
/// would silently break the grep below. Keep in sync with `pac/ra6m3`.
const RA6M3_EXPECTED: &[(&str, u64)] = &[
    ("GPT32EH0", 0),
    ("GPT32EH1", 1),
    ("GPT32EH2", 2),
    ("GPT32EH3", 3),
    ("GPT32E4", 4),
    ("GPT32E5", 5),
    ("GPT32E6", 6),
    ("GPT32E7", 7),
    ("GPT328", 8),
    ("GPT329", 9),
    ("GPT3210", 10),
    ("GPT3211", 11),
    ("GPT3212", 12),
    ("GPT3213", 13),
];

/// Guard against the PAC's generated `lib.rs` changing shape (e.g. a future
/// svd2rust/bindgen update altering the `pub type GPT… = …Periph<_, ADDR>` form)
/// in a way that silently makes the grep miss channels. For an MCU we know the
/// answer for, verify the derived list matches exactly.
fn sanity_check(mcu: &str, channels: &[(String, u64)]) {
    if mcu != "ra6m3" {
        return;
    }
    let got: Vec<(&str, u64)> = channels.iter().map(|(n, c)| (n.as_str(), *c)).collect();
    assert!(
        got == RA6M3_EXPECTED,
        "GPT channel grep produced an unexpected result for ra6m3; the PAC's \
         generated `lib.rs` format likely changed and `build.rs` needs updating.\n\
         expected: {RA6M3_EXPECTED:?}\n     got: {got:?}",
    );
}

/// Read the active PAC's generated `lib.rs` and return `(mcu, channels)` where
/// channels is `(type name, channel number)` sorted by channel number. Returns
/// `None` if no PAC source could be found.
fn gpt_channels() -> Option<(String, Vec<(String, u64)>)> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let pac_root = manifest.parent()?.join("pac");

    // The MCU is selected by feature (e.g. `ra6m3` -> `CARGO_FEATURE_RA6M3`),
    // and the matching PAC lives at `../pac/<mcu>/src/lib.rs`.
    let (mcu, lib_rs) = env::vars().find_map(|(key, _)| {
        let mcu = key.strip_prefix("CARGO_FEATURE_")?;
        // MCU features are `ra<digit>...`; skip unrelated features.
        if !(mcu.starts_with("RA") && mcu[2..].starts_with(|c: char| c.is_ascii_digit())) {
            return None;
        }
        let mcu = mcu.to_lowercase();
        let lib_rs = pac_root.join(&mcu).join("src/lib.rs");
        lib_rs.exists().then_some((mcu, lib_rs))
    })?;

    println!("cargo:rerun-if-changed={}", lib_rs.display());
    let src = fs::read_to_string(&lib_rs).ok()?;

    let mut channels: Vec<(String, u64)> = Vec::new();
    for line in src.lines() {
        let Some(rest) = line.trim_start().strip_prefix("pub type GPT") else {
            continue;
        };
        let Some((name_tail, after)) = rest.split_once(" =") else {
            continue;
        };
        // Control blocks (GPT_OPS, GPT_ODC, ...) are not timer channels.
        if name_tail.contains('_') {
            continue;
        }
        let Some((_, hex)) = after.rsplit_once("0x") else {
            continue;
        };
        let hex: String = hex
            .chars()
            .take_while(|c| c.is_ascii_hexdigit() || *c == '_')
            .filter(|c| *c != '_')
            .collect();
        let Ok(addr) = u64::from_str_radix(&hex, 16) else {
            continue;
        };
        channels.push((format!("GPT{name_tail}"), addr));
    }

    let base = channels.iter().map(|(_, addr)| *addr).min()?;
    let mut channels: Vec<(String, u64)> = channels
        .into_iter()
        .filter(|(_, addr)| (addr - base) % 0x100 == 0)
        .map(|(name, addr)| (name, (addr - base) / 0x100))
        .collect();
    channels.sort_by_key(|(_, n)| *n);
    Some((mcu, channels))
}
