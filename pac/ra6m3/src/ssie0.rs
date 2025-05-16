#[repr(C)]
///Register block
pub struct RegisterBlock {
    ssicr: SSICR,
    ssisr: SSISR,
    _reserved2: [u8; 0x08],
    ssifcr: SSIFCR,
    ssifsr: SSIFSR,
    ssiftdr: SSIFTDR,
    ssifrdr: SSIFRDR,
    ssiofr: SSIOFR,
    ssiscr: SSISCR,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ssicr(&self) -> &SSICR {
        &self.ssicr
    }
    ///0x04 - Status Register
    #[inline(always)]
    pub const fn ssisr(&self) -> &SSISR {
        &self.ssisr
    }
    ///0x10 - FIFO Control Register
    #[inline(always)]
    pub const fn ssifcr(&self) -> &SSIFCR {
        &self.ssifcr
    }
    ///0x14 - FIFO Status Register
    #[inline(always)]
    pub const fn ssifsr(&self) -> &SSIFSR {
        &self.ssifsr
    }
    ///0x18 - Transmit FIFO Data Register
    #[inline(always)]
    pub const fn ssiftdr(&self) -> &SSIFTDR {
        &self.ssiftdr
    }
    ///0x1c - Receive FIFO Data Register
    #[inline(always)]
    pub const fn ssifrdr(&self) -> &SSIFRDR {
        &self.ssifrdr
    }
    ///0x20 - Audio Format Register
    #[inline(always)]
    pub const fn ssiofr(&self) -> &SSIOFR {
        &self.ssiofr
    }
    ///0x24 - Status Control Register
    #[inline(always)]
    pub const fn ssiscr(&self) -> &SSISCR {
        &self.ssiscr
    }
}
/**SSICR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`ssicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssicr`] module*/
pub type SSICR = crate::Reg<ssicr::SSICR_SPEC>;
///Control Register
pub mod ssicr;
/**SSISR (rw) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`ssisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssisr`] module*/
pub type SSISR = crate::Reg<ssisr::SSISR_SPEC>;
///Status Register
pub mod ssisr;
/**SSIFCR (rw) register accessor: FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`ssifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssifcr`] module*/
pub type SSIFCR = crate::Reg<ssifcr::SSIFCR_SPEC>;
///FIFO Control Register
pub mod ssifcr;
/**SSIFSR (rw) register accessor: FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`ssifsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssifsr`] module*/
pub type SSIFSR = crate::Reg<ssifsr::SSIFSR_SPEC>;
///FIFO Status Register
pub mod ssifsr;
/**SSIFTDR (w) register accessor: Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiftdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssiftdr`] module*/
pub type SSIFTDR = crate::Reg<ssiftdr::SSIFTDR_SPEC>;
///Transmit FIFO Data Register
pub mod ssiftdr;
/**SSIFRDR (r) register accessor: Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`ssifrdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssifrdr`] module*/
pub type SSIFRDR = crate::Reg<ssifrdr::SSIFRDR_SPEC>;
///Receive FIFO Data Register
pub mod ssifrdr;
/**SSIOFR (rw) register accessor: Audio Format Register

You can [`read`](crate::Reg::read) this register and get [`ssiofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssiofr`] module*/
pub type SSIOFR = crate::Reg<ssiofr::SSIOFR_SPEC>;
///Audio Format Register
pub mod ssiofr;
/**SSISCR (rw) register accessor: Status Control Register

You can [`read`](crate::Reg::read) this register and get [`ssiscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssiscr`] module*/
pub type SSISCR = crate::Reg<ssiscr::SSISCR_SPEC>;
///Status Control Register
pub mod ssiscr;
