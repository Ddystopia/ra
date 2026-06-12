use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit, zeroed},
    ptr,
};

use crate::{
    DriverBox, TypeStateResult,
    pin_init::{pin_data, PinInit, pin_init_from_closure},
    state_markers::Closed,
};
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

#[repr(C)] // `#[repr(C)]` is for typestate
#[pin_data]
pub struct IoPort<S> {
    ctrl: UnsafePinned<ioport_instance_ctrl_t>,
    cfg: UnsafePinned<ioport_cfg_t>,
    inst: UnsafePinned<ioport_instance_t>,
    regs: pac::PORT0,
    _marker: PhantomData<S>,
}

#[derive(Debug, Copy, Clone)]
pub struct IoPortConfig(pub &'static [ioport_pin_cfg_t]);

unsafe impl<S> crate::LifetimeDriver for IoPort<S> {
    type Target<'a> = IoPort<S>;
}

unsafe impl<S> crate::Block for IoPort<S> {
    type Config = ioport_cfg_t;
    type Instance = ioport_instance_t;
    type Api = ioport_api_t;
    type State = S;

    const API: &ioport_api_t = unsafe { &g_ioport_on_ioport };

    fn ctrl(&self) -> *mut core::ffi::c_void {
        UnsafePinned::raw_get(&raw const self.ctrl).cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

unsafe impl<S> Send for IoPort<S> {}
unsafe impl<S> Sync for IoPort<S> {}

unsafe fn init_open(
    slot: *mut IoPort<Opened>,
    regs: pac::PORT0,
    conf: IoPortConfig,
) -> crate::Result<()> {
    unsafe {
        let this = IoPort {
            ctrl: UnsafePinned::new(core::mem::zeroed()),
            cfg: UnsafePinned::new(conf.c_conf()),
            regs,
            inst: UnsafePinned::new(ioport_instance_t {
                p_ctrl: ptr::null_mut(),
                p_cfg: ptr::null(),
                p_api: <IoPort<Opened> as crate::Block>::API,
            }),
            _marker: PhantomData,
        };
        ptr::write(slot, this);
        (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast();
        (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const().cast();

        let p_ctrl = (*(*slot).inst.get()).p_ctrl;
        let p_cfg = (*(*slot).inst.get()).p_cfg;

        crate::fsp_try_unsafe!(R_IOPORT_Open(p_ctrl, p_cfg))?;
        Ok(())
    }
}

impl IoPort<Opened> {
    pub const fn new_open(ports: pac::PORT0, conf: IoPortConfig) -> impl PinInit<Self, fsp_err_t> {
        unsafe { pin_init_from_closure(move |slot: *mut Self| init_open(slot, ports, conf)) }
    }
    pub fn close(this: DriverBox<IoPort<Opened>>) -> TypeStateResult<IoPort<Closed>, Self> {
        let mut this = ManuallyDrop::new(this);
        let ctrl = this.ctrl.get().cast();
        match crate::fsp_try_unsafe!(R_IOPORT_Close(ctrl)) {
            Ok(()) => Ok(unsafe {
                let ptr = ptr::from_mut(this.get_unchecked_mut());
                DriverBox::new_unchecked(&mut *ptr.cast())
            }),
            Err(err) => Err((ManuallyDrop::into_inner(this), err)),
        }
    }
}

impl IoPort<Closed> {
    pub const fn new() -> Self {
        Self {
            ctrl: unsafe { zeroed() },
            cfg: unsafe { zeroed() },
            inst: unsafe { zeroed() },
            regs: unsafe { zeroed() },
            _marker: PhantomData,
        }
    }
    pub fn open(
        this: DriverBox<IoPort<Closed>>,
        cfg: IoPortConfig,
        ports: pac::PORT0,
    ) -> TypeStateResult<IoPort<Opened>, Self> {
        if this.is_open() {
            return Err((this, ra_fsp_sys::generated::e_fsp_err::FSP_ERR_ALREADY_OPEN));
        }

        unsafe {
            let mut this = ManuallyDrop::new(this);

            let p_this = ptr::from_mut(this.get_unchecked_mut());

            let p_this = p_this.cast::<IoPort<Opened>>();
            init_open(p_this, ports, cfg).map_err(|e| (ManuallyDrop::into_inner(this), e))?;
            Ok(DriverBox::new_unchecked(&mut *p_this))
        }
    }
}

impl<S> IoPort<S> {
    /// # Safety
    ///
    /// `ptr` must point to the `ctrl` field of a live `IoPort<S>` in the
    /// matching state (the struct is `repr(C)` with `ctrl` first), and the
    /// returned reference must be unique for `'a`.
    pub unsafe fn from_ptr<'a>(ptr: *mut ioport_instance_ctrl_t) -> &'a mut Self {
        unsafe { &mut *ptr.cast::<Self>() }
    }
    // NOTE(soundness/policy): on a driver from `IoPort::<Closed>::new()` this
    // is a zeroed placeholder token (PORT0 is a ZST), replaced by the real one
    // at `open` — handing it out bypasses the PAC's singleton discipline, the
    // same single-core policy as `Gpt::regs`.
    pub const fn ports(&self) -> &pac::PORT0 {
        &self.regs
    }
    pub fn is_open(&self) -> bool {
        unsafe { (*self.ctrl.get()).open != 0 }
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
