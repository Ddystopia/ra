use std::{
    collections::HashMap,
    fs,
    io::{BufRead, Write},
    path::{self, Path, PathBuf},
};

use build_cfg::{build_cfg, build_cfg_main};

pub fn load_env<P: AsRef<std::path::Path>>(path: P) -> HashMap<String, String> {
    let mut res = HashMap::new();

    let env_file = std::fs::File::open(path.as_ref())
        .expect(format!("file '{}' must exist", path.as_ref().to_str().unwrap()).as_str());
    let reader = std::io::BufReader::new(env_file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.splitn(2, "=").collect();
        if parts.len() == 2 {
            res.insert(
                parts[0].to_string(),
                parts[1].replace(";;", ";").to_string(),
            );
        } else {
            eprintln!("Skip invalid line '{}' with {} parts", line, parts.len());
        }
    }

    res
}

pub fn wrap_component(modules: &[&str]) {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let fsp_cfg = PathBuf::from(std::env::var("FSP_CFG").expect("FSP_CFG must be set"));
    let cmsis_dir = PathBuf::from(std::env::var("CMSIS_PATH").expect("`CMSIS_PATH` must be set"));
    let ra_fsp_repo = PathBuf::from(std::env::var("FSP_PATH").expect("`FSP_PATH` must be set"));
    let linker_scripts = out_path.join("script");

    if !fsp_cfg.exists() {
        panic!("`FSP_CFG` must be set to a valid path");
    }
    if !cmsis_dir.exists() {
        panic!("`CMSIS_PATH` must be set to a valid path");
    }
    if !ra_fsp_repo.exists() {
        panic!("`FSP_PATH` must be set to a valid path");
    }

    let fsp_dir = ra_fsp_repo.join("ra/fsp");
    let fsp_src = fsp_dir.join("src");

    println!("cargo:rerun-if-changed=script");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CMSIS_PATH");
    println!("cargo:rerun-if-env-changed=FSP_PATH");
    println!("cargo:rerun-if-env-changed=FSP_CFG");

    println!("cargo:rerun-if-changed={}", fsp_cfg.display());
    println!("cargo:rerun-if-changed={}", ra_fsp_repo.display());
    println!("cargo:rerun-if-changed={}", cmsis_dir.display());
    println!("cargo:rerun-if-changed={}", fsp_cfg.display());
    println!("cargo:rerun-if-changed=include");

    println!("cargo:rustc-link-lib=static=fsp_prelinked");
    println!("cargo:rustc-link-search={}", linker_scripts.display());
    println!("cargo:rustc-link-search={}", out_path.display());

    let mut rust_codegen = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_path.join("out.rs"))
        .unwrap();

    let Some((mcu_group, bsp_mcu_group)) = mcu_group().zip(bsp_mcu_group()) else {
        write!(rust_codegen, "compile_error!(\"No MCU group defined\");").unwrap();
        return;
    };

    let include_dirs = vec![
        // User defined configs
        fsp_cfg.to_path_buf(),
        fsp_cfg.join("bsp"),
        // FSP includes
        fsp_dir.join("inc"),
        fsp_dir.join("inc/api"),
        fsp_dir.join("inc/instances"),
        fsp_dir.join("inc/drivers"),
        fsp_dir.join("src/bsp/cmsis/Device/RENESAS/Include"),
        fsp_dir.join("src/bsp/mcu").join(mcu_group),
        fsp_dir.join("src/bsp/mcu/all"),
        cmsis_dir.join("CMSIS/Core/Include/"),
        // Other stuff
        ra_fsp_repo.to_path_buf(),
        Path::new("include").to_path_buf(),
    ];

    // compile fsp library

    let bsp_stems = [
        "startup",
        "system",
        "bsp_io",
        "bsp_clocks",
        "bsp_delay",
        "bsp_irq",
        "bsp_common",
        "bsp_register_protection",
        "bsp_power",
        "bsp_security",
        "bsp_macl",
        "bsp_group_irq", // NMI_Handler
        "bsp_rom_registers",
        "bsp_guard",
        "bsp_sbrk",
    ];
    let mut build = cc::Build::new();

    walkdir::WalkDir::new(&fsp_src)
        .follow_links(true)
        .into_iter()
        .map(|e| e.expect("Can't walk in fsp src"))
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".c"))
        .filter(|e| {
            let stem = e.path().file_stem().unwrap().to_str().unwrap();
            bsp_stems.contains(&stem) || modules.iter().find(|m| stem == **m).is_some()
        })
        .for_each(|e| {
            build.file(e.path());
        });

    let objects = build
        .includes(&include_dirs)
        .define(&bsp_mcu_group, Some("1"))
        .compile_intermediates();

    pre_link_archive("fsp_prelinked", objects);

    assert!(out_path.join(&"libfsp_prelinked.a").exists());

    if out_path.join("script").exists() {
        fs::remove_dir_all(&linker_scripts).unwrap();
    }

    std::process::Command::new("cp")
        .arg("-f")
        .arg("-r")
        .arg("./script")
        .arg(linker_scripts)
        .status()
        .expect("failed to copy `script`");

    let bindgen = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for
        .header_contents(
            "r_ethernet_rs_wrapper.h",
            r#"
                #include "bsp_api.h"
                #include "bsp_cfg.h"
                #include "bsp_mcu_family_cfg.h"
                #include "renesas.h"
                #include "bsp_elc.h"
                #include "bsp_irq.h"

                #include "r_ether.h"
                #include "r_ether_phy.h"
                #include "r_ether_api.h"

                #include <r_ioport_api.h>
                #include <r_ioport.h>
            "#,
        )
        .use_core()
        .clang_arg(format!("-D{bsp_mcu_group}=1"))
        .clang_args(
            include_dirs
                .iter()
                .map(|d| path::absolute(d).expect("Can't resolve absolute path"))
                .map(|e| format!("-I{}", e.as_os_str().to_str().unwrap())),
        )
        .allowlist_item("e_elc_event_.*")
        .allowlist_item("fsp_err_t")
        .rustified_enum(&format!("e_elc_event_{mcu_group}"))
        .allowlist_item("BSP_ICU_VECTOR_MAX_ENTRIES")
        // -
        ;

    let bindgen = if cfg!(feature = "mod-r_ether") {
        bindgen
            // .allowlist_item(".*ether_(?!phy_).*")
            // .allowlist_item("^ETHER_(?!PHY_).*")
            .allowlist_item(".*ether_.*")
            .allowlist_item(".*ETHER_.*")
            .parse_callbacks(Box::new(EtherCallbackArgs))
    } else {
        bindgen
    };

    let bindgen = if cfg!(feature = "mod-r_ether_phy") {
        bindgen
            .allowlist_item(".*ether_phy_.*")
            .allowlist_item(".*ETHER_PHY_.*")
            .rustified_enum("e_ether_padding")
            .rustified_enum("e_ether_phy_mii_type")
            .rustified_enum("e_ether_phy_lsi_type")
    } else {
        bindgen
    };

    let bindgen = if cfg!(feature = "mod-r_ioport") {
        bindgen
            .allowlist_item(".*ioport_.*")
            .allowlist_item(".*io_port_.*")
            .allowlist_item(".*IOPORT_.*")
            .rustified_enum("e_bsp_io_port_pin_t")
            // Those two are consts to do bit logic
            .constified_enum_module("e_ioport_cfg_options")
            .constified_enum_module("e_ioport_peripheral")
    } else {
        bindgen
    };

    bindgen
        .derive_default(true)
        // .derive_debug(true)
        // .derive_partialeq(true)
        .parse_callbacks(Box::new(
            bindgen::CargoCallbacks::new().rerun_on_header_files(true),
        ))
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings")
        .write(Box::new(&mut rust_codegen))
        .expect("Couldn't write bindings!");

    write!(
        &mut rust_codegen,
        "\npub type e_elc_event = e_elc_event_ra6m3;\n"
    )
    .unwrap();
}

