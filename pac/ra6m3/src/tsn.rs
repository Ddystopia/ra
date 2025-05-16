#[repr(C)]
///Register block
pub struct RegisterBlock {
    tscr: TSCR,
}
impl RegisterBlock {
    ///0x00 - Temperature Sensor Control Register
    #[inline(always)]
    pub const fn tscr(&self) -> &TSCR {
        &self.tscr
    }
}
/**TSCR (rw) register accessor: Temperature Sensor Control Register

You can [`read`](crate::Reg::read) this register and get [`tscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tscr`] module*/
pub type TSCR = crate::Reg<tscr::TSCR_SPEC>;
///Temperature Sensor Control Register
pub mod tscr;
