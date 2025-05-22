#![allow(non_upper_case_globals)]

use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::{pin::Pin, ptr};

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

pub use ra_fsp_sys::r_ether::{InterruptCause, interrupt_cause};

use crate::pac::Interrupt;
use crate::unsafe_pinned::UnsafePinned;

use ra_fsp_sys::generated::{
    ETHER_LINK_ESTABLISH_STATUS_UP, ETHER_ZEROCOPY_DISABLE, ETHER_ZEROCOPY_ENABLE,
    R_ETHER_CallbackSet, R_ETHER_Close, R_ETHER_LinkProcess, R_ETHER_Open, R_ETHER_Read,
    R_ETHER_RxBufferUpdate, R_ETHER_TxStatusGet, R_ETHER_WakeOnLANEnable, R_ETHER_Write,
    e_ether_padding, ether_extended_cfg_t, ether_instance_descriptor_t, ether_phy_instance_t,
    fsp_err_t, st_ether_instance_ctrl,
};

pub use ra_fsp_sys::generated::{
    FSP_ERR_ETHER_ERROR_LINK,
    FSP_ERR_ETHER_ERROR_NO_DATA,
    ether_api_t, //
    ether_callback_args_t,
    ether_cfg_t,
    ether_instance_ctrl_t,
    ether_instance_t,
    g_ether_on_ether,
};

pub struct EtherInstance<const BUF_SIZE: usize> {
    inst: UnsafePinned<ether_instance_ctrl_t>,
    tx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    tx_taken: &'static mut [bool],
}

#[repr(C, align(32))]
pub struct Buffer<const BUF_SIZE: usize>(UnsafePinned<[u8; BUF_SIZE]>);

pub struct Buffers<const BUF_SIZE: usize, const TX: usize, const RX: usize> {
    tx_buffers: [Pin<&'static mut Buffer<BUF_SIZE>>; TX],
    rx_buffers: [Pin<&'static mut Buffer<BUF_SIZE>>; RX],
    tx_taken: [bool; TX],
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
    pub irq: Interrupt,
    pub interrupt_priority: Option<u32>,
    pub p_ether_phy_instance: &'static ether_phy_instance_t,

    // if we want this to be `&'static dyn Fn(&ether_callback_args_t)`, then we
    //   need `feature(ptr_metadata)` and a way to extract `<dyn Fn>::call` method
    // if we want this to be `&'static impl Fn(&ether_callback_args_t)`, then we
    //   need tait to be able to have this in statics
    pub callback: Option<extern "C" fn(&mut ether_callback_args_t)>,
    pub tx_descriptors: &'static [Descriptor<BUF_SIZE>],
    pub rx_descriptors: &'static [Descriptor<BUF_SIZE>],
    pub tx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],
    pub rx_buffers: &'static mut [Pin<&'static mut Buffer<BUF_SIZE>>],

    tx_taken: &'static mut [bool],
    c_ext_cfg: UnsafePinned<MaybeUninit<ether_extended_cfg_t>>,
    c_cfg: UnsafePinned<MaybeUninit<ether_cfg_t>>,
}

unsafe extern "C" {
    pub safe fn ether_eint_isr();
}

#[inline(always)]
const fn get_mut<const BUF_SIZE: usize>(
    this: Pin<&mut EtherInstance<BUF_SIZE>>,
) -> *mut core::ffi::c_void {
    unsafe { this.get_unchecked_mut().ptr().cast() }
}

unsafe impl<const BUF_SIZE: usize> Sync for EtherInstance<BUF_SIZE> {}
unsafe impl<const BUF_SIZE: usize> Send for EtherInstance<BUF_SIZE> {}

impl<const BUF_SIZE: usize> EtherInstance<BUF_SIZE> {
    pub const fn new() -> Self {
        Self {
            inst: UnsafePinned::new(st_ether_instance_ctrl {
                open: 0,
                p_ether_cfg: ptr::null(),
                p_rx_descriptor: ptr::null_mut(),
                p_tx_descriptor: ptr::null_mut(),
                p_reg_etherc: ptr::null_mut(),
                p_reg_edmac: ptr::null_mut(),
                previous_link_status: 0,
                link_change: 0,
                magic_packet: 0,
                link_establish_status: 0,
                p_callback: None,
                p_callback_memory: ptr::null_mut(),
                p_context: ptr::null(),
            }),
            tx_buffers: &mut [],
            tx_taken: &mut [],
            rx_buffers: &mut [],
        }
    }

    pub fn is_up(&self) -> bool {
        let status = unsafe { (*self.inst.get()).link_establish_status };

        status == ETHER_LINK_ESTABLISH_STATUS_UP
    }

    pub fn get_open(&self) -> u32 {
        unsafe { (*self.inst.get()).open }
    }

