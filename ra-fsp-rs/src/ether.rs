#![allow(non_upper_case_globals)]
use core::{
    marker::PhantomData,
    mem::{MaybeUninit, take, zeroed},
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr,
};

use crate::{
    Block, Callback, Irq, Result,
    ether_phy::EtherPhy,
    fsp_try_unsafe, log,
    pac::{self, Interrupt},
    state_markers::{Closed, Opened},
    unsafe_pinned::UnsafePinned,
    utils::{self},
};

use pin_init::{PinInit, pin_data, pin_init_from_closure};

pub use ra_fsp_sys::{
    generated::{
        self as api,
        ETHER_CFG_PARAM_CHECKING_ENABLE,
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
    },
    r_ether::InterruptCause,
};

use ra_fsp_sys::generated::{
    R_ETHER_CallbackSet, R_ETHER_Close, R_ETHER_LinkProcess, R_ETHER_Open, R_ETHER_Read,
    R_ETHER_RxBufferUpdate, R_ETHER_TxStatusGet, R_ETHER_WakeOnLANEnable, R_ETHER_Write,
    e_ether_padding, e_fsp_err, ether_ctrl_t, ether_extended_cfg_t, ether_instance_descriptor_t,
    ether_phy_instance_t, fsp_err_t,
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
`Pin<&'static UnsafePinned<[u8]>>` to the user.

Read:


*/

#[pin_data(PinnedDrop)]
pub struct Ether<const BUF_SIZE: usize, S: 'static> {
    ctrl: UnsafePinned<ether_instance_ctrl_t>,
    cfg: UnsafePinned<ether_cfg_t>,
    inst: UnsafePinned<ether_instance_t>,
    user_data: *const (),
    c_ext_cfg: MaybeUninit<UnsafePinned<ether_extended_cfg_t>>,
    tx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    tx_taken: u32,
    regs: pac::ETHERC0,
    _marker: PhantomData<S>,
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

// Is it okay to even have references to this stuct? Hardware can r/w `status`
// etc, and it is `volatile` in C code, and we are doing volatile reads. But,
// like, reference is basically a read, but it is okay to read those fields.
#[repr(transparent)]
struct RawDescripor(ether_instance_descriptor_t);

#[repr(C, align(16))]
pub struct Descriptor<const BUF_SIZE: usize>(
    UnsafePinned<RawDescripor>,
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
    pub irq: Irq,
    pub p_ether_phy_instance: &'static ether_phy_instance_t,

    pub tx_descriptors: &'static [Descriptor<BUF_SIZE>],
    pub rx_descriptors: &'static [Descriptor<BUF_SIZE>],
    pub tx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    pub rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
}

unsafe impl<const BUF_SIZE: usize, S> Sync for Ether<BUF_SIZE, S> {}
unsafe impl<const BUF_SIZE: usize, S> Send for Ether<BUF_SIZE, S> {}
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

unsafe impl<const BUF_SIZE: usize, S> crate::Block for Ether<BUF_SIZE, S> {
    type Config = ether_cfg_t;
    type Instance = ether_instance_t;
    type Api = ether_api_t;
    type State = crate::state_markers::Opened;
    type Context = S;

    const API: &ether_api_t = &API;

    fn ctrl(&self) -> *mut core::ffi::c_void {
        UnsafePinned::raw_get(&raw const self.ctrl).cast()
    }

    fn instance(&self) -> &Self::Instance {
        unsafe { &*self.inst.get() }
    }
}

impl<const BUF_SIZE: usize> Ether<BUF_SIZE, Closed> {
    pub const fn new(ether: crate::pac::ETHERC0) -> Self {
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
    pub fn open(
        self: Pin<&mut Self>,
        cfg: EtherConfig<BUF_SIZE>,
    ) -> Result<Pin<&mut Ether<BUF_SIZE, Opened>>> {
        unsafe {
            let this = ptr::from_mut(self.get_unchecked_mut());
            let regs = ptr::read(&(*this).regs);

            if (*(*this).ctrl.get()).open != 0 {
                return Err(e_fsp_err::FSP_ERR_ALREADY_OPEN);
            }

            let this = this.cast::<Ether<BUF_SIZE, Opened>>();
            init_open(this, regs, cfg)?;
            Ok(Pin::new_unchecked(
                &mut *this.cast::<Ether<BUF_SIZE, Opened>>(),
            ))
        }
    }
}

unsafe fn init_open<const BUF_SIZE: usize>(
    slot: *mut Ether<BUF_SIZE, Opened>,
    gpt: pac::ETHERC0,
    mut cfg: EtherConfig<BUF_SIZE>,
) -> Result<()> {
    unsafe {
        (*slot).regs = gpt;
        (*slot).ctrl = UnsafePinned::new(zeroed());
        (*(*slot).inst.get()).p_ctrl = (*slot).ctrl.get().cast::<core::ffi::c_void>();
        (*(*slot).inst.get()).p_cfg = (*slot).cfg.get().cast_const();
        (*(*slot).inst.get()).p_api = ptr::from_ref(&API);
        (*slot).c_ext_cfg = MaybeUninit::zeroed();
        (*slot).tx_buffers = take(&mut cfg.tx_buffers);
        (*slot).rx_buffers = take(&mut cfg.rx_buffers);
        (*slot).tx_taken = 0;
        (*slot).user_data = ptr::null();
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
            (*p_cfg).interrupt_priority = 0;
            let ipl = &raw mut (*p_cfg).interrupt_priority;
            utils::try_read_priority_into(Some(cfg.irq), ipl.cast::<u8>());

            fsp_try_unsafe!(R_ETHER_Open(p_ctrl.cast::<ether_ctrl_t>(), p_cfg))
        })
    }
}

