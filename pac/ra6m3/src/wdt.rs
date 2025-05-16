#[repr(C)]
///Register block
pub struct RegisterBlock {
    wdtrr: WDTRR,
    _reserved1: [u8; 0x01],
    wdtcr: WDTCR,
    wdtsr: WDTSR,
    wdtrcr: WDTRCR,
    _reserved4: [u8; 0x01],
    wdtcstpr: WDTCSTPR,
}
impl RegisterBlock {
    ///0x00 - WDT Refresh Register
    #[inline(always)]
    pub const fn wdtrr(&self) -> &WDTRR {
        &self.wdtrr
    }
    ///0x02 - WDT Control Register
    #[inline(always)]
    pub const fn wdtcr(&self) -> &WDTCR {
        &self.wdtcr
    }
    ///0x04 - WDT Status Register
    #[inline(always)]
    pub const fn wdtsr(&self) -> &WDTSR {
        &self.wdtsr
    }
    ///0x06 - WDT Reset Control Register
    #[inline(always)]
    pub const fn wdtrcr(&self) -> &WDTRCR {
        &self.wdtrcr
    }
    ///0x08 - WDT Count Stop Control Register
    #[inline(always)]
    pub const fn wdtcstpr(&self) -> &WDTCSTPR {
        &self.wdtcstpr
    }
}
/**WDTRR (rw) register accessor: WDT Refresh Register

You can [`read`](crate::Reg::read) this register and get [`wdtrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtrr`] module*/
pub type WDTRR = crate::Reg<wdtrr::WDTRR_SPEC>;
///WDT Refresh Register
pub mod wdtrr;
/**WDTCR (rw) register accessor: WDT Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtcr`] module*/
pub type WDTCR = crate::Reg<wdtcr::WDTCR_SPEC>;
///WDT Control Register
pub mod wdtcr;
/**WDTSR (rw) register accessor: WDT Status Register

You can [`read`](crate::Reg::read) this register and get [`wdtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtsr`] module*/
pub type WDTSR = crate::Reg<wdtsr::WDTSR_SPEC>;
///WDT Status Register
pub mod wdtsr;
/**WDTRCR (rw) register accessor: WDT Reset Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtrcr`] module*/
pub type WDTRCR = crate::Reg<wdtrcr::WDTRCR_SPEC>;
///WDT Reset Control Register
pub mod wdtrcr;
/**WDTCSTPR (rw) register accessor: WDT Count Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtcstpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcstpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wdtcstpr`] module*/
pub type WDTCSTPR = crate::Reg<wdtcstpr::WDTCSTPR_SPEC>;
///WDT Count Stop Control Register
pub mod wdtcstpr;
