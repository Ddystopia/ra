#[repr(C)]
///Register block
pub struct RegisterBlock {
    ptrstr: PTRSTR,
    stcselr: STCSELR,
    bypass: BYPASS,
}
impl RegisterBlock {
    ///0x00 - EPTPC Reset Register
    #[inline(always)]
    pub const fn ptrstr(&self) -> &PTRSTR {
        &self.ptrstr
    }
    ///0x04 - STCA Clock Select Register
    #[inline(always)]
    pub const fn stcselr(&self) -> &STCSELR {
        &self.stcselr
    }
    ///0x08 - Bypass 1588 module Register
    #[inline(always)]
    pub const fn bypass(&self) -> &BYPASS {
        &self.bypass
    }
}
/**PTRSTR (rw) register accessor: EPTPC Reset Register

You can [`read`](crate::Reg::read) this register and get [`ptrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ptrstr`] module*/
pub type PTRSTR = crate::Reg<ptrstr::PTRSTR_SPEC>;
///EPTPC Reset Register
pub mod ptrstr;
/**STCSELR (rw) register accessor: STCA Clock Select Register

You can [`read`](crate::Reg::read) this register and get [`stcselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stcselr`] module*/
pub type STCSELR = crate::Reg<stcselr::STCSELR_SPEC>;
///STCA Clock Select Register
pub mod stcselr;
/**BYPASS (rw) register accessor: Bypass 1588 module Register

You can [`read`](crate::Reg::read) this register and get [`bypass::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bypass::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bypass`] module*/
pub type BYPASS = crate::Reg<bypass::BYPASS_SPEC>;
///Bypass 1588 module Register
pub mod bypass;
