#[repr(C)]
///Register block
pub struct RegisterBlock {
    docr: DOCR,
    _reserved1: [u8; 0x01],
    dodir: DODIR,
    dodsr: DODSR,
}
impl RegisterBlock {
    ///0x00 - DOC Control Register
    #[inline(always)]
    pub const fn docr(&self) -> &DOCR {
        &self.docr
    }
    ///0x02 - DOC Data Input Register
    #[inline(always)]
    pub const fn dodir(&self) -> &DODIR {
        &self.dodir
    }
    ///0x04 - DOC Data Setting Register
    #[inline(always)]
    pub const fn dodsr(&self) -> &DODSR {
        &self.dodsr
    }
}
/**DOCR (rw) register accessor: DOC Control Register

You can [`read`](crate::Reg::read) this register and get [`docr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`docr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@docr`] module*/
pub type DOCR = crate::Reg<docr::DOCR_SPEC>;
///DOC Control Register
pub mod docr;
/**DODIR (rw) register accessor: DOC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`dodir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dodir`] module*/
pub type DODIR = crate::Reg<dodir::DODIR_SPEC>;
///DOC Data Input Register
pub mod dodir;
/**DODSR (rw) register accessor: DOC Data Setting Register

You can [`read`](crate::Reg::read) this register and get [`dodsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dodsr`] module*/
pub type DODSR = crate::Reg<dodsr::DODSR_SPEC>;
///DOC Data Setting Register
pub mod dodsr;
