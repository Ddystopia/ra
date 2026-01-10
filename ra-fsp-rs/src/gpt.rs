use core::{mem::zeroed, pin::Pin, ptr};

use pin_init::{PinInit, pin_data, pin_init_from_closure};
use ra_fsp_sys::generated::{
    self as api, // -
    self as raw,
    R_GPT_Close,
    R_GPT_Open,
    R_GPT0_Type,
    e_fsp_err,
    fsp_err_t,
    gpt_extended_cfg_t,
    gpt_instance_ctrl_t,
    timer_api_t,
    timer_callback_args_t,
    timer_cfg_t,
    timer_ctrl_t,
    timer_event_t,
    timer_instance_t,
};

use crate::{
    Block, Callback, Result,
    callbacks::CallbackEvent,
    fsp_try_unsafe, pac,
    pin_init::{pin_data, pinned_drop},
    state_markers::{Closed, Opened},
    timer_api::*,
    unsafe_pinned::UnsafePinned,
    utils,
};

pub use channel::*;

// todo: maybe assert that IRQ that was passed there is having this handler?
//       like, so that this is not RTIC.
//       But there are several gpts, so there may be several handlers, and each
//       gpt has the same but different cfg??
//       Can we even mask/unmask them??
//
//       Thinking about this again, each gpt has different IRQs and there is that
//       global map IRQ -> Control Block, thus the same ISR will be bind to different
//       IRQ thus we can ask `&mut Gpt` and assert that current isr is the same as
//       in config. With `R_FSP_CurrentIrqGet` (see utils.rs) we can get it.
unsafe extern "C" {
    pub safe fn gpt_counter_overflow_isr();
    pub safe fn gpt_counter_underflow_isr();
    // pub safe fn gpt_capture_a_isr();
    // pub safe fn gpt_capture_b_isr();
    pub safe fn gpt_capture_compare_a_isr();
    pub safe fn gpt_capture_compare_b_isr();
}

// Todo: autogenerate stuff like this
pub mod channel {
    pub trait Channel: Send {
        const N: usize;
        fn gtcnt() -> u32;
    }

    macro_rules! ch {
        ($periph: ident, $n: literal) => {
            impl Channel for $crate::pac::$periph {
                const N: usize = $n;
                fn gtcnt() -> u32 {
                    let gpt = unsafe { $crate::pac::$periph::steal() };
                    gpt.gtcnt().read().bits()
                }
            }
            impl Channel for &mut $crate::pac::$periph {
                const N: usize = $n;
                fn gtcnt() -> u32 {
                    let gpt = unsafe { $crate::pac::$periph::steal() };
                    gpt.gtcnt().read().bits()
                }
            }
        };
    }

    ch!(GPT32EH0, 0);
    ch!(GPT32EH1, 1);
    ch!(GPT32EH2, 2);
    ch!(GPT32EH3, 3);
    ch!(GPT32E4, 4);
    ch!(GPT32E5, 5);
    ch!(GPT32E6, 6);
    ch!(GPT32E7, 7);
    ch!(GPT328, 8);
    ch!(GPT329, 9);
    ch!(GPT3210, 10);
    ch!(GPT3211, 11);
    ch!(GPT3212, 12);
    ch!(GPT3213, 13);
}

/* todo:

The problem is, I can't track the state using typestate because how would you
transition the state of a pinned reference? You can cast it, but if it was a
reborrow, user can simple drop it and still have a previous state. Something
like `&own T` would've been needed.

Additionally, all drivers anyway use used through the `Pin<&'static mut Gpt<'a, State, F>>`,
So this handle won't create a new one really.

I need to figure out how to implemet Block for that handle - should be easy,
just think about ctrl block and &self or Pin<&mut Self>, as well as instance.

Need some kind of Pin<Box<Gpt>> instead of Pin<&mut Gpt>

*/

#[allow(dead_code)]
pub struct GptHandle<'a, 'f, C: Channel, State: 'static>(Pin<&'a mut Gpt<'f, C, State>>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsrPrototype {
    #[default]
    Overflow,
    Underflow,
    CompareA,
    CompareB,
}

