#[repr(C)]
///Register block
pub struct RegisterBlock {
    mspmpuoad: MSPMPUOAD,
    _reserved1: [u8; 0x02],
    mspmpuctl: MSPMPUCTL,
    mspmpupt: MSPMPUPT,
    mspmpusa: MSPMPUSA,
    mspmpuea: MSPMPUEA,
    pspmpuoad: PSPMPUOAD,
    _reserved6: [u8; 0x02],
    pspmpuctl: PSPMPUCTL,
    pspmpupt: PSPMPUPT,
    pspmpusa: PSPMPUSA,
    pspmpuea: PSPMPUEA,
}
impl RegisterBlock {
    ///0x00 - Stack Pointer Monitor Operation After Detection Register
    #[inline(always)]
    pub const fn mspmpuoad(&self) -> &MSPMPUOAD {
        &self.mspmpuoad
    }
    ///0x04 - Stack Pointer Monitor Access Control Register
    #[inline(always)]
    pub const fn mspmpuctl(&self) -> &MSPMPUCTL {
        &self.mspmpuctl
    }
    ///0x06 - Stack Pointer Monitor Protection Register
    #[inline(always)]
    pub const fn mspmpupt(&self) -> &MSPMPUPT {
        &self.mspmpupt
    }
    ///0x08 - Main Stack Pointer Monitor Start Address Register
    #[inline(always)]
    pub const fn mspmpusa(&self) -> &MSPMPUSA {
        &self.mspmpusa
    }
    ///0x0c - Main Stack Pointer Monitor End Address Register
    #[inline(always)]
    pub const fn mspmpuea(&self) -> &MSPMPUEA {
        &self.mspmpuea
    }
    ///0x10 - Stack Pointer Monitor Operation After Detection Register
    #[inline(always)]
    pub const fn pspmpuoad(&self) -> &PSPMPUOAD {
        &self.pspmpuoad
    }
    ///0x14 - Stack Pointer Monitor Access Control Register
    #[inline(always)]
    pub const fn pspmpuctl(&self) -> &PSPMPUCTL {
        &self.pspmpuctl
    }
    ///0x16 - Stack Pointer Monitor Protection Register
    #[inline(always)]
    pub const fn pspmpupt(&self) -> &PSPMPUPT {
        &self.pspmpupt
    }
    ///0x18 - Process Stack Pointer Monitor Start Address Register
    #[inline(always)]
    pub const fn pspmpusa(&self) -> &PSPMPUSA {
        &self.pspmpusa
    }
    ///0x1c - Process Stack Pointer Monitor End Address Register
    #[inline(always)]
    pub const fn pspmpuea(&self) -> &PSPMPUEA {
        &self.pspmpuea
    }
}
/**MSPMPUOAD (rw) register accessor: Stack Pointer Monitor Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`mspmpuoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspmpuoad`] module*/
pub type MSPMPUOAD = crate::Reg<mspmpuoad::MSPMPUOAD_SPEC>;
///Stack Pointer Monitor Operation After Detection Register
pub mod mspmpuoad;
/**MSPMPUCTL (rw) register accessor: Stack Pointer Monitor Access Control Register

You can [`read`](crate::Reg::read) this register and get [`mspmpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspmpuctl`] module*/
pub type MSPMPUCTL = crate::Reg<mspmpuctl::MSPMPUCTL_SPEC>;
///Stack Pointer Monitor Access Control Register
pub mod mspmpuctl;
/**MSPMPUPT (rw) register accessor: Stack Pointer Monitor Protection Register

You can [`read`](crate::Reg::read) this register and get [`mspmpupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspmpupt`] module*/
pub type MSPMPUPT = crate::Reg<mspmpupt::MSPMPUPT_SPEC>;
///Stack Pointer Monitor Protection Register
pub mod mspmpupt;
/**MSPMPUSA (rw) register accessor: Main Stack Pointer Monitor Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mspmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspmpusa`] module*/
pub type MSPMPUSA = crate::Reg<mspmpusa::MSPMPUSA_SPEC>;
///Main Stack Pointer Monitor Start Address Register
pub mod mspmpusa;
/**MSPMPUEA (rw) register accessor: Main Stack Pointer Monitor End Address Register

You can [`read`](crate::Reg::read) this register and get [`mspmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspmpuea`] module*/
pub type MSPMPUEA = crate::Reg<mspmpuea::MSPMPUEA_SPEC>;
///Main Stack Pointer Monitor End Address Register
pub mod mspmpuea;
/**PSPMPUOAD (rw) register accessor: Stack Pointer Monitor Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`pspmpuoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pspmpuoad`] module*/
pub type PSPMPUOAD = crate::Reg<pspmpuoad::PSPMPUOAD_SPEC>;
///Stack Pointer Monitor Operation After Detection Register
pub mod pspmpuoad;
/**PSPMPUCTL (rw) register accessor: Stack Pointer Monitor Access Control Register

You can [`read`](crate::Reg::read) this register and get [`pspmpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pspmpuctl`] module*/
pub type PSPMPUCTL = crate::Reg<pspmpuctl::PSPMPUCTL_SPEC>;
///Stack Pointer Monitor Access Control Register
pub mod pspmpuctl;
/**PSPMPUPT (rw) register accessor: Stack Pointer Monitor Protection Register

You can [`read`](crate::Reg::read) this register and get [`pspmpupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pspmpupt`] module*/
pub type PSPMPUPT = crate::Reg<pspmpupt::PSPMPUPT_SPEC>;
///Stack Pointer Monitor Protection Register
pub mod pspmpupt;
/**PSPMPUSA (rw) register accessor: Process Stack Pointer Monitor Start Address Register

You can [`read`](crate::Reg::read) this register and get [`pspmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pspmpusa`] module*/
pub type PSPMPUSA = crate::Reg<pspmpusa::PSPMPUSA_SPEC>;
///Process Stack Pointer Monitor Start Address Register
pub mod pspmpusa;
/**PSPMPUEA (rw) register accessor: Process Stack Pointer Monitor End Address Register

You can [`read`](crate::Reg::read) this register and get [`pspmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pspmpuea`] module*/
pub type PSPMPUEA = crate::Reg<pspmpuea::PSPMPUEA_SPEC>;
///Process Stack Pointer Monitor End Address Register
pub mod pspmpuea;
