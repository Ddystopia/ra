#[repr(C)]
///Register block
pub struct RegisterBlock {
    parioad: PARIOAD,
    _reserved1: [u8; 0x03],
    sramprcr: SRAMPRCR,
    _reserved2: [u8; 0x03],
    sramwtsc: SRAMWTSC,
    _reserved3: [u8; 0xb7],
    eccmode: ECCMODE,
    ecc2sts: ECC2STS,
    ecc1stsen: ECC1STSEN,
    ecc1sts: ECC1STS,
    eccprcr: ECCPRCR,
    _reserved8: [u8; 0x0f],
    eccetst: ECCETST,
    _reserved9: [u8; 0x03],
    eccoad: ECCOAD,
}
impl RegisterBlock {
    ///0x00 - SRAM Parity Error Operation After Detection Register
    #[inline(always)]
    pub const fn parioad(&self) -> &PARIOAD {
        &self.parioad
    }
    ///0x04 - SRAM Protection Register
    #[inline(always)]
    pub const fn sramprcr(&self) -> &SRAMPRCR {
        &self.sramprcr
    }
    ///0x08 - RAM Wait State Control Register
    #[inline(always)]
    pub const fn sramwtsc(&self) -> &SRAMWTSC {
        &self.sramwtsc
    }
    ///0xc0 - ECCRAM Operating Mode Control Register
    #[inline(always)]
    pub const fn eccmode(&self) -> &ECCMODE {
        &self.eccmode
    }
    ///0xc1 - ECCRAM 2-Bit Error Status Register
    #[inline(always)]
    pub const fn ecc2sts(&self) -> &ECC2STS {
        &self.ecc2sts
    }
    ///0xc2 - ECCRAM 1-Bit Error Information Update Enable Register
    #[inline(always)]
    pub const fn ecc1stsen(&self) -> &ECC1STSEN {
        &self.ecc1stsen
    }
    ///0xc3 - ECCRAM 1-Bit Error Status Register
    #[inline(always)]
    pub const fn ecc1sts(&self) -> &ECC1STS {
        &self.ecc1sts
    }
    ///0xc4 - ECCRAM Protection Register
    #[inline(always)]
    pub const fn eccprcr(&self) -> &ECCPRCR {
        &self.eccprcr
    }
    ///0xd4 - ECC Test Control Register
    #[inline(always)]
    pub const fn eccetst(&self) -> &ECCETST {
        &self.eccetst
    }
    ///0xd8 - RAM ECC Error Operation After Detection Register
    #[inline(always)]
    pub const fn eccoad(&self) -> &ECCOAD {
        &self.eccoad
    }
}
/**PARIOAD (rw) register accessor: SRAM Parity Error Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`parioad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`parioad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@parioad`] module*/
pub type PARIOAD = crate::Reg<parioad::PARIOAD_SPEC>;
///SRAM Parity Error Operation After Detection Register
pub mod parioad;
/**SRAMPRCR (rw) register accessor: SRAM Protection Register

You can [`read`](crate::Reg::read) this register and get [`sramprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sramprcr`] module*/
pub type SRAMPRCR = crate::Reg<sramprcr::SRAMPRCR_SPEC>;
///SRAM Protection Register
pub mod sramprcr;
/**SRAMWTSC (rw) register accessor: RAM Wait State Control Register

You can [`read`](crate::Reg::read) this register and get [`sramwtsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramwtsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sramwtsc`] module*/
pub type SRAMWTSC = crate::Reg<sramwtsc::SRAMWTSC_SPEC>;
///RAM Wait State Control Register
pub mod sramwtsc;
/**ECCMODE (rw) register accessor: ECCRAM Operating Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`eccmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eccmode`] module*/
pub type ECCMODE = crate::Reg<eccmode::ECCMODE_SPEC>;
///ECCRAM Operating Mode Control Register
pub mod eccmode;
/**ECC2STS (rw) register accessor: ECCRAM 2-Bit Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ecc2sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc2sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc2sts`] module*/
pub type ECC2STS = crate::Reg<ecc2sts::ECC2STS_SPEC>;
///ECCRAM 2-Bit Error Status Register
pub mod ecc2sts;
/**ECC1STSEN (rw) register accessor: ECCRAM 1-Bit Error Information Update Enable Register

You can [`read`](crate::Reg::read) this register and get [`ecc1stsen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1stsen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc1stsen`] module*/
pub type ECC1STSEN = crate::Reg<ecc1stsen::ECC1STSEN_SPEC>;
///ECCRAM 1-Bit Error Information Update Enable Register
pub mod ecc1stsen;
/**ECC1STS (rw) register accessor: ECCRAM 1-Bit Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ecc1sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc1sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc1sts`] module*/
pub type ECC1STS = crate::Reg<ecc1sts::ECC1STS_SPEC>;
///ECCRAM 1-Bit Error Status Register
pub mod ecc1sts;
/**ECCPRCR (rw) register accessor: ECCRAM Protection Register

You can [`read`](crate::Reg::read) this register and get [`eccprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eccprcr`] module*/
pub type ECCPRCR = crate::Reg<eccprcr::ECCPRCR_SPEC>;
///ECCRAM Protection Register
pub mod eccprcr;
/**ECCETST (rw) register accessor: ECC Test Control Register

You can [`read`](crate::Reg::read) this register and get [`eccetst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccetst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eccetst`] module*/
pub type ECCETST = crate::Reg<eccetst::ECCETST_SPEC>;
///ECC Test Control Register
pub mod eccetst;
/**ECCOAD (rw) register accessor: RAM ECC Error Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`eccoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eccoad`] module*/
pub type ECCOAD = crate::Reg<eccoad::ECCOAD_SPEC>;
///RAM ECC Error Operation After Detection Register
pub mod eccoad;