#[repr(C)] // `#[repr(C)]` is for `GptInstance::<Closed>::open`
#[pin_data(PinnedDrop)]
pub struct Gpt<'a, C, State: 'static> {
    user_data: *const (),
    cycle_end_irq: Option<pac::Interrupt>,
    capture_a_irq: Option<pac::Interrupt>,
    capture_b_irq: Option<pac::Interrupt>,
    c_ext_cfg: core::mem::MaybeUninit<UnsafePinned<gpt_extended_cfg_t>>,
    ctrl: UnsafePinned<gpt_instance_ctrl_t>,
    cfg: UnsafePinned<timer_cfg_t>,
    inst: UnsafePinned<timer_instance_t>,
    regs: C,
    _marker: core::marker::PhantomData<(State, &'a ())>,
}

#[derive(Default)]
pub struct GptExtendedConfig {
    pub gtioca: raw::gpt_output_pin_t,
    pub gtiocb: raw::gpt_output_pin_t,
    pub start_source: raw::gpt_source_t,
    pub stop_source: raw::gpt_source_t,
    pub clear_source: raw::gpt_source_t,
    pub capture_a_source: raw::gpt_source_t,
    pub capture_b_source: raw::gpt_source_t,

    pub count_up_source: raw::gpt_source_t,
    pub count_down_source: raw::gpt_source_t,
    pub capture_filter_gtioca: raw::gpt_capture_filter_t,
    pub capture_filter_gtiocb: raw::gpt_capture_filter_t,

    pub capture_a: Option<pac::Interrupt>,
    pub capture_b: Option<pac::Interrupt>,

    pub compare_match_value: [u32; 2],
    pub compare_match_status: u8,

    // `p_pwm_cfg` is not implemented
    // const * p_pwm_cfg: raw::gpt_extended_pwm_cfg_t,
    pub gtior_setting: raw::gpt_gtior_setting_t,
}

const API: timer_api_t = timer_api_t {
    open: Some(api::R_GPT_Open),
    stop: Some(api::R_GPT_Stop),
    start: Some(api::R_GPT_Start),
    reset: Some(api::R_GPT_Reset),
    enable: Some(api::R_GPT_Enable),
    disable: Some(api::R_GPT_Disable),
    periodSet: Some(api::R_GPT_PeriodSet),
    dutyCycleSet: Some(api::R_GPT_DutyCycleSet),
    compareMatchSet: Some(api::R_GPT_CompareMatchSet),
    infoGet: Some(api::R_GPT_InfoGet),
    statusGet: Some(api::R_GPT_StatusGet),
    callbackSet: Some(api::R_GPT_CallbackSet),
    close: Some(api::R_GPT_Close),
};

unsafe impl<C: Channel, S> Block for Gpt<'_, C, S> {
    type Config = timer_cfg_t;
    type Instance = timer_instance_t;
    type Api = timer_api_t;
    type State = S;

    const API: &'static Self::Api = &API;

    fn ctrl(&self) -> *mut core::ffi::c_void {
        UnsafePinned::raw_get(&raw const self.ctrl).cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

unsafe impl<C, S> Send for Gpt<'_, C, S> {}
unsafe impl<C, S> Sync for Gpt<'_, C, S> {}

impl IsrPrototype {
    pub fn call_fsp_isr_handler(self) {
        match self {
            IsrPrototype::Overflow => gpt_counter_overflow_isr(),
            IsrPrototype::Underflow => gpt_counter_underflow_isr(),
            IsrPrototype::CompareA => gpt_capture_compare_a_isr(),
            IsrPrototype::CompareB => gpt_capture_compare_b_isr(),
        }
    }
}

unsafe impl<'a, C: Channel> CallbackEvent<timer_event_t> for Gpt<'a, C, Opened> {
    fn context(this: *mut Self) -> *mut *const Self {
        unsafe {
            let ctrl = UnsafePinned::raw_get(&raw const (*this).ctrl);
            let context = &raw mut (*ctrl).p_context;
            context.cast()
        }
    }

    fn process_args(args: *mut ()) -> (*mut Self, *const (), timer_event_t) {
        unsafe {
            let args = args.cast::<timer_callback_args_t>();

            let this = (*args).p_context.cast::<Self>().cast_mut();
            let event = (*args).event;
            if this.is_null() {
                (ptr::null_mut(), ptr::null(), event)
            } else {
                (this, (*this).user_data, event)
            }
        }
    }

    fn process_static_args(args: *mut ()) -> (*const (), timer_event_t) {
        unsafe {
            let args = args.cast::<timer_callback_args_t>();
            ((*args).p_context.cast::<()>(), (*args).event)
        }
    }

    #[inline(always)]
    fn fsp_callback_set<'b>(
        self: Pin<&'b mut Self>,
        p_callback: unsafe extern "C" fn(*mut ()),
        p_context: *const core::ffi::c_void,
        user_data: *const (),
    ) -> Result<()> {
        unsafe {
            let this = self.get_unchecked_mut();
            this.user_data = user_data;
            fsp_try_unsafe!(api::R_GPT_CallbackSet(
                this.ctrl.get().cast(),
                Some(Self::cast_callback(p_callback)),
                p_context,
                core::ptr::null_mut(),
            ))
        }
    }
}

impl<'a, C: Channel> Gpt<'a, C, Opened> {
    pub fn new_open(
        gpt: C,
        cfg: TimerConf<GptExtendedConfig>,
    ) -> impl PinInit<Gpt<'a, C, Opened>, fsp_err_t> {
        unsafe {
            pin_init_from_closure(|slot: *mut Self| init_open::<C>(slot.cast::<Self>(), gpt, cfg))
        }
    }

