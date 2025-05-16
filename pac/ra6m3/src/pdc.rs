#[repr(C)]
///Register block
pub struct RegisterBlock {
    pccr0: PCCR0,
    pccr1: PCCR1,
    pcsr: PCSR,
    pcmonr: PCMONR,
    pcdr: PCDR,
    vcr: VCR,
    hcr: HCR,
}
impl RegisterBlock {
    ///0x00 - PDC Control Register 0
    #[inline(always)]
    pub const fn pccr0(&self) -> &PCCR0 {
        &self.pccr0
    }
    ///0x04 - PDC Control Register 1
    #[inline(always)]
    pub const fn pccr1(&self) -> &PCCR1 {
        &self.pccr1
    }
    ///0x08 - PDC Status Register
    #[inline(always)]
    pub const fn pcsr(&self) -> &PCSR {
        &self.pcsr
    }
    ///0x0c - PDC Pin Monitor Register
    #[inline(always)]
    pub const fn pcmonr(&self) -> &PCMONR {
        &self.pcmonr
    }
    ///0x10 - PDC Receive Data Register
    #[inline(always)]
    pub const fn pcdr(&self) -> &PCDR {
        &self.pcdr
    }
    ///0x14 - Vertical Capture Register
    #[inline(always)]
    pub const fn vcr(&self) -> &VCR {
        &self.vcr
    }
    ///0x18 - Horizontal Capture Register
    #[inline(always)]
    pub const fn hcr(&self) -> &HCR {
        &self.hcr
    }
}
/**PCCR0 (rw) register accessor: PDC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`pccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pccr0`] module*/
pub type PCCR0 = crate::Reg<pccr0::PCCR0_SPEC>;
///PDC Control Register 0
pub mod pccr0;
/**PCCR1 (rw) register accessor: PDC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pccr1`] module*/
pub type PCCR1 = crate::Reg<pccr1::PCCR1_SPEC>;
///PDC Control Register 1
pub mod pccr1;
/**PCSR (rw) register accessor: PDC Status Register

You can [`read`](crate::Reg::read) this register and get [`pcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcsr`] module*/
pub type PCSR = crate::Reg<pcsr::PCSR_SPEC>;
///PDC Status Register
pub mod pcsr;
/**PCMONR (r) register accessor: PDC Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`pcmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcmonr`] module*/
pub type PCMONR = crate::Reg<pcmonr::PCMONR_SPEC>;
///PDC Pin Monitor Register
pub mod pcmonr;
/**PCDR (r) register accessor: PDC Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`pcdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcdr`] module*/
pub type PCDR = crate::Reg<pcdr::PCDR_SPEC>;
///PDC Receive Data Register
pub mod pcdr;
/**VCR (rw) register accessor: Vertical Capture Register

You can [`read`](crate::Reg::read) this register and get [`vcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vcr`] module*/
pub type VCR = crate::Reg<vcr::VCR_SPEC>;
///Vertical Capture Register
pub mod vcr;
/**HCR (rw) register accessor: Horizontal Capture Register

You can [`read`](crate::Reg::read) this register and get [`hcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hcr`] module*/
pub type HCR = crate::Reg<hcr::HCR_SPEC>;
///Horizontal Capture Register
pub mod hcr;
