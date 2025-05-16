#[repr(C)]
///Register block
pub struct RegisterBlock {
    cacr0: CACR0,
    cacr1: CACR1,
    cacr2: CACR2,
    caicr: CAICR,
    castr: CASTR,
    _reserved5: [u8; 0x01],
    caulvr: CAULVR,
    callvr: CALLVR,
    cacntbr: CACNTBR,
}
impl RegisterBlock {
    ///0x00 - CAC Control Register 0
    #[inline(always)]
    pub const fn cacr0(&self) -> &CACR0 {
        &self.cacr0
    }
    ///0x01 - CAC Control Register 1
    #[inline(always)]
    pub const fn cacr1(&self) -> &CACR1 {
        &self.cacr1
    }
    ///0x02 - CAC Control Register 2
    #[inline(always)]
    pub const fn cacr2(&self) -> &CACR2 {
        &self.cacr2
    }
    ///0x03 - CAC Interrupt Control Register
    #[inline(always)]
    pub const fn caicr(&self) -> &CAICR {
        &self.caicr
    }
    ///0x04 - CAC Status Register
    #[inline(always)]
    pub const fn castr(&self) -> &CASTR {
        &self.castr
    }
    ///0x06 - CAC Upper-Limit Value Setting Register
    #[inline(always)]
    pub const fn caulvr(&self) -> &CAULVR {
        &self.caulvr
    }
    ///0x08 - CAC Lower-Limit Value Setting Register
    #[inline(always)]
    pub const fn callvr(&self) -> &CALLVR {
        &self.callvr
    }
    ///0x0a - CAC Counter Buffer Register
    #[inline(always)]
    pub const fn cacntbr(&self) -> &CACNTBR {
        &self.cacntbr
    }
}
/**CACR0 (rw) register accessor: CAC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cacr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr0`] module*/
pub type CACR0 = crate::Reg<cacr0::CACR0_SPEC>;
///CAC Control Register 0
pub mod cacr0;
/**CACR1 (rw) register accessor: CAC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cacr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr1`] module*/
pub type CACR1 = crate::Reg<cacr1::CACR1_SPEC>;
///CAC Control Register 1
pub mod cacr1;
/**CACR2 (rw) register accessor: CAC Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cacr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr2`] module*/
pub type CACR2 = crate::Reg<cacr2::CACR2_SPEC>;
///CAC Control Register 2
pub mod cacr2;
/**CAICR (rw) register accessor: CAC Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`caicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@caicr`] module*/
pub type CAICR = crate::Reg<caicr::CAICR_SPEC>;
///CAC Interrupt Control Register
pub mod caicr;
/**CASTR (r) register accessor: CAC Status Register

You can [`read`](crate::Reg::read) this register and get [`castr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@castr`] module*/
pub type CASTR = crate::Reg<castr::CASTR_SPEC>;
///CAC Status Register
pub mod castr;
/**CAULVR (rw) register accessor: CAC Upper-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`caulvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caulvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@caulvr`] module*/
pub type CAULVR = crate::Reg<caulvr::CAULVR_SPEC>;
///CAC Upper-Limit Value Setting Register
pub mod caulvr;
/**CALLVR (rw) register accessor: CAC Lower-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`callvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`callvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@callvr`] module*/
pub type CALLVR = crate::Reg<callvr::CALLVR_SPEC>;
///CAC Lower-Limit Value Setting Register
pub mod callvr;
/**CACNTBR (r) register accessor: CAC Counter Buffer Register

You can [`read`](crate::Reg::read) this register and get [`cacntbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacntbr`] module*/
pub type CACNTBR = crate::Reg<cacntbr::CACNTBR_SPEC>;
///CAC Counter Buffer Register
pub mod cacntbr;