#[derive(Debug)]
struct EtherCallbackArgs;
impl bindgen::callbacks::ParseCallbacks for EtherCallbackArgs {
    fn field_visibility(
        &self,
        info: bindgen::callbacks::FieldInfo<'_>,
    ) -> Option<bindgen::FieldVisibilityKind> {
        if info.type_name == "st_ether_callback_args" {
            Some(bindgen::FieldVisibilityKind::PublicCrate)
        } else {
            None
        }
    }
}

macro_rules! add_module {
    ($modules:expr,$module_feat:expr) => {
        if build_cfg!(feature = $module_feat) {
            $modules.push(&$module_feat[4..]);
        }
    };
}

#[build_cfg_main]
fn main() {
    let mut modules = Vec::<&'static str>::new();

    add_module!(modules, "mod-r_icu");
    add_module!(modules, "mod-r_flash_hp");
    add_module!(modules, "mod-r_ioport");
    add_module!(modules, "mod-r_ether");
    add_module!(modules, "mod-r_ether_phy");
    add_module!(modules, "mod-r_ether_phy_target_ics1894");
    add_module!(modules, "mod-r_ether_phy_target_ksz8091rnb");
    add_module!(modules, "mod-r_ether_phy_target_dp83620");
    add_module!(modules, "mod-r_ether_phy_target_ksz8041");

    wrap_component(&modules);
}

