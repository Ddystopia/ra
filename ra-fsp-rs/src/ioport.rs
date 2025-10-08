use core::mem::MaybeUninit;

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

const _: () = assert!(
    IOPORT_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option IOPORT_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

pub struct IoPortInstance {
    ctrl: ioport_instance_ctrl_t,
    cfg: ioport_cfg_t,
    inst: ioport_instance_t,
}

#[derive(Debug, Copy, Clone)]
pub struct IoPortConfig(pub &'static [ioport_pin_cfg_t]);

unsafe impl crate::Block for IoPortInstance {
    type CConfig = ioport_cfg_t;
    type CInstance = ioport_instance_t;
    type CApi = ioport_api_t;

    const API: &ioport_api_t = unsafe { &g_ioport_on_ioport };

    fn instance(&mut self) -> &mut ioport_instance_t {
        self.inst.p_ctrl = (&raw mut self.ctrl).cast::<core::ffi::c_void>();
        self.inst.p_cfg = &raw const self.cfg;
        &mut self.inst
    }
}

unsafe impl Send for IoPortInstance {}
unsafe impl Sync for IoPortInstance {}

impl IoPortInstance {
    pub const fn new(ports: crate::pac::PORT0, conf: IoPortConfig) -> Self {
        _ = ports;

        Self {
            ctrl: unsafe { core::mem::zeroed() },
            cfg: conf.c_conf(),
            inst: ioport_instance_t {
                p_ctrl: core::ptr::null_mut(),
                p_cfg: core::ptr::null(),
                p_api: <Self as crate::Block>::API,
            },
        }
    }
    pub unsafe fn from_ptr<'a>(ptr: *mut ioport_instance_ctrl_t) -> &'a mut Self {
        unsafe { &mut *ptr.cast::<IoPortInstance>() }
    }
    pub fn open(&mut self) -> Result<(), fsp_err_t> {
        match unsafe { R_IOPORT_Open((&mut self.ctrl) as *mut _ as *mut _, self.inst.p_cfg) } {
            0 => Ok(()),
            err => Err(err),
        }
    }
    pub fn close(&mut self) -> Result<(), fsp_err_t> {
        match unsafe { R_IOPORT_Close((&mut self.ctrl) as *mut _ as *mut _) } {
            0 => Ok(()),
            err => Err(err),
        }
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
