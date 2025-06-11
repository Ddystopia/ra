#[repr(C)]
///Register block
pub struct RegisterBlock {
    opscr: OPSCR,
}
impl RegisterBlock {
    ///0x00 - Output Phase Switching Control Register
    #[inline(always)]
    pub const fn opscr(&self) -> &OPSCR {
        &self.opscr
    }
}
/**OPSCR (rw) register accessor: Output Phase Switching Control Register

You can [`read`](crate::Reg::read) this register and get [`opscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@opscr`] module*/
pub type OPSCR = crate::Reg<opscr::OPSCR_SPEC>;
///Output Phase Switching Control Register
pub mod opscr;
