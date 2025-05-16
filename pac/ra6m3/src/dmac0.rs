#[repr(C)]
///Register block
pub struct RegisterBlock {
    dmsar: DMSAR,
    dmdar: DMDAR,
    dmcra: DMCRA,
    dmcrb: DMCRB,
    _reserved4: [u8; 0x02],
    dmtmd: DMTMD,
    _reserved5: [u8; 0x01],
    dmint: DMINT,
    dmamd: DMAMD,
    _reserved7: [u8; 0x02],
    dmofr: DMOFR,
    dmcnt: DMCNT,
    dmreq: DMREQ,
    dmsts: DMSTS,
}
impl RegisterBlock {
    ///0x00 - DMA Source Address Register
    #[inline(always)]
    pub const fn dmsar(&self) -> &DMSAR {
        &self.dmsar
    }
    ///0x04 - DMA Destination Address Register
    #[inline(always)]
    pub const fn dmdar(&self) -> &DMDAR {
        &self.dmdar
    }
    ///0x08 - DMA Transfer Count Register
    #[inline(always)]
    pub const fn dmcra(&self) -> &DMCRA {
        &self.dmcra
    }
    ///0x0c - DMA Block Transfer Count Register
    #[inline(always)]
    pub const fn dmcrb(&self) -> &DMCRB {
        &self.dmcrb
    }
    ///0x10 - DMA Transfer Mode Register
    #[inline(always)]
    pub const fn dmtmd(&self) -> &DMTMD {
        &self.dmtmd
    }
    ///0x13 - DMA Interrupt Setting Register
    #[inline(always)]
    pub const fn dmint(&self) -> &DMINT {
        &self.dmint
    }
    ///0x14 - DMA Address Mode Register
    #[inline(always)]
    pub const fn dmamd(&self) -> &DMAMD {
        &self.dmamd
    }
    ///0x18 - DMA Offset Register
    #[inline(always)]
    pub const fn dmofr(&self) -> &DMOFR {
        &self.dmofr
    }
    ///0x1c - DMA Transfer Enable Register
    #[inline(always)]
    pub const fn dmcnt(&self) -> &DMCNT {
        &self.dmcnt
    }
    ///0x1d - DMA Software Start Register
    #[inline(always)]
    pub const fn dmreq(&self) -> &DMREQ {
        &self.dmreq
    }
    ///0x1e - DMAC Module Activation Register
    #[inline(always)]
    pub const fn dmsts(&self) -> &DMSTS {
        &self.dmsts
    }
}
/**DMSAR (rw) register accessor: DMA Source Address Register

You can [`read`](crate::Reg::read) this register and get [`dmsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmsar`] module*/
pub type DMSAR = crate::Reg<dmsar::DMSAR_SPEC>;
///DMA Source Address Register
pub mod dmsar;
/**DMDAR (rw) register accessor: DMA Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`dmdar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmdar`] module*/
pub type DMDAR = crate::Reg<dmdar::DMDAR_SPEC>;
///DMA Destination Address Register
pub mod dmdar;
/**DMCRA (rw) register accessor: DMA Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmcra`] module*/
pub type DMCRA = crate::Reg<dmcra::DMCRA_SPEC>;
///DMA Transfer Count Register
pub mod dmcra;
/**DMCRB (rw) register accessor: DMA Block Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmcrb`] module*/
pub type DMCRB = crate::Reg<dmcrb::DMCRB_SPEC>;
///DMA Block Transfer Count Register
pub mod dmcrb;
/**DMTMD (rw) register accessor: DMA Transfer Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmtmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmtmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmtmd`] module*/
pub type DMTMD = crate::Reg<dmtmd::DMTMD_SPEC>;
///DMA Transfer Mode Register
pub mod dmtmd;
/**DMINT (rw) register accessor: DMA Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`dmint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmint`] module*/
pub type DMINT = crate::Reg<dmint::DMINT_SPEC>;
///DMA Interrupt Setting Register
pub mod dmint;
/**DMAMD (rw) register accessor: DMA Address Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmamd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmamd`] module*/
pub type DMAMD = crate::Reg<dmamd::DMAMD_SPEC>;
///DMA Address Mode Register
pub mod dmamd;
/**DMOFR (rw) register accessor: DMA Offset Register

You can [`read`](crate::Reg::read) this register and get [`dmofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmofr`] module*/
pub type DMOFR = crate::Reg<dmofr::DMOFR_SPEC>;
///DMA Offset Register
pub mod dmofr;
/**DMCNT (rw) register accessor: DMA Transfer Enable Register

You can [`read`](crate::Reg::read) this register and get [`dmcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmcnt`] module*/
pub type DMCNT = crate::Reg<dmcnt::DMCNT_SPEC>;
///DMA Transfer Enable Register
pub mod dmcnt;
/**DMREQ (rw) register accessor: DMA Software Start Register

You can [`read`](crate::Reg::read) this register and get [`dmreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmreq`] module*/
pub type DMREQ = crate::Reg<dmreq::DMREQ_SPEC>;
///DMA Software Start Register
pub mod dmreq;
/**DMSTS (rw) register accessor: DMAC Module Activation Register

You can [`read`](crate::Reg::read) this register and get [`dmsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmsts`] module*/
pub type DMSTS = crate::Reg<dmsts::DMSTS_SPEC>;
///DMAC Module Activation Register
pub mod dmsts;