    /// Takes the buffer out of the current tx descriptor. Returns `None` if
    /// all descriptors are currently used or if there is no descriptor.
    ///
    /// Descriptor is not moved. Note that the only way to move the descriptor is to transmit the message.
    pub fn take_tx_buf(self: Pin<&mut Self>) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        unsafe {
            let this = self.get_unchecked_mut();
            let p_inst = this.inst.get();
            let p_desc = (*p_inst).p_tx_descriptor;
            if !Descriptor::<BUF_SIZE>::is_available(p_desc) {
                #[cfg(feature = "log")]
                log::error!("TX not available");
                return None;
            }

            let p_conf = (*p_inst).p_ether_cfg;
            let p_extend = (*p_conf).p_extend.cast::<ether_extended_cfg_t>();
            let p_tx_descriptors = (*p_extend).p_tx_descriptors;
            let position = p_desc.offset_from(p_tx_descriptors);
            let position = usize::try_from(position).ok()?;

            if this.tx_taken[position] {
                log::error!("TX taken");
                return None;
            }

            this.tx_taken[position] = true;

            let buffer = this.tx_buffers[position].as_mut().get_unchecked_mut();

            Some(Pin::new_unchecked(&mut *ptr::from_mut(buffer)))
        }
    }

    pub fn update_rx_buffers<'a>(self: Pin<&'a mut Self>, cause: InterruptCause) {
        if !cause.went_up {
            return;
        }

        let this = unsafe { self.get_unchecked_mut() };
        let instance = this.inst.get().cast();

        for buffer in &mut *this.rx_buffers {
            unsafe {
                let ptr = buffer.as_mut().get_unchecked_mut().0.get();

                R_ETHER_RxBufferUpdate(instance, ptr.cast());
            }
        }
    }

    pub fn tx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Option<Pin<&'static mut Buffer<BUF_SIZE>>> {
        let this = unsafe { self.get_unchecked_mut() };
        let ptr = ptr::from_ref(buffer.as_ref().get_ref());

        match this
            .tx_buffers
            .iter()
            .map(|b| b.as_ref().get_ref())
            .map(ptr::from_ref)
            .position(|p| p == ptr)
        {
            Some(i) => this.tx_taken[i] = false,
            None => {
                log::error!("Not found which TX is update");
                return Some(buffer);
            }
        }

        None
    }

    #[inline(always)]
    const fn ptr(&self) -> *mut ether_instance_ctrl_t {
        UnsafePinned::raw_get(&raw const self.inst)
    }
}

