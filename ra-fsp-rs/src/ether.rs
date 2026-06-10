#![allow(non_upper_case_globals)]
use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit, replace, take, zeroed},
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr,
};

use crate::{
    Block, Callback, DriverBox, Result, TypeStateResult,
    callbacks::CallbackEvent,
    ether_phy::EtherPhy,
    fsp_try_unsafe, log,
    pac::{self, Interrupt},
    state_markers::{Closed, Opened},
    unsafe_pinned::UnsafePinned,
    utils::{self},
};

use crate::pin_init::{PinInit, pin_data, pin_init_from_closure, pinned_drop};

use ra_fsp_sys::generated as api;
pub use ra_fsp_sys::generated::{
    ETHER_CFG_PARAM_CHECKING_ENABLE,
    e_ether_event::{ETHER_EVENT_INTERRUPT, ETHER_EVENT_LINK_OFF, ETHER_EVENT_LINK_ON},
    e_ether_link_establish_status::ETHER_LINK_ESTABLISH_STATUS_UP,
    e_ether_zerocopy::{ETHER_ZEROCOPY_DISABLE, ETHER_ZEROCOPY_ENABLE},
    e_fsp_err::FSP_ERR_ASSERTION,
    e_fsp_err::FSP_ERR_ETHER_ERROR_LINK,
    e_fsp_err::FSP_ERR_ETHER_ERROR_NO_DATA,
    ether_api_t, //
    ether_callback_args_t,
    ether_cfg_t,
    ether_instance_ctrl_t,
    ether_instance_t,
    g_ether_on_ether,
};

#[derive(Debug)]
#[non_exhaustive]
pub struct InterruptCause {
    pub channel: u32,
    pub went_up: bool,
    pub went_down: bool,
    pub receive: bool,
    pub transmits: bool,
}

use ra_fsp_sys::generated::{
    BSP_IRQ_DISABLED, R_ETHER_CallbackSet, R_ETHER_Close, R_ETHER_LinkProcess, R_ETHER_Open,
    R_ETHER_Read, R_ETHER_RxBufferUpdate, R_ETHER_TxStatusGet, R_ETHER_WakeOnLANEnable,
    R_ETHER_Write, e_ether_padding, e_fsp_err, ether_ctrl_t, ether_extended_cfg_t,
    ether_instance_descriptor_t, ether_phy_instance_t, fsp_err_t,
};

