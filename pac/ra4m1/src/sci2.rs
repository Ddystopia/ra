#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved_0_smr: [u8; 0x01],
    brr: BRR,
    _reserved_2_scr: [u8; 0x01],
    tdr: TDR,
    _reserved_4_ssr: [u8; 0x01],
    rdr: RDR,
    scmr: SCMR,
    semr: SEMR,
    snfr: SNFR,
    simr1: SIMR1,
    simr2: SIMR2,
    simr3: SIMR3,
    sisr: SISR,
    spmr: SPMR,
    tdrhl: TDRHL,
    rdrhl: RDRHL,
    mddr: MDDR,
    dccr: DCCR,
    _reserved18: [u8; 0x06],
    cdr: CDR,
    sptr: SPTR,
}
impl RegisterBlock {
    ///0x00 - Serial mode register (SCMR.SMIF = 1)
    #[inline(always)]
    pub const fn smr_smci(&self) -> &SMR_SMCI {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Serial Mode Register (SCMR.SMIF = 0)
    #[inline(always)]
    pub const fn smr(&self) -> &SMR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x01 - Bit Rate Register
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    ///0x02 - Serial Control Register (SCMR.SMIF =1)
    #[inline(always)]
    pub const fn scr_smci(&self) -> &SCR_SMCI {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x02 - Serial Control Register (SCMR.SMIF = 0)
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x03 - Transmit Data Register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x04 - Serial Status Register(SCMR.SMIF = 1)
    #[inline(always)]
    pub const fn ssr_smci(&self) -> &SSR_SMCI {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x05 - Receive Data Register
    #[inline(always)]
    pub const fn rdr(&self) -> &RDR {
        &self.rdr
    }
    ///0x06 - Smart Card Mode Register
    #[inline(always)]
    pub const fn scmr(&self) -> &SCMR {
        &self.scmr
    }
    ///0x07 - Serial Extended Mode Register
    #[inline(always)]
    pub const fn semr(&self) -> &SEMR {
        &self.semr
    }
    ///0x08 - Noise Filter Setting Register
    #[inline(always)]
    pub const fn snfr(&self) -> &SNFR {
        &self.snfr
    }
    ///0x09 - I2C Mode Register 1
    #[inline(always)]
    pub const fn simr1(&self) -> &SIMR1 {
        &self.simr1
    }
    ///0x0a - I2C Mode Register 2
    #[inline(always)]
    pub const fn simr2(&self) -> &SIMR2 {
        &self.simr2
    }
    ///0x0b - I2C Mode Register 3
    #[inline(always)]
    pub const fn simr3(&self) -> &SIMR3 {
        &self.simr3
    }
    ///0x0c - I2C Status Register
    #[inline(always)]
    pub const fn sisr(&self) -> &SISR {
        &self.sisr
    }
    ///0x0d - SPI Mode Register
    #[inline(always)]
    pub const fn spmr(&self) -> &SPMR {
        &self.spmr
    }
    ///0x0e - Transmit 9-bit Data Register
    #[inline(always)]
    pub const fn tdrhl(&self) -> &TDRHL {
        &self.tdrhl
    }
    ///0x10 - Receive 9-bit Data Register
    #[inline(always)]
    pub const fn rdrhl(&self) -> &RDRHL {
        &self.rdrhl
    }
    ///0x12 - Modulation Duty Register
    #[inline(always)]
    pub const fn mddr(&self) -> &MDDR {
        &self.mddr
    }
    ///0x13 - Data Compare Match Control Register
    #[inline(always)]
    pub const fn dccr(&self) -> &DCCR {
        &self.dccr
    }
    ///0x1a - Compare Match Data Register
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        &self.cdr
    }
    ///0x1c - Serial Port Register
    #[inline(always)]
    pub const fn sptr(&self) -> &SPTR {
        &self.sptr
    }
}
/**SMR (rw) register accessor: Serial Mode Register (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`smr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smr`] module*/
pub type SMR = crate::Reg<smr::SMR_SPEC>;
///Serial Mode Register (SCMR.SMIF = 0)
pub mod smr;
/**SMR_SMCI (rw) register accessor: Serial mode register (SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`smr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smr_smci`] module*/
pub type SMR_SMCI = crate::Reg<smr_smci::SMR_SMCI_SPEC>;
///Serial mode register (SCMR.SMIF = 1)
pub mod smr_smci;
/**BRR (rw) register accessor: Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brr`] module*/
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///Bit Rate Register
pub mod brr;
/**SCR (rw) register accessor: Serial Control Register (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scr`] module*/
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///Serial Control Register (SCMR.SMIF = 0)
pub mod scr;
/**SCR_SMCI (rw) register accessor: Serial Control Register (SCMR.SMIF =1)

You can [`read`](crate::Reg::read) this register and get [`scr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scr_smci`] module*/
pub type SCR_SMCI = crate::Reg<scr_smci::SCR_SMCI_SPEC>;
///Serial Control Register (SCMR.SMIF =1)
pub mod scr_smci;
/**TDR (rw) register accessor: Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///Transmit Data Register
pub mod tdr;
/**SSR (rw) register accessor: Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssr`] module*/
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///Serial Status Register(SCMR.SMIF = 0 and FCR.FM=0)
pub mod ssr;
/**SSR_SMCI (rw) register accessor: Serial Status Register(SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`ssr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssr_smci`] module*/
pub type SSR_SMCI = crate::Reg<ssr_smci::SSR_SMCI_SPEC>;
///Serial Status Register(SCMR.SMIF = 1)
pub mod ssr_smci;
/**RDR (r) register accessor: Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdr`] module*/
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///Receive Data Register
pub mod rdr;
/**SCMR (rw) register accessor: Smart Card Mode Register

You can [`read`](crate::Reg::read) this register and get [`scmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scmr`] module*/
pub type SCMR = crate::Reg<scmr::SCMR_SPEC>;
///Smart Card Mode Register
pub mod scmr;
/**SEMR (rw) register accessor: Serial Extended Mode Register

You can [`read`](crate::Reg::read) this register and get [`semr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@semr`] module*/
pub type SEMR = crate::Reg<semr::SEMR_SPEC>;
///Serial Extended Mode Register
pub mod semr;
/**SNFR (rw) register accessor: Noise Filter Setting Register

You can [`read`](crate::Reg::read) this register and get [`snfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snfr`] module*/
pub type SNFR = crate::Reg<snfr::SNFR_SPEC>;
///Noise Filter Setting Register
pub mod snfr;
/**SIMR1 (rw) register accessor: I2C Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`simr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@simr1`] module*/
pub type SIMR1 = crate::Reg<simr1::SIMR1_SPEC>;
///I2C Mode Register 1
pub mod simr1;
/**SIMR2 (rw) register accessor: I2C Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`simr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@simr2`] module*/
pub type SIMR2 = crate::Reg<simr2::SIMR2_SPEC>;
///I2C Mode Register 2
pub mod simr2;
/**SIMR3 (rw) register accessor: I2C Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`simr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@simr3`] module*/
pub type SIMR3 = crate::Reg<simr3::SIMR3_SPEC>;
///I2C Mode Register 3
pub mod simr3;
/**SISR (r) register accessor: I2C Status Register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sisr`] module*/
pub type SISR = crate::Reg<sisr::SISR_SPEC>;
///I2C Status Register
pub mod sisr;
/**SPMR (rw) register accessor: SPI Mode Register

You can [`read`](crate::Reg::read) this register and get [`spmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spmr`] module*/
pub type SPMR = crate::Reg<spmr::SPMR_SPEC>;
///SPI Mode Register
pub mod spmr;
/**TDRHL (rw) register accessor: Transmit 9-bit Data Register

You can [`read`](crate::Reg::read) this register and get [`tdrhl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdrhl`] module*/
pub type TDRHL = crate::Reg<tdrhl::TDRHL_SPEC>;
///Transmit 9-bit Data Register
pub mod tdrhl;
/**RDRHL (r) register accessor: Receive 9-bit Data Register

You can [`read`](crate::Reg::read) this register and get [`rdrhl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdrhl`] module*/
pub type RDRHL = crate::Reg<rdrhl::RDRHL_SPEC>;
///Receive 9-bit Data Register
pub mod rdrhl;
/**MDDR (rw) register accessor: Modulation Duty Register

You can [`read`](crate::Reg::read) this register and get [`mddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mddr`] module*/
pub type MDDR = crate::Reg<mddr::MDDR_SPEC>;
///Modulation Duty Register
pub mod mddr;
/**DCCR (rw) register accessor: Data Compare Match Control Register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dccr`] module*/
pub type DCCR = crate::Reg<dccr::DCCR_SPEC>;
///Data Compare Match Control Register
pub mod dccr;
/**CDR (rw) register accessor: Compare Match Data Register

You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cdr`] module*/
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
///Compare Match Data Register
pub mod cdr;
/**SPTR (rw) register accessor: Serial Port Register

You can [`read`](crate::Reg::read) this register and get [`sptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sptr`] module*/
pub type SPTR = crate::Reg<sptr::SPTR_SPEC>;
///Serial Port Register
pub mod sptr;
