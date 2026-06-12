#![allow(non_upper_case_globals)]
//! Ethernet driver (ETHERC/EDMAC) — safe wrapper over Renesas FSP `r_ether`.
//!
//! # Flow
//!
//! [`Ether`] is opened from an [`EtherConfig`] carrying the TX/RX descriptor
//! rings and buffer rosters. FSP is opened with `pp_ether_buffers == NULL`
//! (see [INV-NULLBUF](#invariants)), so FSP neither owns nor tracks packet
//! buffers — this wrapper owns the buffer↔descriptor binding and RX re-arming.
//!
//! - **RX (zero-copy):** [`Ether::read_zerocopy`] hands back an [`RxFrame`]
//!   guard that derefs to the frame bytes. Dropping it re-arms the same buffer
//!   (`R_ETHER_BufferRelease`); [`RxFrame::replace_buffer`] keeps the frame and
//!   donates a fresh buffer to the ring instead.
//! - **TX (zero-copy):** a caller's buffer pool layered on a take/submit/park
//!   trio: [`Ether::take_tx_buf`] reclaims a completed buffer,
//!   [`Ether::write_zerocopy`] submits the next one and returns the buffer it
//!   displaced, [`Ether::tx_buffer_update`] parks a buffer without submitting.
//! - **Link:** [`Ether::link_process`] polls the PHY; on each link-up FSP
//!   rebuilds the RX ring unarmed and [`Ether::update_rx_buffers`] re-arms it
//!   from the roster.
//!
//! # Invariants
//!
//! Inline comments reference these by tag rather than re-deriving them.
//!
//! - **INV-NULLBUF** — FSP is opened with `pp_ether_buffers == NULL`. It stores
//!   whatever buffer pointer it is handed into the current descriptor and
//!   advances, keeping no buffer list of its own. Hence the wrapper keeps the
//!   `tx_buffers`/`rx_buffers` rosters and owns re-arming the RX ring after a
//!   link bounce.
//!
//! - **INV-TXMOVE** — TX buffer ownership is tracked by *move*: one parking slot
//!   `tx_buffers[i]: Option<…>` per TX descriptor `i`, never a side bitset. A
//!   `Pin<&'static mut Buffer>` is a unique owning token, so a buffer is never
//!   handed out twice. The reachable state space is (slot present?) × descriptor
//!   `TACT`:
//!
//!   | slot | TACT | state                              | owner    |
//!   |------|------|------------------------------------|----------|
//!   | Some |  0   | free (unused or fully transmitted) | driver   |
//!   | None |  0   | taken, not yet returned/submitted  | user     |
//!   | Some |  1   | submitted, in flight               | hardware |
//!   | None |  1   | unreachable                        | n/a      |
//!
//!   `(None, TACT=1)` is unreachable: `TACT` is set only by `write_zerocopy`,
//!   which in the same step parks the buffer (→ `Some`); a slot is emptied only
//!   by `take_tx_buf`, which first checks `TACT` clear. The code still rejects it
//!   defensively (harmless). `TACT` is read with a volatile load of the
//!   descriptor (plain SRAM, not MMIO), so reclamation never depends on
//!   TC-interrupt timing — the TC interrupt is only a wakeup.
//!
//!   Note the asymmetry with RX: `R_ETHER_Read` does *not* advance
//!   `p_rx_descriptor` (which is why a `&'static mut Buffer` minted from the RX
//!   ring could be handed out twice — the reason RX uses the [`RxFrame`] guard
//!   instead). `R_ETHER_Write` *does* advance `p_tx_descriptor`, but TX safety
//!   does not rest on that. It rests on this invariant: buffer tokens are only
//!   ever *moved* between caller and parking slot, never minted from the ring,
//!   so no buffer is aliased; and the TACT check on every take/submit/park keeps
//!   a buffer the EDMAC owns (TACT=1) out of the caller's hands entirely.
//!
//! - **INV-RXLOAN** — RX-buffer aliasing safety rests entirely on the [`RxFrame`]
//!   guard holding the driver handle exclusively for its whole lifetime. Every
//!   path that re-arms a buffer (`update_rx_buffers`, `R_ETHER_BufferRelease`,
//!   `R_ETHER_RxBufferUpdate`) needs `&mut Ether`, and every live `&`/`&mut` into
//!   an RX buffer is reached either through the guard or through `&mut Ether`. So
//!   a re-arm (EDMAC may DMA into the buffer) and a live reference to that buffer
//!   are mutually exclusive by borrowck. `mem::forget(guard)` stays sound:
//!   moving the guard into `forget` requires every borrow of it — hence every
//!   `&mut [u8]` into the buffer — to be dead first (the guard itself holds only
//!   a raw `*mut Buffer`), so no live reference remains when the EDMAC takes the
//!   buffer back. **Load-bearing:** never hand out an RX-buffer handle that
//!   outlives or is decoupled from the driver borrow.
//!
//! - **INV-DESCSIZE** — `size_of::<Descriptor<BUF_SIZE>>() ==
//!   size_of::<ether_instance_descriptor_t>()`: `Descriptor` is a `#[repr(C)]`
//!   newtype around that struct plus a ZST, and the inner struct is already 16
//!   bytes so `align(16)` adds no padding. This makes the
//!   `[Descriptor] → [ether_instance_descriptor_t]` cast in `c_conf` sound and
//!   makes `offset_from` yield the `tx_buffers` slot index (see `tx_position_of`).
//!
//! - **INV-TXLEN** — `tx_buffers.len() >= num_tx_descriptors`, enforced
//!   unconditionally at open in `c_conf`. FSP keeps `p_tx_descriptor` within
//!   `0..num_tx_descriptors`, so a descriptor-derived slot index is always in
//!   bounds — the safety precondition for the unchecked path in `tx_position_of`.
//!
//! - **INV-RBL** — `BUF_SIZE` is a multiple of 32 and `>= 60`. It is written
//!   verbatim into each RX descriptor's RD1.RBL (`ether_buffer_size`); the RA6M3
//!   manual (p. 932) requires RBL to be an integral multiple of 32, otherwise the
//!   EDMAC's write bound is undefined — load-bearing for memory safety, not just
//!   frame sizing. `Buffer` is `#[repr(C, align(32))]` (the EDMAC requires the
//!   buffer pointer 32-byte aligned) and, given the multiple-of-32 size, is
//!   exactly `BUF_SIZE` bytes with no padding. `>= 60` is the minimum Ethernet
//!   frame; with the multiple-of-32 rule the effective minimum is 64.
//!
//! - **INV-RFL** — A received frame's per-buffer length is `descriptor.size` =
//!   RD1.RFL, the count of frame bytes the EDMAC actually stored, written back per
//!   packet (distinct from `descriptor.buffer_size` = RD1.RBL = `BUF_SIZE`, the
//!   capacity). RFL is hardware-bounded by RBL, so `R_ETHER_Read` never reports
//!   more than `BUF_SIZE + padding` bytes and the EDMAC never writes past one
//!   buffer. A frame larger than `BUF_SIZE` is split across descriptors (see
//!   INV-WHOLEFRAME). Read-path length clamps are no-ops when sized right, safety
//!   backstops otherwise.
//!
//!   `padding` here is the EDMAC RPADIR data-insertion feature (`e_ether_padding`,
//!   0–3 bytes), NOT the descriptor padding of INV-DESCSIZE. When enabled the
//!   EDMAC inserts `padding` zero-bytes into the received frame at offset
//!   `padding_offset` (the classic use is 2 bytes after the 14-byte Ethernet
//!   header to 4-align the IP header). FSP returns `RFL + padding` and, in
//!   non-zerocopy mode, `memcpy`s that many bytes — so the destination must hold
//!   `RFL + padding` (≤ `BUF_SIZE + padding`); the `+padding` is *added* because
//!   padding makes the copy larger, not smaller.
//!
//!   `received_size = RFL + padding` is meaningful **only for a single-buffer
//!   frame** (RD0.RFP == 11). Per the RA6M3 manual §31.3.3 (p. 934) the EDMAC
//!   writes RFL back **only on a frame-final descriptor** (RFP 11/01); for a
//!   non-final buffer of a *split* frame (RFP 10/00, "became full") it writes RFP
//!   and RACT but **not RFL**, leaving `descriptor.size` at its init value (0) or
//!   stale. So if a frame splits (`BUF_SIZE` smaller than the frame), FSP's
//!   `size + padding` for those buffers is garbage; with a large stale `size`,
//!   `received_size` can exceed `BUF_SIZE` and FSP's non-zerocopy `memcpy`
//!   **over-reads the source RX buffer** (read-only; the destination is still
//!   guarded). The single-buffer case is always exact: there `RFL + padding`
//!   equals the buffer's physical byte count (frame + inserted padding) ≤
//!   `BUF_SIZE`. Zerocopy is immune regardless (no `memcpy`; `len.min(BUF_SIZE)`
//!   clamp). FSP's padding feature, like the NetX glue (`rm_netxduo_ether.c`), is
//!   written for one buffer per frame — see INV-WHOLEFRAME.
//!
//! - **INV-WHOLEFRAME** — A small `BUF_SIZE` is fully supported and memory-safe:
//!   every buffer is RBL-bounded (INV-RBL), so a frame larger than `BUF_SIZE` just
//!   splits across descriptors. The *limitation* is functional, not a safety one:
//!   the read path surfaces one buffer at a time and does not currently expose
//!   RD0.RFP (head/middle/end) or a corrected per-fragment length, so split frames
//!   arrive as indistinguishable fragments. (Reassembly is the caller's job, not
//!   the driver's; the planned fix is to *expose* RFP + length, not to stitch.)
//!   Callers who need each frame delivered whole must
//!   therefore size `BUF_SIZE >= 1518 + padding` (max on-wire frame + any RPADIR
//!   padding, ×32) so no frame ever splits — this is the regime FSP/NetX assume.
//!   Callers who knowingly accept fragments (or only handle known-small traffic)
//!   may use any `BUF_SIZE >= 60` / ×32; just avoid the INV-RFL over-read corner
//!   (don't combine a splitting `BUF_SIZE` with non-zerocopy + `padding`).
//!
//! - **INV-REARM** — `update_rx_buffers` gates on ring state, not on an event
//!   token. After a link-up reset every descriptor has `p_buffer == NULL` and
//!   `p_rx_descriptor` is descriptor 0; in steady state every `p_buffer` is
//!   non-NULL. A NULL `p_buffer` at the ring head is thus an unforgeable "FSP
//!   just reset the ring" marker, making re-arming idempotent and bounce-proof:
//!   it happens exactly once per FSP reset, regardless of polling cadence or a
//!   replayed `went_up` event.
use core::{
    cell::RefMut,
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

#[repr(C)] // `#[repr(C)]` is for typestate
#[pin_data(PinnedDrop)]
pub struct Ether<'a, const BUF_SIZE: usize, S: 'static> {
    ctrl: UnsafePinned<ether_instance_ctrl_t>,
    cfg: UnsafePinned<ether_cfg_t>,
    inst: UnsafePinned<ether_instance_t>,
    user_data: *const (),
    c_ext_cfg: MaybeUninit<UnsafePinned<ether_extended_cfg_t>>,
    // TX parking slots, one per TX descriptor; `None` means the buffer is held
    // by the user. Kept Rust-side per INV-NULLBUF; indexed by descriptor via
    // `tx_position_of`. Ownership rules: INV-TXMOVE.
    tx_buffers: &'static mut [Option<Pin<&'static mut Buffer<BUF_SIZE>>>],
    // RX buffer roster, kept per INV-NULLBUF so `update_rx_buffers` can re-arm
    // the ring after a link-up (INV-REARM). `replace_buffer` keeps it truthful
    // when a buffer is swapped out of the ring.
    rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    regs: pac::ETHERC0,
    // Mode and padding cached at open so the per-call guards are a single
    // self-relative load instead of a chain through the FSP cfg pointer. FSP
    // never mutates these; written once in `init_open`.
    zerocopy: bool,
    // Extra bytes FSP may copy past the payload (`e_ether_padding` as `u32`);
    // used by `read_non_zerocopy` to size the destination (INV-RFL).
    padding: u32,
    // TX ring base, cached at open to avoid the
    // ctrl→p_ether_cfg→p_extend→p_tx_descriptors chain on the hot TX path.
    // Same allocation as `p_tx_descriptor`, so `offset_from` between them is
    // valid (INV-DESCSIZE). Covered by the `unsafe impl Send/Sync` below.
    tx_descriptors_base: *const ether_instance_descriptor_t,
    _marker: PhantomData<(S, &'a ())>,
}

/// A `BUF_SIZE`-byte DMA frame buffer (`align(32)`, see INV-RBL).
#[repr(C, align(32))]
pub struct Buffer<const BUF_SIZE: usize> {
    buf: UnsafePinned<[u8; BUF_SIZE]>,
}

/// Exclusive handle to an opened [`Ether`] driver.
///
/// [`RxFrame`] is generic over this so the guard can either *borrow* the
/// driver (`Pin<&mut Ether>`, the common case via [`Ether::read_zerocopy`])
/// or *own* an enclosing borrow guard (`RefMut<DriverBox<Ether>>`, used by
/// the smoltcp glue whose tokens cannot hold a reference into the `RefCell`
/// they travel with).  Implement it for your own wrapper (e.g. a mutex
/// guard) to use [`RxFrame::read`] with custom driver storage.
pub trait EtherMut<'eth, const BUF_SIZE: usize> {
    /// Reborrows the handle as the pinned driver.
    fn ether_mut(&mut self) -> Pin<&mut Ether<'eth, BUF_SIZE, Opened>>;
}

