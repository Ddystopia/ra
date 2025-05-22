#![no_std]
#![feature(ptr_metadata)]

// todo: set stackoverflow protection: splim
// todo: study bsp_cfg.h in more depth
// BSP_FEATURE_ICU_FIXED_IELSR_COUNT

pub mod generated {
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(unused_imports)]

    include!(concat!(env!("OUT_DIR"), "/out.rs"));
}

#[cfg(feature = "mod-r_ether")]
pub mod r_ether {
    use crate::generated::{
        ether_callback_args_t, ETHER_EVENT_INTERRUPT, ETHER_EVENT_LINK_OFF, ETHER_EVENT_LINK_ON,
    };

    #[derive(Debug)]
    #[non_exhaustive]
    pub struct InterruptCause {
        pub went_up: bool,
        pub went_down: bool,
        pub receive: bool,
        pub transmits: bool,
    }

    pub fn interrupt_cause(args: &mut ether_callback_args_t) -> InterruptCause {
        const EVENT_TAKEN: u32 = u32::MAX;

        /* Transmit Complete. (all pending transmissions) */
        const ETHER_EDMAC_INTERRUPT_FACTOR_TC: u32 = 1 << 21;
        /* Frame Receive. */
        const ETHER_EDMAC_INTERRUPT_FACTOR_FR: u32 = 1 << 18;

        let mut cause = InterruptCause {
            receive: false,
            transmits: false,
            went_up: false,
            went_down: false,
        };

        match core::mem::replace(&mut args.event, EVENT_TAKEN) {
            EVENT_TAKEN => (),
            ETHER_EVENT_INTERRUPT => {
                let receive_mask = ETHER_EDMAC_INTERRUPT_FACTOR_FR;
                let trasmit_mask = ETHER_EDMAC_INTERRUPT_FACTOR_TC;

                /* Packet received. */
                if receive_mask == (args.status_eesr & receive_mask) {
                    cause.receive = true;
                }

                if trasmit_mask == (args.status_eesr & trasmit_mask) {
                    cause.transmits = true;
                }
            }
            ETHER_EVENT_LINK_ON => {
                cause.went_up = true;
            }
            ETHER_EVENT_LINK_OFF => {
                cause.went_down = true;

                /*
                 * When the link is re-established, the Ethernet driver will reset all of the buffer descriptors.
                 */
            }
            _ => {}
        };

        cause
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn SysTick_Handler() {
    unsafe extern "C" {
        fn SysTick();
    }
    unsafe { SysTick() }
}

#[unsafe(no_mangle)]
pub extern "C" fn __assert_func(file: *const u8, line: i32, func: *const u8, expr: *const u8) {
    let file = unsafe { core::ffi::CStr::from_ptr(file) };
    let func = unsafe { core::ffi::CStr::from_ptr(func) };
    let expr = unsafe { core::ffi::CStr::from_ptr(expr) };

    let file = file.to_str().unwrap_or("<Invalid UTF-8>");
    let func = func.to_str().unwrap_or("<Invalid UTF-8>");
    let expr = expr.to_str().unwrap_or("<Invalid UTF-8>");

    panic!("Assertion failed in file: {file}, line: {line}, function: {func}, expression: {expr}",);
}

#[cfg(feature = "log")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __fsp_log_func(
    level: u32,
    module: *const u8,
    file: *const u8,
    _line: i32,
    fmt: *const u8,
) {
    let module = unsafe { core::ffi::CStr::from_ptr(module) };
    let file = unsafe { core::ffi::CStr::from_ptr(file) };
    let fmt = unsafe { core::ffi::CStr::from_ptr(fmt) };
    let module = module.to_str().unwrap_or("<Invalid UTF-8>");
    let file = file.to_str().unwrap_or("<Invalid UTF-8>");
    let fmt = fmt.to_str().unwrap_or("<Invalid UTF-8>");

    let lvl = match level {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        3 => log::Level::Debug,
        4 => log::Level::Trace,
        _ => log::Level::Info,
    };

    log::__private_api::log(
        log::__log_logger!(__log_global_logger),
        format_args!("{fmt}"),
        lvl,
        &(module, file, log::__private_api::loc()),
        (),
    );
}

// todo:
// void _log(unsigned level, const char *module, const char *file, int line, const char *fmt, ...)
// 		__attribute__ ((format (printf, 5, 6)));
// #define FSP_LOG_PRINT(X)    _log(1, "bsp", __FILE__, __LINE__, "%s", (X))
//

#[cfg(feature = "mod-r_ioport")]
mod r_ioport_impl {
    use super::generated::*;

    unsafe impl Sync for ioport_instance_ctrl_t {}
    unsafe impl Sync for ioport_instance_t {}
    unsafe impl Sync for ioport_cfg_t {}
    unsafe impl Sync for ioport_pin_cfg_t {}
}

#[cfg(feature = "mod-r_ether")]
mod r_ether_impl {
    use super::generated::*;

    unsafe impl Sync for ether_cfg_t {}
    unsafe impl Sync for ether_api_t {}
    unsafe impl Sync for ether_instance_ctrl_t {}
    unsafe impl Sync for ether_instance_t {}
    unsafe impl Sync for ether_extended_cfg_t {}
    unsafe impl Sync for ether_instance_descriptor_t {}
    unsafe impl Sync for ether_callback_args_t {}
    unsafe impl Send for ether_callback_args_t {}
}

#[cfg(feature = "mod-r_ether_phy")]
mod r_ether_phy_impl {
    use super::generated::*;

    unsafe impl Sync for ether_phy_cfg_t {}
    unsafe impl Sync for ether_phy_api_t {}
    unsafe impl Sync for ether_phy_instance_ctrl_t {}
    unsafe impl Sync for ether_phy_instance_t {}
}