fn pre_link_archive(new_name: &str, objects: Vec<PathBuf>) {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("Output dir must be set"));
    let joined_obj_name = format!("{new_name}.o");
    let archive_name = format!("lib{new_name}.a");

    let ld = std::env::var("LD").expect("LD must be set");

    std::process::Command::new(&ld)
        .arg("-r")
        .args(objects)
        .arg("-o")
        .arg(&joined_obj_name)
        .current_dir(&out_path)
        .spawn()
        .expect("failed to prelink")
        .wait()
        .unwrap();

    cc::Build::new()
        .get_archiver()
        .arg("rcs")
        .arg(&archive_name)
        .arg(&joined_obj_name)
        .current_dir(&out_path)
        .spawn()
        .expect("failed to archive")
        .wait()
        .unwrap();
}

fn bsp_mcu_group() -> Option<String> {
    Some(format!("BSP_MCU_GROUP_{}", mcu_group()?.to_uppercase()))
}

fn mcu_group() -> Option<&'static str> {
    Some(if cfg!(feature = "ra0e1") {
        "ra0e1"
    } else if cfg!(feature = "ra2a1") {
        "ra2a1"
    } else if cfg!(feature = "ra2a2") {
        "ra2a2"
    } else if cfg!(feature = "ra2e1") {
        "ra2e1"
    } else if cfg!(feature = "ra2e2") {
        "ra2e2"
    } else if cfg!(feature = "ra2e3") {
        "ra2e3"
    } else if cfg!(feature = "ra2l1") {
        "ra2l1"
    } else if cfg!(feature = "ra4e1") {
        "ra4e1"
    } else if cfg!(feature = "ra4e2") {
        "ra4e2"
    } else if cfg!(feature = "ra4m1") {
        "ra4m1"
    } else if cfg!(feature = "ra4m2") {
        "ra4m2"
    } else if cfg!(feature = "ra4m3") {
        "ra4m3"
    } else if cfg!(feature = "ra4t1") {
        "ra4t1"
    } else if cfg!(feature = "ra4w1") {
        "ra4w1"
    } else if cfg!(feature = "ra6e1") {
        "ra6e1"
    } else if cfg!(feature = "ra6e2") {
        "ra6e2"
    } else if cfg!(feature = "ra6m1") {
        "ra6m1"
    } else if cfg!(feature = "ra6m2") {
        "ra6m2"
    } else if cfg!(feature = "ra6m3") {
        "ra6m3"
    } else if cfg!(feature = "ra6m4") {
        "ra6m4"
    } else if cfg!(feature = "ra6m5") {
        "ra6m5"
    } else if cfg!(feature = "ra6t1") {
        "ra6t1"
    } else if cfg!(feature = "ra6t2") {
        "ra6t2"
    } else if cfg!(feature = "ra6t3") {
        "ra6t3"
    } else if cfg!(feature = "ra8m1") {
        "ra8m1"
    } else if cfg!(feature = "ra8d1") {
        "ra8d1"
    } else if cfg!(feature = "ra8t1") {
        "ra8t1"
    } else {
        return None;
    })
}

