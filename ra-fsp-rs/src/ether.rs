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
//! # Dynamic per-buffer size
//!
//! Buffers are *size-erased*: each [`Buffer`] in a ring carries its own capacity
//! as slice metadata, so a single `Ether` can host a mixed pool — e.g. small
//! buffers for control/syslog traffic and large ones for bulk data — instead of
//! one compile-time size shared by every buffer. Sized storage is allocated as
//! `Buffer<[u8; CAP]>` and erased to `Buffer` (= `Buffer<[u8]>`) via
//! [`Buffer::erase`]. Each descriptor's RD1.RBL is (re-)programmed from its
//! buffer's capacity on every re-arm (INV-DYNRBL).
//!
//! # Split frames
//!
//! The driver is a thin, auditable shell over the hardware: it does **not**
//! reassemble multi-buffer frames, own a packet model, or know about IP/MTU —
//! that belongs to the layer above. When a frame spans multiple buffers it
//! surfaces each fragment with its hardware position ([`RxFrame::position`],
//! RD0.RFP) and a correct per-fragment length (INV-FRAGLEN); the upper layer
//! concatenates fragments by position.
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
//! - **INV-DESCSIZE** — `size_of::<Descriptor>() ==
//!   size_of::<ether_instance_descriptor_t>()`: `Descriptor` is a `#[repr(C)]`
//!   newtype around that struct, and the inner struct is already 16 bytes so
//!   `align(16)` adds no padding. This makes the
//!   `[Descriptor] → [ether_instance_descriptor_t]` cast in `c_conf` sound and
//!   makes `offset_from` yield the `tx_buffers` slot index (see `tx_position_of`)
//!   and the RX descriptor index (see `update_rx_buffers`). Descriptors stay
//!   homogeneous even though *buffers* gain per-element sizes.
//!
//! - **INV-TXLEN** — `tx_buffers.len() >= num_tx_descriptors`, enforced
//!   unconditionally at open in `c_conf`. FSP keeps `p_tx_descriptor` within
//!   `0..num_tx_descriptors`, so a descriptor-derived slot index is always in
//!   bounds — the safety precondition for the unchecked path in `tx_position_of`.
//!
//! - **INV-RBL** — Every buffer's capacity is a multiple of 32 and `>= 60`. It is
//!   written verbatim into the owning RX descriptor's RD1.RBL; the RA6M3 manual
//!   (p. 932) requires RBL to be an integral multiple of 32, otherwise the
//!   EDMAC's write bound is undefined — load-bearing for memory safety, not just
//!   frame sizing. [`Buffer`] is `#[repr(C, align(32))]` (the EDMAC requires the
//!   buffer pointer 32-byte aligned) and, given the multiple-of-32 size, is
//!   exactly `cap` bytes with no padding. `>= 60` is the minimum Ethernet frame;
//!   with the multiple-of-32 rule the effective minimum is 64. These bounds are
//!   asserted per buffer at erase time ([`Buffer::erase`]) and re-checked at open.
//!
//! - **INV-DYNRBL** — Each RX descriptor's RD1.RBL holds *its own* buffer's
//!   capacity, not a single global size. FSP's `ether_init_descriptors` resets
//!   every RD1.RBL to `cfg.ether_buffer_size` on every link-up; `update_rx_buffers`
//!   re-applies each descriptor's true capacity immediately before the
//!   `R_ETHER_RxBufferUpdate` that arms it. The write is race-free: during the
//!   reset→re-arm window descriptors are `p_buffer == NULL` / RACT clear, so no
//!   DMA occurs; on the same core, writing `buffer_size` then arming (which sets
//!   RACT) is program-ordered, and the descriptor is plain SRAM.
//!   `ether_buffer_size` is set to the *max* capacity in the pool so the reset
//!   window is conservative even before re-arm. `R_ETHER_RxBufferUpdate` and
//!   `R_ETHER_BufferRelease` leave `buffer_size` untouched, so a per-descriptor
//!   RBL survives steady-state re-arming.
//!
//! - **INV-RFL** — A received frame's per-buffer length is `descriptor.size` =
//!   RD1.RFL, the count of frame bytes the EDMAC actually stored, written back per
//!   packet (distinct from `descriptor.buffer_size` = RD1.RBL = the buffer's
//!   capacity). RFL is hardware-bounded by RBL, so `R_ETHER_Read` never reports
//!   more than `cap + padding` bytes and the EDMAC never writes past one buffer.
//!   A frame larger than its buffer is split across descriptors (see
//!   INV-WHOLEFRAME / INV-FRAGLEN). Read-path length clamps are no-ops when sized
//!   right, safety backstops otherwise.
//!
//!   `padding` here is the EDMAC RPADIR data-insertion feature (`e_ether_padding`,
//!   0–3 bytes), NOT the descriptor padding of INV-DESCSIZE. When enabled the
//!   EDMAC inserts `padding` zero-bytes into the received frame at offset
//!   `padding_offset` (the classic use is 2 bytes after the 14-byte Ethernet
//!   header to 4-align the IP header). FSP returns `RFL + padding` and, in
//!   non-zerocopy mode, `memcpy`s that many bytes — so the destination must hold
//!   `RFL + padding` (≤ `cap + padding`); the `+padding` is *added* because
//!   padding makes the copy larger, not smaller.
//!
//!   `RFL + padding` is meaningful **only for a frame-final buffer** (RD0.RFP ==
//!   11 single-buffer, or 01 end-of-split). Per the RA6M3 manual §31.3.3 (p. 934)
//!   the EDMAC writes RFL back only on a frame-final descriptor; for a non-final
//!   buffer of a *split* frame (RFP 10/00, "became full") it writes RFP and RACT
//!   but **not RFL**, leaving `descriptor.size` at its init value (0) or stale. So
//!   if a frame splits, FSP's `size + padding` for those buffers is garbage; with
//!   a large stale `size`, `received_size` can exceed the buffer and FSP's
//!   non-zerocopy `memcpy` **over-reads the source RX buffer** (read-only; the
//!   destination is still guarded). The zerocopy read path sidesteps this: it
//!   computes the per-fragment length itself (INV-FRAGLEN) rather than trusting
//!   FSP's `size + padding`. FSP's own padding/`memcpy`, like the NetX glue
//!   (`rm_netxduo_ether.c`), is written for one buffer per frame — non-zerocopy +
//!   split + padding is the unsupported corner.
//!
//! - **INV-FRAGLEN** — On the zerocopy read path the driver derives the fragment
//!   length from RD0.RFP and RD1.RFL itself; it ignores FSP's `R_ETHER_Read`
//!   length out-param (which is `RFL + padding` unconditionally and so
//!   over-reports an `End` fragment and is garbage for non-final fragments):
//!   - `Whole` (RFP 11): single-buffer frame holding the frame data + the
//!     head-inserted padding → length = `RFL + padding` (≤ `cap`; padding counts
//!     toward "buffer full").
//!   - `End` (RFP 01): tail fragment, no padding inserted here → length = `RFL`.
//!   - `Head`/`Middle` (RFP 10/00): RFL is *not* written; the buffer "became
//!     full," so the fragment occupies the whole buffer → length = `cap` (the
//!     descriptor's RBL). `RFL` is ignored.
//!   This keeps the returned slice in-bounds without relying on FSP's
//!   `size + padding`. RPADIR padding is inserted once near the head (PADR ≤ 63),
//!   so only `Whole`/`Head` fragments carry it; the driver never adds `padding` to
//!   a non-head fragment length. [`RxFrame::position`] surfaces RFP so the caller
//!   can strip padding on the head and concatenate by position.
//!
//! - **INV-WHOLEFRAME** — A small buffer is fully supported and memory-safe:
//!   every buffer is RBL-bounded (INV-RBL), so a frame larger than the buffer just
//!   splits across descriptors and the zerocopy read path surfaces each fragment
//!   with a correct length and position (INV-FRAGLEN). Reassembly is the caller's
//!   job, not the driver's. Callers that want every frame delivered whole size
//!   every buffer `>= 1518 + padding` (max on-wire frame + any RPADIR padding,
//!   ×32) so no frame ever splits — the regime FSP/NetX assume. Callers that
//!   accept fragments may use any capacity `>= 60` / ×32; on the non-zerocopy read
//!   path, additionally avoid the INV-RFL over-read corner (don't combine a
//!   splitting buffer with non-zerocopy + `padding`).
//!
//! - **INV-RXLEN** — `rx_buffers.len() <= rx_descriptors.len()`, enforced at open
//!   in `c_conf`. `update_rx_buffers` walks the roster in order and, for entry `i`,
//!   writes RD1.RBL through `rx_descriptors_base.add(i)` before arming it. The
//!   bound keeps that index inside the descriptor allocation (memory-safety
//!   load-bearing for the volatile RBL write) and prevents over-arming the ring.
//!   Zerocopy roster callers size it `== rx_descriptors.len()` (one buffer per
//!   descriptor); FSP-owned-buffer (non-zerocopy / `pp_ether_buffers`) configs
//!   leave the roster empty, trivially satisfying it.
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
pub struct Ether<'a, S: 'static> {
    ctrl: UnsafePinned<ether_instance_ctrl_t>,
    cfg: UnsafePinned<ether_cfg_t>,
    inst: UnsafePinned<ether_instance_t>,
    user_data: *const (),
    c_ext_cfg: MaybeUninit<UnsafePinned<ether_extended_cfg_t>>,
    // TX parking slots, one per TX descriptor; `None` means the buffer is held
    // by the user. Kept Rust-side per INV-NULLBUF; indexed by descriptor via
    // `tx_position_of`. Ownership rules: INV-TXMOVE.
    tx_buffers: &'static mut [Option<Pin<&'static mut Buffer>>],
    // RX buffer roster, kept per INV-NULLBUF so `update_rx_buffers` can re-arm
    // the ring after a link-up (INV-REARM). `replace_buffer` keeps it truthful
    // when a buffer is swapped out of the ring. Roster order == RX descriptor
    // order (INV-DYNRBL).
    rx_buffers: &'static mut [Pin<&'static mut Buffer>],
    regs: pac::ETHERC0,
    // Mode and padding cached at open so the per-call guards are a single
    // self-relative load instead of a chain through the FSP cfg pointer. FSP
    // never mutates these; written once in `init_open`.
    zerocopy: bool,
    // Extra bytes FSP may copy past the payload (`e_ether_padding` as `u32`);
    // used by `read_non_zerocopy` to size the destination (INV-RFL).
    padding: u32,
    // Largest buffer capacity in the pool (= FSP `ether_buffer_size`). Used by
    // `read_non_zerocopy` to bound the FSP `memcpy` destination (INV-RFL); FSP
    // sizes its non-zerocopy copies to this single value.
    max_buf_size: u32,
    // TX/RX ring bases, cached at open to avoid the
    // ctrl→p_ether_cfg→p_extend→p_*_descriptors chain on the hot paths.
    // Same allocation as `p_tx_descriptor`/`p_rx_descriptor`, so `offset_from`
    // and `add` are valid (INV-DESCSIZE). Covered by the `unsafe impl Send/Sync`.
    tx_descriptors_base: *const ether_instance_descriptor_t,
    rx_descriptors_base: *const ether_instance_descriptor_t,
    _marker: PhantomData<(S, &'a ())>,
}

