#[repr(C)]
///Register block
pub struct RegisterBlock {
    irqcr: [IRQCR; 13],
    _reserved1: [u8; 0x01],
    irqcr14: IRQCR14,
    irqcr15: IRQCR14,
    _reserved3: [u8; 0xf0],
    nmicr: NMICR,
    _reserved4: [u8; 0x1f],
    nmier: NMIER,
    _reserved5: [u8; 0x0e],
    nmiclr: NMICLR,
    _reserved6: [u8; 0x0e],
    nmisr: NMISR,
    _reserved7: [u8; 0x5e],
    wupen: WUPEN,
    _reserved8: [u8; 0x5c],
    selsr0: SELSR0,
    _reserved9: [u8; 0x7e],
    delsr: (),
    _reserved10: [u8; 0x80],
    ielsr: [IELSR; 32],
}
impl RegisterBlock {
    ///0x00..0x0d - IRQ Control Register %s
    #[inline(always)]
    pub const fn irqcr(&self, n: usize) -> &IRQCR {
        &self.irqcr[n]
    }
    ///Iterator for array of:
    ///0x00..0x0d - IRQ Control Register %s
    #[inline(always)]
    pub fn irqcr_iter(&self) -> impl Iterator<Item = &IRQCR> {
        self.irqcr.iter()
    }
    ///0x0e - IRQ Control Register 14
    #[inline(always)]
    pub const fn irqcr14(&self) -> &IRQCR14 {
        &self.irqcr14
    }
    ///0x0e - IRQ Control Register 15
    #[inline(always)]
    pub const fn irqcr15(&self) -> &IRQCR14 {
        &self.irqcr15
    }
    ///0x100 - NMI Pin Interrupt Control Register
    #[inline(always)]
    pub const fn nmicr(&self) -> &NMICR {
        &self.nmicr
    }
    ///0x120 - Non-Maskable Interrupt Enable Register
    #[inline(always)]
    pub const fn nmier(&self) -> &NMIER {
        &self.nmier
    }
    ///0x130 - Non-Maskable Interrupt Status Clear Register
    #[inline(always)]
    pub const fn nmiclr(&self) -> &NMICLR {
        &self.nmiclr
    }
    ///0x140 - Non-Maskable Interrupt Status Register
    #[inline(always)]
    pub const fn nmisr(&self) -> &NMISR {
        &self.nmisr
    }
    ///0x1a0 - Wake Up Interrupt Enable Register
    #[inline(always)]
    pub const fn wupen(&self) -> &WUPEN {
        &self.wupen
    }
    ///0x200 - SYS Event Link Setting Register
    #[inline(always)]
    pub const fn selsr0(&self) -> &SELSR0 {
        &self.selsr0
    }
    ///0x280..0x288 - DMAC Event Link Setting Register %s
    #[inline(always)]
    pub const fn delsr(&self, n: usize) -> &DELSR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(640)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x280..0x288 - DMAC Event Link Setting Register %s
    #[inline(always)]
    pub fn delsr_iter(&self) -> impl Iterator<Item = &DELSR> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(640)
                .add(4 * n)
                .cast()
        })
    }
    ///0x300..0x380 - ICU Event Link Setting Register %s
    #[inline(always)]
    pub const fn ielsr(&self, n: usize) -> &IELSR {
        &self.ielsr[n]
    }
    ///Iterator for array of:
    ///0x300..0x380 - ICU Event Link Setting Register %s
    #[inline(always)]
    pub fn ielsr_iter(&self) -> impl Iterator<Item = &IELSR> {
        self.ielsr.iter()
    }
}
/**IRQCR (rw) register accessor: IRQ Control Register %s

You can [`read`](crate::Reg::read) this register and get [`irqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irqcr`] module*/
pub type IRQCR = crate::Reg<irqcr::IRQCR_SPEC>;
///IRQ Control Register %s
pub mod irqcr;
pub use irqcr as irqcr14;
pub use IRQCR as IRQCR14;
/**NMISR (r) register accessor: Non-Maskable Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nmisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmisr`] module*/
pub type NMISR = crate::Reg<nmisr::NMISR_SPEC>;
///Non-Maskable Interrupt Status Register
pub mod nmisr;
/**NMIER (rw) register accessor: Non-Maskable Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmier`] module*/
pub type NMIER = crate::Reg<nmier::NMIER_SPEC>;
///Non-Maskable Interrupt Enable Register
pub mod nmier;
/**NMICLR (rw) register accessor: Non-Maskable Interrupt Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`nmiclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmiclr`] module*/
pub type NMICLR = crate::Reg<nmiclr::NMICLR_SPEC>;
///Non-Maskable Interrupt Status Clear Register
pub mod nmiclr;
/**NMICR (rw) register accessor: NMI Pin Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`nmicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmicr`] module*/
pub type NMICR = crate::Reg<nmicr::NMICR_SPEC>;
///NMI Pin Interrupt Control Register
pub mod nmicr;
/**IELSR (rw) register accessor: ICU Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`ielsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ielsr`] module*/
pub type IELSR = crate::Reg<ielsr::IELSR_SPEC>;
///ICU Event Link Setting Register %s
pub mod ielsr;
/**DELSR (rw) register accessor: DMAC Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`delsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@delsr`] module*/
pub type DELSR = crate::Reg<delsr::DELSR_SPEC>;
///DMAC Event Link Setting Register %s
pub mod delsr;
/**SELSR0 (rw) register accessor: SYS Event Link Setting Register

You can [`read`](crate::Reg::read) this register and get [`selsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@selsr0`] module*/
pub type SELSR0 = crate::Reg<selsr0::SELSR0_SPEC>;
///SYS Event Link Setting Register
pub mod selsr0;
/**WUPEN (rw) register accessor: Wake Up Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`wupen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wupen`] module*/
pub type WUPEN = crate::Reg<wupen::WUPEN_SPEC>;
///Wake Up Interrupt Enable Register
pub mod wupen;