const _: () = assert!(
    ETHER_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option ETHER_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

unsafe extern "C" {
    pub safe fn ether_eint_isr();
}

/*

Write:
    Non-Zerocopy:
      action:       memcpy from provided buffer to descriptors buffer
      precondition: buffer inside descriptor must have TD0_TACT is 0
      /* fallthrough */
    Zerocopy:
      action:       store the pointer into the descriptor and submit to the
                    queue. todo: return that pointer to the user when TD0_TACT
                    becomes 0 again.

      precondition: fsp code does not check TD0_TACT and simply overwrites the
                    pointer to the buffers and submits the descriptor. I don't
                    know what is it gonna do, but it would not be bad if we
                    required this TD0_TACT to be 0 too

=> For write descriptor we can see, that if `TD0_TACT == 0`, we can return
`Pin<&'static mut Buffer<BUF_SIZE>>` to the user, when he wants to take the buffer
for the purpose of writing

*/

#[repr(C)] // `#[repr(C)]` is for typestate
#[pin_data(PinnedDrop)]
pub struct Ether<'a, const BUF_SIZE: usize, S: 'static> {
    ctrl: UnsafePinned<ether_instance_ctrl_t>,
    cfg: UnsafePinned<ether_cfg_t>,
    inst: UnsafePinned<ether_instance_t>,
    user_data: *const (),
    c_ext_cfg: MaybeUninit<UnsafePinned<ether_extended_cfg_t>>,
    tx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    tx_taken: u32,
    regs: pac::ETHERC0,
    _marker: PhantomData<(S, &'a ())>,
}

#[repr(C, align(32))]
pub struct Buffer<const BUF_SIZE: usize> {
    buf: UnsafePinned<[u8; BUF_SIZE]>,
    tx_taken_position: UnsafePinned<u8>,
}

pub struct Buffers<const BUF_SIZE: usize, const TX: usize, const RX: usize> {
    tx_buffers: [Pin<&'static mut Buffer<BUF_SIZE>>; TX],
    rx_buffers: [Pin<&'static mut Buffer<BUF_SIZE>>; RX],
}

#[repr(C, align(16))]
pub struct Descriptor<const BUF_SIZE: usize>(
    ether_instance_descriptor_t,
    PhantomData<[u8; BUF_SIZE]>,
);

pub struct EtherConfig<const BUF_SIZE: usize> {
    pub channel: u8,
    pub zerocopy: bool,
    pub multicast: bool,
    pub promiscuous: bool,
    pub flow_control: bool,
    pub padding: e_ether_padding,
    pub padding_offset: u32,
    pub broadcast_filter: u32,
    pub p_mac_address: &'static [u8; 6],

    pub pp_ether_buffers: Option<&'static mut [&'static mut Buffer<BUF_SIZE>]>,
    pub irq: Option<Interrupt>,
    pub p_ether_phy_instance: &'static ether_phy_instance_t,

    pub tx_descriptors: &'static mut [Descriptor<BUF_SIZE>],
    pub rx_descriptors: &'static mut [Descriptor<BUF_SIZE>],
    pub tx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    pub rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
}

unsafe impl<const BUF_SIZE: usize, S> Sync for Ether<'_, BUF_SIZE, S> {}
unsafe impl<const BUF_SIZE: usize, S> Send for Ether<'_, BUF_SIZE, S> {}
unsafe impl<const BUF_SIZE: usize> Sync for Descriptor<BUF_SIZE> {}
unsafe impl<const BUF_SIZE: usize> Send for Descriptor<BUF_SIZE> {}

const API: ether_api_t = ether_api_t {
    open: Some(api::R_ETHER_Open),
    close: Some(api::R_ETHER_Close),
    read: Some(api::R_ETHER_Read),
    bufferRelease: Some(api::R_ETHER_BufferRelease),
    rxBufferUpdate: Some(api::R_ETHER_RxBufferUpdate),
    write: Some(api::R_ETHER_Write),
    linkProcess: Some(api::R_ETHER_LinkProcess),
    wakeOnLANEnable: Some(api::R_ETHER_WakeOnLANEnable),
    txStatusGet: Some(api::R_ETHER_TxStatusGet),
    callbackSet: Some(api::R_ETHER_CallbackSet),
};

unsafe impl<const BUF_SIZE: usize, S> crate::LifetimeDriver for Ether<'static, BUF_SIZE, S> {
    type Target<'a> = Ether<'a, BUF_SIZE, S>;
}

unsafe impl<const BUF_SIZE: usize, S> crate::Block for Ether<'_, BUF_SIZE, S> {
    type Config = ether_cfg_t;
    type Instance = ether_instance_t;
    type Api = ether_api_t;
    type State = S;

    const API: &'static ether_api_t = &API;

    fn ctrl(&self) -> *mut core::ffi::c_void {
        UnsafePinned::raw_get(&raw const self.ctrl).cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

impl<const BUF_SIZE: usize> Ether<'_, BUF_SIZE, Closed> {
    pub const fn new(ether: pac::ETHERC0) -> Self {
        Self {
            regs: ether,
            user_data: ptr::null(),
            ctrl: UnsafePinned::new(unsafe { ::core::mem::zeroed() }),
            c_ext_cfg: MaybeUninit::zeroed(),
            tx_buffers: &mut [],
            tx_taken: 0,
            rx_buffers: &mut [],
            _marker: PhantomData,
            cfg: UnsafePinned::new(unsafe { ::core::mem::zeroed() }),
            inst: UnsafePinned::new(ether_instance_t {
                p_ctrl: ptr::null_mut(),
                p_cfg: ptr::null(),
                p_api: <Self as crate::Block>::API,
            }),
        }
    }
    pub fn open<'any>(
        this: DriverBox<Self>,
        cfg: EtherConfig<BUF_SIZE>,
    ) -> TypeStateResult<Ether<'any, BUF_SIZE, Opened>, Self> {
        if this.is_open() {
            return Err((this, e_fsp_err::FSP_ERR_ALREADY_OPEN));
        }

        unsafe {
            let mut this = ManuallyDrop::new(this);

            let p_this = ptr::from_mut(this.get_unchecked_mut());
            let regs = ptr::read(&(*p_this).regs);

            let p_this = p_this.cast::<Ether<'_, BUF_SIZE, Opened>>();
            init_open(p_this, regs, cfg).map_err(|e| (ManuallyDrop::into_inner(this), e))?;
            Ok(DriverBox::new_unchecked(&mut *p_this))
        }
    }
}

