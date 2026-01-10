use core::{
    any::TypeId,
    mem::zeroed,
    pin::Pin,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use crate::pin_init::{PinInit, pin_data, pin_init_from_closure, pinned_drop};
use ra_fsp_sys::generated::{
    self as raw, self as api, R_GLCDC_BASE, R_GLCDC_BufferChange, R_GLCDC_Close, R_GLCDC_Open,
    R_GLCDC_Type, display_api_t, display_cfg_t, display_ctrl_t, display_frame_layer_t,
    display_instance_t, e_display_state, e_fsp_err, fsp_err_t, glcdc_extended_cfg_t,
    glcdc_instance_ctrl_t,
};

use crate::{
    Block, Callback, Result,
    display_api::*,
    fsp_try_unsafe, pac,
    state_markers::{Closed, Opened},
    unsafe_pinned::UnsafePinned,
    utils,
};

unsafe extern "C" {
    pub safe fn glcdc_line_detect_isr();
}

struct CallbackContext {
    regs: pac::GLCDC,
    user_data: *const (),
    current_owned_buffer: [AtomicPtr<u8>; 2],
}

#[repr(C)] // `#[repr(C)]` is for `Glcdc::<Closed>::open`
#[pin_data(PinnedDrop)]
pub struct Glcdc<State: 'static> {
    c_ext_cfg: core::mem::MaybeUninit<UnsafePinned<glcdc_extended_cfg_t>>,
    ctrl: UnsafePinned<glcdc_instance_ctrl_t>,
    cfg: UnsafePinned<display_cfg_t>,
    inst: UnsafePinned<display_instance_t>, // points to cfg and ctrl
    prev_owned_buffer: [*mut u8; 2],
    callback_ctx: UnsafePinned<CallbackContext>,
    _marker: core::marker::PhantomData<State>,
}

#[allow(non_snake_case)]
#[derive(Default)]
pub struct GlcdcExtendedConfig {
    pub tcon_hsync: raw::glcdc_tcon_pin_t,
    pub tcon_vsync: raw::glcdc_tcon_pin_t,
    pub tcon_de: raw::glcdc_tcon_pin_t,
    pub correction_proc_order: raw::glcdc_correction_proc_order_t,
    pub clksrc: raw::glcdc_clk_src_t,
    pub clock_div_ratio: raw::glcdc_panel_clk_div_t,
    pub dithering_mode: raw::glcdc_dithering_mode_t,
    pub dithering_pattern_A: raw::glcdc_dithering_pattern_t,
    pub dithering_pattern_B: raw::glcdc_dithering_pattern_t,
    pub dithering_pattern_C: raw::glcdc_dithering_pattern_t,
    pub dithering_pattern_D: raw::glcdc_dithering_pattern_t,
    // unsupported
    // pub phy_layer: *mut ::core::ffi::c_void,
}

const API: display_api_t = display_api_t {
    open: Some(api::R_GLCDC_Open),
    close: Some(api::R_GLCDC_Close),
    start: Some(api::R_GLCDC_Start),
    stop: Some(api::R_GLCDC_Stop),
    layerChange: Some(api::R_GLCDC_LayerChange),
    bufferChange: Some(api::R_GLCDC_BufferChange),
    correction: Some(api::R_GLCDC_ColorCorrection),
    clut: Some(api::R_GLCDC_ClutUpdate),
    clutEdit: Some(api::R_GLCDC_ClutEdit),
    colorKeySet: Some(api::R_GLCDC_ColorKeySet),
    statusGet: Some(api::R_GLCDC_StatusGet),
};

unsafe impl<S> crate::LifetimeDriver for Glcdc<S> {
    type Target<'a> = Glcdc<S>;
}

unsafe impl<S> Block for Glcdc<S> {
    type Config = display_cfg_t;
    type Instance = display_instance_t;
    type Api = display_api_t;
    type State = S;

    const API: &'static Self::Api = &API;

