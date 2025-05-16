#[repr(C)]
///Register block
pub struct RegisterBlock {
    ctsucr0: CTSUCR0,
    ctsucr1: CTSUCR1,
    ctsusdprs: CTSUSDPRS,
    ctsusst: CTSUSST,
    ctsumch0: CTSUMCH0,
    ctsumch1: CTSUMCH1,
    ctsuchac0: CTSUCHAC0,
    ctsuchac1: CTSUCHAC1,
    ctsuchac2: CTSUCHAC2,
    _reserved9: [u8; 0x02],
    ctsuchtrc0: CTSUCHTRC0,
    ctsuchtrc1: CTSUCHTRC1,
    ctsuchtrc2: CTSUCHTRC2,
    _reserved12: [u8; 0x02],
    ctsudclkc: CTSUDCLKC,
    ctsust: CTSUST,
    ctsussc: CTSUSSC,
    ctsuso0: CTSUSO0,
    ctsuso1: CTSUSO1,
    ctsusc: CTSUSC,
    ctsurc: CTSURC,
    ctsuerrs: CTSUERRS,
}
impl RegisterBlock {
    ///0x00 - CTSU Control Register 0
    #[inline(always)]
    pub const fn ctsucr0(&self) -> &CTSUCR0 {
        &self.ctsucr0
    }
    ///0x01 - CTSU Control Register 1
    #[inline(always)]
    pub const fn ctsucr1(&self) -> &CTSUCR1 {
        &self.ctsucr1
    }
    ///0x02 - CTSU Synchronous Noise Reduction Setting Register
    #[inline(always)]
    pub const fn ctsusdprs(&self) -> &CTSUSDPRS {
        &self.ctsusdprs
    }
    ///0x03 - CTSU Sensor Stabilization Wait Control Register
    #[inline(always)]
    pub const fn ctsusst(&self) -> &CTSUSST {
        &self.ctsusst
    }
    ///0x04 - CTSU Measurement Channel Register 0
    #[inline(always)]
    pub const fn ctsumch0(&self) -> &CTSUMCH0 {
        &self.ctsumch0
    }
    ///0x05 - CTSU Measurement Channel Register 1
    #[inline(always)]
    pub const fn ctsumch1(&self) -> &CTSUMCH1 {
        &self.ctsumch1
    }
    ///0x06 - CTSU Channel Enable Control Register 0
    #[inline(always)]
    pub const fn ctsuchac0(&self) -> &CTSUCHAC0 {
        &self.ctsuchac0
    }
    ///0x07 - CTSU Channel Enable Control Register 1
    #[inline(always)]
    pub const fn ctsuchac1(&self) -> &CTSUCHAC1 {
        &self.ctsuchac1
    }
    ///0x08 - CTSU Channel Enable Control Register 2
    #[inline(always)]
    pub const fn ctsuchac2(&self) -> &CTSUCHAC2 {
        &self.ctsuchac2
    }
    ///0x0b - CTSU Channel Transmit/Receive Control Register 0
    #[inline(always)]
    pub const fn ctsuchtrc0(&self) -> &CTSUCHTRC0 {
        &self.ctsuchtrc0
    }
    ///0x0c - CTSU Channel Transmit/Receive Control Register 1
    #[inline(always)]
    pub const fn ctsuchtrc1(&self) -> &CTSUCHTRC1 {
        &self.ctsuchtrc1
    }
    ///0x0d - CTSU Channel Transmit/Receive Control Register 2
    #[inline(always)]
    pub const fn ctsuchtrc2(&self) -> &CTSUCHTRC2 {
        &self.ctsuchtrc2
    }
    ///0x10 - CTSU High-Pass Noise Reduction Control Register
    #[inline(always)]
    pub const fn ctsudclkc(&self) -> &CTSUDCLKC {
        &self.ctsudclkc
    }
    ///0x11 - CTSU Status Register
    #[inline(always)]
    pub const fn ctsust(&self) -> &CTSUST {
        &self.ctsust
    }
    ///0x12 - CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register
    #[inline(always)]
    pub const fn ctsussc(&self) -> &CTSUSSC {
        &self.ctsussc
    }
    ///0x14 - CTSU Sensor Offset Register 0
    #[inline(always)]
    pub const fn ctsuso0(&self) -> &CTSUSO0 {
        &self.ctsuso0
    }
    ///0x16 - CTSU Sensor Offset Register 1
    #[inline(always)]
    pub const fn ctsuso1(&self) -> &CTSUSO1 {
        &self.ctsuso1
    }
    ///0x18 - CTSU Sensor Counter
    #[inline(always)]
    pub const fn ctsusc(&self) -> &CTSUSC {
        &self.ctsusc
    }
    ///0x1a - CTSU Reference Counter
    #[inline(always)]
    pub const fn ctsurc(&self) -> &CTSURC {
        &self.ctsurc
    }
    ///0x1c - CTSU Error Status Register
    #[inline(always)]
    pub const fn ctsuerrs(&self) -> &CTSUERRS {
        &self.ctsuerrs
    }
}
/**CTSUCR0 (rw) register accessor: CTSU Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsucr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsucr0`] module*/
pub type CTSUCR0 = crate::Reg<ctsucr0::CTSUCR0_SPEC>;
///CTSU Control Register 0
pub mod ctsucr0;
/**CTSUCR1 (rw) register accessor: CTSU Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsucr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsucr1`] module*/
pub type CTSUCR1 = crate::Reg<ctsucr1::CTSUCR1_SPEC>;
///CTSU Control Register 1
pub mod ctsucr1;
/**CTSUSDPRS (rw) register accessor: CTSU Synchronous Noise Reduction Setting Register

You can [`read`](crate::Reg::read) this register and get [`ctsusdprs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusdprs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsusdprs`] module*/
pub type CTSUSDPRS = crate::Reg<ctsusdprs::CTSUSDPRS_SPEC>;
///CTSU Synchronous Noise Reduction Setting Register
pub mod ctsusdprs;
/**CTSUSST (rw) register accessor: CTSU Sensor Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsusst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsusst`] module*/
pub type CTSUSST = crate::Reg<ctsusst::CTSUSST_SPEC>;
///CTSU Sensor Stabilization Wait Control Register
pub mod ctsusst;
/**CTSUMCH0 (rw) register accessor: CTSU Measurement Channel Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsumch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsumch0`] module*/
pub type CTSUMCH0 = crate::Reg<ctsumch0::CTSUMCH0_SPEC>;
///CTSU Measurement Channel Register 0
pub mod ctsumch0;
/**CTSUMCH1 (r) register accessor: CTSU Measurement Channel Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsumch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsumch1`] module*/
pub type CTSUMCH1 = crate::Reg<ctsumch1::CTSUMCH1_SPEC>;
///CTSU Measurement Channel Register 1
pub mod ctsumch1;
/**CTSUCHAC0 (rw) register accessor: CTSU Channel Enable Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchac0`] module*/
pub type CTSUCHAC0 = crate::Reg<ctsuchac0::CTSUCHAC0_SPEC>;
///CTSU Channel Enable Control Register 0
pub mod ctsuchac0;
/**CTSUCHAC1 (rw) register accessor: CTSU Channel Enable Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchac1`] module*/
pub type CTSUCHAC1 = crate::Reg<ctsuchac1::CTSUCHAC1_SPEC>;
///CTSU Channel Enable Control Register 1
pub mod ctsuchac1;
/**CTSUCHAC2 (rw) register accessor: CTSU Channel Enable Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchac2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchac2`] module*/
pub type CTSUCHAC2 = crate::Reg<ctsuchac2::CTSUCHAC2_SPEC>;
///CTSU Channel Enable Control Register 2
pub mod ctsuchac2;
/**CTSUCHTRC0 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchtrc0`] module*/
pub type CTSUCHTRC0 = crate::Reg<ctsuchtrc0::CTSUCHTRC0_SPEC>;
///CTSU Channel Transmit/Receive Control Register 0
pub mod ctsuchtrc0;
/**CTSUCHTRC1 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchtrc1`] module*/
pub type CTSUCHTRC1 = crate::Reg<ctsuchtrc1::CTSUCHTRC1_SPEC>;
///CTSU Channel Transmit/Receive Control Register 1
pub mod ctsuchtrc1;
/**CTSUCHTRC2 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchtrc2`] module*/
pub type CTSUCHTRC2 = crate::Reg<ctsuchtrc2::CTSUCHTRC2_SPEC>;
///CTSU Channel Transmit/Receive Control Register 2
pub mod ctsuchtrc2;
/**CTSUDCLKC (rw) register accessor: CTSU High-Pass Noise Reduction Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsudclkc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsudclkc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsudclkc`] module*/
pub type CTSUDCLKC = crate::Reg<ctsudclkc::CTSUDCLKC_SPEC>;
///CTSU High-Pass Noise Reduction Control Register
pub mod ctsudclkc;
/**CTSUST (rw) register accessor: CTSU Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsust::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsust`] module*/
pub type CTSUST = crate::Reg<ctsust::CTSUST_SPEC>;
///CTSU Status Register
pub mod ctsust;
/**CTSUSSC (rw) register accessor: CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsussc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsussc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsussc`] module*/
pub type CTSUSSC = crate::Reg<ctsussc::CTSUSSC_SPEC>;
///CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register
pub mod ctsussc;
/**CTSUSO0 (rw) register accessor: CTSU Sensor Offset Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuso0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuso0`] module*/
pub type CTSUSO0 = crate::Reg<ctsuso0::CTSUSO0_SPEC>;
///CTSU Sensor Offset Register 0
pub mod ctsuso0;
/**CTSUSO1 (rw) register accessor: CTSU Sensor Offset Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuso1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuso1`] module*/
pub type CTSUSO1 = crate::Reg<ctsuso1::CTSUSO1_SPEC>;
///CTSU Sensor Offset Register 1
pub mod ctsuso1;
/**CTSUSC (r) register accessor: CTSU Sensor Counter

You can [`read`](crate::Reg::read) this register and get [`ctsusc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsusc`] module*/
pub type CTSUSC = crate::Reg<ctsusc::CTSUSC_SPEC>;
///CTSU Sensor Counter
pub mod ctsusc;
/**CTSURC (r) register accessor: CTSU Reference Counter

You can [`read`](crate::Reg::read) this register and get [`ctsurc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsurc`] module*/
pub type CTSURC = crate::Reg<ctsurc::CTSURC_SPEC>;
///CTSU Reference Counter
pub mod ctsurc;
/**CTSUERRS (r) register accessor: CTSU Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsuerrs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuerrs`] module*/
pub type CTSUERRS = crate::Reg<ctsuerrs::CTSUERRS_SPEC>;
///CTSU Error Status Register
pub mod ctsuerrs;