unsafe fn init_open<const BUF_SIZE: usize>(
    slot: *mut Ether<'_, BUF_SIZE, Opened>,
    regs: pac::ETHERC0,
    mut cfg: EtherConfig<BUF_SIZE>,
) -> Result<()> {
    unsafe {
        let this = Ether {
            regs,
            ctrl: zeroed(),
            inst: zeroed(),
            c_ext_cfg: zeroed(),
            tx_buffers: take(&mut cfg.tx_buffers),
            rx_buffers: take(&mut cfg.rx_buffers),
            tx_taken: 0,
            user_data: ptr::null(),
            cfg: zeroed(),
            _marker: PhantomData,
        };
        ptr::write(slot, this);
        (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast::<core::ffi::c_void>();
        (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const();
        (*(*slot).inst.get()).p_api = ptr::from_ref(&API);
        (*slot).cfg = {
            let c_ext_projection = Pin::new_unchecked(&mut (*slot).c_ext_cfg);
            UnsafePinned::new(cfg.c_conf(c_ext_projection))
        };

        let p_ctrl = UnsafePinned::raw_get(&raw const (*slot).ctrl);
        let p_cfg = UnsafePinned::raw_get(&raw const (*slot).cfg);

        // FSP needs to IRQs to setup contexts, but it will additionally
        // unconditionally set priorities from cfg.
        // Thus we read them and give to FSP. FSP will thus not change them.
        // Critical section is for nothing to change priorities between out
        // read and FSP's write.
        critical_section::with(|_| {
            let mut ipl = 0;
            utils::try_read_priority_into(cfg.irq, &mut ipl);
            (*p_cfg).interrupt_priority = ipl as u32;

            fsp_try_unsafe!(R_ETHER_Open(p_ctrl.cast::<ether_ctrl_t>(), p_cfg))
        })
    }
}

#[pinned_drop]
impl<const BUF_SIZE: usize, S: 'static> PinnedDrop for Ether<'_, BUF_SIZE, S> {
    fn drop(self: Pin<&mut Self>) {
        if self.is_open() {
            fsp_try_unsafe!(R_ETHER_Close(self.ctrl_void())).expect("Error closing Ether");
        }
    }
}

// Todo: I think with frunk I may generalize even this

unsafe impl<'a, const BUF_SIZE: usize> CallbackEvent<InterruptCause>
    for Ether<'a, BUF_SIZE, Opened>
{
    fn context(this: *mut Self) -> *mut *const Self {
        unsafe {
            let ctrl = UnsafePinned::raw_get(&raw const (*this).ctrl);
            let context = &raw mut (*ctrl).p_context;
            context.cast()
        }
    }

    fn process_args(args: *mut ()) -> (*mut Self, *const (), InterruptCause) {
        unsafe {
            let args = args.cast::<ether_callback_args_t>();

            let this = (*args).p_context.cast::<Self>().cast_mut();
            let cause = InterruptCause::from_event(&*args);
            if this.is_null() {
                (ptr::null_mut(), ptr::null(), cause)
            } else {
                (this, (*this).user_data, cause)
            }
        }
    }

    fn process_static_args(args: *mut ()) -> (*const (), InterruptCause) {
        unsafe {
            let args = args.cast::<ether_callback_args_t>();
            let cause = InterruptCause::from_event(&*args);
            ((*args).p_context.cast::<()>(), cause)
        }
    }

    fn user_data(this: *mut Self) -> *const () {
        unsafe { (*this).user_data }
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
            fsp_try_unsafe!(R_ETHER_CallbackSet(
                this.ctrl.get().cast(),
                Some(Self::cast_callback(p_callback)),
                p_context,
                core::ptr::null_mut(),
            ))?;
            this.user_data = user_data;
            Ok(())
        }
    }
}

impl<'a, const BUF_SIZE: usize> Ether<'a, BUF_SIZE, Opened> {
    pub fn new_open(
        gpt: pac::ETHERC0,
        cfg: EtherConfig<BUF_SIZE>,
    ) -> impl PinInit<Self, fsp_err_t> {
        unsafe {
            pin_init_from_closure(|slot: *mut Ether<'a, BUF_SIZE, Opened>| {
                init_open(slot.cast::<Ether<'a, BUF_SIZE, Opened>>(), gpt, cfg)
            })
        }
    }

    /// Call this method on interrupt of [`IsrPrototype`].
    /// Calling it outside the configured ISR (e.g. from thread mode or the wrong IRQ) is a no-op.
    #[inline(always)]
    pub fn handle_isr(self: Pin<&mut Self>) {
        let cfg_irq = unsafe { (*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg).irq };
        let active = utils::current_irq_get().map(|i| i as u16 as i32);
        if active != Some(cfg_irq) {
            return;
        }
        CallbackEvent::with_callback_provenance(self, || ether_eint_isr());
    }

    // May be non-static because calling that callback requires some form of `&mut Self`
    /// For this callback to be invoked, call [`gpt_counter_overflow_isr`], [`gpt_capture_compare_a_isr`] etc in the interrupt handler.
    pub fn callback_set<F>(self: Pin<&mut Self>, context: &'a F) -> Result<()>
    where
        F: Callback<InterruptCause, Self> + Sync,
    {
        CallbackEvent::callback_set(self, context)
    }

    // Must be static because Gpt might be closed dropped etc during `F`'s call.
    pub fn callback_set_static<F>(self: Pin<&mut Self>, context: &'static F) -> Result<()>
    where
        F: Callback<InterruptCause> + Sync,
    {
        CallbackEvent::callback_set_static(self, context)
    }

    #[inline(always)]
    pub fn read_zerocopy(
        self: Pin<&mut Self>,
    ) -> Result<(Pin<&'static mut Buffer<BUF_SIZE>>, usize)> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_ENABLE {
            return Err(FSP_ERR_ASSERTION);
        }

        let mut p_buf: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        let mut len = 0;

        fsp_try_unsafe!(R_ETHER_Read(
            self.ctrl_void(),
            ptr::from_mut(&mut p_buf).cast(),
            &mut len
        ))?;

        if !p_buf.is_aligned() || p_buf.is_null() {
            log::error!("ether(read): buffer is not aligned or null. p_buf: {p_buf:p}, len: {len}");
            return Err(FSP_ERR_ASSERTION);
        }

        let len = len as usize;
        // FSP reports len = frame_size + padding, which can exceed BUF_SIZE when padding is
        // enabled. Clamp to avoid callers slicing past the buffer boundary.
        #[cfg(debug_assertions)]
        if len > BUF_SIZE {
            log::warn!("ether(read): reported len {len} > BUF_SIZE {BUF_SIZE}, clamping");
        }
        let len = len.min(BUF_SIZE);

        Ok((unsafe { Pin::new_unchecked(&mut *p_buf) }, len))
    }
    #[inline(always)]
    pub fn read_non_zerocopy(self: Pin<&mut Self>, buffer: &mut [u8]) -> Result<usize> {
        // SAFETY: C code is not writing to the cfg, this is a shared read.
        let cfg = unsafe { &*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg };
        if cfg.zerocopy != ETHER_ZEROCOPY_DISABLE {
            return Err(FSP_ERR_ASSERTION);
        }

        // `R_ETHER_Read` (non-zerocopy) `memcpy`s `descriptor.size + padding` bytes
        // into `buffer` WITHOUT knowing its length. A received frame is bounded by
        // `BUF_SIZE`, and FSP inserts up to `padding` extra bytes,
        // so the destination must hold at least `BUF_SIZE + padding` bytes.
        // Without this guard a too-small `buffer` is an out-of-bounds write (UB)
        // reachable from safe code.
        let required = BUF_SIZE + cfg.padding as usize;
        if buffer.len() < required {
            log::error!(
                "ether(read_non_zerocopy): buffer too small: {} < {}",
                buffer.len(),
                required
            );
            return Err(FSP_ERR_ASSERTION);
        }

        let p_buf = ptr::from_mut(buffer);
        let mut len = 0;

        fsp_try_unsafe!(R_ETHER_Read(self.ctrl_void(), p_buf.cast(), &mut len))?;

        Ok(len as usize)
    }
    #[inline(always)]
    pub fn rx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Result<()> {
        let ptr = unsafe { buffer.get_unchecked_mut().buf.get() };

        fsp_try_unsafe!(R_ETHER_RxBufferUpdate(self.ctrl_void(), ptr.cast()))
    }
    #[inline(always)]
    pub fn write_zerocopy(
        mut self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
        len: usize,
    ) -> Result<()> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_ENABLE {
            return Err(FSP_ERR_ASSERTION);
        }

        let ptr = buffer.as_ref().get_ref().buf.get();
        let len = len.min(BUF_SIZE);

        match fsp_try_unsafe!(R_ETHER_Write(
            self.as_mut().ctrl_void(),
            ptr.cast(),
            len as u32
        )) {
            Ok(()) => Ok(()),
            Err(err) => {
                self.as_mut().tx_buffer_update(buffer);
                Err(err)
            }
        }
    }
    #[inline(always)]
    pub fn write_non_zerocopy(self: Pin<&mut Self>, buffer: &[u8]) -> Result<()> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_DISABLE {
            return Err(FSP_ERR_ASSERTION);
        }

        let len = buffer.len().min(BUF_SIZE);
        let ptr = buffer.as_ptr().cast_mut();
        fsp_try_unsafe!(R_ETHER_Write(self.ctrl_void(), ptr.cast(), len as u32))
    }
    #[inline(always)]
    pub fn link_process(mut self: Pin<&mut Self>) -> Result<()> {
        let ctrl = self.as_mut().ctrl_void();
        CallbackEvent::with_callback_provenance(self, || fsp_try_unsafe!(R_ETHER_LinkProcess(ctrl)))
    }
    #[inline(always)]
    pub fn wake_on_lan_enable(self: Pin<&mut Self>) -> Result<()> {
        fsp_try_unsafe!(R_ETHER_WakeOnLANEnable(self.ctrl_void()))
    }
    #[inline(always)]
    pub fn tx_status_get(self: Pin<&mut Self>) -> Result<()> {
        let mut ptr: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        fsp_try_unsafe!(R_ETHER_TxStatusGet(
            self.ctrl_void(),
            ptr::from_mut(&mut ptr).cast()
        ))?;

        Ok(())
    }

    /// Takes the buffer out of the current tx descriptor. Returns `None` if
    /// all descriptors are currently used or if there is no descriptor.
    ///
    /// Descriptor is not moved. Note that the only way to move the descriptor is to transmit the message.
    ///
    /// If you want to put it back, use [`Self::tx_buffer_update`].
    #[inline(always)]
    pub fn take_tx_buf(self: Pin<&mut Self>) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_inst = this.ctrl.get();
            let p_desc = (*p_inst).p_tx_descriptor;

            let p_conf = (*p_inst).p_ether_cfg;
            let p_extend = (*p_conf).p_extend.cast::<ether_extended_cfg_t>();
            let p_tx_descriptors = (*p_extend).p_tx_descriptors;

            // SAFETY of the `get_unchecked_mut` below: `position` is the index of
            // the current TX descriptor, which is sound to use as a `tx_buffers`
            // index because:
            //
            // - `offset_from` counts `ether_instance_descriptor_t` steps, which equals
            //   the `[Descriptor<BUF_SIZE>]` index because the two have the same size:
            //   `Descriptor` is a `#[repr(C)]` newtype around `ether_instance_descriptor_t`
            //   plus a ZST, and on the target the inner struct is already 16 bytes so the
            //   `align(16)` adds no padding. (This equality is also what makes the
            //   `[Descriptor] -> [ether_instance_descriptor_t]` cast in `c_conf` sound.)
            //   FSP further guarantees `p_desc`/`p_tx_descriptors` share the allocation.
            // - FSP keeps `p_tx_descriptor` in `0..num_tx_descriptors` (debug_assert).
            // - `EtherConfig::c_conf` enforces `tx_buffers.len() >= num_tx_descriptors`
            //   unconditionally at open, so `position < tx_buffers.len()` always (and
            //   `position < 4`, a valid `u32` bit index).
            let position = p_desc.offset_from(p_tx_descriptors);

            const { assert!(size_of::<Descriptor<BUF_SIZE>>() == size_of::<ether_instance_descriptor_t>()) };
            debug_assert!(position >= 0 && (position as usize) < this.tx_buffers.len());

            let position = position as u8;

            if this.tx_taken & (1 << position) != 0 {
                log::error!("TX taken");
                return None;
            }

            debug_assert!(Descriptor::<BUF_SIZE>::is_available(p_desc));

            this.tx_taken |= 1 << position;

            let buffer = this
                .tx_buffers
                .get_unchecked_mut(position as usize)
                .as_mut()
                .get_unchecked_mut();

            ptr::write(buffer.tx_taken_position.get(), position);

            Some(Pin::new_unchecked(&mut *ptr::from_mut(buffer)))
        }
    }

    #[inline(always)]
    pub fn update_rx_buffers(self: Pin<&mut Self>, cause: InterruptCause) {
        if !cause.went_up {
            return;
        }

        let this = unsafe { self.get_unchecked_mut() };
        let instance = this.ctrl.get().cast();

        for buffer in &mut *this.rx_buffers {
            unsafe {
                let ptr = buffer.as_mut().get_unchecked_mut().buf.get();

                R_ETHER_RxBufferUpdate(instance, ptr.cast());
            }
        }
    }

    #[inline(always)]
    pub fn update_tx_buffers(self: Pin<&mut Self>, cause: InterruptCause) {
        if !cause.transmits {
            return;
        }

        let this = unsafe { self.get_unchecked_mut() };
        this.tx_taken = 0;
        // log::info!("Update TX buffers");
    }

    #[inline(always)]
    pub fn tx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let position = *buffer.as_ref().tx_taken_position.get();

            this.tx_taken &= !(1 << position);
        }

        None
    }

    // FIXME: Return all buffers and descriptors, and that ether phy too.
    pub fn close<'any>(
        this: DriverBox<Self>,
    ) -> TypeStateResult<Ether<'any, BUF_SIZE, Closed>, Self> {
        debug_assert!(this.is_open());

        let mut this = ManuallyDrop::new(this);

        match fsp_try_unsafe!(R_ETHER_Close(this.as_mut().ctrl_void())) {
            Err(err) => Err((ManuallyDrop::into_inner(this), err)),
            Ok(()) => unsafe {
                let ptr = ptr::from_mut(this.get_unchecked_mut()).cast();
                Ok(DriverBox::new_unchecked(&mut *ptr))
            },
        }
    }
}

