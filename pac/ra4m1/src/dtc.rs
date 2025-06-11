#[repr(C)]
///Register block
pub struct RegisterBlock {
    dtccr: DTCCR,
    _reserved1: [u8; 0x03],
    dtcvbr: DTCVBR,
    _reserved2: [u8; 0x04],
    dtcst: DTCST,
    _reserved3: [u8; 0x01],
    dtcsts: DTCSTS,
}
impl RegisterBlock {
    ///0x00 - DTC Control Register
    #[inline(always)]
    pub const fn dtccr(&self) -> &DTCCR {
        &self.dtccr
    }
    ///0x04 - DTC Vector Base Register
    #[inline(always)]
    pub const fn dtcvbr(&self) -> &DTCVBR {
        &self.dtcvbr
    }
    ///0x0c - DTC Module Start Register
    #[inline(always)]
    pub const fn dtcst(&self) -> &DTCST {
        &self.dtcst
    }
    ///0x0e - DTC Status Register
    #[inline(always)]
    pub const fn dtcsts(&self) -> &DTCSTS {
        &self.dtcsts
    }
}
/**DTCCR (rw) register accessor: DTC Control Register

You can [`read`](crate::Reg::read) this register and get [`dtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtccr`] module*/
pub type DTCCR = crate::Reg<dtccr::DTCCR_SPEC>;
///DTC Control Register
pub mod dtccr;
/**DTCVBR (rw) register accessor: DTC Vector Base Register

You can [`read`](crate::Reg::read) this register and get [`dtcvbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcvbr`] module*/
pub type DTCVBR = crate::Reg<dtcvbr::DTCVBR_SPEC>;
///DTC Vector Base Register
pub mod dtcvbr;
/**DTCST (rw) register accessor: DTC Module Start Register

You can [`read`](crate::Reg::read) this register and get [`dtcst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcst`] module*/
pub type DTCST = crate::Reg<dtcst::DTCST_SPEC>;
///DTC Module Start Register
pub mod dtcst;
/**DTCSTS (r) register accessor: DTC Status Register

You can [`read`](crate::Reg::read) this register and get [`dtcsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcsts`] module*/
pub type DTCSTS = crate::Reg<dtcsts::DTCSTS_SPEC>;
///DTC Status Register
pub mod dtcsts;
