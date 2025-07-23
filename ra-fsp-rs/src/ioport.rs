#![allow(non_upper_case_globals)]

use core::{mem::MaybeUninit, pin::Pin};

use crate::unsafe_pinned::UnsafePinned;

use ra_fsp_sys::generated::{R_IOPORT_Close, R_IOPORT_Open, fsp_err_t};

pub use ra_fsp_sys::generated::{
    e_bsp_io_port_pin_t, //
    e_ioport_cfg_options,
    e_ioport_peripheral,
    g_ioport_on_ioport,
    ioport_api_t,
    ioport_cfg_t,
    ioport_instance_ctrl_t,
    ioport_instance_t,
    ioport_pin_cfg_t,
};

pub struct IoPortInstance(UnsafePinned<ioport_instance_ctrl_t>);

#[derive(Debug, Copy, Clone)]
pub struct IoPortConfig(pub &'static [ioport_pin_cfg_t]);

// Could be a trait if const members are allowed
#[doc(hidden)]
pub mod _for_c_dyn_macro {
    #![allow(non_camel_case_types)]

    use super::*;

    pub type Config = ioport_cfg_t;
    pub type Instance = IoPortInstance;
    pub type CInstance = ioport_instance_t;
    pub type CApi = ioport_api_t;

    pub const C_API: &'static CApi = unsafe { &*&raw const g_ioport_on_ioport };
}

// todo: ensure that drivers to not store `p_ctrl`, or else we need
//       to ensure `'static` lifetime of `self`, but still allow `&mut`.
//       If that stored pointer is used concurrently with `&mut`, would this
//       introduce races? It's okay to alias `&mut` due to `UnsafePinned`.
pub unsafe trait IoPort {
    fn open(self: Pin<&mut Self>, conf: &'static ioport_cfg_t) -> Result<(), fsp_err_t>;
    fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t>;
    fn c_api(&self) -> &'static ioport_api_t {
        unsafe { &*&raw const g_ioport_on_ioport }
    }
}

unsafe impl IoPort for IoPortInstance {
    fn open(self: Pin<&mut Self>, conf: &'static ioport_cfg_t) -> Result<(), fsp_err_t> {
        match unsafe { R_IOPORT_Open(get_mut(self), conf) } {
            0 => Ok(()),
            err => Err(err),
        }
    }
    fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t> {
        match unsafe { R_IOPORT_Close(get_mut(self)) } {
            0 => Ok(()),
            err => Err(err),
        }
    }
    fn c_api(&self) -> &'static ioport_api_t {
        _for_c_dyn_macro::C_API
    }
}

#[inline(always)]
const fn get_mut(this: Pin<&mut IoPortInstance>) -> *mut core::ffi::c_void {
    unsafe { this.get_unchecked_mut().ptr().cast() }
}

impl IoPortInstance {
    pub const fn new() -> Self {
        // There is always `open` field and methods check it. When zeroed, they
        // will return with an error unless it is opened.

        let zeroed: ioport_instance_ctrl_t = unsafe { MaybeUninit::zeroed().assume_init() };

        Self(UnsafePinned::new(zeroed))
    }

    #[inline(always)]
    pub const fn ptr(&self) -> *mut ioport_instance_ctrl_t {
        UnsafePinned::raw_get(&raw const self.0)
    }
}

impl IoPortConfig {
    pub const fn new(data: &'static [ioport_pin_cfg_t]) -> Self {
        Self(data)
    }
    pub const fn c_conf(self) -> ioport_cfg_t {
        let mut cfg: ioport_cfg_t = unsafe { MaybeUninit::zeroed().assume_init() };

        cfg.number_of_pins = self.0.len() as u16;
        cfg.p_pin_cfg_data = self.0.as_ptr();

        cfg
    }
}

// fsp_err_t R_IOPORT_Open (ioport_ctrl_t * const p_ctrl, const ioport_cfg_t * p_cfg)
// fsp_err_t R_IOPORT_Close (ioport_ctrl_t * const p_ctrl)
// fsp_err_t R_IOPORT_PinsCfg (ioport_ctrl_t * const p_ctrl, const ioport_cfg_t * p_cfg)
// fsp_err_t R_IOPORT_PinCfg (ioport_ctrl_t * const p_ctrl, bsp_io_port_pin_t pin, uint32_t cfg)
// fsp_err_t R_IOPORT_PinEventInputRead (ioport_ctrl_t * const p_ctrl, bsp_io_port_pin_t pin, bsp_io_level_t * p_pin_event)
// fsp_err_t R_IOPORT_PinEventOutputWrite (ioport_ctrl_t * const p_ctrl, bsp_io_port_pin_t pin, bsp_io_level_t pin_value)
// fsp_err_t R_IOPORT_PinRead (ioport_ctrl_t * const p_ctrl, bsp_io_port_pin_t pin, bsp_io_level_t * p_pin_value)
// fsp_err_t R_IOPORT_PinWrite (ioport_ctrl_t * const p_ctrl, bsp_io_port_pin_t pin, bsp_io_level_t level)
// fsp_err_t R_IOPORT_PortDirectionSet (ioport_ctrl_t * const p_ctrl, bsp_io_port_t         port, ioport_size_t         direction_values, ioport_size_t         mask)
// fsp_err_t R_IOPORT_PortEventInputRead (ioport_ctrl_t * const p_ctrl, bsp_io_port_t port, ioport_size_t * p_event_data)
// fsp_err_t R_IOPORT_PortEventOutputWrite (ioport_ctrl_t * const p_ctrl, bsp_io_port_t         port, ioport_size_t         event_data, ioport_size_t         mask_value)
// fsp_err_t R_IOPORT_PortRead (ioport_ctrl_t * const p_ctrl, bsp_io_port_t port, ioport_size_t * p_port_value)
// fsp_err_t R_IOPORT_PortWrite (ioport_ctrl_t * const p_ctrl, bsp_io_port_t port, ioport_size_t value, ioport_size_t mask)
