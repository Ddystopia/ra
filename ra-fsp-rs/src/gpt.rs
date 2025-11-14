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
    Block, Callback, Result, fsp_try_unsafe, pac,
    state_markers::{Closed, Opened},
    timer_api::*,
    unsafe_pinned::UnsafePinned,
    utils,
};

// todo: maybe assert that IRQ that was passed there is having this handler?
//       like, so that this is not RTIC.
//       But there are several gpts, so there may be several handlers, and each
//       gpt has the same but different cfg??
//       Can we even mask/unmask them??
unsafe extern "C" {
    pub safe fn gpt_counter_overflow_isr();
    pub safe fn gpt_counter_underflow_isr();
    pub safe fn gpt_capture_a_isr();
    pub safe fn gpt_capture_b_isr();
    pub safe fn gpt_capture_compare_a_isr();
    pub safe fn gpt_capture_compare_b_isr();
}

pub enum GptRegister {
    GPT32EH0(pac::GPT32EH0),
    GPT32EH1(pac::GPT32EH1),
    GPT32EH2(pac::GPT32EH2),
    GPT32EH3(pac::GPT32EH3),

    GPT32E4(pac::GPT32E4),
    GPT32E5(pac::GPT32E5),
    GPT32E6(pac::GPT32E6),
    GPT32E7(pac::GPT32E7),

    GPT328(pac::GPT328),
    GPT329(pac::GPT329),

    GPT3210(pac::GPT3210),
    GPT3211(pac::GPT3211),
    GPT3212(pac::GPT3212),
    GPT3213(pac::GPT3213),
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
pub struct GptHandle<'a, 'f, State: 'static>(Pin<&'a mut Gpt<'f, State>>);

#[repr(C)] // `#[repr(C)]` is for `GptInstance::<Closed>::open`
#[pin_data(PinnedDrop)]
pub struct Gpt<'a, State: 'static> {
    user_data: *const (),
    cycle_end_irq: Option<pac::Interrupt>,
    capture_a_irq: Option<pac::Interrupt>,
    capture_b_irq: Option<pac::Interrupt>,
    c_ext_cfg: core::mem::MaybeUninit<UnsafePinned<gpt_extended_cfg_t>>,
    ctrl: UnsafePinned<gpt_instance_ctrl_t>,
    cfg: UnsafePinned<timer_cfg_t>,
    inst: UnsafePinned<timer_instance_t>,
    regs: GptRegister,
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

unsafe impl<S> Block for Gpt<'_, S> {
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

unsafe impl<S> Send for Gpt<'_, S> {}
unsafe impl<S> Sync for Gpt<'_, S> {}

impl<'a> Gpt<'a, Opened> {
    pub fn new_open(
        gpt: GptRegister,
        cfg: TimerConf<GptExtendedConfig>,
    ) -> impl PinInit<Gpt<'a, Opened>, fsp_err_t> {
        unsafe {
            pin_init_from_closure(|slot: *mut Gpt<'a, Opened>| {
                init_open(slot.cast::<Gpt<'a, Opened>>(), gpt, cfg)
            })
        }
    }

    // It may be generalized to the timer lever, but I don't see the clear
    // reason to overcomplicate for now. In short, `F` would move to the trait.
    pub fn callback_set<F>(self: Pin<&mut Self>, context: &'a F) -> Result<()>
    where
        F: Callback<timer_event_t>,
    {
        unsafe extern "C" fn trampoline<F: Callback<timer_event_t>>(
            args: *mut timer_callback_args_t,
        ) {
            unsafe {
                let this = (*args).p_context.cast::<Gpt<Opened>>();
                let context = (*this).user_data.cast::<F>();
                let event = (*args).event;

                debug_assert!(context != ptr::null());
                F::call(&*context, event);
            }
        }

        unsafe {
            let this = self.get_unchecked_mut();
            let ctrl = this.ctrl.get();

            this.user_data = ptr::from_ref(context).cast();

            fsp_try_unsafe!(api::R_GPT_CallbackSet(
                ctrl.cast(),
                Some(trampoline::<F>),
                ptr::from_ref(this).cast::<core::ffi::c_void>(),
                core::ptr::null_mut()
            ))
        }
    }
}

#[pin_init::pinned_drop]
impl<S: 'static> PinnedDrop for Gpt<'_, S> {
    fn drop(self: Pin<&mut Self>) {
        // todo: ensure we did not preempt `trampoline` that was about to use
        //       `p_context` for stuff.
        if self.is_open() {
            fsp_try_unsafe!(R_GPT_Close(self.ctrl_void())).expect("Error closing GPT timer");
        }
    }
}

unsafe fn init_open(
    slot: *mut Gpt<'_, Opened>,
    gpt: GptRegister,
    mut cfg: TimerConf<GptExtendedConfig>,
) -> Result<()> {
    if cfg.channel != gpt.channel() {
        log::error!("GPT: channel mismatch");
        return Err(e_fsp_err::FSP_ERR_ASSERTION);
    }

    unsafe {
        let this = Gpt {
            user_data: ptr::null(),
            cycle_end_irq: Some(cfg.cycle_end),
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

impl<'f> Gpt<'f, Closed> {
    pub fn new(regs: GptRegister) -> Self {
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
    ) -> Result<Pin<&mut Gpt<'f, Opened>>> {
        unsafe {
            let this = ptr::from_mut(self.get_unchecked_mut());
            let regs = ptr::read(&(*this).regs);

            if (*(*this).ctrl.get()).open != 0 {
                return Err(e_fsp_err::FSP_ERR_ALREADY_OPEN);
            }

            let this = this.cast::<Gpt<Opened>>();
            init_open(this, regs, cfg)?;
            Ok(Pin::new_unchecked(&mut *this))
        }
    }
}

impl<S> Gpt<'_, S> {
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
    pub const fn regs_full(&self) -> &GptRegister {
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

impl GptRegister {
    pub const fn channel(&self) -> u8 {
        match self {
            GptRegister::GPT32EH0(_) => 0,
            GptRegister::GPT32EH1(_) => 1,
            GptRegister::GPT32EH2(_) => 2,
            GptRegister::GPT32EH3(_) => 3,
            GptRegister::GPT32E4(_) => 4,
            GptRegister::GPT32E5(_) => 5,
            GptRegister::GPT32E6(_) => 6,
            GptRegister::GPT32E7(_) => 7,
            GptRegister::GPT328(_) => 8,
            GptRegister::GPT329(_) => 9,
            GptRegister::GPT3210(_) => 10,
            GptRegister::GPT3211(_) => 11,
            GptRegister::GPT3212(_) => 12,
            GptRegister::GPT3213(_) => 13,
        }
    }
}