/// A zero-copy RX frame loan.
///
/// Created by [`Ether::read_zerocopy`] (plain borrow) or [`RxFrame::read`]
/// (any [`EtherMut`] handle); lives until released. The guard holds the driver
/// handle exclusively for its whole lifetime — the soundness mechanism behind
/// INV-RXLOAN — so no other driver call is possible while it is alive.
///
/// # Frame access
///
/// `Deref<Target = [u8]>` / `DerefMut` yield `&buffer[..len]`. The slice is
/// valid to read and write: while the guard is alive the descriptor is
/// software-owned (RACT == 0) and nothing can re-arm it.
///
/// # Releasing the frame
///
/// * **`Drop`** (implicit) — calls `R_ETHER_BufferRelease`, which re-arms the
///   same buffer into the ring and advances to the next descriptor. Errors are
///   ignored: `BufferRelease` can only fail with the link down or in
///   magic-packet mode, and the ring is fully re-armed on the next link-up, so
///   the frame slot is never permanently lost.
/// * **[`RxFrame::release`]** — explicit fallible variant; use when the caller
///   wants to observe the error (rare in practice).
/// * **[`RxFrame::replace_buffer`]** — swap a fresh buffer into the ring and
///   take ownership of the current buffer (the zero-copy keep-the-frame path).
///
/// # Holding the guard blocks TX
///
/// The guard holds the driver exclusively, so **all** driver calls — including
/// TX — are blocked for its lifetime. To transmit while still needing the frame
/// data, either copy it out before dropping the guard, or call
/// [`RxFrame::replace_buffer`] to donate a fresh buffer and keep this one.
///
/// # `mem::forget` semantics
///
/// Forgetting the guard is sound (see INV-RXLOAN): no release fires, the
/// descriptor stays software-owned with the frame pending, and the next
/// `read_zerocopy` returns the same frame. The cost is that the frame is never
/// released until a re-read or a link bounce. Forgetting a guard with an
/// *owning* handle (e.g. the smoltcp token's `RefMut`) leaks that borrow, so
/// every later `RefCell` borrow panics — sound, but don't do it.
#[must_use = "dropping RxFrame calls BufferRelease; use release() if you need the error"]
pub struct RxFrame<'eth, D, const BUF_SIZE: usize>
where
    D: EtherMut<'eth, BUF_SIZE>,
{
    // Exclusive driver handle — the carrier of INV-RXLOAN.
    driver: D,
    // Current RX buffer; stable for the guard's lifetime (descriptor stays
    // software-owned, RACT == 0).
    buf: *mut Buffer<BUF_SIZE>,
    // Frame length: `R_ETHER_Read` size clamped to `BUF_SIZE` (INV-RFL).
    len: usize,
    // `'eth` (the driver's callback-context lifetime) is used only through
    // `D`'s trait bound, which doesn't count as a use for the compiler.
    _eth: PhantomData<&'eth ()>,
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
    /// `mem::forget` semantics.  To construct the guard from an owning handle
    /// (e.g. a `RefMut` over the driver) use [`RxFrame::read`] directly.
    ///
    /// # Errors
    ///
    /// Propagates errors from `R_ETHER_Read` (e.g.
    /// [`FSP_ERR_ETHER_ERROR_NO_DATA`] when no frame is ready, or link-down
    /// errors).  Returns [`FSP_ERR_ASSERTION`] if the driver is not in
    /// zerocopy mode or if FSP returns a null or misaligned buffer pointer.
    #[inline(always)]
    pub fn read_zerocopy<'drv>(
        self: Pin<&'drv mut Self>,
    ) -> Result<RxFrame<'a, Pin<&'drv mut Self>, BUF_SIZE>> {
        RxFrame::read(self).map_err(|(_, err)| err)
    }
    #[inline(always)]
    pub fn read_non_zerocopy(self: Pin<&mut Self>, buffer: &mut [u8]) -> Result<usize> {
        if self.zerocopy {
            return Err(FSP_ERR_ASSERTION);
        }

        // `R_ETHER_Read` (non-zerocopy) `memcpy`s `RFL + padding` bytes into
        // `buffer` without knowing its length. By INV-RFL that is at most
        // `BUF_SIZE + padding`; requiring the destination to hold that turns an
        // otherwise safe-reachable out-of-bounds write into an error. (The
        // matching source over-read in the undersized-split + padding corner is
        // FSP-internal and unpreventable here — honor INV-WHOLEFRAME to avoid it.)
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

            // TACT=1: descriptor in flight; submitting would clobber it and let
            // us hand the hardware-owned buffer back as "reclaimed" (INV-TXMOVE).
            if !Descriptor::<BUF_SIZE>::is_available(p_desc) {
                return Err((buffer, e_fsp_err::FSP_ERR_ETHER_ERROR_TRANSMIT_BUFFER_FULL));
            }

            // Snapshot the slot index before the Write: `R_ETHER_Write` advances
            // the ring pointer, so the post-Write value would index the wrong slot.
            let position = this.tx_position_of(p_desc);

            let ptr = buffer.as_ref().get_ref().buf.get();
            let len = len.min(BUF_SIZE);

            match fsp_try_unsafe!(R_ETHER_Write(this.ctrl().cast(), ptr.cast(), len as u32)) {
                // Submitted: `buffer` is now in flight (TACT tracks it). Park it
                // and hand back whatever the slot held — the reclaimed buffer for
                // a one-call pool swap, or `None` if `take_tx_buf` emptied it.
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

    /// Index of `p_desc` within the TX ring, which is also its `tx_buffers` slot
    /// index. `offset_from` is valid and yields the slot index by INV-DESCSIZE
    /// (`p_desc` and `tx_descriptors_base` share the `tx_descriptors` allocation);
    /// the result is in bounds by INV-TXLEN. The `debug_assert` re-checks both.
    #[inline(always)]
    unsafe fn tx_position_of(&self, p_desc: *const ether_instance_descriptor_t) -> usize {
        const { assert!(size_of::<Descriptor<BUF_SIZE>>() == size_of::<ether_instance_descriptor_t>()) };
        let position = unsafe { p_desc.offset_from(self.tx_descriptors_base) };
        debug_assert!(position >= 0 && (position as usize) < self.tx_buffers.len());
        position as usize
    }

    /// Takes the buffer out of the current TX descriptor's slot. Returns `None`
    /// per INV-TXMOVE if the buffer is already held by the user (slot empty) or
    /// the hardware is still transmitting it (TACT set).
    ///
    /// The descriptor is not advanced — only a transmit advances it. Put the
    /// buffer back with [`Self::tx_buffer_update`].
    #[inline(always)]
    pub fn take_tx_buf(self: Pin<&mut Self>) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // TACT=1: in flight, EDMAC-owned; never hand it out (INV-TXMOVE).
            if !Descriptor::<BUF_SIZE>::is_available(p_desc) {
                return None;
            }

            // Move out, leaving `None`; the slot then stays empty until parked
            // back, so a buffer is never handed out twice (INV-TXMOVE).
            let position = this.tx_position_of(p_desc);
            this.tx_buffers.get_mut(position)?.take()
        }
    }

    /// Re-arms the RX descriptor ring after a link-up, handing every buffer
    /// back to the EDMAC.
    //
    // The NULL-`p_buffer`-at-head marker that INV-REARM relies on is committed
    // by FSP: on a link-up transition `R_ETHER_LinkProcess` rebuilds the ring
    // through `ether_configure_mac` -> `ether_init_descriptors`
    // (`r_ether.c:1585`); with `pp_ether_buffers == NULL` (INV-NULLBUF) every
    // descriptor is left `p_buffer == NULL` / `RACT` clear and `p_rx_descriptor`
    // reset to descriptor 0. A link-*down* leaves the ring untouched, and in
    // steady state every `p_buffer` is non-NULL (`R_ETHER_RxBufferUpdate` only
    // swaps in non-NULL, `read_zerocopy` never clears it).
    #[inline(always)]
    pub fn update_rx_buffers(self: Pin<&mut Self>) {
        // Gate per INV-REARM: act only once FSP reports the link up *and* the
        // ring is still post-reset. `is_up()` rejects the do_link-failed window
        // (ring memset but link not yet up); the head-descriptor check below
        // rejects steady state (every descriptor already armed).
        if !self.is_up() {
            return;
        }

        let this = unsafe { self.get_unchecked_mut() };
        let ctrl = this.ctrl.get();

        // `p_rx_descriptor` is descriptor 0 after the reset and is the first
        // descriptor `R_ETHER_RxBufferUpdate` arms, so its `p_buffer` is the
        // NULL reset marker (INV-REARM).
        let head = unsafe { (*ctrl).p_rx_descriptor };
        if head.is_null() || !unsafe { (*head).p_buffer.is_null() } {
            return;
        }

        let instance = ctrl.cast();

        // Re-arm every roster buffer unconditionally. This hands buffers to the
        // EDMAC (RACT=1, it may DMA into them); soundness is INV-RXLOAN — the
        // gate above guarantees no live `RxFrame` guard (hence no live reference
        // into any buffer) can coexist with this loop. Each roster entry is a
        // unique `&'static mut` armed once into successive descriptors, so no
        // buffer is armed into two descriptors.
        for buffer in &mut *this.rx_buffers {
            unsafe {
                let b = buffer.as_mut().get_unchecked_mut();
                let ptr = b.buf.get();
                R_ETHER_RxBufferUpdate(instance, ptr.cast());
            }
        }
    }

    /// Parks `buffer` into the current descriptor's slot *without* submitting it,
    /// e.g. to return a buffer obtained from [`Self::take_tx_buf`] that ended up
    /// not being sent.
    ///
    /// Returns the buffer previously parked in the slot, now reclaimed (the
    /// normal result when swapping a pool buffer in, not an error), or `None` if
    /// the slot was empty (the usual take-then-return case). If the descriptor is
    /// in flight (`TACT` set) the input `buffer` is handed straight back, since
    /// the slot holds a hardware-owned buffer that must not be displaced.
    #[inline(always)]
    pub fn tx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // TACT=1: never displace a hardware-owned, in-flight buffer (INV-TXMOVE).
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
        // INV-RBL. Undersizing BUF_SIZE is memory-safe (it only caps the largest
        // frame received whole, the NetX `payload_size` analog); the bounds below
        // are the load-bearing part.
        const { assert!(BUF_SIZE >= 60) };
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
    /// Enable EDMAC RPADIR padding insertion (see INV-RFL): `padding` zero-bytes
    /// inserted into each received frame at `offset`. Requires single-buffer
    /// reception — size `BUF_SIZE >= 1518 + padding` (INV-WHOLEFRAME), else the
    /// frame splits and the non-zerocopy read path hits the continuation
    /// over-read described in INV-RFL.
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
        // INV-RBL, re-asserted here because `EtherConfig` fields are `pub` so a
        // literal-constructed config bypasses `new`; every open funnels through
        // `c_conf`.
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

        // INV-TXLEN, enforced unconditionally here (once, at open) — `tx_buffers`
        // may NOT be shorter than `tx_descriptors`, even for callers that never
        // use the zero-copy TX path.
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
        // SAFETY: the `&self` borrow already excludes concurrent mutation of the
        // bytes; this only re-derives a shared ref through `UnsafePinned`.
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