/// A 32-byte-aligned DMA frame buffer (`align(32)`, see INV-RBL).
///
/// Allocate sized storage as `Buffer<[u8; CAP]>` (via [`Buffer::new`]) and erase
/// it to the size-erased `Buffer` (= `Buffer<[u8]>`) with [`Buffer::erase`]; the
/// erased handle keeps `CAP` as slice metadata, which becomes the descriptor's
/// RD1.RBL (INV-DYNRBL).
#[repr(C, align(32))]
pub struct Buffer<T: ?Sized = [u8]> {
    buf: UnsafePinned<T>,
}

/// Hardware position of a received fragment within its frame (RD0.RFP).
///
/// A frame larger than the buffer it lands in is split across descriptors in
/// ring order. The driver surfaces this so the caller can reassemble:
/// concatenate `Head` → `Middle`* → `End`, or take a `Whole` fragment as-is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramePos {
    /// RFP 11 — the entire frame is in this one buffer.
    Whole = 0b11,
    /// RFP 10 — first fragment of a split frame (carries any RPADIR padding).
    Head = 0b10,
    /// RFP 00 — an interior fragment of a split frame.
    Middle = 0b00,
    /// RFP 01 — last fragment of a split frame (RFL is valid here).
    End = 0b01,
}

/// Exclusive handle to an opened [`Ether`] driver.
///
/// [`RxFrame`] is generic over this so the guard can either *borrow* the
/// driver (`Pin<&mut Ether>`, the common case via [`Ether::read_zerocopy`])
/// or *own* an enclosing borrow guard (`RefMut<DriverBox<Ether>>`, used by
/// the smoltcp glue whose tokens cannot hold a reference into the `RefCell`
/// they travel with).  Implement it for your own wrapper (e.g. a mutex
/// guard) to use [`RxFrame::read`] with custom driver storage.
pub trait EtherMut<'eth> {
    /// Reborrows the handle as the pinned driver.
    fn ether_mut(&mut self) -> Pin<&mut Ether<'eth, Opened>>;
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
/// `Deref<Target = [u8]>` / `DerefMut` yield `&buffer[..len]`, where `len` is
/// the driver-computed fragment length (INV-FRAGLEN). The slice is valid to read
/// and write: while the guard is alive the descriptor is software-owned
/// (RACT == 0) and nothing can re-arm it. [`RxFrame::position`] reports the
/// fragment's position within its frame for callers that reassemble split frames.
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
pub struct RxFrame<'eth, D>
where
    D: EtherMut<'eth>,
{
    // Exclusive driver handle — the carrier of INV-RXLOAN.
    driver: D,
    // Current RX buffer (size-erased; carries `cap` as slice metadata). Stable
    // for the guard's lifetime (descriptor stays software-owned, RACT == 0).
    buf: *mut Buffer,
    // Frame fragment length, derived from RFP per INV-FRAGLEN.
    len: usize,
    // Fragment position within its frame (RD0.RFP), for reassembly.
    pos: FramePos,
    // `'eth` (the driver's callback-context lifetime) is used only through
    // `D`'s trait bound, which doesn't count as a use for the compiler.
    _eth: PhantomData<&'eth ()>,
}

