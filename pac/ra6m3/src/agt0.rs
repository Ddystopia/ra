#[repr(C)]
///Register block
pub struct RegisterBlock {
    agt: AGT,
    agtcma: AGTCMA,
    agtcmb: AGTCMB,
    _reserved3: [u8; 0x02],
    agtcr: AGTCR,
    agtmr1: AGTMR1,
    agtmr2: AGTMR2,
    _reserved6: [u8; 0x01],
    agtioc: AGTIOC,
    agtisr: AGTISR,
    agtcmsr: AGTCMSR,
    agtiosel: AGTIOSEL,
}
impl RegisterBlock {
    ///0x00 - AGT Counter Register
    #[inline(always)]
    pub const fn agt(&self) -> &AGT {
        &self.agt
    }
    ///0x02 - AGT Compare Match A Register
    #[inline(always)]
    pub const fn agtcma(&self) -> &AGTCMA {
        &self.agtcma
    }
    ///0x04 - AGT Compare Match B Register
    #[inline(always)]
    pub const fn agtcmb(&self) -> &AGTCMB {
        &self.agtcmb
    }
    ///0x08 - AGT Control Register
    #[inline(always)]
    pub const fn agtcr(&self) -> &AGTCR {
        &self.agtcr
    }
    ///0x09 - AGT Mode Register 1
    #[inline(always)]
    pub const fn agtmr1(&self) -> &AGTMR1 {
        &self.agtmr1
    }
    ///0x0a - AGT Mode Register 2
    #[inline(always)]
    pub const fn agtmr2(&self) -> &AGTMR2 {
        &self.agtmr2
    }
    ///0x0c - AGT I/O Control Register
    #[inline(always)]
    pub const fn agtioc(&self) -> &AGTIOC {
        &self.agtioc
    }
    ///0x0d - AGT Event Pin Select Register
    #[inline(always)]
    pub const fn agtisr(&self) -> &AGTISR {
        &self.agtisr
    }
    ///0x0e - AGT Compare Match Function Select Register
    #[inline(always)]
    pub const fn agtcmsr(&self) -> &AGTCMSR {
        &self.agtcmsr
    }
    ///0x0f - AGT Pin Select Register
    #[inline(always)]
    pub const fn agtiosel(&self) -> &AGTIOSEL {
        &self.agtiosel
    }
}
/**AGT (rw) register accessor: AGT Counter Register

You can [`read`](crate::Reg::read) this register and get [`agt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agt`] module*/
pub type AGT = crate::Reg<agt::AGT_SPEC>;
///AGT Counter Register
pub mod agt;
/**AGTCMA (rw) register accessor: AGT Compare Match A Register

You can [`read`](crate::Reg::read) this register and get [`agtcma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcma`] module*/
pub type AGTCMA = crate::Reg<agtcma::AGTCMA_SPEC>;
///AGT Compare Match A Register
pub mod agtcma;
/**AGTCMB (rw) register accessor: AGT Compare Match B Register

You can [`read`](crate::Reg::read) this register and get [`agtcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcmb`] module*/
pub type AGTCMB = crate::Reg<agtcmb::AGTCMB_SPEC>;
///AGT Compare Match B Register
pub mod agtcmb;
/**AGTCR (rw) register accessor: AGT Control Register

You can [`read`](crate::Reg::read) this register and get [`agtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcr`] module*/
pub type AGTCR = crate::Reg<agtcr::AGTCR_SPEC>;
///AGT Control Register
pub mod agtcr;
/**AGTMR1 (rw) register accessor: AGT Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`agtmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtmr1`] module*/
pub type AGTMR1 = crate::Reg<agtmr1::AGTMR1_SPEC>;
///AGT Mode Register 1
pub mod agtmr1;
/**AGTMR2 (rw) register accessor: AGT Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`agtmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtmr2`] module*/
pub type AGTMR2 = crate::Reg<agtmr2::AGTMR2_SPEC>;
///AGT Mode Register 2
pub mod agtmr2;
/**AGTIOC (rw) register accessor: AGT I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`agtioc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtioc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtioc`] module*/
pub type AGTIOC = crate::Reg<agtioc::AGTIOC_SPEC>;
///AGT I/O Control Register
pub mod agtioc;
/**AGTISR (rw) register accessor: AGT Event Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtisr`] module*/
pub type AGTISR = crate::Reg<agtisr::AGTISR_SPEC>;
///AGT Event Pin Select Register
pub mod agtisr;
/**AGTCMSR (rw) register accessor: AGT Compare Match Function Select Register

You can [`read`](crate::Reg::read) this register and get [`agtcmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcmsr`] module*/
pub type AGTCMSR = crate::Reg<agtcmsr::AGTCMSR_SPEC>;
///AGT Compare Match Function Select Register
pub mod agtcmsr;
/**AGTIOSEL (rw) register accessor: AGT Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtiosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtiosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtiosel`] module*/
pub type AGTIOSEL = crate::Reg<agtiosel::AGTIOSEL_SPEC>;
///AGT Pin Select Register
pub mod agtiosel;