#[pin_init::pinned_drop]
impl<const BUF_SIZE: usize, S: 'static> PinnedDrop for Ether<BUF_SIZE, S> {
    fn drop(self: Pin<&mut Self>) {
        if self.is_open() {
            fsp_try_unsafe!(R_ETHER_Close(self.ctrl_void())).expect("Error closing Ether");
        }
    }
}

impl<const BUF_SIZE: usize> Ether<BUF_SIZE, Opened> {
    pub fn new(gpt: pac::ETHERC0, cfg: EtherConfig<BUF_SIZE>) -> impl PinInit<Self, fsp_err_t> {
        unsafe {
            pin_init_from_closure(|slot: *mut Ether<BUF_SIZE, Opened>| {
                init_open(slot.cast::<Ether<BUF_SIZE, Opened>>(), gpt, cfg)
            })
        }
    }

    pub fn callback_set<F>(self: Pin<&mut Self>, context: &'static F) -> Result<()>
    where
        F: Callback<InterruptCause>,
    {
        unsafe extern "C" fn trampoline<const BUF_SIZE: usize, F: Callback<InterruptCause>>(
            args: *mut ether_callback_args_t,
        ) {
            unsafe {
                let p_context = (*args).p_context;
                let this = p_context.cast::<Ether<BUF_SIZE, Opened>>();
                let context = (*this).user_data.cast::<F>();
                let cause = InterruptCause::from_event(&mut *args);

                debug_assert!(context != ptr::null());
                F::call(&*context, cause);
            }
        }

        unsafe {
            let this = self.get_unchecked_mut();
            let ctrl = this.ctrl.get();

            this.user_data = ptr::from_ref(context).cast();

            fsp_try_unsafe!(R_ETHER_CallbackSet(
                ctrl.cast(),
                Some(trampoline::<BUF_SIZE, F>),
                ptr::from_ref(this).cast::<core::ffi::c_void>(),
                core::ptr::null_mut()
            ))
        }
    }

    // pub fn close(self: Pin<&mut Self>) -> Result<()> {
    //     fsp_try_unsafe!(R_ETHER_Close(self.ctrl_void()))
    // }

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

        Ok((unsafe { Pin::new_unchecked(&mut *p_buf) }, len as usize))
    }
    pub fn read_non_zerocopy(self: Pin<&mut Self>, buffer: &mut [u8]) -> Result<usize> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_DISABLE {
            return Err(FSP_ERR_ASSERTION);
        }

        let p_buf = ptr::from_mut(buffer);
        let mut len = 0;

        fsp_try_unsafe!(R_ETHER_Read(self.ctrl_void(), p_buf.cast(), &mut len))?;

        Ok(len as usize)
    }
    pub fn rx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Result<()> {
        let ptr = unsafe { buffer.get_unchecked_mut().buf.get() };

        fsp_try_unsafe!(R_ETHER_RxBufferUpdate(self.ctrl_void(), ptr.cast()))
    }
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
    pub fn write_non_zerocopy(self: Pin<&mut Self>, buffer: &[u8]) -> Result<()> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().ctrl.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_DISABLE {
            return Err(FSP_ERR_ASSERTION);
        }

        let len = buffer.len().min(BUF_SIZE);
        let ptr = buffer.as_ptr().cast_mut();
        fsp_try_unsafe!(R_ETHER_Write(self.ctrl_void(), ptr.cast(), len as u32))
    }
    pub fn link_process(self: Pin<&mut Self>) -> Result<()> {
        fsp_try_unsafe!(R_ETHER_LinkProcess(self.ctrl_void()))
    }
    pub fn wake_on_lan_enable(self: Pin<&mut Self>) -> Result<()> {
        fsp_try_unsafe!(R_ETHER_WakeOnLANEnable(self.ctrl_void()))
    }
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
    pub fn take_tx_buf(self: Pin<&mut Self>) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_inst = this.ctrl.get();
            let p_desc = (*p_inst).p_tx_descriptor;

            let p_conf = (*p_inst).p_ether_cfg;
            let p_extend = (*p_conf).p_extend.cast::<ether_extended_cfg_t>();
            let p_tx_descriptors = (*p_extend).p_tx_descriptors;
            let position = p_desc.offset_from(p_tx_descriptors);
            let position = position as u8;
            if this.tx_taken & (1 << position) != 0 {
                log::error!("TX taken");
                return None;
            }

            debug_assert!(Descriptor::<BUF_SIZE>::is_available(p_desc));

            this.tx_taken |= 1 << position;

            let buffer = this.tx_buffers[position as usize]
                .as_mut()
                .get_unchecked_mut();

            ptr::write(buffer.tx_taken_position.get(), position);

            Some(Pin::new_unchecked(&mut *ptr::from_mut(buffer)))
        }
    }

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

    pub fn update_tx_buffers(self: Pin<&mut Self>, cause: InterruptCause) {
        if !cause.transmits {
            return;
        }

        let this = unsafe { self.get_unchecked_mut() };
        this.tx_taken = 0;
        // log::info!("Update TX buffers");
    }

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
}