/*
CFLAGS = """-isysroot=/opt/arm-bare_newlibnanolto_cortex_m4f_nommu-eabihf/arm-bare_newlibnanolto_cortex_m4f_nommu-eabihf \
  -DBOARD_HEATHUB_V_0_1=1 -DFIRMWARE_VERSION=\\\"0.14.2\\\" -DFW_VERSION_MAJOR=0 -DFW_VERSION_MINOR=14 -DFW_VERSION_PATCH=2 \
  -DHW_ID=\\\"84:3010:0001\\\" -DHW_ID_PID=0x3010 -DHW_ID_REV=0x0001 -DHW_ID_VID=0x84 -DSTATICFS_CONSUMER=1 -DTX_INCLUDE_USER_DEFINE_FILE \
  -DURTU_APP=\\\"HEATHUB\\\" -DURTU_APP_HEATHUB=1 -DURTU_APP_LC=\\\"heathub\\\" -DURTU_BOARD=HEATHUB_V_0_1 -DURTU_BOARD_QSPI_FLASH=0 \
  -DURTU_BOARD_U1_MODE=URTU_AO_M_VOLTAGE -DURTU_BOARD_U2_MODE=URTU_AO_M_VOLTAGE -DURTU_IWDT_ENABLE=1 -DURTU_TLS_TEST=0 -DWIFI_WF200=1 \
  -I/home/ddystopia/job/fw-micrortu/inc \
  -I/home/ddystopia/job/fw-micrortu/ra_gen \
  -I/home/ddystopia/job/fw-micrortu/dep/cmsis5/CMSIS/Core/Include \
  -I/home/ddystopia/job/fw-micrortu/dep/cmsis5/Device/ARM/ARMCM4/Include \
  -I/home/ddystopia/job/fw-micrortu/ra_cfg/fsp_cfg \
  -I/home/ddystopia/job/fw-micrortu/ra_cfg/fsp_cfg/bsp \
  -I/home/ddystopia/job/fw-micrortu/dep/fsp/ra/fsp/inc \
  -I/home/ddystopia/job/fw-micrortu/dep/fsp/ra/fsp/inc/api \
  -I/home/ddystopia/job/fw-micrortu/dep/fsp/ra/fsp/inc/instances \
  -I/home/ddystopia/job/fw-micrortu/dep/fsp/ra/fsp/src/bsp/mcu/all \
  -I/home/ddystopia/job/fw-micrortu/dep/staticfs \
  -I/home/ddystopia/job/fw-micrortu/ra_cfg/fsp_cfg/azure/tx \
  -I/home/ddystopia/job/fw-micrortu/dep/fsp/ra/fsp/src/rm_threadx_port \
  -I/home/ddystopia/job/fw-micrortu/dep/threadx/common/inc \
  -I/home/ddystopia/job/fw-micrortu/dep/threadx/ports/cortex_m4/gnu/inc \
  -I/home/ddystopia/job/fw-micrortu/dep/ra-bsp/inc \
  -Wall -mlittle-endian -mthumb -mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16 -mno-unaligned-access --std=gnu11 \
  -ffunction-sections -fdata-sections -Woverride-init -fno-short-enums -gdwarf-4 -flto=auto -g3 -O0"""
*/
