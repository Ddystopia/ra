#[repr(C)]
///Register block
pub struct RegisterBlock {
    spcr: SPCR,
    sslp: SSLP,
    sppcr: SPPCR,
    spsr: SPSR,
    _reserved_4_spdr: [u8; 0x04],
    _reserved5: [u8; 0x02],
    spbr: SPBR,
    spdcr: SPDCR,
    spckd: SPCKD,
    sslnd: SSLND,
    spnd: SPND,
    spcr2: SPCR2,
    spcmd0: SPCMD0,
}
impl RegisterBlock {
    ///0x00 - SPI Control Register
    #[inline(always)]
    pub const fn spcr(&self) -> &SPCR {
        &self.spcr
    }
    ///0x01 - SPI Slave Select Polarity Register
    #[inline(always)]
    pub const fn sslp(&self) -> &SSLP {
        &self.sslp
    }
    ///0x02 - SPI Pin Control Register
    #[inline(always)]
    pub const fn sppcr(&self) -> &SPPCR {
        &self.sppcr
    }
    ///0x03 - SPI Status Register
    #[inline(always)]
    pub const fn spsr(&self) -> &SPSR {
        &self.spsr
    }
    ///0x04 - SPI Data Register ( halfword access )
    #[inline(always)]
    pub const fn spdr_ha(&self) -> &SPDR_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - SPI Data Register
    #[inline(always)]
    pub const fn spdr(&self) -> &SPDR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x0a - SPI Bit Rate Register
    #[inline(always)]
    pub const fn spbr(&self) -> &SPBR {
        &self.spbr
    }
    ///0x0b - SPI Data Control Register
    #[inline(always)]
    pub const fn spdcr(&self) -> &SPDCR {
        &self.spdcr
    }
    ///0x0c - SPI Clock Delay Register
    #[inline(always)]
    pub const fn spckd(&self) -> &SPCKD {
        &self.spckd
    }
    ///0x0d - SPI Slave Select Negation Delay Register
    #[inline(always)]
    pub const fn sslnd(&self) -> &SSLND {
        &self.sslnd
    }
    ///0x0e - SPI Next-Access Delay Register
    #[inline(always)]
    pub const fn spnd(&self) -> &SPND {
        &self.spnd
    }
    ///0x0f - SPI Control Register 2
    #[inline(always)]
    pub const fn spcr2(&self) -> &SPCR2 {
        &self.spcr2
    }
    ///0x10 - SPI Command Register 0
    #[inline(always)]
    pub const fn spcmd0(&self) -> &SPCMD0 {
        &self.spcmd0
    }
}
/**SPCR (rw) register accessor: SPI Control Register

You can [`read`](crate::Reg::read) this register and get [`spcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcr`] module*/
pub type SPCR = crate::Reg<spcr::SPCR_SPEC>;
///SPI Control Register
pub mod spcr;
/**SSLP (rw) register accessor: SPI Slave Select Polarity Register

You can [`read`](crate::Reg::read) this register and get [`sslp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sslp`] module*/
pub type SSLP = crate::Reg<sslp::SSLP_SPEC>;
///SPI Slave Select Polarity Register
pub mod sslp;
/**SPPCR (rw) register accessor: SPI Pin Control Register

You can [`read`](crate::Reg::read) this register and get [`sppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sppcr`] module*/
pub type SPPCR = crate::Reg<sppcr::SPPCR_SPEC>;
///SPI Pin Control Register
pub mod sppcr;
/**SPSR (rw) register accessor: SPI Status Register

You can [`read`](crate::Reg::read) this register and get [`spsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spsr`] module*/
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
///SPI Status Register
pub mod spsr;
/**SPDR (rw) register accessor: SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdr`] module*/
pub type SPDR = crate::Reg<spdr::SPDR_SPEC>;
///SPI Data Register
pub mod spdr;
/**SPDR_HA (rw) register accessor: SPI Data Register ( halfword access )

You can [`read`](crate::Reg::read) this register and get [`spdr_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdr_ha`] module*/
pub type SPDR_HA = crate::Reg<spdr_ha::SPDR_HA_SPEC>;
///SPI Data Register ( halfword access )
pub mod spdr_ha;
/**SPBR (rw) register accessor: SPI Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`spbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spbr`] module*/
pub type SPBR = crate::Reg<spbr::SPBR_SPEC>;
///SPI Bit Rate Register
pub mod spbr;
/**SPDCR (rw) register accessor: SPI Data Control Register

You can [`read`](crate::Reg::read) this register and get [`spdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdcr`] module*/
pub type SPDCR = crate::Reg<spdcr::SPDCR_SPEC>;
///SPI Data Control Register
pub mod spdcr;
/**SPCKD (rw) register accessor: SPI Clock Delay Register

You can [`read`](crate::Reg::read) this register and get [`spckd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spckd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spckd`] module*/
pub type SPCKD = crate::Reg<spckd::SPCKD_SPEC>;
///SPI Clock Delay Register
pub mod spckd;
/**SSLND (rw) register accessor: SPI Slave Select Negation Delay Register

You can [`read`](crate::Reg::read) this register and get [`sslnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sslnd`] module*/
pub type SSLND = crate::Reg<sslnd::SSLND_SPEC>;
///SPI Slave Select Negation Delay Register
pub mod sslnd;
/**SPND (rw) register accessor: SPI Next-Access Delay Register

You can [`read`](crate::Reg::read) this register and get [`spnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spnd`] module*/
pub type SPND = crate::Reg<spnd::SPND_SPEC>;
///SPI Next-Access Delay Register
pub mod spnd;
/**SPCR2 (rw) register accessor: SPI Control Register 2

You can [`read`](crate::Reg::read) this register and get [`spcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcr2`] module*/
pub type SPCR2 = crate::Reg<spcr2::SPCR2_SPEC>;
///SPI Control Register 2
pub mod spcr2;
/**SPCMD0 (rw) register accessor: SPI Command Register 0

You can [`read`](crate::Reg::read) this register and get [`spcmd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcmd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcmd0`] module*/
pub type SPCMD0 = crate::Reg<spcmd0::SPCMD0_SPEC>;
///SPI Command Register 0
pub mod spcmd0;