impl<const BUF_SIZE: usize, S> Ether<'_, BUF_SIZE, S> {
    #[inline(always)]
    const fn ctrl_void(self: Pin<&mut Self>) -> *mut core::ffi::c_void {
        unsafe { self.get_unchecked_mut().ctrl().cast() }
    }

    pub fn is_up(&self) -> bool {
        let status = unsafe { (*self.ctrl.get()).link_establish_status };

        status == ETHER_LINK_ESTABLISH_STATUS_UP
    }

    pub fn is_open(&self) -> bool {
        unsafe { (*self.ctrl.get()).open != 0 }
    }

    pub fn get_open(&self) -> u32 {
        unsafe { (*self.ctrl.get()).open }
    }

    #[inline(always)]
    const fn ctrl(&self) -> *mut ether_instance_ctrl_t {
        UnsafePinned::raw_get(&raw const self.ctrl)
    }
}

#[rustfmt::skip]
impl<const BUF_SIZE: usize> EtherConfig<BUF_SIZE> {
    pub fn new(ether_phy_instance: DriverBox<EtherPhy<Closed>>) -> Self {
        const { assert!(BUF_SIZE <= 1514) };
        const { assert!(BUF_SIZE >= 60) };

        let p_ether_phy_instance = ether_phy_instance.leak().into_ref().get_ref().instance();

        Self {
            channel: 0,
            zerocopy: false,
            multicast: false,
            promiscuous: false,
            flow_control: false,
            padding: e_ether_padding::ETHER_PADDING_DISABLE,
            padding_offset: 0,
            broadcast_filter: 0,
            pp_ether_buffers: None,
            p_mac_address: &[0; 6],
            irq: None,
            p_ether_phy_instance,
            rx_descriptors: &mut [],
            tx_descriptors: &mut [],
            tx_buffers: &mut [],
            rx_buffers: &mut [],
        }
    }

