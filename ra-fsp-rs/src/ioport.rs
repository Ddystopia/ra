use core::{cell::UnsafeCell, mem::MaybeUninit, ptr};

use pin_init::{PinInit, pin_init_from_closure};
use ra_fsp_sys::generated::IOPORT_CFG_PARAM_CHECKING_ENABLE;
pub use ra_fsp_sys::generated::{
    //
    // IoPortBlocks,
    R_IOPORT_Close,
    R_IOPORT_Open,
    e_bsp_io_port_pin_t,
    e_ioport_cfg_options,
    e_ioport_peripheral,
    fsp_err_t,
    g_ioport_on_ioport,
    ioport_api_t,
    ioport_cfg_t,
    ioport_instance_ctrl_t,
    ioport_instance_t,
    ioport_pin_cfg_t,
};

use crate::{pac, state_markers::Opened, unsafe_pinned::UnsafePinned};

const _: () = assert!(
    IOPORT_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option IOPORT_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

pub struct IoPort {
    ctrl: UnsafeCell<ioport_instance_ctrl_t>,
    cfg: UnsafePinned<ioport_cfg_t>,
    inst: UnsafePinned<ioport_instance_t>,
    ports: pac::PORT0,
}

#[derive(Debug, Copy, Clone)]
pub struct IoPortConfig(pub &'static [ioport_pin_cfg_t]);

unsafe impl crate::Block for IoPort {
    type Config = ioport_cfg_t;
    type Instance = ioport_instance_t;
    type Api = ioport_api_t;
    type State = Opened;
    type Context = ();

    const API: &ioport_api_t = unsafe { &g_ioport_on_ioport };

    fn ctrl(&self) -> *mut core::ffi::c_void {
        UnsafeCell::raw_get(&raw const self.ctrl).cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

unsafe impl Send for IoPort {}
unsafe impl Sync for IoPort {}

impl IoPort {
    pub const fn new(ports: pac::PORT0, conf: IoPortConfig) -> impl PinInit<Self> {
        unsafe {
            pin_init_from_closure(move |slot: *mut Self| {
                *slot = Self {
                    ctrl: UnsafeCell::new(core::mem::zeroed()),
                    cfg: UnsafePinned::new(conf.c_conf()),
                    ports,
                    inst: UnsafePinned::new(ioport_instance_t {
                        p_ctrl: ptr::null_mut(),
                        p_cfg: ptr::null(),
                        p_api: <Self as crate::Block>::API,
                    }),
                };
                (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast();
                (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const().cast();
                Ok(())
            })
        }
    }
    pub unsafe fn from_ptr<'a>(ptr: *mut ioport_instance_ctrl_t) -> &'a mut Self {
        unsafe { &mut *ptr.cast::<IoPort>() }
    }
    pub fn open(&mut self) -> Result<(), fsp_err_t> {
        let ctrl = (&mut self.ctrl) as *mut _ as *mut _;
        crate::fsp_try_unsafe!(R_IOPORT_Open(ctrl, self.cfg.get()))
    }
    pub fn close(&mut self) -> Result<(), fsp_err_t> {
        let ctrl = (&mut self.ctrl) as *mut _ as *mut _;
        crate::fsp_try_unsafe!(R_IOPORT_Close(ctrl))
    }
    pub const fn ports(&self) -> &pac::PORT0 {
        &self.ports
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
