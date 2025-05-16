#[repr(C)]
///Register block
pub struct RegisterBlock {
    sfmsmd: SFMSMD,
    sfmssc: SFMSSC,
    sfmskc: SFMSKC,
    sfmsst: SFMSST,
    sfmcom: SFMCOM,
    sfmcmd: SFMCMD,
    sfmcst: SFMCST,
    _reserved7: [u8; 0x04],
    sfmsic: SFMSIC,
    sfmsac: SFMSAC,
    sfmsdc: SFMSDC,
    _reserved10: [u8; 0x04],
    sfmspc: SFMSPC,
    sfmpmd: SFMPMD,
    _reserved12: [u8; 0x07cc],
    sfmcnt1: SFMCNT1,
}
impl RegisterBlock {
    ///0x00 - Transfer Mode Control Register
    #[inline(always)]
    pub const fn sfmsmd(&self) -> &SFMSMD {
        &self.sfmsmd
    }
    ///0x04 - Chip Selection Control Register
    #[inline(always)]
    pub const fn sfmssc(&self) -> &SFMSSC {
        &self.sfmssc
    }
    ///0x08 - Clock Control Register
    #[inline(always)]
    pub const fn sfmskc(&self) -> &SFMSKC {
        &self.sfmskc
    }
    ///0x0c - Status Register
    #[inline(always)]
    pub const fn sfmsst(&self) -> &SFMSST {
        &self.sfmsst
    }
    ///0x10 - Communication Port Register
    #[inline(always)]
    pub const fn sfmcom(&self) -> &SFMCOM {
        &self.sfmcom
    }
    ///0x14 - Communication Mode Control Register
    #[inline(always)]
    pub const fn sfmcmd(&self) -> &SFMCMD {
        &self.sfmcmd
    }
    ///0x18 - Communication Status Register
    #[inline(always)]
    pub const fn sfmcst(&self) -> &SFMCST {
        &self.sfmcst
    }
    ///0x20 - Instruction Code Register
    #[inline(always)]
    pub const fn sfmsic(&self) -> &SFMSIC {
        &self.sfmsic
    }
    ///0x24 - Address Mode Control Register
    #[inline(always)]
    pub const fn sfmsac(&self) -> &SFMSAC {
        &self.sfmsac
    }
    ///0x28 - Dummy Cycle Control Register
    #[inline(always)]
    pub const fn sfmsdc(&self) -> &SFMSDC {
        &self.sfmsdc
    }
    ///0x30 - SPI Protocol Control Register
    #[inline(always)]
    pub const fn sfmspc(&self) -> &SFMSPC {
        &self.sfmspc
    }
    ///0x34 - Port Control Register
    #[inline(always)]
    pub const fn sfmpmd(&self) -> &SFMPMD {
        &self.sfmpmd
    }
    ///0x804 - External QSPI Address Register 1
    #[inline(always)]
    pub const fn sfmcnt1(&self) -> &SFMCNT1 {
        &self.sfmcnt1
    }
}
/**SFMSMD (rw) register accessor: Transfer Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsmd`] module*/
pub type SFMSMD = crate::Reg<sfmsmd::SFMSMD_SPEC>;
///Transfer Mode Control Register
pub mod sfmsmd;
/**SFMSSC (rw) register accessor: Chip Selection Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmssc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmssc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmssc`] module*/
pub type SFMSSC = crate::Reg<sfmssc::SFMSSC_SPEC>;
///Chip Selection Control Register
pub mod sfmssc;
/**SFMSKC (rw) register accessor: Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmskc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmskc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmskc`] module*/
pub type SFMSKC = crate::Reg<sfmskc::SFMSKC_SPEC>;
///Clock Control Register
pub mod sfmskc;
/**SFMSST (r) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmsst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsst`] module*/
pub type SFMSST = crate::Reg<sfmsst::SFMSST_SPEC>;
///Status Register
pub mod sfmsst;
/**SFMCOM (rw) register accessor: Communication Port Register

You can [`read`](crate::Reg::read) this register and get [`sfmcom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcom`] module*/
pub type SFMCOM = crate::Reg<sfmcom::SFMCOM_SPEC>;
///Communication Port Register
pub mod sfmcom;
/**SFMCMD (rw) register accessor: Communication Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcmd`] module*/
pub type SFMCMD = crate::Reg<sfmcmd::SFMCMD_SPEC>;
///Communication Mode Control Register
pub mod sfmcmd;
/**SFMCST (rw) register accessor: Communication Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmcst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcst`] module*/
pub type SFMCST = crate::Reg<sfmcst::SFMCST_SPEC>;
///Communication Status Register
pub mod sfmcst;
/**SFMSIC (rw) register accessor: Instruction Code Register

You can [`read`](crate::Reg::read) this register and get [`sfmsic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsic`] module*/
pub type SFMSIC = crate::Reg<sfmsic::SFMSIC_SPEC>;
///Instruction Code Register
pub mod sfmsic;
/**SFMSAC (rw) register accessor: Address Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsac`] module*/
pub type SFMSAC = crate::Reg<sfmsac::SFMSAC_SPEC>;
///Address Mode Control Register
pub mod sfmsac;
/**SFMSDC (rw) register accessor: Dummy Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsdc`] module*/
pub type SFMSDC = crate::Reg<sfmsdc::SFMSDC_SPEC>;
///Dummy Cycle Control Register
pub mod sfmsdc;
/**SFMSPC (rw) register accessor: SPI Protocol Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmspc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmspc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmspc`] module*/
pub type SFMSPC = crate::Reg<sfmspc::SFMSPC_SPEC>;
///SPI Protocol Control Register
pub mod sfmspc;
/**SFMPMD (rw) register accessor: Port Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmpmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmpmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmpmd`] module*/
pub type SFMPMD = crate::Reg<sfmpmd::SFMPMD_SPEC>;
///Port Control Register
pub mod sfmpmd;
/**SFMCNT1 (rw) register accessor: External QSPI Address Register 1

You can [`read`](crate::Reg::read) this register and get [`sfmcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcnt1`] module*/
pub type SFMCNT1 = crate::Reg<sfmcnt1::SFMCNT1_SPEC>;
///External QSPI Address Register 1
pub mod sfmcnt1;
