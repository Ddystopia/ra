#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved_0_pdr: [u8; 0x04],
    _reserved_1_eidr: [u8; 0x04],
    _reserved_2_porr: [u8; 0x04],
    _reserved_3_eorr: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - Output data register
    #[inline(always)]
    pub const fn podr(&self) -> &PODR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Port Control Register 1
    #[inline(always)]
    pub const fn pcntr1(&self) -> &PCNTR1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x02 - Data direction register
    #[inline(always)]
    pub const fn pdr(&self) -> &PDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x04 - Event input data register
    #[inline(always)]
    pub const fn eidr(&self) -> &EIDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Port Control Register 2
    #[inline(always)]
    pub const fn pcntr2(&self) -> &PCNTR2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - Input data register
    #[inline(always)]
    pub const fn pidr(&self) -> &PIDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x08 - Output set register
    #[inline(always)]
    pub const fn porr(&self) -> &PORR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - Port Control Register 3
    #[inline(always)]
    pub const fn pcntr3(&self) -> &PCNTR3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0a - Output reset register
    #[inline(always)]
    pub const fn posr(&self) -> &POSR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
    ///0x0c - Event output set register
    #[inline(always)]
    pub const fn eorr(&self) -> &EORR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0c - Port Control Register 4
    #[inline(always)]
    pub const fn pcntr4(&self) -> &PCNTR4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    ///0x0e - Event output reset register
    #[inline(always)]
    pub const fn eosr(&self) -> &EOSR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
}
/**PCNTR1 (rw) register accessor: Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pcntr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr1`] module*/
pub type PCNTR1 = crate::Reg<pcntr1::PCNTR1_SPEC>;
///Port Control Register 1
pub mod pcntr1;
/**PODR (rw) register accessor: Output data register

You can [`read`](crate::Reg::read) this register and get [`podr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@podr`] module*/
pub type PODR = crate::Reg<podr::PODR_SPEC>;
///Output data register
pub mod podr;
/**PDR (rw) register accessor: Data direction register

You can [`read`](crate::Reg::read) this register and get [`pdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pdr`] module*/
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
///Data direction register
pub mod pdr;
/**PCNTR2 (r) register accessor: Port Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pcntr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr2`] module*/
pub type PCNTR2 = crate::Reg<pcntr2::PCNTR2_SPEC>;
///Port Control Register 2
pub mod pcntr2;
/**EIDR (r) register accessor: Event input data register

You can [`read`](crate::Reg::read) this register and get [`eidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eidr`] module*/
pub type EIDR = crate::Reg<eidr::EIDR_SPEC>;
///Event input data register
pub mod eidr;
/**PIDR (r) register accessor: Input data register

You can [`read`](crate::Reg::read) this register and get [`pidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pidr`] module*/
pub type PIDR = crate::Reg<pidr::PIDR_SPEC>;
///Input data register
pub mod pidr;
/**PCNTR3 (w) register accessor: Port Control Register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr3`] module*/
pub type PCNTR3 = crate::Reg<pcntr3::PCNTR3_SPEC>;
///Port Control Register 3
pub mod pcntr3;
/**PORR (w) register accessor: Output set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@porr`] module*/
pub type PORR = crate::Reg<porr::PORR_SPEC>;
///Output set register
pub mod porr;
/**POSR (w) register accessor: Output reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@posr`] module*/
pub type POSR = crate::Reg<posr::POSR_SPEC>;
///Output reset register
pub mod posr;
/**PCNTR4 (rw) register accessor: Port Control Register 4

You can [`read`](crate::Reg::read) this register and get [`pcntr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr4`] module*/
pub type PCNTR4 = crate::Reg<pcntr4::PCNTR4_SPEC>;
///Port Control Register 4
pub mod pcntr4;
/**EORR (rw) register accessor: Event output set register

You can [`read`](crate::Reg::read) this register and get [`eorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eorr`] module*/
pub type EORR = crate::Reg<eorr::EORR_SPEC>;
///Event output set register
pub mod eorr;
/**EOSR (rw) register accessor: Event output reset register

You can [`read`](crate::Reg::read) this register and get [`eosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eosr`] module*/
pub type EOSR = crate::Reg<eosr::EOSR_SPEC>;
///Event output reset register
pub mod eosr;