impl<const BUF_SIZE: usize, S> Ether<BUF_SIZE, S> {
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

const fn assert_descriptor_unused<const BUF_SIZE: usize>(descriptor: &Descriptor<BUF_SIZE>) {
    let descriptor = descriptor.0.get().cast_const() as *const ether_instance_descriptor_t;

    let buffer_size = unsafe {
        // hw is not writing to that field, thus it is fine to make normal load.
        (*descriptor).buffer_size
    };

    assert!(buffer_size == 0, "Descriptor already in use");
}

#[rustfmt::skip]
impl<const BUF_SIZE: usize> EtherConfig<BUF_SIZE> {
    pub fn new(ether_phy_instance: Pin<&'static mut EtherPhy<Closed>>) -> Self {
        const { assert!(BUF_SIZE <= 1514) };
        const { assert!(BUF_SIZE >= 60) };

        let p_ether_phy_instance = ether_phy_instance.into_ref().get_ref().instance();

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
            irq: Irq::new(Interrupt::IEL0, None),
            p_ether_phy_instance,
            rx_descriptors: &[],
            tx_descriptors: &[],
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
    pub const fn irq(mut self, irq: Irq) -> Self { self.irq = irq;  self }
    pub const fn ether_buffers(mut self, buffers: &'static mut [&'static mut Buffer<BUF_SIZE>]) -> Self { self.pp_ether_buffers = Some(buffers); self }
    pub const fn rx_descriptors(mut self, descriptors: &'static [Descriptor<BUF_SIZE>]) -> Self { self.rx_descriptors = descriptors; self }
    pub const fn tx_descriptors(mut self, descriptors: &'static [Descriptor<BUF_SIZE>]) -> Self { self.tx_descriptors = descriptors; self }
    pub const fn buffers<const TX: usize, const RX: usize>(mut self, buffers: &'static mut Buffers<BUF_SIZE, TX, RX>) -> Self { 
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
        let num_tx_descriptors = self.tx_descriptors.len() as u8;
        let num_rx_descriptors = self.rx_descriptors.len() as u8;

        {
            let mut i = 0;
            while i < num_tx_descriptors as usize {
                assert_descriptor_unused(&self.tx_descriptors[i]);
                i += 1;
            }
            let mut i = 0;
            while i < num_rx_descriptors as usize {
                assert_descriptor_unused(&self.rx_descriptors[i]);
                i += 1;
            }
        }

        let tx_desc = Descriptor::pinned_array(Pin::static_ref(self.tx_descriptors));
        let rx_desc = Descriptor::pinned_array(Pin::static_ref(self.rx_descriptors));

        assert!(num_tx_descriptors != 0, "Descriptors cannot be empty");
        assert!(num_rx_descriptors != 0, "Descriptors cannot be empty");
        assert!(num_rx_descriptors <= 4, "Max 4 descriptors");
        assert!(num_tx_descriptors <= 4, "Max 4 descriptors");

        if let Some(pp_ether_buffers) = &self.pp_ether_buffers {
            if self.zerocopy  {
                assert!(pp_ether_buffers.len() as u8 == num_rx_descriptors);
            } else {
                assert!(pp_ether_buffers.len() as u8 == num_tx_descriptors + num_rx_descriptors);
            }
        };

        let p_extend = unsafe {
            ext.get_unchecked_mut().write(UnsafePinned::new(ether_extended_cfg_t {
                p_tx_descriptors: tx_desc.get_ref().get().cast(),
                p_rx_descriptors: rx_desc.get_ref().get().cast(),
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
            irq: self.irq.int as u16 as _, 
            interrupt_priority: self.irq.prio.expect("Interrupt priority is not set") as u32,
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
            UnsafePinned::new(RawDescripor(ether_instance_descriptor_t {
                status: 0,
                size: 0,
                p_buffer: ptr::null_mut(),
                buffer_size: 0,
                p_next: ptr::null_mut(),
            })),
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

    #[inline(always)]
    const fn pinned_array(array: Pin<&[Self]>) -> Pin<&UnsafePinned<[RawDescripor]>> {
        let ptr = ptr::from_ref(array.get_ref()) as *const UnsafePinned<[RawDescripor]>;

        unsafe { Pin::new_unchecked(&*ptr) }
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

impl<const BUF_SIZE: usize> Deref for Buffer<BUF_SIZE> {
    type Target = [u8; BUF_SIZE];
    fn deref(&self) -> &Self::Target {
        // todo: autite it
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
