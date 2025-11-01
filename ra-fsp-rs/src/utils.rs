use crate::pac;
use cortex_m::{interrupt::InterruptNumber, peripheral::NVIC};

#[derive(Clone, Copy, Debug)]
pub struct Irq {
    pub int: pac::Interrupt,
    pub prio: Option<u8>,
}

impl Irq {
    pub fn new(int: pac::Interrupt, prio: Option<u8>) -> Self {
        Self { int, prio }
    }
    pub const fn extract_irq(this: Option<Self>) -> i32 {
        let invalid_vector = ra_fsp_sys::generated::FSP_INVALID_VECTOR as i32;
        match this {
            Some(i) => i.int as u16 as i32,
            None => invalid_vector,
        }
    }

    pub const fn extract_ipl(this: Option<Self>) -> u8 {
        const _: () = assert!(ra_fsp_sys::generated::BSP_IRQ_DISABLED <= u8::MAX as u32);

        let disabled = ra_fsp_sys::generated::BSP_IRQ_DISABLED as u8;

        match this {
            None | Some(Irq { prio: None, .. }) => disabled,
            Some(Irq { prio: Some(p), .. }) => p,
        }
    }
}

// Cast is safe because it returns an `unsafe` function pointer, which cannot
// be safely called. So we are shifting the unsafety to the caller.
pub const fn cast_callback<T>(callback: extern "C" fn(&mut T)) -> unsafe extern "C" fn(*mut T) {
    unsafe {
        core::mem::transmute::<
            // C code will pass `*mut T` and rust code will receive `&mut T`
            extern "C" fn(&mut T),
            unsafe extern "C" fn(*mut T),
        >(callback)
    }
}

// Cast is safe because it returns an `unsafe` function pointer, which cannot
// be safely called. So we are shifting the unsafety to the caller.
pub const fn cast_callback_opt<T>(
    callback: Option<extern "C" fn(&mut T)>,
) -> Option<unsafe extern "C" fn(*mut T)> {
    match callback {
        Some(callback) => Some(cast_callback(callback)),
        None => None,
    }
}

// fixme: not true on __CORTEX_M == 23
#[allow(dead_code)] // used in assert, idk why it warns
pub const fn fsp_prio_to_hw(priority: u8, nvic_prio_bits: u8) -> u8 {
    (((priority as u32) << (8 - nvic_prio_bits) as u32) & (u8::MAX as u32)) as u8
}

pub const fn hw_prio_to_fsp(hw_priority: u8, nvic_prio_bits: u8) -> u8 {
    ((hw_priority as u32) >> (8 - nvic_prio_bits)) as u8
}

const _: () = assert!(fsp_prio_to_hw(14, 4) == 224);
const _: () = assert!(hw_prio_to_fsp(224, 4) == 14);

pub fn read_fsp_priority<I: InterruptNumber>(irq: I) -> u8 {
    let hw_priority = NVIC::get_priority(irq);

    hw_prio_to_fsp(hw_priority, pac::NVIC_PRIO_BITS)
}

pub unsafe fn try_read_priority_into(irq: Option<Irq>, place: *mut u8) {
    if let Some(irq) = irq
        && irq.prio.is_none()
    {
        unsafe { *place = read_fsp_priority(irq.int) }
    }
}
