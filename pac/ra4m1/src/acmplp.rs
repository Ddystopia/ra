#[repr(C)]
///Register block
pub struct RegisterBlock {
    compmdr: COMPMDR,
    compfir: COMPFIR,
    compocr: COMPOCR,
    _reserved3: [u8; 0x01],
    compsel0: COMPSEL0,
    compsel1: COMPSEL1,
}
impl RegisterBlock {
    ///0x00 - ACMPLP Mode Setting Register
    #[inline(always)]
    pub const fn compmdr(&self) -> &COMPMDR {
        &self.compmdr
    }
    ///0x01 - ACMPLP Filter Control Register
    #[inline(always)]
    pub const fn compfir(&self) -> &COMPFIR {
        &self.compfir
    }
    ///0x02 - ACMPLP Output Control Register
    #[inline(always)]
    pub const fn compocr(&self) -> &COMPOCR {
        &self.compocr
    }
    ///0x04 - Comparator Input Select Register
    #[inline(always)]
    pub const fn compsel0(&self) -> &COMPSEL0 {
        &self.compsel0
    }
    ///0x05 - Comparator Reference Voltage Select Register
    #[inline(always)]
    pub const fn compsel1(&self) -> &COMPSEL1 {
        &self.compsel1
    }
}
/**COMPMDR (rw) register accessor: ACMPLP Mode Setting Register

You can [`read`](crate::Reg::read) this register and get [`compmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@compmdr`] module*/
pub type COMPMDR = crate::Reg<compmdr::COMPMDR_SPEC>;
///ACMPLP Mode Setting Register
pub mod compmdr;
/**COMPFIR (rw) register accessor: ACMPLP Filter Control Register

You can [`read`](crate::Reg::read) this register and get [`compfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@compfir`] module*/
pub type COMPFIR = crate::Reg<compfir::COMPFIR_SPEC>;
///ACMPLP Filter Control Register
pub mod compfir;
/**COMPOCR (rw) register accessor: ACMPLP Output Control Register

You can [`read`](crate::Reg::read) this register and get [`compocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@compocr`] module*/
pub type COMPOCR = crate::Reg<compocr::COMPOCR_SPEC>;
///ACMPLP Output Control Register
pub mod compocr;
/**COMPSEL0 (rw) register accessor: Comparator Input Select Register

You can [`read`](crate::Reg::read) this register and get [`compsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@compsel0`] module*/
pub type COMPSEL0 = crate::Reg<compsel0::COMPSEL0_SPEC>;
///Comparator Input Select Register
pub mod compsel0;
/**COMPSEL1 (rw) register accessor: Comparator Reference Voltage Select Register

You can [`read`](crate::Reg::read) this register and get [`compsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@compsel1`] module*/
pub type COMPSEL1 = crate::Reg<compsel1::COMPSEL1_SPEC>;
///Comparator Reference Voltage Select Register
pub mod compsel1;
