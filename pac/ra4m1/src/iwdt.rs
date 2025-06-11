#[repr(C)]
///Register block
pub struct RegisterBlock {
    iwdtrr: IWDTRR,
    _reserved1: [u8; 0x03],
    iwdtsr: IWDTSR,
}
impl RegisterBlock {
    ///0x00 - IWDT Refresh Register
    #[inline(always)]
    pub const fn iwdtrr(&self) -> &IWDTRR {
        &self.iwdtrr
    }
    ///0x04 - IWDT Status Register
    #[inline(always)]
    pub const fn iwdtsr(&self) -> &IWDTSR {
        &self.iwdtsr
    }
}
/**IWDTRR (rw) register accessor: IWDT Refresh Register

You can [`read`](crate::Reg::read) this register and get [`iwdtrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iwdtrr`] module*/
pub type IWDTRR = crate::Reg<iwdtrr::IWDTRR_SPEC>;
///IWDT Refresh Register
pub mod iwdtrr;
/**IWDTSR (rw) register accessor: IWDT Status Register

You can [`read`](crate::Reg::read) this register and get [`iwdtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iwdtsr`] module*/
pub type IWDTSR = crate::Reg<iwdtsr::IWDTSR_SPEC>;
///IWDT Status Register
pub mod iwdtsr;
