use core::sync::atomic;

use cortex_m::peripheral::NVIC;

#[inline(always)]
pub fn irq_status_clear(irq: crate::pac::Interrupt) {
    // let icu = unsafe { crate::pac::ICU::steal() };
    // icu.ielsr(irq as u16 as usize).write(|w| w.ir().clear_bit());
    // _ = icu.ielsr(irq as u16 as usize).read();

    unsafe { ra_fsp_sys::generated::R_BSP_IrqStatusClear(irq as u16 as i32) }
}

#[inline(always)]
pub fn irq_clear_pending(irq: crate::pac::Interrupt) {
    if ra_fsp_sys::generated::BSP_FEATURE_ICU_HAS_IELSR != 0 {
        irq_status_clear(irq);
        cortex_m::asm::dmb();
    }
    NVIC::unpend(irq);

    // unsafe { ra_fsp_sys::generated::R_BSP_IrqClearPending(irq as u16 as i32) }
}

#[inline(always)]
pub fn irq_enable_no_clear(irq: crate::pac::Interrupt) {
    atomic::compiler_fence(atomic::Ordering::SeqCst);
    NVIC::unpend(irq);
    atomic::compiler_fence(atomic::Ordering::SeqCst);

    // unsafe { ra_fsp_sys::generated::R_BSP_IrqEnableNoClear(irq as u16 as i32) }
}

#[inline(always)]
pub fn irq_enable(irq: crate::pac::Interrupt) {
    irq_clear_pending(irq);
    irq_enable_no_clear(irq);

    // unsafe { ra_fsp_sys::generated::R_BSP_IrqEnable(irq as u16 as i32) }
}

#[inline(always)]
pub fn irq_disable(irq: crate::pac::Interrupt) {
    NVIC::mask(irq);
    cortex_m::asm::dsb();
    cortex_m::asm::isb();

    // unsafe { ra_fsp_sys::generated::R_BSP_IrqDisable(irq as u16 as i32) }
}