    fn ctrl(&self) -> *mut core::ffi::c_void {
        self.ctrl.get().cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

unsafe impl<S: 'static> Send for Glcdc<S> {}
unsafe impl<S: 'static> Sync for Glcdc<S> {}

impl Glcdc<Closed> {
    pub fn new(regs: pac::GLCDC) -> Self {
        const {
            assert!(
                e_display_state::DISPLAY_STATE_CLOSED as u32 == 0,
                "`DISPLAY_STATE_CLOSED` must be zero"
            );
        }
        Self {
            callback_ctx: UnsafePinned::new(CallbackContext {
                regs,
                user_data: ptr::null(),
                current_owned_buffer: [AtomicPtr::default(), AtomicPtr::default()],
            }),
            prev_owned_buffer: [ptr::null_mut(); 2],
            c_ext_cfg: core::mem::MaybeUninit::zeroed(),
            ctrl: UnsafePinned::new(unsafe { zeroed() }),
            cfg: UnsafePinned::new(unsafe { zeroed() }),
            inst: UnsafePinned::new(unsafe { zeroed() }),
            _marker: core::marker::PhantomData,
        }
    }
    pub fn open(
        self: Pin<&mut Self>,
        cfg: DisplayConf<GlcdcExtendedConfig>,
    ) -> Result<Pin<&mut Glcdc<Opened>>> {
        unsafe {
            let this = ptr::from_mut(self.get_unchecked_mut());
            let regs = ptr::read(&(*(*this).callback_ctx.get()).regs);

            if (*(*this).ctrl.get()).state != e_display_state::DISPLAY_STATE_CLOSED {
                return Err(e_fsp_err::FSP_ERR_ALREADY_OPEN);
            }

            let this = this.cast::<Glcdc<Opened>>();
            init_open(this, regs, cfg)?;
            Ok(Pin::new_unchecked(&mut *this))
        }
    }
}

impl Glcdc<Opened> {
    pub fn new_open(
        glcdc: pac::GLCDC,
        cfg: DisplayConf<GlcdcExtendedConfig>,
    ) -> impl PinInit<Glcdc<Opened>, fsp_err_t> {
        unsafe { pin_init_from_closure(|slot| init_open(slot, glcdc, cfg)) }
    }
}

#[pinned_drop]
impl<S: 'static> PinnedDrop for Glcdc<S> {
    fn drop(mut self: Pin<&mut Self>) {
        if TypeId::of::<Closed>() == TypeId::of::<S>() {
            return;
        }

        // - If we are in the thread (can't preempt an interrupt), we can't preempt the callback.
        //   - Interrupt is not running, thus after we reset callback,
        //      we are sure nobody will use the `p_context` anymore, so we can free it.
        //   - Todo: assert that drop is not called from interrupt context.
        // - If we are in an interrupt with lower priority, we are only allowed this in RTIC.
        //     - In this case require `&Glcdc` in interrupt. It will prevent drop.
        critical_section::with(|_| unsafe {
            let this = self.as_mut().get_unchecked_mut();
            let ctrl = this.ctrl.get().cast::<glcdc_instance_ctrl_t>();
            (*ctrl).p_callback = None;
            (*ctrl).p_context = ptr::null();
        });

        loop {
            match fsp_try_unsafe!({
                let this = self.as_mut().get_unchecked_mut();
                R_GLCDC_Close(this.ctrl.get().cast::<display_ctrl_t>())
            }) {
                // wait until display is not being updated
                Err(e_fsp_err::FSP_ERR_INVALID_UPDATE_TIMING) => continue,
                Ok(()) => return,
                Err(e) => panic!("Error closing GLCDC: {e:0X}"),
            }
        }
    }
}

unsafe extern "C" fn trampoline<F: Callback<raw::display_event_t>>(
    args: *mut raw::display_callback_args_t,
) {
    unsafe {
        let context = &*((*args).p_context as *const CallbackContext);

        context.current_owned_buffer[0].store(
            context.regs.gr1_flm2().read().bits() as *mut u8,
            Ordering::Release,
        );
        context.current_owned_buffer[1].store(
            context.regs.gr2_flm2().read().bits() as *mut u8,
            Ordering::Release,
        );

        if let Some(cb) = context.user_data.cast::<F>().as_ref() {
            let event = (*args).event;
            F::call(cb, event)
        }
    }
}

unsafe fn init_open(
    slot: *mut Glcdc<Opened>,
    glcdc: pac::GLCDC,
    mut cfg: DisplayConf<GlcdcExtendedConfig>,
) -> Result<()> {
    let display_cfg = match cfg.c_conf() {
        Ok(c) => c,
        Err(e) => {
            log::error!("GLCDC config error: {e}");
            return Err(e_fsp_err::FSP_ERR_INVALID_ARGUMENT);
        }
    };

    unsafe {
        let this = Glcdc {
            ctrl: UnsafePinned::new(zeroed()),
            inst: UnsafePinned::new(display_instance_t {
                p_ctrl: ptr::null_mut(),
                p_cfg: ptr::null(),
                p_api: ptr::from_ref(&API),
            }),
            c_ext_cfg: core::mem::MaybeUninit::uninit(),
            cfg: UnsafePinned::new(display_cfg),
            prev_owned_buffer: [ptr::null_mut(); 2],
            callback_ctx: UnsafePinned::new(CallbackContext {
                regs: glcdc,
                user_data: ptr::null(),
                current_owned_buffer: [
                    AtomicPtr::new(display_cfg.input[0].p_base as *mut u8),
                    AtomicPtr::new(display_cfg.input[1].p_base as *mut u8),
                ],
            }),
            _marker: core::marker::PhantomData,
        };
        ptr::write(slot, this);

        (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast::<core::ffi::c_void>();
        (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const();

        let p_extend = (*slot)
            .c_ext_cfg
            .write(UnsafePinned::new(cfg.extend.c_conf()))
            .get()
            .cast::<core::ffi::c_void>();

        let p_ctrl = UnsafePinned::raw_get(&raw const (*slot).ctrl);
        let p_cfg = UnsafePinned::raw_get(&raw const (*slot).cfg);

        (*p_cfg).p_extend = p_extend;
        (*p_cfg).p_callback = Some(trampoline::<()>);
        (*p_cfg).p_context = (*slot).callback_ctx.get().cast_const().cast();

        // FSP needs to IRQs to setup contexts, but it will additionally
        // unconditionally set priorities from cfg.
        // Thus we read them and give to FSP. FSP will thus not change them.
        // Critical section is for nothing to change priorities between out
        // read and FSP's write.
        critical_section::with(|_| {
            utils::try_read_priority_into(cfg.line_detect, &mut (*p_cfg).line_detect_ipl);
            utils::try_read_priority_into(cfg.underflow_1, &mut (*p_cfg).underflow_1_ipl);
            utils::try_read_priority_into(cfg.underflow_2, &mut (*p_cfg).underflow_2_ipl);

            fsp_try_unsafe!(R_GLCDC_Open(p_ctrl.cast::<display_ctrl_t>(), p_cfg))
        })
    }
}

impl GlcdcExtendedConfig {
    pub const fn c_conf(&self) -> glcdc_extended_cfg_t {
        glcdc_extended_cfg_t {
            tcon_hsync: self.tcon_hsync,
            tcon_vsync: self.tcon_vsync,
            tcon_de: self.tcon_de,
            correction_proc_order: self.correction_proc_order,
            clksrc: self.clksrc,
            clock_div_ratio: self.clock_div_ratio,
            dithering_mode: self.dithering_mode,
            dithering_pattern_A: self.dithering_pattern_A,
            dithering_pattern_B: self.dithering_pattern_B,
            dithering_pattern_C: self.dithering_pattern_C,
            dithering_pattern_D: self.dithering_pattern_D,
            phy_layer: ptr::null_mut(),
        }
    }
}

impl<S: 'static> Glcdc<S> {
    fn callback_ctx(&self) -> &CallbackContext {
        unsafe { &*self.callback_ctx.get() }
    }
    pub fn regs(&self) -> &pac::GLCDC {
        &self.callback_ctx().regs
    }
    pub fn cfg(&self) -> &display_cfg_t {
        // Safety: C code is not writing there.
        unsafe { &*self.cfg.get() }
    }

    pub fn change_buffer(
        self: Pin<&mut Self>,
        layer: display_frame_layer_t,
        mut buffer: FrameBufferMut,
    ) -> core::result::Result<(), (fsp_err_t, FrameBufferMut)> {
        let this = unsafe { self.get_unchecked_mut() };
        let ptr = buffer.as_mut_ptr();
        let ctrl = this.ctrl.get().cast::<display_ctrl_t>();

        let ctx = this.callback_ctx();
        let used_buffer = ctx.current_owned_buffer[layer as usize].load(Ordering::Relaxed);

        fsp_try_unsafe!(R_GLCDC_BufferChange(ctrl, ptr, layer)).map_err(|e| (e, buffer))?;

        this.prev_owned_buffer[layer as usize] = used_buffer;

        Ok(())
    }

    pub fn take_buffer(
        self: Pin<&mut Self>,
        layer: display_frame_layer_t,
    ) -> Option<FrameBufferMut> {
        let this = unsafe { self.get_unchecked_mut() };
        let cfg = this.cfg();
        let layer = layer as usize;

        let bpp = bpp(cfg.input[layer].format) as usize;
        let hstride = cfg.input[layer].hstride as usize * bpp / 8;
        let vsize = cfg.input[layer].vsize as usize;

        if layer != 0 && layer != 1 || layer >= cfg.input.len() as usize {
            return None;
        }

        let used_buf = {
            let callback_ctx = unsafe { &*this.callback_ctx.get() };

            callback_ctx.current_owned_buffer[layer].load(Ordering::Acquire)
        };

        let prev_buf = this.prev_owned_buffer[layer];

        if prev_buf.is_null() || prev_buf == used_buf {
            return None;
        }

        this.prev_owned_buffer[layer] = ptr::null_mut();

        // Safety:
        // - `DisplayConf::c_conf` ensures that length correct.
        // - `Glcdc::update_buffer` ensures that length is enough.
        // - Hardware is not using this buffer now.
        unsafe { Some(FrameBufferMut::from_raw_parts(prev_buf, hstride * vsize)) }
    }

    pub fn set_callback<F: Callback<raw::display_event_t>>(
        self: Pin<&mut Self>,
        callback: &'static F,
    ) -> Result<()> {
        critical_section::with(|_| unsafe {
            let this = self.get_unchecked_mut();
            let ctrl = this.ctrl.get().cast::<glcdc_instance_ctrl_t>();
            (*this.callback_ctx.get()).user_data = ptr::from_ref(callback).cast::<()>();
            (*ctrl).p_callback = Some(trampoline::<F>);
        });
        Ok(())
    }
}

pub const fn regs() -> *mut R_GLCDC_Type {
    R_GLCDC_BASE as *mut R_GLCDC_Type
}
