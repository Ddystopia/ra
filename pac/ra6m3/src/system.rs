#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    sbycr: SBYCR,
    _reserved1: [u8; 0x0e],
    mstpcra: MSTPCRA,
    sckdivcr: SCKDIVCR,
    sckdivcr2: SCKDIVCR2,
    _reserved4: [u8; 0x01],
    sckscr: SCKSCR,
    _reserved5: [u8; 0x01],
    pllccr: PLLCCR,
    pllcr: PLLCR,
    _reserved7: [u8; 0x05],
    bckcr: BCKCR,
    _reserved8: [u8; 0x01],
    mosccr: MOSCCR,
    _reserved9: [u8; 0x03],
    hococr: HOCOCR,
    _reserved10: [u8; 0x01],
    mococr: MOCOCR,
    fllcr1: FLLCR1,
    fllcr2: FLLCR2,
    oscsf: OSCSF,
    _reserved14: [u8; 0x01],
    ckocr: CKOCR,
    trckcr: TRCKCR,
    ostdcr: OSTDCR,
    ostdsr: OSTDSR,
    _reserved18: [u8; 0x10],
    ebckocr: EBCKOCR,
    sdckocr: SDCKOCR,
    _reserved20: [u8; 0x0d],
    mocoutcr: MOCOUTCR,
    hocoutcr: HOCOUTCR,
    _reserved22: [u8; 0x2f],
    snzcr: SNZCR,
    _reserved23: [u8; 0x01],
    snzedcr: SNZEDCR,
    _reserved24: [u8; 0x03],
    snzreqcr: SNZREQCR,
    _reserved25: [u8; 0x04],
    opccr: OPCCR,
    _reserved26: [u8; 0x01],
    moscwtcr: MOSCWTCR,
    _reserved27: [u8; 0x02],
    hocowtcr: HOCOWTCR,
    _reserved28: [u8; 0x04],
    sopccr: SOPCCR,
    _reserved29: [u8; 0x15],
    rstsr1: RSTSR1,
    _reserved30: [u8; 0x1e],
    lvdcr1: (),
    _reserved31: [u8; 0x01],
    lvdsr: (),
    _reserved32: [u8; 0x031d],
    prcr: PRCR,
    dpsbycr: DPSBYCR,
    _reserved34: [u8; 0x01],
    dpsier0: DPSIER0,
    dpsier1: DPSIER1,
    dpsier2: DPSIER2,
    dpsier3: DPSIER3,
    dpsifr0: DPSIFR0,
    dpsifr1: DPSIFR1,
    dpsifr2: DPSIFR2,
    dpsifr3: DPSIFR3,
    dpsiegr0: DPSIEGR0,
    dpsiegr1: DPSIEGR1,
    dpsiegr2: DPSIEGR2,
    _reserved45: [u8; 0x01],
    syocdcr: SYOCDCR,
    stconr: STCONR,
    rstsr0: RSTSR0,
    rstsr2: RSTSR2,
    _reserved49: [u8; 0x01],
    momcr: MOMCR,
    _reserved50: [u8; 0x02],
    fwepror: FWEPROR,
    lvcmpcr: LVCMPCR,
    lvdlvlr: LVDLVLR,
    _reserved53: [u8; 0x01],
    lvdcr0: [LVDCR0; 2],
    _reserved54: [u8; 0x64],
    sosccr: SOSCCR,
    somcr: SOMCR,
    _reserved56: [u8; 0x0e],
    lococr: LOCOCR,
    _reserved57: [u8; 0x01],
    locoutcr: LOCOUTCR,
    _reserved58: [u8; 0x28],
    vbtictlr: VBTICTLR,
    _reserved59: [u8; 0x44],
    vbtbkr: [VBTBKR; 512],
}
impl RegisterBlock {
    ///0x0c - Standby Control Register
    #[inline(always)]
    pub const fn sbycr(&self) -> &SBYCR {
        &self.sbycr
    }
    ///0x1c - Module Stop Control Register A
    #[inline(always)]
    pub const fn mstpcra(&self) -> &MSTPCRA {
        &self.mstpcra
    }
    ///0x20 - System Clock Division Control Register
    #[inline(always)]
    pub const fn sckdivcr(&self) -> &SCKDIVCR {
        &self.sckdivcr
    }
    ///0x24 - System Clock Division Control Register 2
    #[inline(always)]
    pub const fn sckdivcr2(&self) -> &SCKDIVCR2 {
        &self.sckdivcr2
    }
    ///0x26 - System Clock Source Control Register
    #[inline(always)]
    pub const fn sckscr(&self) -> &SCKSCR {
        &self.sckscr
    }
    ///0x28 - PLL Clock Control Register
    #[inline(always)]
    pub const fn pllccr(&self) -> &PLLCCR {
        &self.pllccr
    }
    ///0x2a - PLL Control Register
    #[inline(always)]
    pub const fn pllcr(&self) -> &PLLCR {
        &self.pllcr
    }
    ///0x30 - External Bus Clock Control Register
    #[inline(always)]
    pub const fn bckcr(&self) -> &BCKCR {
        &self.bckcr
    }
    ///0x32 - Main Clock Oscillator Control Register
    #[inline(always)]
    pub const fn mosccr(&self) -> &MOSCCR {
        &self.mosccr
    }
    ///0x36 - High-Speed On-Chip Oscillator Control Register
    #[inline(always)]
    pub const fn hococr(&self) -> &HOCOCR {
        &self.hococr
    }
    ///0x38 - Middle-Speed On-Chip Oscillator Control Register
    #[inline(always)]
    pub const fn mococr(&self) -> &MOCOCR {
        &self.mococr
    }
    ///0x39 - FLL Control Register 1
    #[inline(always)]
    pub const fn fllcr1(&self) -> &FLLCR1 {
        &self.fllcr1
    }
    ///0x3a - FLL Control Register 2
    #[inline(always)]
    pub const fn fllcr2(&self) -> &FLLCR2 {
        &self.fllcr2
    }
    ///0x3c - Oscillation Stabilization Flag Register
    #[inline(always)]
    pub const fn oscsf(&self) -> &OSCSF {
        &self.oscsf
    }
    ///0x3e - Clock Out Control Register
    #[inline(always)]
    pub const fn ckocr(&self) -> &CKOCR {
        &self.ckocr
    }
    ///0x3f - Trace Clock Control Register
    #[inline(always)]
    pub const fn trckcr(&self) -> &TRCKCR {
        &self.trckcr
    }
    ///0x40 - Oscillation Stop Detection Control Register
    #[inline(always)]
    pub const fn ostdcr(&self) -> &OSTDCR {
        &self.ostdcr
    }
    ///0x41 - Oscillation Stop Detection Status Register
    #[inline(always)]
    pub const fn ostdsr(&self) -> &OSTDSR {
        &self.ostdsr
    }
    ///0x52 - External Bus Clock Output Control Register
    #[inline(always)]
    pub const fn ebckocr(&self) -> &EBCKOCR {
        &self.ebckocr
    }
    ///0x53 - SDRAM Clock Output Control Register
    #[inline(always)]
    pub const fn sdckocr(&self) -> &SDCKOCR {
        &self.sdckocr
    }
    ///0x61 - MOCO User Trimming Control Register
    #[inline(always)]
    pub const fn mocoutcr(&self) -> &MOCOUTCR {
        &self.mocoutcr
    }
    ///0x62 - HOCO User Trimming Control Register
    #[inline(always)]
    pub const fn hocoutcr(&self) -> &HOCOUTCR {
        &self.hocoutcr
    }
    ///0x92 - Snooze Control Register
    #[inline(always)]
    pub const fn snzcr(&self) -> &SNZCR {
        &self.snzcr
    }
    ///0x94 - Snooze End Control Register
    #[inline(always)]
    pub const fn snzedcr(&self) -> &SNZEDCR {
        &self.snzedcr
    }
    ///0x98 - Snooze Request Control Register
    #[inline(always)]
    pub const fn snzreqcr(&self) -> &SNZREQCR {
        &self.snzreqcr
    }
    ///0xa0 - Operating Power Control Register
    #[inline(always)]
    pub const fn opccr(&self) -> &OPCCR {
        &self.opccr
    }
    ///0xa2 - Main Clock Oscillator Wait Control Register
    #[inline(always)]
    pub const fn moscwtcr(&self) -> &MOSCWTCR {
        &self.moscwtcr
    }
    ///0xa5 - High-speed on-chip oscillator wait control register
    #[inline(always)]
    pub const fn hocowtcr(&self) -> &HOCOWTCR {
        &self.hocowtcr
    }
    ///0xaa - Sub Operating Power Control Register
    #[inline(always)]
    pub const fn sopccr(&self) -> &SOPCCR {
        &self.sopccr
    }
    ///0xc0 - Reset Status Register 1
    #[inline(always)]
    pub const fn rstsr1(&self) -> &RSTSR1 {
        &self.rstsr1
    }
    ///0xe0 - Voltage Monitor %s Circuit Control Register 1
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `LVD1CR1` register.</div>
    #[inline(always)]
    pub const fn lvdcr1(&self, n: usize) -> &LVDCR1 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(224)
                .add(2 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0xe0 - Voltage Monitor %s Circuit Control Register 1
    #[inline(always)]
    pub fn lvdcr1_iter(&self) -> impl Iterator<Item = &LVDCR1> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(224)
                .add(2 * n)
                .cast()
        })
    }
    ///0xe0 - Voltage Monitor 1 Circuit Control Register 1
    #[inline(always)]
    pub const fn lvd1cr1(&self) -> &LVDCR1 {
        self.lvdcr1(0)
    }
    ///0xe2 - Voltage Monitor 2 Circuit Control Register 1
    #[inline(always)]
    pub const fn lvd2cr1(&self) -> &LVDCR1 {
        self.lvdcr1(1)
    }
    ///0xe1 - Voltage Monitor %s Circuit Status Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `LVD1SR` register.</div>
    #[inline(always)]
    pub const fn lvdsr(&self, n: usize) -> &LVDSR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(225)
                .add(2 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0xe1 - Voltage Monitor %s Circuit Status Register
    #[inline(always)]
    pub fn lvdsr_iter(&self) -> impl Iterator<Item = &LVDSR> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(225)
                .add(2 * n)
                .cast()
        })
    }
    ///0xe1 - Voltage Monitor 1 Circuit Status Register
    #[inline(always)]
    pub const fn lvd1sr(&self) -> &LVDSR {
        self.lvdsr(0)
    }
    ///0xe3 - Voltage Monitor 2 Circuit Status Register
    #[inline(always)]
    pub const fn lvd2sr(&self) -> &LVDSR {
        self.lvdsr(1)
    }
    ///0x3fe - Protect Register
    #[inline(always)]
    pub const fn prcr(&self) -> &PRCR {
        &self.prcr
    }
    ///0x400 - Deep Standby Control Register
    #[inline(always)]
    pub const fn dpsbycr(&self) -> &DPSBYCR {
        &self.dpsbycr
    }
    ///0x402 - Deep Standby Interrupt Enable Register 0
    #[inline(always)]
    pub const fn dpsier0(&self) -> &DPSIER0 {
        &self.dpsier0
    }
    ///0x403 - Deep Standby Interrupt Enable Register 1
    #[inline(always)]
    pub const fn dpsier1(&self) -> &DPSIER1 {
        &self.dpsier1
    }
    ///0x404 - Deep Standby Interrupt Enable Register 2
    #[inline(always)]
    pub const fn dpsier2(&self) -> &DPSIER2 {
        &self.dpsier2
    }
    ///0x405 - Deep Standby Interrupt Enable Register 3
    #[inline(always)]
    pub const fn dpsier3(&self) -> &DPSIER3 {
        &self.dpsier3
    }
    ///0x406 - Deep Standby Interrupt Flag Register 0
    #[inline(always)]
    pub const fn dpsifr0(&self) -> &DPSIFR0 {
        &self.dpsifr0
    }
    ///0x407 - Deep Standby Interrupt Flag Register 1
    #[inline(always)]
    pub const fn dpsifr1(&self) -> &DPSIFR1 {
        &self.dpsifr1
    }
    ///0x408 - Deep Standby Interrupt Flag Register 2
    #[inline(always)]
    pub const fn dpsifr2(&self) -> &DPSIFR2 {
        &self.dpsifr2
    }
    ///0x409 - Deep Standby Interrupt Flag Register 3
    #[inline(always)]
    pub const fn dpsifr3(&self) -> &DPSIFR3 {
        &self.dpsifr3
    }
    ///0x40a - Deep Standby Interrupt Edge Register 0
    #[inline(always)]
    pub const fn dpsiegr0(&self) -> &DPSIEGR0 {
        &self.dpsiegr0
    }
    ///0x40b - Deep Standby Interrupt Edge Register 1
    #[inline(always)]
    pub const fn dpsiegr1(&self) -> &DPSIEGR1 {
        &self.dpsiegr1
    }
    ///0x40c - Deep Standby Interrupt Edge Register 2
    #[inline(always)]
    pub const fn dpsiegr2(&self) -> &DPSIEGR2 {
        &self.dpsiegr2
    }
    ///0x40e - System Control OCD Control Register
    #[inline(always)]
    pub const fn syocdcr(&self) -> &SYOCDCR {
        &self.syocdcr
    }
    ///0x40f - Standby Condition Register
    #[inline(always)]
    pub const fn stconr(&self) -> &STCONR {
        &self.stconr
    }
    ///0x410 - Reset Status Register 0
    #[inline(always)]
    pub const fn rstsr0(&self) -> &RSTSR0 {
        &self.rstsr0
    }
    ///0x411 - Reset Status Register 2
    #[inline(always)]
    pub const fn rstsr2(&self) -> &RSTSR2 {
        &self.rstsr2
    }
    ///0x413 - Main Clock Oscillator Mode Oscillation Control Register
    #[inline(always)]
    pub const fn momcr(&self) -> &MOMCR {
        &self.momcr
    }
    ///0x416 - Flash P/E Protect Register
    #[inline(always)]
    pub const fn fwepror(&self) -> &FWEPROR {
        &self.fwepror
    }
    ///0x417 - Voltage Monitor Circuit Control Register
    #[inline(always)]
    pub const fn lvcmpcr(&self) -> &LVCMPCR {
        &self.lvcmpcr
    }
    ///0x418 - Voltage Detection Level Select Register
    #[inline(always)]
    pub const fn lvdlvlr(&self) -> &LVDLVLR {
        &self.lvdlvlr
    }
    ///0x41a - Voltage Monitor %s Circuit Control Register 0
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `LVD1CR0` register.</div>
    #[inline(always)]
    pub const fn lvdcr0(&self, n: usize) -> &LVDCR0 {
        &self.lvdcr0[n]
    }
    ///Iterator for array of:
    ///0x41a - Voltage Monitor %s Circuit Control Register 0
    #[inline(always)]
    pub fn lvdcr0_iter(&self) -> impl Iterator<Item = &LVDCR0> {
        self.lvdcr0.iter()
    }
    ///0x41a - Voltage Monitor 1 Circuit Control Register 0
    #[inline(always)]
    pub const fn lvd1cr0(&self) -> &LVDCR0 {
        self.lvdcr0(0)
    }
    ///0x41b - Voltage Monitor 2 Circuit Control Register 0
    #[inline(always)]
    pub const fn lvd2cr0(&self) -> &LVDCR0 {
        self.lvdcr0(1)
    }
    ///0x480 - Sub-clock oscillator control register
    #[inline(always)]
    pub const fn sosccr(&self) -> &SOSCCR {
        &self.sosccr
    }
    ///0x481 - Sub Clock Oscillator Mode Control Register
    #[inline(always)]
    pub const fn somcr(&self) -> &SOMCR {
        &self.somcr
    }
    ///0x490 - Low-Speed On-Chip Oscillator Control Register
    #[inline(always)]
    pub const fn lococr(&self) -> &LOCOCR {
        &self.lococr
    }
    ///0x492 - LOCO User Trimming Control Register
    #[inline(always)]
    pub const fn locoutcr(&self) -> &LOCOUTCR {
        &self.locoutcr
    }
    ///0x4bb - VBATT Input Control Register
    #[inline(always)]
    pub const fn vbtictlr(&self) -> &VBTICTLR {
        &self.vbtictlr
    }
    ///0x500..0x700 - VBATT Backup Register \[%s\]
    #[inline(always)]
    pub const fn vbtbkr(&self, n: usize) -> &VBTBKR {
        &self.vbtbkr[n]
    }
    ///Iterator for array of:
    ///0x500..0x700 - VBATT Backup Register \[%s\]
    #[inline(always)]
    pub fn vbtbkr_iter(&self) -> impl Iterator<Item = &VBTBKR> {
        self.vbtbkr.iter()
    }
}
/**SCKDIVCR (rw) register accessor: System Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckdivcr`] module*/
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
///System Clock Division Control Register
pub mod sckdivcr;
/**SCKDIVCR2 (rw) register accessor: System Clock Division Control Register 2

You can [`read`](crate::Reg::read) this register and get [`sckdivcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckdivcr2`] module*/
pub type SCKDIVCR2 = crate::Reg<sckdivcr2::SCKDIVCR2_SPEC>;
///System Clock Division Control Register 2
pub mod sckdivcr2;
/**SCKSCR (rw) register accessor: System Clock Source Control Register

You can [`read`](crate::Reg::read) this register and get [`sckscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckscr`] module*/
pub type SCKSCR = crate::Reg<sckscr::SCKSCR_SPEC>;
///System Clock Source Control Register
pub mod sckscr;
/**PLLCCR (rw) register accessor: PLL Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`pllccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllccr`] module*/
pub type PLLCCR = crate::Reg<pllccr::PLLCCR_SPEC>;
///PLL Clock Control Register
pub mod pllccr;
/**PLLCR (rw) register accessor: PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`pllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllcr`] module*/
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
///PLL Control Register
pub mod pllcr;
/**BCKCR (rw) register accessor: External Bus Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`bckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bckcr`] module*/
pub type BCKCR = crate::Reg<bckcr::BCKCR_SPEC>;
///External Bus Clock Control Register
pub mod bckcr;
/**MOSCCR (rw) register accessor: Main Clock Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mosccr`] module*/
pub type MOSCCR = crate::Reg<mosccr::MOSCCR_SPEC>;
///Main Clock Oscillator Control Register
pub mod mosccr;
/**HOCOCR (rw) register accessor: High-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`hococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hococr`] module*/
pub type HOCOCR = crate::Reg<hococr::HOCOCR_SPEC>;
///High-Speed On-Chip Oscillator Control Register
pub mod hococr;
/**MOCOCR (rw) register accessor: Middle-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mococr`] module*/
pub type MOCOCR = crate::Reg<mococr::MOCOCR_SPEC>;
///Middle-Speed On-Chip Oscillator Control Register
pub mod mococr;
/**FLLCR1 (rw) register accessor: FLL Control Register 1

You can [`read`](crate::Reg::read) this register and get [`fllcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fllcr1`] module*/
pub type FLLCR1 = crate::Reg<fllcr1::FLLCR1_SPEC>;
///FLL Control Register 1
pub mod fllcr1;
/**FLLCR2 (rw) register accessor: FLL Control Register 2

You can [`read`](crate::Reg::read) this register and get [`fllcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fllcr2`] module*/
pub type FLLCR2 = crate::Reg<fllcr2::FLLCR2_SPEC>;
///FLL Control Register 2
pub mod fllcr2;
/**OSCSF (r) register accessor: Oscillation Stabilization Flag Register

You can [`read`](crate::Reg::read) this register and get [`oscsf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oscsf`] module*/
pub type OSCSF = crate::Reg<oscsf::OSCSF_SPEC>;
///Oscillation Stabilization Flag Register
pub mod oscsf;
/**CKOCR (rw) register accessor: Clock Out Control Register

You can [`read`](crate::Reg::read) this register and get [`ckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckocr`] module*/
pub type CKOCR = crate::Reg<ckocr::CKOCR_SPEC>;
///Clock Out Control Register
pub mod ckocr;
/**TRCKCR (rw) register accessor: Trace Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`trckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trckcr`] module*/
pub type TRCKCR = crate::Reg<trckcr::TRCKCR_SPEC>;
///Trace Clock Control Register
pub mod trckcr;
/**OSTDCR (rw) register accessor: Oscillation Stop Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`ostdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ostdcr`] module*/
pub type OSTDCR = crate::Reg<ostdcr::OSTDCR_SPEC>;
///Oscillation Stop Detection Control Register
pub mod ostdcr;
/**OSTDSR (rw) register accessor: Oscillation Stop Detection Status Register

You can [`read`](crate::Reg::read) this register and get [`ostdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ostdsr`] module*/
pub type OSTDSR = crate::Reg<ostdsr::OSTDSR_SPEC>;
///Oscillation Stop Detection Status Register
pub mod ostdsr;
/**EBCKOCR (rw) register accessor: External Bus Clock Output Control Register

You can [`read`](crate::Reg::read) this register and get [`ebckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ebckocr`] module*/
pub type EBCKOCR = crate::Reg<ebckocr::EBCKOCR_SPEC>;
///External Bus Clock Output Control Register
pub mod ebckocr;
/**SDCKOCR (rw) register accessor: SDRAM Clock Output Control Register

You can [`read`](crate::Reg::read) this register and get [`sdckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdckocr`] module*/
pub type SDCKOCR = crate::Reg<sdckocr::SDCKOCR_SPEC>;
///SDRAM Clock Output Control Register
pub mod sdckocr;
/**MOCOUTCR (rw) register accessor: MOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mocoutcr`] module*/
pub type MOCOUTCR = crate::Reg<mocoutcr::MOCOUTCR_SPEC>;
///MOCO User Trimming Control Register
pub mod mocoutcr;
/**HOCOUTCR (rw) register accessor: HOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hocoutcr`] module*/
pub type HOCOUTCR = crate::Reg<hocoutcr::HOCOUTCR_SPEC>;
///HOCO User Trimming Control Register
pub mod hocoutcr;
/**MOMCR (rw) register accessor: Main Clock Oscillator Mode Oscillation Control Register

You can [`read`](crate::Reg::read) this register and get [`momcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@momcr`] module*/
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
///Main Clock Oscillator Mode Oscillation Control Register
pub mod momcr;
/**SOSCCR (rw) register accessor: Sub-clock oscillator control register

You can [`read`](crate::Reg::read) this register and get [`sosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sosccr`] module*/
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
///Sub-clock oscillator control register
pub mod sosccr;
/**SOMCR (rw) register accessor: Sub Clock Oscillator Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`somcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@somcr`] module*/
pub type SOMCR = crate::Reg<somcr::SOMCR_SPEC>;
///Sub Clock Oscillator Mode Control Register
pub mod somcr;
/**LOCOCR (rw) register accessor: Low-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`lococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lococr`] module*/
pub type LOCOCR = crate::Reg<lococr::LOCOCR_SPEC>;
///Low-Speed On-Chip Oscillator Control Register
pub mod lococr;
/**LOCOUTCR (rw) register accessor: LOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`locoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@locoutcr`] module*/
pub type LOCOUTCR = crate::Reg<locoutcr::LOCOUTCR_SPEC>;
///LOCO User Trimming Control Register
pub mod locoutcr;
/**MOSCWTCR (rw) register accessor: Main Clock Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@moscwtcr`] module*/
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
///Main Clock Oscillator Wait Control Register
pub mod moscwtcr;
/**HOCOWTCR (rw) register accessor: High-speed on-chip oscillator wait control register

You can [`read`](crate::Reg::read) this register and get [`hocowtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocowtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hocowtcr`] module*/
pub type HOCOWTCR = crate::Reg<hocowtcr::HOCOWTCR_SPEC>;
///High-speed on-chip oscillator wait control register
pub mod hocowtcr;
/**SBYCR (rw) register accessor: Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`sbycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sbycr`] module*/
pub type SBYCR = crate::Reg<sbycr::SBYCR_SPEC>;
///Standby Control Register
pub mod sbycr;
/**MSTPCRA (rw) register accessor: Module Stop Control Register A

You can [`read`](crate::Reg::read) this register and get [`mstpcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcra`] module*/
pub type MSTPCRA = crate::Reg<mstpcra::MSTPCRA_SPEC>;
///Module Stop Control Register A
pub mod mstpcra;
/**SNZCR (rw) register accessor: Snooze Control Register

You can [`read`](crate::Reg::read) this register and get [`snzcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzcr`] module*/
pub type SNZCR = crate::Reg<snzcr::SNZCR_SPEC>;
///Snooze Control Register
pub mod snzcr;
/**SNZEDCR (rw) register accessor: Snooze End Control Register

You can [`read`](crate::Reg::read) this register and get [`snzedcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzedcr`] module*/
pub type SNZEDCR = crate::Reg<snzedcr::SNZEDCR_SPEC>;
///Snooze End Control Register
pub mod snzedcr;
/**SNZREQCR (rw) register accessor: Snooze Request Control Register

You can [`read`](crate::Reg::read) this register and get [`snzreqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzreqcr`] module*/
pub type SNZREQCR = crate::Reg<snzreqcr::SNZREQCR_SPEC>;
///Snooze Request Control Register
pub mod snzreqcr;
/**OPCCR (rw) register accessor: Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`opccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@opccr`] module*/
pub type OPCCR = crate::Reg<opccr::OPCCR_SPEC>;
///Operating Power Control Register
pub mod opccr;
/**SOPCCR (rw) register accessor: Sub Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`sopccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sopccr`] module*/
pub type SOPCCR = crate::Reg<sopccr::SOPCCR_SPEC>;
///Sub Operating Power Control Register
pub mod sopccr;
/**DPSBYCR (rw) register accessor: Deep Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`dpsbycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsbycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsbycr`] module*/
pub type DPSBYCR = crate::Reg<dpsbycr::DPSBYCR_SPEC>;
///Deep Standby Control Register
pub mod dpsbycr;
/**DPSIER0 (rw) register accessor: Deep Standby Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier0`] module*/
pub type DPSIER0 = crate::Reg<dpsier0::DPSIER0_SPEC>;
///Deep Standby Interrupt Enable Register 0
pub mod dpsier0;
/**DPSIER1 (rw) register accessor: Deep Standby Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier1`] module*/
pub type DPSIER1 = crate::Reg<dpsier1::DPSIER1_SPEC>;
///Deep Standby Interrupt Enable Register 1
pub mod dpsier1;
/**DPSIER2 (rw) register accessor: Deep Standby Interrupt Enable Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier2`] module*/
pub type DPSIER2 = crate::Reg<dpsier2::DPSIER2_SPEC>;
///Deep Standby Interrupt Enable Register 2
pub mod dpsier2;
/**DPSIER3 (rw) register accessor: Deep Standby Interrupt Enable Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier3`] module*/
pub type DPSIER3 = crate::Reg<dpsier3::DPSIER3_SPEC>;
///Deep Standby Interrupt Enable Register 3
pub mod dpsier3;
/**DPSIFR0 (rw) register accessor: Deep Standby Interrupt Flag Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsifr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr0`] module*/
pub type DPSIFR0 = crate::Reg<dpsifr0::DPSIFR0_SPEC>;
///Deep Standby Interrupt Flag Register 0
pub mod dpsifr0;
/**DPSIFR1 (rw) register accessor: Deep Standby Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsifr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr1`] module*/
pub type DPSIFR1 = crate::Reg<dpsifr1::DPSIFR1_SPEC>;
///Deep Standby Interrupt Flag Register 1
pub mod dpsifr1;
/**DPSIFR2 (rw) register accessor: Deep Standby Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsifr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr2`] module*/
pub type DPSIFR2 = crate::Reg<dpsifr2::DPSIFR2_SPEC>;
///Deep Standby Interrupt Flag Register 2
pub mod dpsifr2;
/**DPSIFR3 (rw) register accessor: Deep Standby Interrupt Flag Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsifr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr3`] module*/
pub type DPSIFR3 = crate::Reg<dpsifr3::DPSIFR3_SPEC>;
///Deep Standby Interrupt Flag Register 3
pub mod dpsifr3;
/**DPSIEGR0 (rw) register accessor: Deep Standby Interrupt Edge Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsiegr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsiegr0`] module*/
pub type DPSIEGR0 = crate::Reg<dpsiegr0::DPSIEGR0_SPEC>;
///Deep Standby Interrupt Edge Register 0
pub mod dpsiegr0;
/**DPSIEGR1 (rw) register accessor: Deep Standby Interrupt Edge Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsiegr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsiegr1`] module*/
pub type DPSIEGR1 = crate::Reg<dpsiegr1::DPSIEGR1_SPEC>;
///Deep Standby Interrupt Edge Register 1
pub mod dpsiegr1;
/**DPSIEGR2 (rw) register accessor: Deep Standby Interrupt Edge Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsiegr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsiegr2`] module*/
pub type DPSIEGR2 = crate::Reg<dpsiegr2::DPSIEGR2_SPEC>;
///Deep Standby Interrupt Edge Register 2
pub mod dpsiegr2;
/**SYOCDCR (rw) register accessor: System Control OCD Control Register

You can [`read`](crate::Reg::read) this register and get [`syocdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syocdcr`] module*/
pub type SYOCDCR = crate::Reg<syocdcr::SYOCDCR_SPEC>;
///System Control OCD Control Register
pub mod syocdcr;
/**STCONR (rw) register accessor: Standby Condition Register

You can [`read`](crate::Reg::read) this register and get [`stconr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stconr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stconr`] module*/
pub type STCONR = crate::Reg<stconr::STCONR_SPEC>;
///Standby Condition Register
pub mod stconr;
/**LVDCR1 (rw) register accessor: Voltage Monitor %s Circuit Control Register 1

You can [`read`](crate::Reg::read) this register and get [`lvdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvdcr1`] module*/
pub type LVDCR1 = crate::Reg<lvdcr1::LVDCR1_SPEC>;
///Voltage Monitor %s Circuit Control Register 1
pub mod lvdcr1;
/**LVDSR (rw) register accessor: Voltage Monitor %s Circuit Status Register

You can [`read`](crate::Reg::read) this register and get [`lvdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvdsr`] module*/
pub type LVDSR = crate::Reg<lvdsr::LVDSR_SPEC>;
///Voltage Monitor %s Circuit Status Register
pub mod lvdsr;
/**LVCMPCR (rw) register accessor: Voltage Monitor Circuit Control Register

You can [`read`](crate::Reg::read) this register and get [`lvcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvcmpcr`] module*/
pub type LVCMPCR = crate::Reg<lvcmpcr::LVCMPCR_SPEC>;
///Voltage Monitor Circuit Control Register
pub mod lvcmpcr;
/**LVDLVLR (rw) register accessor: Voltage Detection Level Select Register

You can [`read`](crate::Reg::read) this register and get [`lvdlvlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdlvlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvdlvlr`] module*/
pub type LVDLVLR = crate::Reg<lvdlvlr::LVDLVLR_SPEC>;
///Voltage Detection Level Select Register
pub mod lvdlvlr;
/**LVDCR0 (rw) register accessor: Voltage Monitor %s Circuit Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lvdcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvdcr0`] module*/
pub type LVDCR0 = crate::Reg<lvdcr0::LVDCR0_SPEC>;
///Voltage Monitor %s Circuit Control Register 0
pub mod lvdcr0;
/**VBTICTLR (rw) register accessor: VBATT Input Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtictlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtictlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtictlr`] module*/
pub type VBTICTLR = crate::Reg<vbtictlr::VBTICTLR_SPEC>;
///VBATT Input Control Register
pub mod vbtictlr;
/**VBTBKR (rw) register accessor: VBATT Backup Register \[%s\]

You can [`read`](crate::Reg::read) this register and get [`vbtbkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtbkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtbkr`] module*/
pub type VBTBKR = crate::Reg<vbtbkr::VBTBKR_SPEC>;
///VBATT Backup Register \[%s\]
pub mod vbtbkr;
/**PRCR (rw) register accessor: Protect Register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prcr`] module*/
pub type PRCR = crate::Reg<prcr::PRCR_SPEC>;
///Protect Register
pub mod prcr;
/**RSTSR0 (rw) register accessor: Reset Status Register 0

You can [`read`](crate::Reg::read) this register and get [`rstsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsr0`] module*/
pub type RSTSR0 = crate::Reg<rstsr0::RSTSR0_SPEC>;
///Reset Status Register 0
pub mod rstsr0;
/**RSTSR2 (rw) register accessor: Reset Status Register 2

You can [`read`](crate::Reg::read) this register and get [`rstsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsr2`] module*/
pub type RSTSR2 = crate::Reg<rstsr2::RSTSR2_SPEC>;
///Reset Status Register 2
pub mod rstsr2;
/**RSTSR1 (rw) register accessor: Reset Status Register 1

You can [`read`](crate::Reg::read) this register and get [`rstsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsr1`] module*/
pub type RSTSR1 = crate::Reg<rstsr1::RSTSR1_SPEC>;
///Reset Status Register 1
pub mod rstsr1;
/**FWEPROR (rw) register accessor: Flash P/E Protect Register

You can [`read`](crate::Reg::read) this register and get [`fwepror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwepror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fwepror`] module*/
pub type FWEPROR = crate::Reg<fwepror::FWEPROR_SPEC>;
///Flash P/E Protect Register
pub mod fwepror;
