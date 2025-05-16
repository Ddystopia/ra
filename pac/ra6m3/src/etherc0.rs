#[repr(C)]
///Register block
pub struct RegisterBlock {
    ecmr: ECMR,
    _reserved1: [u8; 0x04],
    rflr: RFLR,
    _reserved2: [u8; 0x04],
    ecsr: ECSR,
    _reserved3: [u8; 0x04],
    ecsipr: ECSIPR,
    _reserved4: [u8; 0x04],
    pir: PIR,
    _reserved5: [u8; 0x04],
    psr: PSR,
    _reserved6: [u8; 0x14],
    rdmlr: RDMLR,
    _reserved7: [u8; 0x0c],
    ipgr: IPGR,
    apr: APR,
    mpr: MPR,
    _reserved10: [u8; 0x04],
    rfcf: RFCF,
    tpauser: TPAUSER,
    tpausecr: TPAUSECR,
    bcfrr: BCFRR,
    _reserved14: [u8; 0x50],
    mahr: MAHR,
    _reserved15: [u8; 0x04],
    malr: MALR,
    _reserved16: [u8; 0x04],
    trocr: TROCR,
    cdcr: CDCR,
    lccr: LCCR,
    cndcr: CNDCR,
    _reserved20: [u8; 0x04],
    cefcr: CEFCR,
    frecr: FRECR,
    tsfrcr: TSFRCR,
    tlfrcr: TLFRCR,
    rfcr: RFCR,
    mafcr: MAFCR,
}
impl RegisterBlock {
    ///0x00 - ETHERC Mode Register
    #[inline(always)]
    pub const fn ecmr(&self) -> &ECMR {
        &self.ecmr
    }
    ///0x08 - Receive Frame Maximum Length Register
    #[inline(always)]
    pub const fn rflr(&self) -> &RFLR {
        &self.rflr
    }
    ///0x10 - ETHERC Status Register
    #[inline(always)]
    pub const fn ecsr(&self) -> &ECSR {
        &self.ecsr
    }
    ///0x18 - ETHERC Interrupt Enable Register
    #[inline(always)]
    pub const fn ecsipr(&self) -> &ECSIPR {
        &self.ecsipr
    }
    ///0x20 - PHY Interface Register
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    ///0x28 - PHY Status Register
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    ///0x40 - Random Number Generation Counter Upper Limit Setting Register
    #[inline(always)]
    pub const fn rdmlr(&self) -> &RDMLR {
        &self.rdmlr
    }
    ///0x50 - IPG Register
    #[inline(always)]
    pub const fn ipgr(&self) -> &IPGR {
        &self.ipgr
    }
    ///0x54 - Automatic PAUSE Frame Register
    #[inline(always)]
    pub const fn apr(&self) -> &APR {
        &self.apr
    }
    ///0x58 - Manual PAUSE Frame Register
    #[inline(always)]
    pub const fn mpr(&self) -> &MPR {
        &self.mpr
    }
    ///0x60 - Received PAUSE Frame Counter
    #[inline(always)]
    pub const fn rfcf(&self) -> &RFCF {
        &self.rfcf
    }
    ///0x64 - PAUSE Frame Retransmit Count Setting Register
    #[inline(always)]
    pub const fn tpauser(&self) -> &TPAUSER {
        &self.tpauser
    }
    ///0x68 - PAUSE Frame Retransmit Counter
    #[inline(always)]
    pub const fn tpausecr(&self) -> &TPAUSECR {
        &self.tpausecr
    }
    ///0x6c - Broadcast Frame Receive Count Setting Register
    #[inline(always)]
    pub const fn bcfrr(&self) -> &BCFRR {
        &self.bcfrr
    }
    ///0xc0 - MAC Address Upper Bit Register
    #[inline(always)]
    pub const fn mahr(&self) -> &MAHR {
        &self.mahr
    }
    ///0xc8 - MAC Address Lower Bit Register
    #[inline(always)]
    pub const fn malr(&self) -> &MALR {
        &self.malr
    }
    ///0xd0 - Transmit Retry Over Counter Register
    #[inline(always)]
    pub const fn trocr(&self) -> &TROCR {
        &self.trocr
    }
    ///0xd4 - Late Collision Detect Counter Register
    #[inline(always)]
    pub const fn cdcr(&self) -> &CDCR {
        &self.cdcr
    }
    ///0xd8 - Lost Carrier Counter Register
    #[inline(always)]
    pub const fn lccr(&self) -> &LCCR {
        &self.lccr
    }
    ///0xdc - Carrier Not Detect Counter Register
    #[inline(always)]
    pub const fn cndcr(&self) -> &CNDCR {
        &self.cndcr
    }
    ///0xe4 - CRC Error Frame Receive Counter Register
    #[inline(always)]
    pub const fn cefcr(&self) -> &CEFCR {
        &self.cefcr
    }
    ///0xe8 - Frame Receive Error Counter Register
    #[inline(always)]
    pub const fn frecr(&self) -> &FRECR {
        &self.frecr
    }
    ///0xec - Too-Short Frame Receive Counter Register
    #[inline(always)]
    pub const fn tsfrcr(&self) -> &TSFRCR {
        &self.tsfrcr
    }
    ///0xf0 - Too-Long Frame Receive Counter Register
    #[inline(always)]
    pub const fn tlfrcr(&self) -> &TLFRCR {
        &self.tlfrcr
    }
    ///0xf4 - Received Alignment Error Frame Counter Register
    #[inline(always)]
    pub const fn rfcr(&self) -> &RFCR {
        &self.rfcr
    }
    ///0xf8 - Multicast Address Frame Receive Counter Register
    #[inline(always)]
    pub const fn mafcr(&self) -> &MAFCR {
        &self.mafcr
    }
}
/**ECMR (rw) register accessor: ETHERC Mode Register

You can [`read`](crate::Reg::read) this register and get [`ecmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecmr`] module*/
pub type ECMR = crate::Reg<ecmr::ECMR_SPEC>;
///ETHERC Mode Register
pub mod ecmr;
/**RFLR (rw) register accessor: Receive Frame Maximum Length Register

You can [`read`](crate::Reg::read) this register and get [`rflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rflr`] module*/
pub type RFLR = crate::Reg<rflr::RFLR_SPEC>;
///Receive Frame Maximum Length Register
pub mod rflr;
/**ECSR (rw) register accessor: ETHERC Status Register

You can [`read`](crate::Reg::read) this register and get [`ecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecsr`] module*/
pub type ECSR = crate::Reg<ecsr::ECSR_SPEC>;
///ETHERC Status Register
pub mod ecsr;
/**ECSIPR (rw) register accessor: ETHERC Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ecsipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecsipr`] module*/
pub type ECSIPR = crate::Reg<ecsipr::ECSIPR_SPEC>;
///ETHERC Interrupt Enable Register
pub mod ecsipr;
/**PIR (rw) register accessor: PHY Interface Register

You can [`read`](crate::Reg::read) this register and get [`pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pir`] module*/
pub type PIR = crate::Reg<pir::PIR_SPEC>;
///PHY Interface Register
pub mod pir;
/**PSR (r) register accessor: PHY Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///PHY Status Register
pub mod psr;
/**RDMLR (rw) register accessor: Random Number Generation Counter Upper Limit Setting Register

You can [`read`](crate::Reg::read) this register and get [`rdmlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdmlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdmlr`] module*/
pub type RDMLR = crate::Reg<rdmlr::RDMLR_SPEC>;
///Random Number Generation Counter Upper Limit Setting Register
pub mod rdmlr;
/**IPGR (rw) register accessor: IPG Register

You can [`read`](crate::Reg::read) this register and get [`ipgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ipgr`] module*/
pub type IPGR = crate::Reg<ipgr::IPGR_SPEC>;
///IPG Register
pub mod ipgr;
/**APR (rw) register accessor: Automatic PAUSE Frame Register

You can [`read`](crate::Reg::read) this register and get [`apr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@apr`] module*/
pub type APR = crate::Reg<apr::APR_SPEC>;
///Automatic PAUSE Frame Register
pub mod apr;
/**MPR (w) register accessor: Manual PAUSE Frame Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mpr`] module*/
pub type MPR = crate::Reg<mpr::MPR_SPEC>;
///Manual PAUSE Frame Register
pub mod mpr;
/**RFCF (r) register accessor: Received PAUSE Frame Counter

You can [`read`](crate::Reg::read) this register and get [`rfcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfcf`] module*/
pub type RFCF = crate::Reg<rfcf::RFCF_SPEC>;
///Received PAUSE Frame Counter
pub mod rfcf;
/**TPAUSER (rw) register accessor: PAUSE Frame Retransmit Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`tpauser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpauser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tpauser`] module*/
pub type TPAUSER = crate::Reg<tpauser::TPAUSER_SPEC>;
///PAUSE Frame Retransmit Count Setting Register
pub mod tpauser;
/**TPAUSECR (r) register accessor: PAUSE Frame Retransmit Counter

You can [`read`](crate::Reg::read) this register and get [`tpausecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tpausecr`] module*/
pub type TPAUSECR = crate::Reg<tpausecr::TPAUSECR_SPEC>;
///PAUSE Frame Retransmit Counter
pub mod tpausecr;
/**BCFRR (rw) register accessor: Broadcast Frame Receive Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`bcfrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcfrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcfrr`] module*/
pub type BCFRR = crate::Reg<bcfrr::BCFRR_SPEC>;
///Broadcast Frame Receive Count Setting Register
pub mod bcfrr;
/**MAHR (rw) register accessor: MAC Address Upper Bit Register

You can [`read`](crate::Reg::read) this register and get [`mahr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mahr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mahr`] module*/
pub type MAHR = crate::Reg<mahr::MAHR_SPEC>;
///MAC Address Upper Bit Register
pub mod mahr;
/**MALR (rw) register accessor: MAC Address Lower Bit Register

You can [`read`](crate::Reg::read) this register and get [`malr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`malr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@malr`] module*/
pub type MALR = crate::Reg<malr::MALR_SPEC>;
///MAC Address Lower Bit Register
pub mod malr;
/**TROCR (rw) register accessor: Transmit Retry Over Counter Register

You can [`read`](crate::Reg::read) this register and get [`trocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trocr`] module*/
pub type TROCR = crate::Reg<trocr::TROCR_SPEC>;
///Transmit Retry Over Counter Register
pub mod trocr;
/**CDCR (rw) register accessor: Late Collision Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cdcr`] module*/
pub type CDCR = crate::Reg<cdcr::CDCR_SPEC>;
///Late Collision Detect Counter Register
pub mod cdcr;
/**LCCR (rw) register accessor: Lost Carrier Counter Register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lccr`] module*/
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
///Lost Carrier Counter Register
pub mod lccr;
/**CNDCR (rw) register accessor: Carrier Not Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cndcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cndcr`] module*/
pub type CNDCR = crate::Reg<cndcr::CNDCR_SPEC>;
///Carrier Not Detect Counter Register
pub mod cndcr;
/**CEFCR (rw) register accessor: CRC Error Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`cefcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cefcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cefcr`] module*/
pub type CEFCR = crate::Reg<cefcr::CEFCR_SPEC>;
///CRC Error Frame Receive Counter Register
pub mod cefcr;
/**FRECR (rw) register accessor: Frame Receive Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`frecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frecr`] module*/
pub type FRECR = crate::Reg<frecr::FRECR_SPEC>;
///Frame Receive Error Counter Register
pub mod frecr;
/**TSFRCR (rw) register accessor: Too-Short Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tsfrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsfrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsfrcr`] module*/
pub type TSFRCR = crate::Reg<tsfrcr::TSFRCR_SPEC>;
///Too-Short Frame Receive Counter Register
pub mod tsfrcr;
/**TLFRCR (rw) register accessor: Too-Long Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tlfrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlfrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tlfrcr`] module*/
pub type TLFRCR = crate::Reg<tlfrcr::TLFRCR_SPEC>;
///Too-Long Frame Receive Counter Register
pub mod tlfrcr;
/**RFCR (rw) register accessor: Received Alignment Error Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfcr`] module*/
pub type RFCR = crate::Reg<rfcr::RFCR_SPEC>;
///Received Alignment Error Frame Counter Register
pub mod rfcr;
/**MAFCR (rw) register accessor: Multicast Address Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`mafcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mafcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mafcr`] module*/
pub type MAFCR = crate::Reg<mafcr::MAFCR_SPEC>;
///Multicast Address Frame Receive Counter Register
pub mod mafcr;
