#[repr(C)]
///Register block
pub struct RegisterBlock {
    ircr: IRCR,
}
impl RegisterBlock {
    ///0x00 - IrDA Control Register
    #[inline(always)]
    pub const fn ircr(&self) -> &IRCR {
        &self.ircr
    }
}
/**IRCR (rw) register accessor: IrDA Control Register

You can [`read`](crate::Reg::read) this register and get [`ircr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ircr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ircr`] module*/
pub type IRCR = crate::Reg<ircr::IRCR_SPEC>;
///IrDA Control Register
pub mod ircr;
