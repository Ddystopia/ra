use crate::pac;
use cortex_m::{interrupt::InterruptNumber, peripheral::NVIC};

pub(crate) const fn extract_irq(this: Option<pac::Interrupt>) -> i32 {
    let invalid_vector = ra_fsp_sys::generated::FSP_INVALID_VECTOR as i32;
    match this {
        Some(i) => i as u16 as i32,
        None => invalid_vector,
    }
}

// fixme: not true on __CORTEX_M == 23
/// Convert FSP priority to hardware priority: Smaller value means higher priority.
/// Note that RTIC uses reversed priority scheme - smaller value means lower priority.
pub const fn fsp_prio_to_hw(priority: u8, nvic_prio_bits: u8) -> u8 {
    (((priority as u32) << (8 - nvic_prio_bits) as u32) & (u8::MAX as u32)) as u8
}

/// Convert hardware priority to FSP priority: Smaller value means higher priority.
/// Note that RTIC uses reversed priority scheme - smaller value means lower priority.
pub const fn hw_prio_to_fsp(hw_priority: u8, nvic_prio_bits: u8) -> u8 {
    ((hw_priority as u32) >> (8 - nvic_prio_bits)) as u8
}

const _: () = assert!(fsp_prio_to_hw(14, 4) == 224);
const _: () = assert!(hw_prio_to_fsp(224, 4) == 14);

pub fn read_fsp_priority<I: InterruptNumber>(irq: I) -> u8 {
    let hw_priority = NVIC::get_priority(irq);

    hw_prio_to_fsp(hw_priority, pac::NVIC_PRIO_BITS)
}

/// # Safety
///
/// Changing priority levels can break priority-based critical sections
/// (see [`register::basepri`]) and compromise memory safety.
///
/// [`register::basepri`](https://docs.rs/cortex-m/0.7.7/cortex_m/register/basepri/index.html)
pub unsafe fn set_fsp_priority<I: InterruptNumber>(nvic: &mut NVIC, irq: I, fsp_priority: u8) {
    let hw_priority = fsp_prio_to_hw(fsp_priority, pac::NVIC_PRIO_BITS);
    unsafe { nvic.set_priority(irq, hw_priority) }
}

pub(crate) fn try_read_priority_into(interrupt: impl Into<Option<pac::Interrupt>>, place: &mut u8) {
    if let Some(int) = interrupt.into() {
        *place = read_fsp_priority(int)
    }
}