impl<'eth, D, const BUF_SIZE: usize> RxFrame<'eth, D, BUF_SIZE>
where
    D: EtherMut<'eth, BUF_SIZE>,
{
    /// Reads the pending frame, taking `driver` as the guard's exclusive
    /// handle.  [`Ether::read_zerocopy`] is the convenience form for a plain
    /// `Pin<&mut Ether>` borrow; use this directly with an owning handle
    /// (e.g. `RefMut<DriverBox<Ether>>`) when the guard must not borrow its
    /// surroundings.
    ///
    /// # Errors
    ///
    /// On error the handle is handed back together with the error code so
    /// the caller can keep using the driver.  Propagates errors from
    /// `R_ETHER_Read` (e.g. [`FSP_ERR_ETHER_ERROR_NO_DATA`] when no frame is
    /// ready, or link-down errors); [`FSP_ERR_ASSERTION`] if the driver is
    /// not in zerocopy mode or FSP returns a null or misaligned pointer.
    #[inline(always)]
    pub fn read(mut driver: D) -> core::result::Result<Self, (D, fsp_err_t)> {
        if !driver.ether_mut().zerocopy {
            return Err((driver, FSP_ERR_ASSERTION));
        }

        let mut p_buf: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        let mut len = 0u32;

        if let Err(err) = fsp_try_unsafe!(R_ETHER_Read(
            driver.ether_mut().ctrl_void(),
            ptr::from_mut(&mut p_buf).cast(),
            &mut len
        )) {
            return Err((driver, err));
        }

        if !p_buf.is_aligned() || p_buf.is_null() {
            log::error!("ether(read): buffer is not aligned or null. p_buf: {p_buf:p}, len: {len}");
            return Err((driver, FSP_ERR_ASSERTION));
        }

        let len = len as usize;
        // `len` is `RFL + padding`; clamp per INV-RFL (no-op when sized right,
        // safety backstop otherwise).
        let len = len.min(BUF_SIZE);

        // `p_buf` is the current descriptor's RX buffer, software-owned (RACT == 0,
        // FSP just returned it). It stays that way because `driver` — moved into
        // the guard below — is now the only route to the driver (INV-RXLOAN).
        Ok(RxFrame {
            driver,
            buf: p_buf,
            len,
            _eth: PhantomData,
        })
    }

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
        fsp_try_unsafe!(R_ETHER_BufferRelease(this.driver.ether_mut().ctrl_void()))
    }

    /// Keep the current frame and swap a fresh buffer into the ring.
    ///
    /// Calls `R_ETHER_RxBufferUpdate` with `new`; on success:
    /// - `new` enters the ring,
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

        let new_raw = unsafe { new.get_unchecked_mut() as *mut Buffer<BUF_SIZE> };
        let new_data_ptr = unsafe { (*new_raw).buf.get() };
        let result = fsp_try_unsafe!(R_ETHER_RxBufferUpdate(
            this.driver.ether_mut().ctrl_void(),
            new_data_ptr.cast()
        ));

        match result {
            Ok(()) => {
                // `new` is now in the ring.  Find the guard's buffer in
                // rx_buffers and replace it with `new`.
                let old_raw = this.buf;
                let driver = unsafe { this.driver.ether_mut().get_unchecked_mut() };
                let replaced = driver.rx_buffers.iter_mut().find(|b| {
                    // `b` is `&mut Pin<&'static mut Buffer<BUF_SIZE>>`; deref
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
                    Ok(unsafe { Pin::new_unchecked(&mut *old_raw) })
                }
            }
            Err(e) => {
                // FFI failed (unreachable in practice): fall back to
                // BufferRelease (ignore its error) and return `new` to the
                // caller together with the original error.
                let _ = fsp_try_unsafe!(R_ETHER_BufferRelease(this.driver.ether_mut().ctrl_void()));
                Err((unsafe { Pin::new_unchecked(&mut *new_raw) }, e))
            }
        }
    }
}

