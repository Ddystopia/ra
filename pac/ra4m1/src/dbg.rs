#[repr(C)]
///Register block
pub struct RegisterBlock {
    dbgstr: DBGSTR,
    _reserved1: [u8; 0x0c],
    dbgstopcr: DBGSTOPCR,
    _reserved2: [u8; 0x0c],
    tracectr: TRACECTR,
}
impl RegisterBlock {
    ///0x00 - Debug Status Register
    #[inline(always)]
    pub const fn dbgstr(&self) -> &DBGSTR {
        &self.dbgstr
    }
    ///0x10 - Debug Stop Control Register
    #[inline(always)]
    pub const fn dbgstopcr(&self) -> &DBGSTOPCR {
        &self.dbgstopcr
    }
    ///0x20 - Trace Control Register
    #[inline(always)]
    pub const fn tracectr(&self) -> &TRACECTR {
        &self.tracectr
    }
}
/**DBGSTR (r) register accessor: Debug Status Register

You can [`read`](crate::Reg::read) this register and get [`dbgstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgstr`] module*/
pub type DBGSTR = crate::Reg<dbgstr::DBGSTR_SPEC>;
///Debug Status Register
pub mod dbgstr;
/**DBGSTOPCR (rw) register accessor: Debug Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgstopcr`] module*/
pub type DBGSTOPCR = crate::Reg<dbgstopcr::DBGSTOPCR_SPEC>;
///Debug Stop Control Register
pub mod dbgstopcr;
/**TRACECTR (rw) register accessor: Trace Control Register

You can [`read`](crate::Reg::read) this register and get [`tracectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tracectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tracectr`] module*/
pub type TRACECTR = crate::Reg<tracectr::TRACECTR_SPEC>;
///Trace Control Register
pub mod tracectr;
