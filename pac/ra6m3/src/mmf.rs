#[repr(C)]
///Register block
pub struct RegisterBlock {
    mmsfr: MMSFR,
    mmen: MMEN,
}
impl RegisterBlock {
    ///0x00 - MemMirror Special Function Register
    #[inline(always)]
    pub const fn mmsfr(&self) -> &MMSFR {
        &self.mmsfr
    }
    ///0x04 - MemMirror Enable Register
    #[inline(always)]
    pub const fn mmen(&self) -> &MMEN {
        &self.mmen
    }
}
/**MMSFR (rw) register accessor: MemMirror Special Function Register

You can [`read`](crate::Reg::read) this register and get [`mmsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmsfr`] module*/
pub type MMSFR = crate::Reg<mmsfr::MMSFR_SPEC>;
///MemMirror Special Function Register
pub mod mmsfr;
/**MMEN (rw) register accessor: MemMirror Enable Register

You can [`read`](crate::Reg::read) this register and get [`mmen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmen`] module*/
pub type MMEN = crate::Reg<mmen::MMEN_SPEC>;
///MemMirror Enable Register
pub mod mmen;