pub struct Buffers<const TX: usize, const RX: usize> {
    tx_buffers: [Option<Pin<&'static mut Buffer>>; TX],
    rx_buffers: [Pin<&'static mut Buffer>; RX],
}

#[repr(C, align(16))]
pub struct Descriptor(ether_instance_descriptor_t);

pub struct EtherConfig {
    pub channel: u8,
    pub zerocopy: bool,
    pub multicast: bool,
    pub promiscuous: bool,
    pub flow_control: bool,
    pub padding: e_ether_padding,
    pub padding_offset: u32,
    pub broadcast_filter: u32,
    pub p_mac_address: &'static [u8; 6],

    pub pp_ether_buffers: Option<&'static mut [&'static mut Buffer]>,
    pub irq: Option<Interrupt>,
    pub p_ether_phy_instance: &'static ether_phy_instance_t,

    pub tx_descriptors: &'static mut [Descriptor],
    pub rx_descriptors: &'static mut [Descriptor],
    pub tx_buffers: &'static mut [Option<Pin<&'static mut Buffer>>],
    pub rx_buffers: &'static mut [Pin<&'static mut Buffer>],
}

unsafe impl<S> Sync for Ether<'_, S> {}
unsafe impl<S> Send for Ether<'_, S> {}
unsafe impl Sync for Descriptor {}
unsafe impl Send for Descriptor {}

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

unsafe impl<S> crate::LifetimeDriver for Ether<'static, S> {
    type Target<'a> = Ether<'a, S>;
}

unsafe impl<S> crate::Block for Ether<'_, S> {
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