const fn cast_callback(
    callback: extern "C" fn(&mut ether_callback_args_t),
) -> unsafe extern "C" fn(*mut ether_callback_args_t) {
    unsafe {
        core::mem::transmute::<
            extern "C" fn(&mut ether_callback_args_t),
            unsafe extern "C" fn(*mut ether_callback_args_t),
        >(callback)
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
    pub const fn new(p_ether_phy_instance: &'static ether_phy_instance_t) -> Self {
        const { assert!(BUF_SIZE <= 1514) };
        const { assert!(BUF_SIZE >= 60) };

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
            irq: Interrupt::IEL0,
            interrupt_priority: None,
            p_ether_phy_instance,
            callback: None,
            rx_descriptors: &[],
            tx_descriptors: &[],
            tx_buffers: &mut [],
            rx_buffers: &mut [],
            tx_taken: &mut [],
            c_ext_cfg: UnsafePinned::new(MaybeUninit::uninit()),
            c_cfg: UnsafePinned::new(MaybeUninit::uninit()),
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
    pub const fn irq(mut self, irq: Interrupt) -> Self { self.irq = irq;  self }
    /// # Safety
    ///
    /// This function can emulate [`cortex_m::peripheral::NVIC::set_priority`].
    pub const unsafe fn irq_priority(mut self, fsp_priority: u32) -> Self {  self.interrupt_priority = Some(fsp_priority); self }
    pub const fn callback(mut self, callback: extern "C" fn(&mut ether_callback_args_t)) -> Self { self.callback = Some(callback); self }
    pub const fn ether_buffers(mut self, buffers: &'static mut [&'static mut Buffer<BUF_SIZE>]) -> Self { self.pp_ether_buffers = Some(buffers); self }
    pub const fn rx_descriptors(mut self, descriptors: &'static [Descriptor<BUF_SIZE>]) -> Self { self.rx_descriptors = descriptors; self }
    pub const fn tx_descriptors(mut self, descriptors: &'static [Descriptor<BUF_SIZE>]) -> Self { self.tx_descriptors = descriptors; self }
    pub const fn buffers<const TX: usize, const RX: usize>(mut self, buffers: &'static mut Buffers<BUF_SIZE, TX, RX>) -> Self { 
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers; 
        self.tx_taken = &mut buffers.tx_taken; 
        self 
    }
    pub const fn set_buffers<const TX: usize, const RX: usize>(&mut self, buffers: &'static mut Buffers<BUF_SIZE, TX, RX>) {
        self.rx_buffers = &mut buffers.rx_buffers;
        self.tx_buffers = &mut buffers.tx_buffers; 
        self.tx_taken = &mut buffers.tx_taken; 
    }
    pub fn unchange_irq_priority(&mut self) {
        let hw_priority = cortex_m::peripheral::NVIC::get_priority(self.irq);
        self.interrupt_priority = Some(hw_prio_to_fsp(hw_priority, crate::pac::NVIC_PRIO_BITS));
    }

    pub const fn c_conf(&'static self) -> &'static ether_cfg_t {
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
            (*self.c_ext_cfg.get()).write(ether_extended_cfg_t {
                p_tx_descriptors: tx_desc.get_ref().get().cast(),
                p_rx_descriptors: rx_desc.get_ref().get().cast(),
            })
        };

        let pp_ether_buffers = match &self.pp_ether_buffers {
            // Shouldn't we use `UnafePinned::get` somewhere there?
            Some(p) => ptr::from_ref(&**p).cast_mut().cast(),
            None => ptr::null_mut(),
        };

        let c_cfg = ether_cfg_t {
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
            irq: self.irq as u16 as _,
            interrupt_priority: self.interrupt_priority.expect("Interrupt priority is not set"),
            p_callback: match self.callback {
                Some(c) => Some(cast_callback(c)),
                None => None,
            },
            p_ether_phy_instance: self.p_ether_phy_instance,
            p_context: ptr::null(),
            p_extend: ptr::from_mut(p_extend).cast(),
        };

        unsafe { (*self.c_cfg.get()).write(c_cfg) }
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
        Self(UnsafePinned::new([0; BUF_SIZE]))
    }

    pub fn as_mut_bytes(self: Pin<&mut Self>) -> &mut [u8; BUF_SIZE] {
        unsafe { &mut *self.get_unchecked_mut().0.get() }
    }
}

impl<const BUF_SIZE: usize, const TX: usize, const RX: usize> Buffers<BUF_SIZE, TX, RX> {
    // Todo: figure out a way to make this in const
    //        [&'static mut Buffer<BUF_SIZE>; TX] -> [&'static mut Buffer<BUF_SIZE>; TX]
    pub const fn new(
        tx_buffers: [&'static mut Buffer<BUF_SIZE>; TX],
        rx_buffers: [&'static mut Buffer<BUF_SIZE>; RX],
    ) -> Self {
        Self {
            tx_buffers: unsafe { core::mem::transmute_copy(&tx_buffers) },
            rx_buffers: unsafe { core::mem::transmute_copy(&rx_buffers) },
            // rx_buffers: rx_buffers.map(Pin::static_mut),
            // rx_buffers: rx_buffers.map(Pin::static_mut),
            tx_taken: [false; TX],
        }
    }
}

impl<const BUF_SIZE: usize> Deref for Buffer<BUF_SIZE> {
    type Target = [u8; BUF_SIZE];
    fn deref(&self) -> &Self::Target {
        // todo: autite it
        unsafe { &*self.0.get() }
    }
}
impl<const BUF_SIZE: usize> DerefMut for Buffer<BUF_SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0.get() }
    }
}

macro_rules! fsp_try {
    ($call:expr) => {
        match unsafe { $call } {
            0 => Ok(()),
            err => Err(err),
        }
    };
}

// fixme: not true on __CORTEX_M == 23
#[allow(dead_code)] // used in assert, idk why it warns
const fn fsp_prio_to_hw(priority: u32, nvic_prio_bits: u8) -> u8 {
    ((priority << (8 - nvic_prio_bits) as u32) & (u8::MAX as u32)) as u8
}

const fn hw_prio_to_fsp(hw_priority: u8, nvic_prio_bits: u8) -> u32 {
    (hw_priority as u32) >> (8 - nvic_prio_bits) as u32
}

const _: () = assert!(fsp_prio_to_hw(14, 4) == 224);
const _: () = assert!(hw_prio_to_fsp(224, 4) == 14);

impl<const BUF_SIZE: usize> EtherInstance<BUF_SIZE> {
    pub fn open(
        mut self: Pin<&mut Self>,
        conf: &'static mut EtherConfig<BUF_SIZE>,
    ) -> Result<(), fsp_err_t> {
        if conf.interrupt_priority.is_none() {
            conf.unchange_irq_priority();
        }
        unsafe {
            use core::mem::replace;

            let this = self.as_mut().get_unchecked_mut();

            this.tx_buffers = replace(&mut conf.tx_buffers, &mut []);
            this.rx_buffers = replace(&mut conf.rx_buffers, &mut []);
            this.tx_taken = replace(&mut conf.tx_taken, &mut []);
        }

        fsp_try!(R_ETHER_Open(get_mut(self), conf.c_conf()))
    }
    pub fn close(self: Pin<&mut Self>) -> Result<(), fsp_err_t> {
        fsp_try!(R_ETHER_Close(get_mut(self)))
    }
    pub fn read_zerocopy(
        self: Pin<&mut Self>,
    ) -> Result<(Pin<&'static mut Buffer<BUF_SIZE>>, usize), fsp_err_t> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().inst.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_ENABLE {
            return Err(ra_fsp_sys::generated::FSP_ERR_ASSERTION);
        }

        let mut p_buf: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        let mut len = 0;

        fsp_try!(R_ETHER_Read(
            get_mut(self),
            ptr::from_mut(&mut p_buf).cast(),
            &mut len
        ))?;

        if !p_buf.is_aligned() || p_buf.is_null() {
            log::error!("ether(read): buffer is not aligned or null. p_buf: {p_buf:p}, len: {len}");
            return Err(ra_fsp_sys::generated::FSP_ERR_ASSERTION);
        }

        Ok((unsafe { Pin::new_unchecked(&mut *p_buf) }, len as usize))
    }
    pub fn read_non_zerocopy(self: Pin<&mut Self>, buffer: &mut [u8]) -> Result<usize, fsp_err_t> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().inst.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_DISABLE {
            return Err(ra_fsp_sys::generated::FSP_ERR_ASSERTION);
        }

        let p_buf = ptr::from_mut(buffer);
        let mut len = 0;

        fsp_try!(R_ETHER_Read(get_mut(self), p_buf.cast(), &mut len))?;

        Ok(len as usize)
    }
    pub fn rx_buffer_update(
        self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
    ) -> Result<(), fsp_err_t> {
        let ptr = unsafe { buffer.get_unchecked_mut().0.get() };

        fsp_try!(R_ETHER_RxBufferUpdate(get_mut(self), ptr.cast()))
    }
    pub fn write_zerocopy(
        mut self: Pin<&mut Self>,
        buffer: Pin<&'static mut Buffer<BUF_SIZE>>,
        len: usize,
    ) -> Result<(), fsp_err_t> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().inst.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_ENABLE {
            return Err(ra_fsp_sys::generated::FSP_ERR_ASSERTION);
        }

        let ptr = buffer.as_ref().get_ref().0.get();
        let len = len.min(BUF_SIZE);

        // - If we will successfully submit the buffer, it will become
        //   unavailable and user will not be able to get it
        // - If we faced the error, `buffer` is consumed from user, so we can
        //   allow them to take it with `take_tx_buf`.
        self.as_mut().tx_buffer_update(buffer);

        fsp_try!(R_ETHER_Write(get_mut(self), ptr.cast(), len as u32))?;

        Ok(())
    }
    pub fn write_non_zerocopy(self: Pin<&mut Self>, buffer: &[u8]) -> Result<(), fsp_err_t> {
        let zerocopy = unsafe { (*(*self.as_ref().get_ref().inst.get()).p_ether_cfg).zerocopy };
        if zerocopy != ETHER_ZEROCOPY_DISABLE {
            return Err(ra_fsp_sys::generated::FSP_ERR_ASSERTION);
        }

        let len = buffer.len().min(BUF_SIZE);
        let ptr = buffer.as_ptr().cast_mut();
        fsp_try!(R_ETHER_Write(get_mut(self), ptr.cast(), len as u32))
    }
    pub fn link_process(self: Pin<&mut Self>) -> Result<(), fsp_err_t> {
        fsp_try!(R_ETHER_LinkProcess(get_mut(self)))
    }
    pub fn wake_on_lan_enable(self: Pin<&mut Self>) -> Result<(), fsp_err_t> {
        fsp_try!(R_ETHER_WakeOnLANEnable(get_mut(self)))
    }
    pub fn tx_status_get(self: Pin<&mut Self>) -> Result<(), fsp_err_t> {
        let mut ptr: *mut Buffer<BUF_SIZE> = ptr::null_mut();
        fsp_try!(R_ETHER_TxStatusGet(
            get_mut(self),
            ptr::from_mut(&mut ptr).cast()
        ))?;

        Ok(())
    }
    pub fn callback_set(
        self: Pin<&mut Self>,
        callback: Option<extern "C" fn(&mut ether_callback_args_t)>,
    ) -> Result<(), fsp_err_t> {
        fsp_try!(R_ETHER_CallbackSet(
            get_mut(self),
            callback.map(cast_callback),
            ptr::null_mut(),
            ptr::null_mut(), // todo: is it needed, considering `&mut`, rtic? It is for nested stuff
        ))
    }
}
