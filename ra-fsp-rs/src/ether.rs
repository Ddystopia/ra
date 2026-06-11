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
    BSP_IRQ_DISABLED, R_ETHER_BufferRelease, R_ETHER_CallbackSet, R_ETHER_Close,
    R_ETHER_LinkProcess, R_ETHER_Open, R_ETHER_Read, R_ETHER_RxBufferUpdate, R_ETHER_TxStatusGet,
    R_ETHER_WakeOnLANEnable, R_ETHER_Write, e_ether_padding, e_fsp_err, ether_ctrl_t,
    ether_extended_cfg_t, ether_instance_descriptor_t, ether_phy_instance_t, fsp_err_t,
};

const _: () = assert!(
    ETHER_CFG_PARAM_CHECKING_ENABLE == 1,
    "The FSP configuration option ETHER_CFG_PARAM_CHECKING_ENABLE is required with this crate, please enable it"
);

unsafe extern "C" {
    pub unsafe fn ether_eint_isr();
}

/*

TX buffer ownership is tracked by *move*, not by a side bitset: each TX
descriptor `i` has one parking slot `tx_buffers[i]: Option<...>`. The full state
space is (slot present?) x (descriptor TACT):

| slot | TACT | state                              | owner    |
|------|------|------------------------------------|----------|
| Some |  0   | free (unused or fully transmitted) | driver   |
| None |  0   | taken, not yet returned/submitted  | user     |
| Some |  1   | submitted, in flight               | hardware |
| None |  1   | unreachable (see below)            | n/a      |

`(None, TACT=1)` is unreachable: TACT is set only by `write_zerocopy`, which in
the same step parks the submitted buffer into the slot (making it `Some`); and a
slot is emptied to `None` only by `take_tx_buf`, which first checks TACT is
clear. So a `None` slot always implies TACT clear. The code still treats it
defensively - `take_tx_buf` returns `None` for any non-available descriptor and
the park operations reject `TACT=1` - so the unreachable state is harmless.

Each operation handles every reachable state:
- `take_tx_buf`: `(Some,0)` -> move out, return the buffer; `(None,0)` -> `None`
  (already taken); `(Some,1)` -> `None` (in flight, `is_available` is false).
- `write_zerocopy` / `tx_buffer_update` (parking): `(Some,1)` is rejected (never
  displace a hardware-owned buffer); `(Some,0)` -> the parked free buffer is
  *reclaimed* and returned to the caller (this is the normal one-call pool swap,
  not an error); `(None,0)` -> park, return `None`.

Because a `Pin<&'static mut Buffer>` is a unique owning token that cannot be
duplicated in safe code, no buffer is ever handed out twice. TACT is read with a
volatile load of the descriptor (plain SRAM, not MMIO), so reclamation never
depends on TC interrupt timing; the TC interrupt is purely a wakeup signal.
Compare `rm_netxduo_ether.c`, which brings its own packet buffers and swaps them
through the descriptors the same way, re-deriving completion from TDFAR + TACT
via TxStatusGet.

This take/swap pair is the primitive a caller layers a packet pool on top of:
`take_tx_buf` reclaims a completed buffer, `write_zerocopy` submits the next and
hands back the buffer it displaced.

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
    /// One parking slot per TX descriptor. `None` means the buffer is currently
    /// held by the user; ownership is tracked by move, not a side bitset. See
    /// the ownership table at the top of this file.
    tx_buffers: &'static mut [Option<Pin<&'static mut Buffer<BUF_SIZE>>>],
    rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    regs: pac::ETHERC0,
    /// Cached at open so the per-call mode guard is a single self-relative load
    /// + compare instead of a dependent chain through the FSP cfg pointer.
    /// FSP never mutates this field; it is written once in `init_open`.
    zerocopy: bool,
    /// Extra bytes FSP may copy past the frame payload (`e_ether_padding` as
    /// `u32`). Cached at open for the same reason as `zerocopy`; used by
    /// `read_non_zerocopy` to compute the minimum destination buffer size.
    padding: u32,
    /// Base pointer of the TX descriptor ring, cached at open.  Eliminates the
    /// ctrl→p_ether_cfg→p_extend→p_tx_descriptors four-load chain in the hot
    /// TX path.  Derived from the same allocation as `p_tx_descriptor` (both
    /// come from `EtherConfig::tx_descriptors`), so `offset_from` between them
    /// is valid.  Written once in `init_open`; FSP never writes Rust-side fields
    /// so no `UnsafePinned` is needed.  The raw-pointer field is covered by the
    /// existing `unsafe impl Send/Sync` on `Ether`.
    tx_descriptors_base: *const ether_instance_descriptor_t,
    _marker: PhantomData<(S, &'a ())>,
}

/// **RAM overhead note:** `align(32)` means the trailing `rx_loaned` word
/// rounds each `Buffer` up to the next 32-byte boundary.  With a `BUF_SIZE`
/// that is already a multiple of 32 (the typical case) that is +32 bytes per
/// buffer — paid by both the TX and RX pools.  This is an accepted tradeoff for
/// the O(1) loan flag: the alternative (a side bitmap in `Ether`) would require
/// index bookkeeping on the hot RX path, which this design deliberately avoids.
#[repr(C, align(32))]
pub struct Buffer<const BUF_SIZE: usize> {
    buf: UnsafePinned<[u8; BUF_SIZE]>,
    /// Nonzero while this RX buffer is loaned to the user (between
    /// `read_zerocopy` and `rx_buffer_update`). Consulted only on the cold
    /// link-up path (`update_rx_buffers`) so the buffer isn't re-armed into the
    /// DMA ring while the user still holds it.
    ///
    /// `u32` rather than `bool` or `u8`: on Cortex-M4, byte loads are
    /// single-cycle and zero-extend for free, so a narrower type would perform
    /// identically.  The word width is kept as a conventional idiom for
    /// ISR-visible flags and costs nothing here because the alignment padding
    /// already swallows the extra bytes regardless.
    rx_loaned: UnsafePinned<u32>,
}

/// A zero-copy RX frame loan.
///
/// Created by [`Ether::read_zerocopy`]; lives until released.  The guard
/// **borrows the driver mutably** for its entire lifetime, which is the
/// soundness mechanism: borrowck statically forbids a second `read_zerocopy`,
/// `update_rx_buffers`, `close`, or any other driver call while the guard is
/// alive, without any runtime flag on the hot path.
///
/// # Frame access
///
/// `Deref<Target = [u8]>` / `DerefMut` yield `&buffer[..len]`.  The slice is
/// valid to read and write: while the guard is alive the descriptor is
/// software-owned (RACT == 0) and nothing can re-arm it.
///
/// # Releasing the frame
///
/// * **`Drop`** (implicit) — calls `R_ETHER_BufferRelease`, which re-arms the
///   same buffer into the ring and advances to the next descriptor.  Errors
///   from `BufferRelease` are silently ignored: the call can only fail when the
///   link is down or in magic-packet mode; in either case the ring is
///   re-initialized on the next link-up and every descriptor is re-armed then,
///   so the frame will not be stuck forever.
/// * **[`RxFrame::release`]** — explicit fallible variant; use when the caller
///   wants to observe the error (rare in practice).
/// * **[`RxFrame::replace_buffer`]** — swap a fresh buffer into the ring and
///   take ownership of the current buffer (the zero-copy keep-the-frame path).
///
/// # Holding the guard blocks TX
///
/// The guard holds `Pin<&mut Ether<...>>`, so **all** driver calls —
/// including TX — are blocked for the guard's lifetime.  To transmit while
/// still needing the frame data, either:
/// - copy the data out of the guard before dropping it, or
/// - call `replace_buffer` to donate a fresh buffer and reclaim the frame
///   buffer with its data intact.
///
/// # `mem::forget` semantics
///
/// Forgetting the guard is **sound**: no release fires, the descriptor stays
/// software-owned (RACT == 0) with the frame pending, and the *next*
/// `read_zerocopy` returns the same frame as a fresh guard (FSP never advances
/// the descriptor without a release, so no aliasing arises — the forgotten
/// guard's borrows are gone by then).
///
/// Cost of forgetting: the buffer's `rx_loaned` flag remains set until the
/// re-read.  If the link bounces in that window, `update_rx_buffers` skips
/// this buffer permanently (it looks loaned), costing one fewer RX ring slot
/// for the rest of the session.  In short: sound, but don't do it.
///
/// # Internal note
///
/// The guard holds the full driver borrow — a natural hook for growing TX
/// methods on the guard in the future (e.g. replying to a frame while still
/// holding it), without any extra indirection.
#[must_use = "dropping RxFrame calls BufferRelease; use release() if you need the error"]
pub struct RxFrame<'drv, 'eth, const BUF_SIZE: usize> {
    /// Mutable borrow of the driver.  This is the compile-time exclusivity
    /// guarantee: no second loan is possible while `driver` is alive.
    ///
    /// `'drv` is the borrow lifetime of the driver reference; `'eth` is
    /// `Ether`'s own `'a` lifetime parameter (covering the callback context).
    /// Keeping them separate avoids conflating the two lifetimes, which would
    /// cause variance errors at the call site.
    driver: Pin<&'drv mut Ether<'eth, BUF_SIZE, Opened>>,
    /// Raw pointer to the current RX buffer; stable for the guard's lifetime
    /// because the descriptor is software-owned (RACT == 0).
    buf: *mut Buffer<BUF_SIZE>,
    /// Frame length: `R_ETHER_Read` size clamped to `BUF_SIZE`.
    len: usize,
}

pub struct Buffers<const BUF_SIZE: usize, const TX: usize, const RX: usize> {
    tx_buffers: [Option<Pin<&'static mut Buffer<BUF_SIZE>>>; TX],
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
    pub tx_buffers: &'static mut [Option<Pin<&'static mut Buffer<BUF_SIZE>>>],
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
            rx_buffers: &mut [],
            zerocopy: false,
            padding: 0,
            tx_descriptors_base: ptr::null(),
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
        // Capture before c_conf takes ownership of cfg fields.
        let zerocopy = cfg.zerocopy;
        let padding = cfg.padding as u32;

        let this = Ether {
            regs,
            ctrl: zeroed(),
            inst: zeroed(),
            c_ext_cfg: zeroed(),
            tx_buffers: take(&mut cfg.tx_buffers),
            rx_buffers: take(&mut cfg.rx_buffers),
            user_data: ptr::null(),
            zerocopy,
            padding,
            tx_descriptors_base: ptr::null(),
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
        let p_extend = *(*(*slot).cfg.get()).p_extend.cast::<ether_extended_cfg_t>();
        (*slot).tx_descriptors_base = p_extend.p_tx_descriptors;

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
    #[inline(always)]
    fn context(this: *mut Self) -> *mut *const Self {
        unsafe {
            let ctrl = UnsafePinned::raw_get(&raw const (*this).ctrl);
            let context = &raw mut (*ctrl).p_context;
            context.cast()
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    fn process_static_args(args: *mut ()) -> (*const (), InterruptCause) {
        unsafe {
            let args = args.cast::<ether_callback_args_t>();
            let cause = InterruptCause::from_event(&*args);
            ((*args).p_context.cast::<()>(), cause)
        }
    }

    #[inline(always)]
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
        CallbackEvent::with_callback_provenance(self, || unsafe { ether_eint_isr() });
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

    /// Returns `true` if the current RX descriptor appears to be software-owned
    /// (RACT clear), meaning a received frame is likely waiting.
    ///
    /// This is one volatile SRAM read — no FFI, no MMIO. It can produce a false
    /// positive when the ring is unarmed (e.g. link-down, descriptors zeroed —
    /// RACT is clear because the hardware never set it). In that case the
    /// subsequent [`Self::read_zerocopy`] will return the real error. Use this
    /// as a cheap fast-path filter, not as an authoritative status check.
    #[inline(always)]
    pub fn rx_pending(&self) -> bool {
        unsafe {
            let p_desc = (*self.ctrl()).p_rx_descriptor;
            Descriptor::<BUF_SIZE>::is_available(p_desc)
        }
    }

    /// Borrow the current RX frame as a zero-copy guard.
    ///
    /// Returns an [`RxFrame`] that:
    /// - dereferences to `&[u8]` / `&mut [u8]` for the received frame bytes,
    /// - on `Drop` calls `R_ETHER_BufferRelease` to re-arm the same buffer and
    ///   advance the ring (discards the frame),
    /// - provides [`RxFrame::release`] (fallible explicit drop) and
    ///   [`RxFrame::replace_buffer`] (keep the frame, donate a fresh buffer).
    ///
    /// Holding the guard exclusively borrows the driver, so no other driver
    /// call (including TX) is possible until the guard is dropped.  See
    /// [`RxFrame`] for the full ownership story, TX-blocking implications, and
    /// `mem::forget` semantics.
    ///
    /// # Errors
    ///
    /// Propagates errors from `R_ETHER_Read` (e.g.
    /// [`FSP_ERR_ETHER_ERROR_NO_DATA`] when no frame is ready, or link-down
    /// errors).  Returns [`FSP_ERR_ASSERTION`] if the driver is not in
    /// zerocopy mode or if FSP returns a null or misaligned buffer pointer.
    #[inline(always)]
    pub fn read_zerocopy<'drv>(
        mut self: Pin<&'drv mut Self>,
    ) -> Result<RxFrame<'drv, 'a, BUF_SIZE>> {
        if !self.zerocopy {
            return Err(FSP_ERR_ASSERTION);
        }

        let mut p_buf: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        let mut len = 0u32;

        fsp_try_unsafe!(R_ETHER_Read(
            self.as_mut().ctrl_void(),
            ptr::from_mut(&mut p_buf).cast(),
            &mut len
        ))?;

        if !p_buf.is_aligned() || p_buf.is_null() {
            log::error!("ether(read): buffer is not aligned or null. p_buf: {p_buf:p}, len: {len}");
            return Err(FSP_ERR_ASSERTION);
        }

        let len = len as usize;
        // `len` is `RFL + padding`; RFL is per-buffer, hardware-bounded by
        // BUF_SIZE (RA6M3 RD1.RFL), so this clamp is a no-op when sized right and
        // a safety backstop otherwise. A frame larger than BUF_SIZE is split by
        // the EDMAC and surfaced a fragment at a time (not reassembled here).
        let len = len.min(BUF_SIZE);

        // Record that this RX buffer is now loaned to the user, so the link-up
        // path (`update_rx_buffers`) won't re-arm it into the DMA ring while we
        // still hold it. One store, no scan; the flag is read only on link-up.
        unsafe { ptr::write((*p_buf).rx_loaned.get(), 1) };

        // SAFETY: `p_buf` is the FSP-owned RX buffer for the current descriptor.
        // The descriptor is software-owned (RACT == 0, FSP just returned it).
        // `rx_loaned` is now 1, so `update_rx_buffers` will skip it.  The
        // returned guard borrows `self` mutably, guaranteeing exclusive access
        // until the guard is dropped.
        Ok(RxFrame {
            driver: unsafe { Pin::new_unchecked(self.get_unchecked_mut()) },
            buf: p_buf,
            len,
        })
    }

    /// Like [`Self::read_zerocopy`] but returns the raw buffer pointer and
    /// length instead of a borrow-checked guard.
    ///
    /// # Safety
    ///
    /// The caller must guarantee:
    /// 1. The loan is exclusive: no second raw read is issued before the
    ///    buffer is returned via [`Self::rx_buffer_update`].
    /// 2. The buffer is returned (via `rx_buffer_update`) before the driver
    ///    is closed or the RX ring is re-initialized.
    ///
    /// These invariants are upheld by the smoltcp token discipline:
    /// `receive(&mut self)` mutably borrows the device, smoltcp consumes the
    /// RX token before the TX token, and the token's `Drop` always calls
    /// `rx_buffer_update`.  No other caller should use this function.
    #[inline(always)]
    pub(crate) unsafe fn read_zerocopy_raw(
        mut self: Pin<&mut Self>,
    ) -> Result<(Pin<&'static mut Buffer<BUF_SIZE>>, usize)> {
        if !self.zerocopy {
            return Err(FSP_ERR_ASSERTION);
        }

        let mut p_buf: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        let mut len = 0u32;

        fsp_try_unsafe!(R_ETHER_Read(
            self.as_mut().ctrl_void(),
            ptr::from_mut(&mut p_buf).cast(),
            &mut len
        ))?;

        if !p_buf.is_aligned() || p_buf.is_null() {
            log::error!("ether(read): buffer is not aligned or null. p_buf: {p_buf:p}, len: {len}");
            return Err(FSP_ERR_ASSERTION);
        }

        let len = len as usize;
        // `len` is `RFL + padding`; RFL is per-buffer, hardware-bounded by
        // BUF_SIZE (RA6M3 RD1.RFL), so this clamp is a no-op when sized right and
        // a safety backstop otherwise. A frame larger than BUF_SIZE is split by
        // the EDMAC and surfaced a fragment at a time (not reassembled here).
        let len = len.min(BUF_SIZE);

        // Record that this RX buffer is now loaned to the user, so the link-up
        // path (`update_rx_buffers`) won't re-arm it into the DMA ring while we
        // still hold it. One store, no scan; the flag is read only on link-up.
        unsafe { ptr::write((*p_buf).rx_loaned.get(), 1) };

        Ok((unsafe { Pin::new_unchecked(&mut *p_buf) }, len))
    }
    #[inline(always)]
    pub fn read_non_zerocopy(self: Pin<&mut Self>, buffer: &mut [u8]) -> Result<usize> {
        if self.zerocopy {
            return Err(FSP_ERR_ASSERTION);
        }

        // `R_ETHER_Read` (non-zerocopy) `memcpy`s `descriptor.size + padding`
        // (`RFL + padding`) bytes into `buffer` WITHOUT knowing its length. Per the
        // RA6M3 manual (RD1.RFL, §31), RFL is the per-buffer frame length, hardware-
        // bounded by the descriptor's RBL (= BUF_SIZE), so the copy is at most
        // `BUF_SIZE + padding` bytes; the destination must hold that. Without this
        // guard a too-small `buffer` is an out-of-bounds write (UB) reachable from
        // safe code.
        let required = BUF_SIZE + self.padding as usize;
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
    /// Swaps `buffer` into the current RX descriptor and re-arms it.
    ///
    /// Not public: in the guard-based API, arming an arbitrary `'static`
    /// buffer into the current descriptor from safe code could double-arm one
    /// buffer into two descriptors (UB via DMA under a live loan from a
    /// different descriptor).  The smoltcp glue calls this through
    /// `read_zerocopy_raw`, which upholds the loan-exclusivity invariant via
    /// token discipline.  External callers should use [`RxFrame::replace_buffer`]
    /// instead.
    #[inline(always)]
    pub(crate) fn rx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Result<()> {
        let b = unsafe { buffer.get_unchecked_mut() };
        // Going back into the DMA ring: no longer loaned to the user.
        unsafe { ptr::write(b.rx_loaned.get(), 0) };
        let ptr = b.buf.get();

        fsp_try_unsafe!(R_ETHER_RxBufferUpdate(self.ctrl_void(), ptr.cast()))
    }
    /// Submits `buffer` to the current TX descriptor and parks it into that
    /// descriptor's slot.
    ///
    /// On success returns the buffer that was previously parked in the slot, now
    /// reclaimed (the completed buffer of the previous transmit on this
    /// descriptor) - this is the normal one-call pool swap - or `None` if the
    /// slot was empty (e.g. the buffer came from [`Self::take_tx_buf`]).
    ///
    /// On failure the buffer was *not* submitted; it is handed back unchanged in
    /// the `Err` together with the error code, so the caller can retry or park it
    /// with [`Self::tx_buffer_update`]. The call fails (rather than clobbering a
    /// transmission in progress) when the descriptor is still in flight.
    #[inline(always)]
    pub fn write_zerocopy(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
        len: usize,
    ) -> core::result::Result<
        Option<Pin<&'static mut Buffer<BUF_SIZE>>>,
        (Pin<&'static mut Buffer<BUF_SIZE>>, fsp_err_t),
    > {
        unsafe {
            let this = self.get_unchecked_mut();

            if !this.zerocopy {
                return Err((buffer, FSP_ERR_ASSERTION));
            }

            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // (Some, TACT=1): the EDMAC is still transmitting on this descriptor.
            // Submitting would clobber the in-flight descriptor and let us hand
            // the hardware-owned buffer back as "reclaimed" - reject instead.
            if !Descriptor::<BUF_SIZE>::is_available(p_desc) {
                return Err((buffer, e_fsp_err::FSP_ERR_ETHER_ERROR_TRANSMIT_BUFFER_FULL));
            }

            // Compute position from the pre-Write snapshot of p_tx_descriptor;
            // R_ETHER_Write advances the ring pointer so the post-Write value
            // would index the wrong slot.
            let position = this.tx_position_of(p_desc);

            let ptr = buffer.as_ref().get_ref().buf.get();
            let len = len.min(BUF_SIZE);

            match fsp_try_unsafe!(R_ETHER_Write(this.ctrl().cast(), ptr.cast(), len as u32)) {
                // Submitted: `buffer` is now in flight (tracked by TACT from here
                // on). Park it and hand back whatever free buffer the slot held -
                // the reclaimed buffer for a one-call pool swap, or `None` if the
                // slot was emptied by `take_tx_buf`.
                Ok(()) => Ok(this
                    .tx_buffers
                    .get_mut(position)
                    .and_then(|s| s.replace(buffer))),
                // Not submitted (descriptor untouched, slot unchanged): hand the
                // buffer back so nothing is lost.
                Err(e) => Err((buffer, e)),
            }
        }
    }
    #[inline(always)]
    pub fn write_non_zerocopy(self: Pin<&mut Self>, buffer: &[u8]) -> Result<()> {
        if self.zerocopy {
            return Err(FSP_ERR_ASSERTION);
        }

        let len = buffer.len().min(BUF_SIZE);
        let ptr = buffer.as_ptr().cast_mut();
        fsp_try_unsafe!(R_ETHER_Write(self.ctrl_void(), ptr.cast(), len as u32))
    }
    /// Polls the PHY for link-state changes and fires the EINT callback
    /// ([`InterruptCause::went_up`] / [`InterruptCause::went_down`]) when the
    /// state transitions.
    ///
    /// # Cost — MDIO busy-wait when `ETHER_CFG_USE_LINKSTA == 0`
    ///
    /// With the default FSP configuration (`ETHER_CFG_USE_LINKSTA == 0`),
    /// every call to `R_ETHER_LinkProcess` invokes the PHY driver's
    /// `linkStatusGet`, which performs a full MDIO management-frame read.
    /// MDIO is software-driven (bit-banged) through the ETHERC PIR register
    /// with busy-wait delays — the most expensive "register access" in the
    /// entire Ethernet driver, costing tens of microseconds of pure CPU stall
    /// per call.
    ///
    /// If the board wires the PHY's LINKSTA pin to the ETHERC and the FSP_CFG
    /// option `ETHER_CFG_USE_LINKSTA` is set to `1`, the link state is read
    /// from an ETHERC register bit instead and the call is cheap.
    ///
    /// # Recommended call rate
    ///
    /// **Do not call this from the hot network-poll loop.** Call it from a
    /// slow periodic task. At the 10 ms period currently used in ra6m3-rtic
    /// the MDIO stall costs roughly a few tenths of a percent of a 120 MHz
    /// core — tolerable, but needless if the rate is reduced. Periods of
    /// 100 ms – 1 s are plenty: link flaps are rare, and state transitions are
    /// also surfaced immediately via the EINT interrupt callback, so polling
    /// adds latency robustness only, not functionality.
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

    /// Index of the given descriptor pointer within the TX ring.  This is also
    /// the `tx_buffers` slot index because:
    ///
    /// - `offset_from` counts `ether_instance_descriptor_t` steps, which equals
    ///   the `[Descriptor<BUF_SIZE>]` index because the two have the same size:
    ///   `Descriptor` is a `#[repr(C)]` newtype around `ether_instance_descriptor_t`
    ///   plus a ZST, and on the target the inner struct is already 16 bytes so the
    ///   `align(16)` adds no padding. (This equality is also what makes the
    ///   `[Descriptor] -> [ether_instance_descriptor_t]` cast in `c_conf` sound.)
    ///   `p_desc` and `tx_descriptors_base` both derive from `EtherConfig::tx_descriptors`,
    ///   so `offset_from` is valid.
    /// - FSP keeps `p_tx_descriptor` in `0..num_tx_descriptors` (debug_assert).
    /// - `EtherConfig::c_conf` enforces `tx_buffers.len() >= num_tx_descriptors`
    ///   unconditionally at open, so `position < tx_buffers.len()` always.
    #[inline(always)]
    unsafe fn tx_position_of(&self, p_desc: *const ether_instance_descriptor_t) -> usize {
        const { assert!(size_of::<Descriptor<BUF_SIZE>>() == size_of::<ether_instance_descriptor_t>()) };
        let position = unsafe { p_desc.offset_from(self.tx_descriptors_base) };
        debug_assert!(position >= 0 && (position as usize) < self.tx_buffers.len());
        position as usize
    }

    /// Takes the buffer out of the current tx descriptor's slot. Returns `None`
    /// if that buffer is already held by the user (slot empty) or the hardware
    /// is still transmitting it (descriptor TACT set) - see the ownership table
    /// at the top of this file.
    ///
    /// Descriptor is not moved. Note that the only way to move the descriptor is to transmit the message.
    ///
    /// If you want to put it back, use [`Self::tx_buffer_update`].
    #[inline(always)]
    pub fn take_tx_buf(self: Pin<&mut Self>) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // The EDMAC owns the descriptor and its buffer until it writes
            // TACT back to zero; never hand out an in-flight buffer.
            if !Descriptor::<BUF_SIZE>::is_available(p_desc) {
                return None;
            }

            // Move the buffer out of its slot, leaving `None`. A subsequent
            // `take` returns `None` until the buffer is parked back, so a buffer
            // is never handed out twice.
            let position = this.tx_position_of(p_desc);
            this.tx_buffers.get_mut(position)?.take()
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
                let b = buffer.as_mut().get_unchecked_mut();

                // Don't re-arm a buffer the user is currently holding (loaned by
                // `read_zerocopy`); handing it back to the EDMAC would let the
                // hardware DMA into a buffer that still has a live `&mut` out.
                if *b.rx_loaned.get() != 0 {
                    continue;
                }

                let ptr = b.buf.get();
                R_ETHER_RxBufferUpdate(instance, ptr.cast());
            }
        }
    }

    /// Parks `buffer` into the current descriptor's slot *without* submitting it,
    /// e.g. to return a buffer obtained from [`Self::take_tx_buf`] that ended up
    /// not being sent.
    ///
    /// Returns the buffer previously parked in the slot, now reclaimed - this is
    /// the normal result when swapping a pool buffer in, not an error - or `None`
    /// if the slot was empty (the usual take-then-return case). If the descriptor
    /// is currently in flight (`TACT` set) the slot holds a hardware-owned buffer
    /// that must not be displaced, so the input `buffer` is handed straight back.
    #[inline(always)]
    pub fn tx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // (Some, TACT=1): never displace a hardware-owned, in-flight buffer.
            if !Descriptor::<BUF_SIZE>::is_available(p_desc) {
                return Some(buffer);
            }

            let position = this.tx_position_of(p_desc);
            this.tx_buffers
                .get_mut(position)
                .and_then(|slot| slot.replace(buffer))
        }
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
        // BUF_SIZE is a free knob (the NetX `payload_size` analog): undersizing is
        // memory-safe, it only caps the largest frame received whole. 60 is the
        // min Ethernet frame, required to form/transmit a minimum-size frame;
        // together with the multiple-of-32 rule the effective minimum is 64.
        const { assert!(BUF_SIZE >= 60) };
        // BUF_SIZE goes unrounded into each RX descriptor's RD1.RBL field
        // (`ether_buffer_size`, FSP `ether_init_descriptors`), and the RA6M3
        // manual (p. 932) requires RBL to be "an integral multiple of 32". An
        // out-of-spec RBL leaves the EDMAC's write bound undefined, and
        // `rx_loaned` sits at offset BUF_SIZE in `Buffer`, so the bound is
        // load-bearing for memory safety, not just for frame sizing.
        const {
            assert!(
                BUF_SIZE % 32 == 0,
                "BUF_SIZE must be a multiple of 32 (RD1.RBL hardware requirement)"
            )
        };

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
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers;
        self
    }
    pub const fn set_buffers<const TX: usize, const RX: usize>(&mut self, buffers: &'static mut Buffers<BUF_SIZE, TX, RX>) {
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers; 
    }

    /// This function constructs a `ether_cfg_t` from this config struct.
    /// Beware!!! `ether_cfg_t` returned has pointer with `ext`'s address and provenance.
    /// Using those pointers is unsafe thus this function is still safe.
    pub const fn c_conf(&mut self, ext: Pin<&mut MaybeUninit<UnsafePinned<ether_extended_cfg_t>>>) -> ether_cfg_t {
        // Re-asserted here (not only in `new`) because `EtherConfig` fields are
        // `pub`, so a literal-constructed config bypasses `new`. Every open
        // funnels through `c_conf`. See `new` for why this bound is load-bearing.
        const {
            assert!(
                BUF_SIZE % 32 == 0 && BUF_SIZE >= 60,
                "BUF_SIZE must be >= 60 and a multiple of 32 (RD1.RBL hardware requirement)"
            )
        };

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
            rx_loaned: UnsafePinned::new(0),
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

impl<'drv, 'eth, const BUF_SIZE: usize> RxFrame<'drv, 'eth, BUF_SIZE> {
    /// Explicitly release the frame, returning any error from
    /// `R_ETHER_BufferRelease`.
    ///
    /// Equivalent to `Drop` but surfaces the error.  The guard is consumed
    /// regardless of whether the FFI call succeeds: the descriptor has been
    /// logically returned to the ring (or, on link-down, will be re-armed on
    /// the next link-up).
    ///
    /// # Errors
    ///
    /// `R_ETHER_BufferRelease` can fail only when the link is down or the
    /// driver is in magic-packet mode; in either case the ring is
    /// re-initialized on the next link-up, so a failure here is not fatal.
    pub fn release(self) -> Result<()> {
        let mut this = ManuallyDrop::new(self);
        let result = fsp_try_unsafe!(R_ETHER_BufferRelease(this.driver.as_mut().ctrl_void()));
        // Clear rx_loaned whether or not BufferRelease succeeded: there is no
        // live `&mut` anymore, and on link-bounce re-init the buffer must be
        // eligible for re-arm.
        unsafe { ptr::write((*this.buf).rx_loaned.get(), 0) };
        result
    }

    /// Keep the current frame and swap a fresh buffer into the ring.
    ///
    /// Calls `R_ETHER_RxBufferUpdate` with `new`; on success:
    /// - `new` enters the ring (its `rx_loaned` is cleared),
    /// - the guard's buffer (containing the received frame) is returned to the
    ///   caller as `Pin<&'static mut Buffer<BUF_SIZE>>`,
    /// - the ring advances to the next descriptor.
    ///
    /// This is the zero-copy keep-the-frame path: the caller now owns the frame
    /// buffer and can process it at leisure, while the ring immediately has a
    /// fresh slot to receive the next frame.
    ///
    /// On FFI error (unreachable in practice — RACT == 0 is guaranteed while
    /// the guard is alive and all param checks pass by construction), falls back
    /// to `Drop` behavior (`BufferRelease`, error ignored) and returns
    /// `Err((new, err))` so the caller can recycle `new`.
    ///
    /// # Bookkeeping
    ///
    /// `Ether` keeps an `rx_buffers` array that `update_rx_buffers` re-arms
    /// after a link bounce.  This method keeps that array truthful by finding
    /// the entry whose pointer matches the guard's buffer and swapping in `new`.
    /// The replaced entry is exactly the `Pin<&'static mut Buffer>` returned to
    /// the caller.  Without this, swap-cycling callers would cause the array to
    /// drift from ring reality, eventually causing a link bounce to re-arm stale
    /// buffers or drop donated ones.
    pub fn replace_buffer(
        self,
        new: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> core::result::Result<
        Pin<&'static mut Buffer<BUF_SIZE>>,
        (Pin<&'static mut Buffer<BUF_SIZE>>, fsp_err_t),
    > {
        let mut this = ManuallyDrop::new(self);

        // Clear rx_loaned on `new` before it enters the ring.
        let new_raw = unsafe { new.get_unchecked_mut() as *mut Buffer<BUF_SIZE> };
        unsafe { ptr::write((*new_raw).rx_loaned.get(), 0) };

        let new_data_ptr = unsafe { (*new_raw).buf.get() };
        let result = fsp_try_unsafe!(R_ETHER_RxBufferUpdate(
            this.driver.as_mut().ctrl_void(),
            new_data_ptr.cast()
        ));

        match result {
            Ok(()) => {
                // `new` is now in the ring.  Find the guard's buffer in
                // rx_buffers and replace it with `new`.
                let old_raw = this.buf;
                let driver = unsafe { this.driver.as_mut().get_unchecked_mut() };
                let replaced = driver.rx_buffers.iter_mut().find(|b| {
                    // `b` is `&&mut Pin<&'static mut Buffer<BUF_SIZE>>`; deref
                    // through Pin to get `&Buffer` (via Deref on the Pin), then
                    // compare raw addresses.  No mutation needed here.
                    core::ptr::eq(&***b as *const Buffer<BUF_SIZE>, old_raw as *const _)
                });

                if let Some(slot) = replaced {
                    // SAFETY: `new` just left our hands into the ring (FFI
                    // succeeded), so it is now `'static`.  We constructed the
                    // `Pin<&'static mut>` from a `&'static mut` reference.
                    let new_pin = unsafe { Pin::new_unchecked(&mut *new_raw) };
                    let old_pin = core::mem::replace(slot, new_pin);
                    // Clear rx_loaned on the returned buffer: it is now the
                    // caller's; the flag must be 0 so a future re-arm (if the
                    // caller donates it back) goes through correctly.
                    unsafe { ptr::write((*old_raw).rx_loaned.get(), 0) };
                    Ok(old_pin)
                } else {
                    // Invariant violation: every buffer in a descriptor should
                    // be listed in rx_buffers.  Log and assert, but don't
                    // unsound-panic: the ring is already updated, so proceed.
                    log::error!(
                        "replace_buffer: guard buffer {old_raw:p} not found in rx_buffers; \
                         rx_buffers has drifted from ring reality"
                    );
                    debug_assert!(false, "replace_buffer: guard buffer not in rx_buffers");
                    // Return the old buffer as best-effort even without the
                    // bookkeeping update.
                    unsafe { ptr::write((*old_raw).rx_loaned.get(), 0) };
                    Ok(unsafe { Pin::new_unchecked(&mut *old_raw) })
                }
            }
            Err(e) => {
                // FFI failed (unreachable in practice): fall back to
                // BufferRelease (ignore its error) and return `new` to the
                // caller together with the original error.
                let _ = fsp_try_unsafe!(R_ETHER_BufferRelease(this.driver.as_mut().ctrl_void()));
                unsafe { ptr::write((*this.buf).rx_loaned.get(), 0) };
                Err((unsafe { Pin::new_unchecked(&mut *new_raw) }, e))
            }
        }
    }
}

impl<const BUF_SIZE: usize> Deref for RxFrame<'_, '_, BUF_SIZE> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        // SAFETY: `buf` is the descriptor-current RX buffer (RACT == 0).
        // The guard exclusively borrows the driver, so nothing else can read
        // or write this buffer concurrently.  The explicit `&(...)` suppresses
        // the implicit-autoref lint while keeping identical semantics.
        unsafe { &(&*(*self.buf).buf.get())[..self.len] }
    }
}

impl<const BUF_SIZE: usize> DerefMut for RxFrame<'_, '_, BUF_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Same as Deref above.  `DerefMut` is sound: while the guard
        // lives the descriptor is software-owned (RACT == 0) and the driver
        // borrow prevents re-arming.
        unsafe { &mut (&mut *(*self.buf).buf.get())[..self.len] }
    }
}

impl<const BUF_SIZE: usize> Drop for RxFrame<'_, '_, BUF_SIZE> {
    fn drop(&mut self) {
        // Re-arm the same buffer into the ring and advance the descriptor.
        // Errors are silently ignored: `R_ETHER_BufferRelease` can only fail
        // when the link is down or in magic-packet mode; in either case the
        // ring is re-initialized on the next link-up and every descriptor is
        // re-armed then, so no frame slot is permanently lost.
        let _ = fsp_try_unsafe!(R_ETHER_BufferRelease(self.driver.as_mut().ctrl_void()));
        // Clear the loan flag unconditionally, even if BufferRelease failed:
        // there is no live `&mut` anymore, and on link-bounce re-init the
        // buffer must be eligible for re-arm by `update_rx_buffers`.
        unsafe { ptr::write((*self.buf).rx_loaned.get(), 0) };
    }
}
