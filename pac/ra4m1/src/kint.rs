#[repr(C)]
///Register block
pub struct RegisterBlock {
    krctl: KRCTL,
    _reserved1: [u8; 0x03],
    krf: KRF,
    _reserved2: [u8; 0x03],
    krm: KRM,
}
impl RegisterBlock {
    ///0x00 - KEY Return Control Register
    #[inline(always)]
    pub const fn krctl(&self) -> &KRCTL {
        &self.krctl
    }
    ///0x04 - KEY Return Flag Register
    #[inline(always)]
    pub const fn krf(&self) -> &KRF {
        &self.krf
    }
    ///0x08 - KEY Return Mode Register
    #[inline(always)]
    pub const fn krm(&self) -> &KRM {
        &self.krm
    }
}
/**KRCTL (rw) register accessor: KEY Return Control Register

You can [`read`](crate::Reg::read) this register and get [`krctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@krctl`] module*/
pub type KRCTL = crate::Reg<krctl::KRCTL_SPEC>;
///KEY Return Control Register
pub mod krctl;
/**KRF (rw) register accessor: KEY Return Flag Register

You can [`read`](crate::Reg::read) this register and get [`krf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

<div class="warning">The register is <b>modified</b> in some way after a read operation.</div>

For information about available fields see [`mod@krf`] module*/
pub type KRF = crate::Reg<krf::KRF_SPEC>;
///KEY Return Flag Register
pub mod krf;
/**KRM (rw) register accessor: KEY Return Mode Register

You can [`read`](crate::Reg::read) this register and get [`krm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@krm`] module*/
pub type KRM = crate::Reg<krm::KRM_SPEC>;
///KEY Return Mode Register
pub mod krm;