    pub const fn channel(mut self, channel: u8) -> Self { self.channel = channel; self }
    pub const fn zerocopy(mut self) -> Self { self.zerocopy = true; self }
    pub const fn multicast(mut self) -> Self { self.multicast = true; self }
    pub const fn promiscuous(mut self) -> Self { self.promiscuous = true; self }
    pub const fn flow_control(mut self) -> Self { self.flow_control = true; self }
    pub const fn padding(mut self, padding: e_ether_padding, offset: u32) -> Self { self.padding = padding; self.padding_offset = offset; self }
    pub const fn broadcast_filter(mut self, filter: u32) -> Self { self.broadcast_filter = filter; self }
    pub const fn mac(mut self, mac: &'static [u8; 6]) -> Self { self.p_mac_address = mac; self }
    pub const fn irq(mut self, irq: Interrupt) -> Self { self.irq = Some(irq);  self }
    pub const fn ether_buffers(mut self, buffers: &'static mut [&'static mut Buffer<BUF_SIZE>]) -> Self { self.pp_ether_buffers = Some(buffers); self }
    pub const fn rx_descriptors(mut self, descriptors: &'static mut [Descriptor<BUF_SIZE>]) -> Self { self.rx_descriptors = descriptors; self }
    pub const fn tx_descriptors(mut self, descriptors: &'static mut [Descriptor<BUF_SIZE>]) -> Self { self.tx_descriptors = descriptors; self }
    pub const fn buffers<const TX: usize, const RX: usize>(mut self, buffers: &'static mut Buffers<BUF_SIZE, TX, RX>) -> Self {
        const { assert!(TX <= u32::BITS as usize, "`u32` bitset is used to account for taken TX buffers.") };
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers;
        self
    }
    pub const fn set_buffers<const TX: usize, const RX: usize>(&mut self, buffers: &'static mut Buffers<BUF_SIZE, TX, RX>) {
        const { assert!(TX <= u32::BITS as usize, "`u32` bitset is used to account for taken TX buffers.") };
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers; 
    }

    /// This function constructs a `ether_cfg_t` from this config struct.
    /// Beware!!! `ether_cfg_t` returned has pointer with `ext`'s address and provenance.
    /// Using those pointers is unsafe thus this function is still safe.
    pub const fn c_conf(&mut self, ext: Pin<&mut MaybeUninit<UnsafePinned<ether_extended_cfg_t>>>) -> ether_cfg_t {
        assert!(self.tx_descriptors.len() != 0, "Descriptors cannot be empty");
        assert!(self.rx_descriptors.len() != 0, "Descriptors cannot be empty");
        assert!(self.rx_descriptors.len() <= 4, "Max 4 descriptors");
        assert!(self.tx_descriptors.len() <= 4, "Max 4 descriptors");

        // `take_tx_buf` indexes `tx_buffers` with `get_unchecked` by the current TX
        // descriptor index, which FSP keeps in `0..num_tx_descriptors`. This bound
        // is the safety precondition for that unchecked access, so it is enforced
        // unconditionally here (once, at open) — `tx_buffers` may NOT be shorter than
        // `tx_descriptors`, even for callers that never use the zero-copy TX path.
        assert!(
            self.tx_buffers.len() >= self.tx_descriptors.len(),
            "There must be at least as many TX buffers as TX descriptors"
        );

        let num_tx_descriptors = self.tx_descriptors.len() as u8;
        let num_rx_descriptors = self.rx_descriptors.len() as u8;

        if let Some(pp_ether_buffers) = &self.pp_ether_buffers {
            if self.zerocopy  {
                assert!(pp_ether_buffers.len() as u8 == num_rx_descriptors);
            } else {
                assert!(pp_ether_buffers.len() as u8 == num_tx_descriptors + num_rx_descriptors);
            }
        };

        let p_extend = unsafe {
            ext.get_unchecked_mut().write(UnsafePinned::new(ether_extended_cfg_t {
                p_tx_descriptors: replace(&mut self.tx_descriptors, &mut []).as_mut_ptr().cast(),
                p_rx_descriptors: replace(&mut self.rx_descriptors, &mut []).as_mut_ptr().cast(),
            }))
        };

        let pp_ether_buffers = match self.pp_ether_buffers.take() {
            Some(p) => ptr::from_mut(p).cast(),
            None => ptr::null_mut(),
        };

        ether_cfg_t {
            channel: self.channel,
            zerocopy: self.zerocopy as _,
            multicast: self.multicast as _,
            promiscuous: self.promiscuous as _,
            flow_control: self.flow_control as _,
            padding: self.padding,
            padding_offset: self.padding_offset,
            broadcast_filter: self.broadcast_filter,
            p_mac_address: ptr::from_ref(self.p_mac_address).cast_mut().cast(),
            pp_ether_buffers,
            num_tx_descriptors,
            num_rx_descriptors,
            ether_buffer_size: BUF_SIZE as u32,
            irq: utils::extract_irq(self.irq),
            interrupt_priority: BSP_IRQ_DISABLED,
            p_callback: None,
            p_ether_phy_instance: self.p_ether_phy_instance,
            p_context: ptr::null(),
            p_extend: ptr::from_mut(p_extend).cast(),
        }
    }
}

impl<const BUF_SIZE: usize> Descriptor<BUF_SIZE> {
    pub const fn new() -> Self {
        Self(
            ether_instance_descriptor_t {
                status: 0,
                size: 0,
                p_buffer: ptr::null_mut(),
                buffer_size: 0,
                p_next: ptr::null_mut(),
            },
            PhantomData,
        )
    }

    #[inline]
    fn is_available(raw: *mut ether_instance_descriptor_t) -> bool {
        const ETHER_TD0_TACT: u32 = 0x80000000;
        const ETHER_RD0_TACT: u32 = 0x80000000;

        const {
            assert!(
                ETHER_RD0_TACT == ETHER_TD0_TACT,
                "The same bit is used for RD0 and TD0",
            )
        };

        unsafe {
            let status_ptr = &raw const (*raw).status;
            let status = status_ptr.read_volatile();

            // TD0 (or RD0) == 1 means that hardware is working on it.
            status & ETHER_TD0_TACT == 0
        }
    }
}

impl<const BUF_SIZE: usize> Drop for Descriptor<BUF_SIZE> {
    fn drop(&mut self) {
        unsafe extern "C" {
            static YOU_MUST_NOT_DROP_ETHER_DESCRIPTOR: [u8; 0];
        }

        let ptr = &raw const YOU_MUST_NOT_DROP_ETHER_DESCRIPTOR;

        ::core::hint::black_box(ptr);

        panic!("Descriptor cannot be dropped. Please store them in a static memory. {ptr:p}");
    }
}

impl<const BUF_SIZE: usize> Buffer<BUF_SIZE> {
    pub const fn new() -> Self {
        Self {
            buf: UnsafePinned::new([0; BUF_SIZE]),
            tx_taken_position: UnsafePinned::new(0),
        }
    }

    pub fn as_mut_bytes(self: Pin<&mut Self>) -> &mut [u8; BUF_SIZE] {
        unsafe { &mut *self.get_unchecked_mut().buf.get() }
    }
}

impl<const BUF_SIZE: usize, const TX: usize, const RX: usize> Buffers<BUF_SIZE, TX, RX> {
    // Todo: figure out a way to make this in const
    //        [&'static mut Buffer<BUF_SIZE>; TX] -> [Pin<&'static mut Buffer<BUF_SIZE>>; TX]
    pub const fn new(
        tx_buffers: [&'static mut Buffer<BUF_SIZE>; TX],
        rx_buffers: [&'static mut Buffer<BUF_SIZE>; RX],
    ) -> Self {
        Self {
            tx_buffers: unsafe { core::mem::transmute_copy(&tx_buffers) },
            rx_buffers: unsafe { core::mem::transmute_copy(&rx_buffers) },
            // rx_buffers: rx_buffers.map(Pin::static_mut),
            // rx_buffers: rx_buffers.map(Pin::static_mut),
        }
    }
}

impl InterruptCause {
    pub fn from_event(args: &ether_callback_args_t) -> Self {
        /* Transmit Complete. (all pending transmissions) */
        const ETHER_EDMAC_INTERRUPT_FACTOR_TC: u32 = 1 << 21;
        /* Frame Receive. */
        const ETHER_EDMAC_INTERRUPT_FACTOR_FR: u32 = 1 << 18;

        let mut cause = InterruptCause {
            channel: args.channel,
            receive: false,
            transmits: false,
            went_up: false,
            went_down: false,
        };

        match args.event {
            ETHER_EVENT_INTERRUPT => {
                let receive_mask = ETHER_EDMAC_INTERRUPT_FACTOR_FR;
                let transmit_mask = ETHER_EDMAC_INTERRUPT_FACTOR_TC;

                /* Packet received. */
                if receive_mask == (args.status_eesr & receive_mask) {
                    cause.receive = true;
                }

                if transmit_mask == (args.status_eesr & transmit_mask) {
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

impl<const BUF_SIZE: usize> Deref for Buffer<BUF_SIZE> {
    type Target = [u8; BUF_SIZE];
    fn deref(&self) -> &Self::Target {
        // todo: audit it
        unsafe { &*self.buf.get() }
    }
}
impl<const BUF_SIZE: usize> DerefMut for Buffer<BUF_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.buf.get() }
    }
}

impl<const BUF_SIZE: usize> Default for Buffer<BUF_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const BUF_SIZE: usize> Default for Descriptor<BUF_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