    // FIXME: maybe allow calling this method even when `callback_set` is called?
    /// Call this method on interrupt of [`IsrPrototype`] IF you used [`Self::callback_set`]. Else it will do nothing.
    #[inline(always)]
    pub fn handle_isr(self: Pin<&mut Self>, isr_prototype: IsrPrototype) {
        CallbackEvent::with_callback_provenance(self, || isr_prototype.call_fsp_isr_handler());
    }

    // May be non-static because calling that callback requires some form of `&mut Self`
    /// For this callback to be invoked, call [`gpt_counter_overflow_isr`], [`gpt_capture_compare_a_isr`] etc in the interrup handler.
    pub fn callback_set<F>(self: Pin<&mut Self>, context: &'a F) -> Result<()>
    where
        F: Callback<timer_event_t, Self>,
    {
        CallbackEvent::callback_set(self, context)
    }

    // Must be static because Gpt might be closed dropped etc during `F`'s call.
    pub fn callback_set_static<F>(self: Pin<&mut Self>, context: &'static F) -> Result<()>
    where
        F: Callback<timer_event_t>,
    {
        CallbackEvent::callback_set_static(self, context)
    }
}

#[pin_init::pinned_drop]
impl<C, S: 'static> PinnedDrop for Gpt<'_, C, S> {
    fn drop(self: Pin<&mut Self>) {
        // SAFETY: We can can close of course, and callbacks are fine too
        // - Non-static callback requires `Pin<&mut Self>`, thus this drop can't overlap.
        // - Static callback doesn't borrow from `self`
        if self.is_open() {
            fsp_try_unsafe!(R_GPT_Close(self.ctrl_void())).expect("Error closing GPT timer");
        }
    }
}

unsafe fn init_open<C: Channel>(
    slot: *mut Gpt<'_, C, Opened>,
    gpt: C,
    mut cfg: TimerConf<GptExtendedConfig>,
) -> Result<()> {
    if cfg.channel as usize != C::N {
        log::error!("GPT: channel mismatch");
        return Err(e_fsp_err::FSP_ERR_ASSERTION);
    }

    unsafe {
        let this = Gpt {
            user_data: ptr::null(),
            cycle_end_irq: cfg.cycle_end,
            capture_a_irq: cfg.extend.capture_a,
            capture_b_irq: cfg.extend.capture_b,
            c_ext_cfg: zeroed(),
            ctrl: zeroed(),
            cfg: zeroed(),
            inst: zeroed(),
            regs: gpt,
            _marker: core::marker::PhantomData,
        };
        ptr::write(slot, this);

        (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast::<core::ffi::c_void>();
        (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const();
        (*(*slot).inst.get()).p_api = ptr::from_ref(&API);

        let p_extend = (*slot)
            .c_ext_cfg
            .write(UnsafePinned::new(cfg.extend.c_conf()))
            .get()
            .cast::<core::ffi::c_void>();

        let p_ctrl = UnsafePinned::raw_get(&raw const (*slot).ctrl);
        let p_cfg = UnsafePinned::raw_get(&raw const (*slot).cfg);

        *p_cfg = cfg.c_conf();
        (*p_cfg).p_extend = p_extend;

        for _ in 0..200 {
            core::arch::asm!("nop");
        }

        // FSP needs to IRQs to setup contexts, but it will additionally
        // unconditionally set priorities from cfg.
        // Thus we read them and give to FSP. FSP will thus not change them.
        // Critical section is for nothing to change priorities between out
        // read and FSP's write.
        critical_section::with(|_| {
            utils::try_read_priority_into(cfg.cycle_end, &mut (*p_cfg).cycle_end_ipl);

            let p_extend = (*p_cfg).p_extend.cast::<gpt_extended_cfg_t>().cast_mut();
            utils::try_read_priority_into(cfg.extend.capture_a, &mut (*p_extend).capture_a_ipl);
            utils::try_read_priority_into(cfg.extend.capture_a, &mut (*p_extend).capture_a_ipl);

            fsp_try_unsafe!(R_GPT_Open(p_ctrl.cast::<timer_ctrl_t>(), p_cfg))
        })
    }
}

impl<'f, C: Channel> Gpt<'f, C, Closed> {
    pub fn new(regs: C) -> Self {
        Self {
            user_data: ptr::null_mut(),
            regs,
            capture_a_irq: None,
            capture_b_irq: None,
            cycle_end_irq: None,
            c_ext_cfg: core::mem::MaybeUninit::zeroed(),
            ctrl: UnsafePinned::new(unsafe { zeroed() }),
            cfg: UnsafePinned::new(unsafe { zeroed() }),
            inst: UnsafePinned::new(unsafe { zeroed() }),
            _marker: core::marker::PhantomData,
        }
    }
    pub fn open(
        self: Pin<&mut Self>,
        cfg: TimerConf<GptExtendedConfig>,
    ) -> Result<Pin<&mut Gpt<'f, C, Opened>>> {
        unsafe {
            let this = ptr::from_mut(self.get_unchecked_mut());
            let regs = ptr::read(&(*this).regs);

            if (*(*this).ctrl.get()).open != 0 {
                return Err(e_fsp_err::FSP_ERR_ALREADY_OPEN);
            }

            let this = this.cast::<Gpt<C, Opened>>();
            init_open::<C>(this, regs, cfg)?;
            Ok(Pin::new_unchecked(&mut *this))
        }
    }
}

impl<C, S> Gpt<'_, C, S> {
    pub fn is_open(&self) -> bool {
        unsafe { (*self.ctrl.get()).open != 0 }
    }

    #[inline(always)]
    fn ctrl_void(self: Pin<&mut Self>) -> *mut core::ffi::c_void {
        self.ctrl().cast()
    }

    #[inline(always)]
    fn ctrl(self: Pin<&mut Self>) -> *mut gpt_instance_ctrl_t {
        UnsafePinned::raw_get(&raw const self.ctrl)
    }

    pub fn cycle_end_irq(&self) -> Option<pac::Interrupt> {
        self.cycle_end_irq
    }
    pub fn capture_a_irq(&self) -> Option<pac::Interrupt> {
        self.capture_a_irq
    }
    pub fn capture_b_irq(&self) -> Option<pac::Interrupt> {
        self.capture_b_irq
    }

    #[inline(always)]
    pub const fn c_regs(&self) -> *mut R_GPT0_Type {
        unsafe { *self.ctrl.get() }.p_reg
    }

    #[inline(always)]
    pub const fn regs_ptr(&self) -> *mut pac::gpt328::RegisterBlock {
        unsafe { *self.ctrl.get() }.p_reg.cast()
    }

    #[inline(always)]
    pub const fn regs(&self) -> &C {
        &self.regs
    }
}

impl GptExtendedConfig {
    pub const fn c_conf(&self) -> gpt_extended_cfg_t {
        gpt_extended_cfg_t {
            gtioca: self.gtioca,
            gtiocb: self.gtiocb,
            start_source: self.start_source,
            stop_source: self.stop_source,
            clear_source: self.clear_source,
            capture_a_source: self.capture_a_source,
            capture_b_source: self.capture_b_source,
            count_up_source: self.count_up_source,
            count_down_source: self.count_down_source,
            capture_filter_gtioca: self.capture_filter_gtioca,
            capture_filter_gtiocb: self.capture_filter_gtiocb,
            capture_a_ipl: ra_fsp_sys::generated::BSP_IRQ_DISABLED as u8,
            capture_b_ipl: ra_fsp_sys::generated::BSP_IRQ_DISABLED as u8,
            capture_a_irq: utils::extract_irq(self.capture_a),
            capture_b_irq: utils::extract_irq(self.capture_b),
            compare_match_value: self.compare_match_value,
            compare_match_status: self.compare_match_status,
            p_pwm_cfg: ptr::null(),
            gtior_setting: self.gtior_setting,
        }
    }
}