impl<'eth, D: EtherMut<'eth, BUF_SIZE>, const BUF_SIZE: usize> Deref for RxFrame<'eth, D, BUF_SIZE> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        // SAFETY: `buf` is the descriptor-current RX buffer (RACT == 0).
        // The guard exclusively borrows the driver, so nothing else can read
        // or write this buffer concurrently.  The explicit `&(...)` suppresses
        // the implicit-autoref lint while keeping identical semantics.
        unsafe { &(&*(*self.buf).buf.get())[..self.len] }
    }
}

impl<'eth, D: EtherMut<'eth, BUF_SIZE>, const BUF_SIZE: usize> DerefMut
    for RxFrame<'eth, D, BUF_SIZE>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Same as Deref above.  `DerefMut` is sound: while the guard
        // lives the descriptor is software-owned (RACT == 0) and the driver
        // borrow prevents re-arming.
        unsafe { &mut (&mut *(*self.buf).buf.get())[..self.len] }
    }
}

impl<'eth, D: EtherMut<'eth, BUF_SIZE>, const BUF_SIZE: usize> Drop for RxFrame<'eth, D, BUF_SIZE> {
    fn drop(&mut self) {
        // Re-arm the same buffer into the ring and advance the descriptor.
        // Errors are silently ignored: `R_ETHER_BufferRelease` can only fail
        // when the link is down or in magic-packet mode; in either case the
        // ring is re-initialized on the next link-up and every descriptor is
        // re-armed then, so no frame slot is permanently lost.
        let _ = fsp_try_unsafe!(R_ETHER_BufferRelease(self.driver.ether_mut().ctrl_void()));
    }
}

impl<'eth, const BUF_SIZE: usize> EtherMut<'eth, BUF_SIZE>
    for Pin<&mut Ether<'eth, BUF_SIZE, Opened>>
{
    #[inline(always)]
    fn ether_mut(&mut self) -> Pin<&mut Ether<'eth, BUF_SIZE, Opened>> {
        self.as_mut()
    }
}

impl<'eth, const BUF_SIZE: usize> EtherMut<'eth, BUF_SIZE>
    for RefMut<'_, DriverBox<Ether<'eth, BUF_SIZE, Opened>>>
{
    #[inline(always)]
    fn ether_mut(&mut self) -> Pin<&mut Ether<'eth, BUF_SIZE, Opened>> {
        (**self).as_mut()
    }
}
