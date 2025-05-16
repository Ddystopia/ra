#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    fcachee: FCACHEE,
    _reserved1: [u8; 0x02],
    fcacheiv: FCACHEIV,
    _reserved2: [u8; 0x16],
    flwt: FLWT,
}
impl RegisterBlock {
    ///0x100 - Flash Cache Enable Register
    #[inline(always)]
    pub const fn fcachee(&self) -> &FCACHEE {
        &self.fcachee
    }
    ///0x104 - Flash Cache Invalidate Register
    #[inline(always)]
    pub const fn fcacheiv(&self) -> &FCACHEIV {
        &self.fcacheiv
    }
    ///0x11c - Flash Wait Cycle Register
    #[inline(always)]
    pub const fn flwt(&self) -> &FLWT {
        &self.flwt
    }
}
/**FCACHEE (rw) register accessor: Flash Cache Enable Register

You can [`read`](crate::Reg::read) this register and get [`fcachee::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcachee::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcachee`] module*/
pub type FCACHEE = crate::Reg<fcachee::FCACHEE_SPEC>;
///Flash Cache Enable Register
pub mod fcachee;
/**FCACHEIV (rw) register accessor: Flash Cache Invalidate Register

You can [`read`](crate::Reg::read) this register and get [`fcacheiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcacheiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcacheiv`] module*/
pub type FCACHEIV = crate::Reg<fcacheiv::FCACHEIV_SPEC>;
///Flash Cache Invalidate Register
pub mod fcacheiv;
/**FLWT (rw) register accessor: Flash Wait Cycle Register

You can [`read`](crate::Reg::read) this register and get [`flwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flwt`] module*/
pub type FLWT = crate::Reg<flwt::FLWT_SPEC>;
///Flash Wait Cycle Register
pub mod flwt;