impl Ether<'_, Closed> {
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
            max_buf_size: 0,
            tx_descriptors_base: ptr::null(),
            rx_descriptors_base: ptr::null(),
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
        cfg: EtherConfig,
    ) -> TypeStateResult<Ether<'any, Opened>, Self> {
        if this.is_open() {
            return Err((this, e_fsp_err::FSP_ERR_ALREADY_OPEN));
        }

        unsafe {
            let mut this = ManuallyDrop::new(this);

            let p_this = ptr::from_mut(this.get_unchecked_mut());
            let regs = ptr::read(&(*p_this).regs);

            let p_this = p_this.cast::<Ether<'_, Opened>>();
            init_open(p_this, regs, cfg).map_err(|e| (ManuallyDrop::into_inner(this), e))?;
            Ok(DriverBox::new_unchecked(&mut *p_this))
        }
    }
}

unsafe fn init_open(
    slot: *mut Ether<'_, Opened>,
    regs: pac::ETHERC0,
    mut cfg: EtherConfig,
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
            max_buf_size: 0,
            tx_descriptors_base: ptr::null(),
            rx_descriptors_base: ptr::null(),
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
        (*slot).rx_descriptors_base = p_extend.p_rx_descriptors;
        (*slot).max_buf_size = (*(*slot).cfg.get()).ether_buffer_size;

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
impl<S: 'static> PinnedDrop for Ether<'_, S> {
    fn drop(self: Pin<&mut Self>) {
        if self.is_open() {
            fsp_try_unsafe!(R_ETHER_Close(self.ctrl_void())).expect("Error closing Ether");
        }
    }
}

// Todo: I think with frunk I may generalize even this

unsafe impl<'a> CallbackEvent<InterruptCause> for Ether<'a, Opened> {
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

impl<'a> Ether<'a, Opened> {
    pub fn new_open(gpt: pac::ETHERC0, cfg: EtherConfig) -> impl PinInit<Self, fsp_err_t> {
        unsafe {
            pin_init_from_closure(|slot: *mut Ether<'a, Opened>| {
                init_open(slot.cast::<Ether<'a, Opened>>(), gpt, cfg)
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
            Descriptor::is_available(p_desc)
        }
    }

    /// Borrow the current RX frame as a zero-copy guard.
    ///
    /// Returns an [`RxFrame`] that:
    /// - dereferences to `&[u8]` / `&mut [u8]` for the received frame bytes,
    /// - reports the fragment's [`FramePos`] via [`RxFrame::position`],
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
    ) -> Result<RxFrame<'a, Pin<&'drv mut Self>>> {
        RxFrame::read(self).map_err(|(_, err)| err)
    }

    #[inline(always)]
    pub fn read_non_zerocopy(self: Pin<&mut Self>, buffer: &mut [u8]) -> Result<usize> {
        if self.zerocopy {
            return Err(FSP_ERR_ASSERTION);
        }

        // `R_ETHER_Read` (non-zerocopy) `memcpy`s `RFL + padding` bytes into
        // `buffer` without knowing its length. By INV-RFL that is at most
        // `max_buf_size + padding` (FSP sizes its own buffers to `max_buf_size`);
        // requiring the destination to hold that turns an otherwise
        // safe-reachable out-of-bounds write into an error. (The matching source
        // over-read in the undersized-split + padding corner is FSP-internal and
        // unpreventable here — honor INV-WHOLEFRAME to avoid it.)
        let required = self.max_buf_size as usize + self.padding as usize;
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
        buffer: Pin<&'static mut Buffer>,
        len: usize,
    ) -> core::result::Result<Option<Pin<&'static mut Buffer>>, (Pin<&'static mut Buffer>, fsp_err_t)>
    {
        unsafe {
            let this = self.get_unchecked_mut();

            if !this.zerocopy {
                return Err((buffer, FSP_ERR_ASSERTION));
            }

            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // TACT=1: descriptor in flight; submitting would clobber it and let
            // us hand the hardware-owned buffer back as "reclaimed" (INV-TXMOVE).
            if !Descriptor::is_available(p_desc) {
                return Err((buffer, e_fsp_err::FSP_ERR_ETHER_ERROR_TRANSMIT_BUFFER_FULL));
            }

            // Snapshot the slot index before the Write: `R_ETHER_Write` advances
            // the ring pointer, so the post-Write value would index the wrong slot.
            let position = this.tx_position_of(p_desc);

            let cap = buffer.as_ref().get_ref().cap();
            let ptr = buffer.as_ref().get_ref().data_ptr();
            let len = len.min(cap);

            match fsp_try_unsafe!(R_ETHER_Write(this.ctrl().cast(), ptr.cast(), len as u32)) {
                // Submitted: `buffer` is now in flight (TACT tracks it). Park it
                // and hand back whatever the slot held — the reclaimed buffer for
                // a one-call pool swap, or `None` if `take_tx_buf` emptied it.
                // `position` is in bounds by INV-TXLEN (the precondition behind
                // `tx_position_of`), so the slot access is unchecked.
                Ok(()) => Ok(this.tx_buffers.get_unchecked_mut(position).replace(buffer)),
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

        let len = buffer.len().min(self.max_buf_size as usize);
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
        let mut ptr: *mut u8 = ptr::null_mut();
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
        const { assert!(size_of::<Descriptor>() == size_of::<ether_instance_descriptor_t>()) };
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
    pub fn take_tx_buf(self: Pin<&mut Self>) -> Option<Pin<&'static mut Buffer>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // TACT=1: in flight, EDMAC-owned; never hand it out (INV-TXMOVE).
            if !Descriptor::is_available(p_desc) {
                return None;
            }

            // Move out, leaving `None`; the slot then stays empty until parked
            // back, so a buffer is never handed out twice (INV-TXMOVE).
            // `position` is in bounds by INV-TXLEN, so the slot access is unchecked.
            let position = this.tx_position_of(p_desc);
            this.tx_buffers.get_unchecked_mut(position).take()
        }
    }

    /// Re-arms the RX descriptor ring after a link-up, handing every buffer
    /// back to the EDMAC and re-applying each descriptor's true RD1.RBL.
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
        let base = this.rx_descriptors_base;

        // Re-arm every roster buffer unconditionally. This hands buffers to the
        // EDMAC (RACT=1, it may DMA into them); soundness is INV-RXLOAN — the
        // gate above guarantees no live `RxFrame` guard (hence no live reference
        // into any buffer) can coexist with this loop. Each roster entry is a
        // unique `&'static mut` armed once into successive descriptors, so no
        // buffer is armed into two descriptors.
        //
        // Roster index `i` == RX descriptor index (both start at 0 and advance in
        // lockstep — `RxBufferUpdate` arms `p_rx_descriptor` = descriptor `i` this
        // iteration). Re-apply this buffer's RD1.RBL (the FSP reset clobbered it
        // with `ether_buffer_size`) *before* arming, per INV-DYNRBL.
        for (i, buffer) in this.rx_buffers.iter_mut().enumerate() {
            unsafe {
                let b = buffer.as_mut().get_unchecked_mut();
                let cap = b.cap();
                let ptr = b.data_ptr();
                // `i < rx_buffers.len() <= rx_descriptors.len()` (INV-RXLEN), so
                // `base.add(i)` is within the descriptor allocation.
                let desc = base.add(i).cast_mut();
                // RD1.RBL is read by the EDMAC; write it volatile (the FSP field
                // is `volatile`), ordered before the arming `RxBufferUpdate`.
                (&raw mut (*desc).buffer_size).write_volatile(cap as u16);
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
        buffer: Pin<&'static mut Buffer>,
    ) -> Option<Pin<&'static mut Buffer>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_desc = (*this.ctrl.get()).p_tx_descriptor;

            // TACT=1: never displace a hardware-owned, in-flight buffer (INV-TXMOVE).
            if !Descriptor::is_available(p_desc) {
                return Some(buffer);
            }

            // `position` is in bounds by INV-TXLEN, so the slot access is unchecked.
            let position = this.tx_position_of(p_desc);
            this.tx_buffers.get_unchecked_mut(position).replace(buffer)
        }
    }

    // FIXME: Return all buffers and descriptors, and that ether phy too.
    pub fn close<'any>(this: DriverBox<Self>) -> TypeStateResult<Ether<'any, Closed>, Self> {
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

impl<S> Ether<'_, S> {
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
impl EtherConfig {
    pub fn new(ether_phy_instance: DriverBox<EtherPhy<Closed>>) -> Self {
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
    /// inserted into each received frame at `offset`. Padding is inserted once
    /// near the head (PADR ≤ 63), so only the `Whole`/`Head` fragment carries it
    /// (INV-FRAGLEN). On the non-zerocopy read path, pair with single-buffer
    /// reception (every buffer `>= 1518 + padding`, INV-WHOLEFRAME) to avoid the
    /// continuation over-read in INV-RFL; the zerocopy path is immune.
    pub const fn padding(mut self, padding: e_ether_padding, offset: u32) -> Self { self.padding = padding; self.padding_offset = offset; self }
    pub const fn broadcast_filter(mut self, filter: u32) -> Self { self.broadcast_filter = filter; self }
    pub const fn mac(mut self, mac: &'static [u8; 6]) -> Self { self.p_mac_address = mac; self }
    pub const fn irq(mut self, irq: Interrupt) -> Self { self.irq = Some(irq);  self }
    pub const fn ether_buffers(mut self, buffers: &'static mut [&'static mut Buffer]) -> Self { self.pp_ether_buffers = Some(buffers); self }
    pub const fn rx_descriptors(mut self, descriptors: &'static mut [Descriptor]) -> Self { self.rx_descriptors = descriptors; self }
    pub const fn tx_descriptors(mut self, descriptors: &'static mut [Descriptor]) -> Self { self.tx_descriptors = descriptors; self }
    pub const fn buffers<const TX: usize, const RX: usize>(mut self, buffers: &'static mut Buffers<TX, RX>) -> Self {
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers;
        self
    }
    pub const fn set_buffers<const TX: usize, const RX: usize>(&mut self, buffers: &'static mut Buffers<TX, RX>) {
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers;
    }

    /// This function constructs a `ether_cfg_t` from this config struct.
    /// Beware!!! `ether_cfg_t` returned has pointer with `ext`'s address and provenance.
    /// Using those pointers is unsafe thus this function is still safe.
    pub fn c_conf(&mut self, ext: Pin<&mut MaybeUninit<UnsafePinned<ether_extended_cfg_t>>>) -> ether_cfg_t {
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

        // INV-RXLEN — load-bearing for memory safety, not just arming. The RX
        // roster maps 1:1 onto the descriptor ring in order: `update_rx_buffers`
        // writes each entry's RD1.RBL through `rx_descriptors_base.add(i)` and
        // arms it. An over-long roster would index past the descriptor array (OOB
        // volatile write) and over-arm the ring, so the roster must not exceed it.
        // (Non-zerocopy / FSP-owned-buffer configs leave the roster empty, which
        // trivially satisfies this; zerocopy callers size it == the ring.)
        assert!(
            self.rx_buffers.len() <= self.rx_descriptors.len(),
            "There must be at least as many RX descriptors as RX buffers"
        );

        let num_tx_descriptors = self.tx_descriptors.len() as u8;
        let num_rx_descriptors = self.rx_descriptors.len() as u8;

        // INV-DYNRBL: FSP's `ether_buffer_size` is the pool max (the reset-window
        // default RBL; each descriptor's true RBL is re-applied in
        // `update_rx_buffers`). Every buffer's capacity already satisfies INV-RBL
        // (×32, >= 60) by the `const` asserts in `Buffer::new` / `Buffer::erase`,
        // and `Buffer`'s `align(32)` guarantees RD2.RBA — there is no way to land a
        // bad capacity in a roster, so no runtime re-check is needed here.
        let mut max_buf_size: u32 = 0;
        for b in self.rx_buffers.iter() {
            max_buf_size = max_buf_size.max(b.as_ref().get_ref().cap() as u32);
        }
        for b in self.tx_buffers.iter().flatten() {
            max_buf_size = max_buf_size.max(b.as_ref().get_ref().cap() as u32);
        }
        if let Some(pp) = &self.pp_ether_buffers {
            for b in pp.iter() {
                max_buf_size = max_buf_size.max(b.cap() as u32);
            }
            if self.zerocopy {
                assert!(pp.len() as u8 == num_rx_descriptors);
            } else {
                assert!(pp.len() as u8 == num_tx_descriptors + num_rx_descriptors);
                // Non-zerocopy: FSP owns its buffers and sizes all copies to the
                // single `ether_buffer_size`, so they must be uniform.
                for b in pp.iter() {
                    assert!(
                        b.cap() as u32 == max_buf_size,
                        "non-zerocopy pp_ether_buffers must all be the same size"
                    );
                }
            }
        }
        assert!(max_buf_size != 0, "no buffers configured");

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
            ether_buffer_size: max_buf_size,
            irq: utils::extract_irq(self.irq),
            interrupt_priority: BSP_IRQ_DISABLED,
            p_callback: None,
            p_ether_phy_instance: self.p_ether_phy_instance,
            p_context: ptr::null(),
            p_extend: ptr::from_mut(p_extend).cast(),
        }
    }
}

impl Descriptor {
    pub const fn new() -> Self {
        Self(ether_instance_descriptor_t {
            status: 0,
            size: 0,
            p_buffer: ptr::null_mut(),
            buffer_size: 0,
            p_next: ptr::null_mut(),
        })
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

impl Drop for Descriptor {
    fn drop(&mut self) {
        unsafe extern "C" {
            static YOU_MUST_NOT_DROP_ETHER_DESCRIPTOR: [u8; 0];
        }

        let ptr = &raw const YOU_MUST_NOT_DROP_ETHER_DESCRIPTOR;

        ::core::hint::black_box(ptr);

        panic!("Descriptor cannot be dropped. Please store them in a static memory. {ptr:p}");
    }
}

impl<const CAP: usize> Buffer<[u8; CAP]> {
    const ASSERT_CAP: () = {
        assert!(
            CAP % 32 == 0 && CAP >= 60 && CAP <= 65504,
            "buffer capacity must be in 60..=65504 and a multiple of 32 \
             (RD1.RBL is a ×32, 16-bit field: max ×32 value is 65504)"
        );
    };

    pub const fn new() -> Self {
        let _: () = Self::ASSERT_CAP;
        Self {
            buf: UnsafePinned::new([0; CAP]),
        }
    }

    /// Erase the compile-time size, yielding the ring-storable handle that keeps
    /// `CAP` as slice metadata (the descriptor's RD1.RBL, INV-DYNRBL).
    ///
    /// Consumes the unique `Pin<&'static mut>` borrow so the erased handle is the
    /// sole owner of the storage (INV-TXMOVE / the RX roster).
    pub fn erase(self: Pin<&'static mut Self>) -> Pin<&'static mut Buffer> {
        let _: () = Self::ASSERT_CAP;
        // SAFETY: `[u8; CAP]` → `[u8]` of length `CAP` is a metadata-only change;
        // `Buffer<[u8]>` and `Buffer<[u8; CAP]>` share address and (32-byte)
        // alignment, the storage is a slice of `CAP` bytes. The `Pin` is rebuilt
        // around the same `'static` allocation.
        unsafe {
            let p: *mut Buffer<[u8; CAP]> = self.get_unchecked_mut();
            let fat = ptr::slice_from_raw_parts_mut(p.cast::<u8>(), CAP) as *mut Buffer;
            Pin::new_unchecked(&mut *fat)
        }
    }

    /// [`Buffer::erase`] from a plain `&'static mut`. Pinning a `'static`
    /// reference whose ownership is immediately consumed (only a raw pointer is
    /// kept afterwards, the storage never moves) is sound. Use this to build a
    /// heterogeneous roster, then [`Buffers::from_parts`].
    pub fn erase_static(buf: &'static mut Self) -> Pin<&'static mut Buffer> {
        // SAFETY: `buf` is `'static` and is consumed here; the erased handle keeps
        // only a raw pointer to the never-moved storage.
        unsafe { Pin::new_unchecked(buf) }.erase()
    }
}

impl Buffer<[u8]> {
    /// The buffer's capacity in bytes (its RD1.RBL). Carried as slice metadata.
    #[inline(always)]
    const fn cap(&self) -> usize {
        let p: *mut [u8] = self.buf.get();
        p.len()
    }

    /// Raw pointer to the first data byte (RD2.RBA, 32-byte aligned).
    #[inline(always)]
    const fn data_ptr(&self) -> *mut u8 {
        self.buf.get().cast()
    }

    /// Mutable view of the whole buffer (TX storage). The caller writes the frame
    /// here before submitting it; sound while the buffer is software-owned.
    pub fn as_mut_bytes(self: Pin<&mut Self>) -> &mut [u8] {
        unsafe { &mut *self.get_unchecked_mut().buf.get() }
    }
}

impl<const TX: usize, const RX: usize> Buffers<TX, RX> {
    /// Build a homogeneous pool: `TX` + `RX` buffers all of the same capacity
    /// `CAP`. For a *mixed* pool, erase each buffer with [`Buffer::erase`] and
    /// build the arrays yourself, then [`Buffers::from_parts`].
    pub fn new<const CAP: usize>(
        tx_buffers: [&'static mut Buffer<[u8; CAP]>; TX],
        rx_buffers: [&'static mut Buffer<[u8; CAP]>; RX],
    ) -> Self {
        Self {
            tx_buffers: tx_buffers.map(|b| Some(Buffer::erase_static(b))),
            rx_buffers: rx_buffers.map(Buffer::erase_static),
        }
    }

    /// Build a (possibly heterogeneous) pool from already-erased buffers.
    pub fn from_parts(
        tx_buffers: [Option<Pin<&'static mut Buffer>>; TX],
        rx_buffers: [Pin<&'static mut Buffer>; RX],
    ) -> Self {
        Self {
            tx_buffers,
            rx_buffers,
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

impl Deref for Buffer<[u8]> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        // SAFETY: the `&self` borrow already excludes concurrent mutation of the
        // bytes; this only re-derives a shared ref through `UnsafePinned`.
        unsafe { &*self.buf.get() }
    }
}
impl DerefMut for Buffer<[u8]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.buf.get() }
    }
}

impl<const CAP: usize> Default for Buffer<[u8; CAP]> {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for Descriptor {
    fn default() -> Self {
        Self::new()
    }
}

impl<'eth, D> RxFrame<'eth, D>
where
    D: EtherMut<'eth>,
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

        // FSP returns the descriptor's data pointer (RD2.RBA), not a `Buffer`.
        // We ignore FSP's length out-param: it is `RFL + padding`
        // unconditionally, which over-reports an `End` fragment (padding belongs
        // to the head, INV-FRAGLEN) and is meaningless for non-final fragments.
        // The driver derives the length itself below.
        let mut p_data: *mut u8 = ptr::null_mut();
        let mut fsp_len = 0u32;

        if let Err(err) = fsp_try_unsafe!(R_ETHER_Read(
            driver.ether_mut().ctrl_void(),
            ptr::from_mut(&mut p_data).cast(),
            &mut fsp_len
        )) {
            return Err((driver, err));
        }

        if p_data as usize % 32 != 0 || p_data.is_null() {
            log::error!(
                "ether(read): buffer is not aligned or null. p_buf: {p_data:p}, len: {fsp_len}"
            );
            return Err((driver, FSP_ERR_ASSERTION));
        }

        // Zerocopy `R_ETHER_Read` does NOT advance `p_rx_descriptor`, so the
        // current descriptor is the one just read: its RD0.RFP, RD1.RFL and
        // RD1.RBL are ours to inspect.
        // RD0/RD1 are DMA memory the EDMAC writes back (RFL) / reads (RBL); the
        // FSP descriptor declares these fields `volatile`. Read them volatile so
        // the compiler can't cache or reorder them against the hardware.
        let p_desc = unsafe { (*driver.ether_mut().ctrl()).p_rx_descriptor };
        let status = unsafe { (&raw const (*p_desc).status).read_volatile() };
        let rfl = unsafe { (&raw const (*p_desc).size).read_volatile() as usize }; // RD1.RFL
        let cap = unsafe { (&raw const (*p_desc).buffer_size).read_volatile() as usize }; // RD1.RBL
        let padding = driver.ether_mut().padding as usize;
        let pos = FramePos::from_status(status);

        // INV-FRAGLEN: length is derived from RFP, not FSP's out-param.
        // - Whole (11): the buffer holds the frame + the head-inserted padding,
        //   `RFL + padding` physical bytes (≤ cap; padding counts toward "full").
        // - End   (01): tail fragment, no padding inserted here → `RFL`.
        // - Head/Middle (10/00): the buffer "became full"; RFL is not written, so
        //   the fragment is the whole buffer → `cap`.
        // NOTE: the `End` arm assumes RD1.RFL holds the *end fragment's* byte
        // count (the RD1.RFL field definition, manual p. 932, "frame stored in
        // the buffer"). If silicon instead writes the *whole-frame* length here
        // (a possible reading of §31.3.3, p. 934), report
        // `rfl - Σ(previous fragment caps)` instead. Unconfirmed on hardware; the
        // field-definition wording is the stronger signal, hence per-fragment.
        let len = match pos {
            FramePos::Whole => (rfl + padding).min(cap),
            FramePos::End => rfl.min(cap),
            FramePos::Head | FramePos::Middle => cap,
        };

        // Rebuild the size-erased handle from the data pointer + the descriptor's
        // RBL (= the buffer's capacity). `p_data` is the current descriptor's RX
        // buffer, software-owned (RACT == 0, FSP just returned it). It stays that
        // way because `driver` — moved into the guard below — is now the only
        // route to the driver (INV-RXLOAN).
        let buf = ptr::slice_from_raw_parts_mut(p_data, cap) as *mut Buffer;

        Ok(RxFrame {
            driver,
            buf,
            len,
            pos,
            _eth: PhantomData,
        })
    }

    /// The fragment's position within its frame (RD0.RFP).
    ///
    /// `Whole` for a single-buffer frame; otherwise concatenate
    /// `Head` → `Middle`* → `End` to reassemble. Only `Whole`/`Head` fragments
    /// carry RPADIR padding (INV-FRAGLEN).
    #[inline(always)]
    pub fn position(&self) -> FramePos {
        self.pos
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
    ///   caller as `Pin<&'static mut Buffer>`,
    /// - the ring advances to the next descriptor.
    ///
    /// This is the zero-copy keep-the-frame path: the caller now owns the frame
    /// buffer and can process it at leisure, while the ring immediately has a
    /// fresh slot to receive the next frame.
    ///
    /// `new` may have a different capacity than the buffer it replaces; the new
    /// RD1.RBL is re-applied here, and again on the next link-up re-arm
    /// (INV-DYNRBL).
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
        new: Pin<&'static mut Buffer>,
    ) -> core::result::Result<Pin<&'static mut Buffer>, (Pin<&'static mut Buffer>, fsp_err_t)> {
        let mut this = ManuallyDrop::new(self);

        let new_cap = new.as_ref().get_ref().cap();
        let new_data_ptr = new.as_ref().get_ref().data_ptr();
        let new_raw: *mut Buffer = unsafe { new.get_unchecked_mut() };

        // Re-apply `new`'s RD1.RBL on the descriptor about to be armed (the
        // current one, not advanced by the zerocopy read), INV-DYNRBL. Volatile
        // write (the FSP field is `volatile`, EDMAC reads RBL) ordered before
        // the arming `RxBufferUpdate` below.
        let p_desc = unsafe { (*this.driver.ether_mut().ctrl()).p_rx_descriptor };
        unsafe { (&raw mut (*p_desc).buffer_size).write_volatile(new_cap as u16) };

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
                    // `b` is `&mut Pin<&'static mut Buffer>`; compare data
                    // addresses (the `Buffer` start) ignoring slice metadata.
                    core::ptr::eq(
                        (&***b as *const Buffer).cast::<u8>(),
                        old_raw.cast::<u8>().cast_const(),
                    )
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

impl FramePos {
    /// Decode RD0.RFP[1:0] (status bits 29:28).
    #[inline(always)]
    fn from_status(status: u32) -> Self {
        const W: u32 = FramePos::Whole as u32;
        const H: u32 = FramePos::Head as u32;
        const E: u32 = FramePos::End as u32;
        const M: u32 = FramePos::Middle as u32;

        match (status >> 28) & 0b11 {
            W => FramePos::Whole,
            H => FramePos::Head,
            E => FramePos::End,
            M => FramePos::Middle,
            _ => unreachable!(),
        }
    }
}

impl<'eth, D: EtherMut<'eth>> Deref for RxFrame<'eth, D> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        // SAFETY: `buf` is the descriptor-current RX buffer (RACT == 0).
        // The guard exclusively borrows the driver, so nothing else can read
        // or write this buffer concurrently.  The explicit `&(...)` suppresses
        // the implicit-autoref lint while keeping identical semantics.
        unsafe { &(&*(*self.buf).buf.get())[..self.len] }
    }
}

impl<'eth, D: EtherMut<'eth>> DerefMut for RxFrame<'eth, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: Same as Deref above.  `DerefMut` is sound: while the guard
        // lives the descriptor is software-owned (RACT == 0) and the driver
        // borrow prevents re-arming.
        unsafe { &mut (&mut *(*self.buf).buf.get())[..self.len] }
    }
}

impl<'eth, D: EtherMut<'eth>> Drop for RxFrame<'eth, D> {
    fn drop(&mut self) {
        // Re-arm the same buffer into the ring and advance the descriptor.
        // Errors are silently ignored: `R_ETHER_BufferRelease` can only fail
        // when the link is down or in magic-packet mode; in either case the
        // ring is re-initialized on the next link-up and every descriptor is
        // re-armed then, so no frame slot is permanently lost.
        let _ = fsp_try_unsafe!(R_ETHER_BufferRelease(self.driver.ether_mut().ctrl_void()));
    }
}

impl<'eth> EtherMut<'eth> for Pin<&mut Ether<'eth, Opened>> {
    #[inline(always)]
    fn ether_mut(&mut self) -> Pin<&mut Ether<'eth, Opened>> {
        self.as_mut()
    }
}

impl<'eth> EtherMut<'eth> for RefMut<'_, DriverBox<Ether<'eth, Opened>>> {
    #[inline(always)]
    fn ether_mut(&mut self) -> Pin<&mut Ether<'eth, Opened>> {
        (**self).as_mut()
    }
}
