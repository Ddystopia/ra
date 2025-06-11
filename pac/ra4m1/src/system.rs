#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    sbycr: SBYCR,
    _reserved1: [u8; 0x0e],
    mstpcra: MSTPCRA,
    sckdivcr: SCKDIVCR,
    _reserved3: [u8; 0x02],
    sckscr: SCKSCR,
    _reserved4: [u8; 0x03],
    pllcr: PLLCR,
    pllccr2: PLLCCR2,
    _reserved6: [u8; 0x05],
    memwait: MEMWAIT,
    mosccr: MOSCCR,
    _reserved8: [u8; 0x03],
    hococr: HOCOCR,
    _reserved9: [u8; 0x01],
    mococr: MOCOCR,
    _reserved10: [u8; 0x03],
    oscsf: OSCSF,
    _reserved11: [u8; 0x01],
    ckocr: CKOCR,
    trckcr: TRCKCR,
    ostdcr: OSTDCR,
    ostdsr: OSTDSR,
    _reserved15: [u8; 0x0e],
    slcdsckcr: SLCDSCKCR,
    _reserved16: [u8; 0x10],
    mocoutcr: MOCOUTCR,
    hocoutcr: HOCOUTCR,
    _reserved18: [u8; 0x2f],
    snzcr: SNZCR,
    _reserved19: [u8; 0x01],
    snzedcr: SNZEDCR,
    _reserved20: [u8; 0x03],
    snzreqcr: SNZREQCR,
    _reserved21: [u8; 0x02],
    flstop: FLSTOP,
    _reserved22: [u8; 0x01],
    opccr: OPCCR,
    _reserved23: [u8; 0x01],
    moscwtcr: MOSCWTCR,
    _reserved24: [u8; 0x02],
    hocowtcr: HOCOWTCR,
    _reserved25: [u8; 0x04],
    sopccr: SOPCCR,
    _reserved26: [u8; 0x15],
    rstsr1: RSTSR1,
    _reserved27: [u8; 0x04],
    bkracr: BKRACR,
    _reserved28: [u8; 0x09],
    usbckcr: USBCKCR,
    _reserved29: [u8; 0x0f],
    lvdcr1: (),
    _reserved30: [u8; 0x01],
    lvdsr: (),
    _reserved31: [u8; 0x031d],
    prcr: PRCR,
    _reserved32: [u8; 0x0e],
    syocdcr: SYOCDCR,
    _reserved33: [u8; 0x01],
    rstsr0: RSTSR0,
    rstsr2: RSTSR2,
    _reserved35: [u8; 0x01],
    momcr: MOMCR,
    _reserved36: [u8; 0x03],
    lvcmpcr: LVCMPCR,
    lvdlvlr: LVDLVLR,
    _reserved38: [u8; 0x01],
    lvdcr0: [LVDCR0; 2],
    _reserved39: [u8; 0x03],
    vbtcr1: VBTCR1,
    _reserved40: [u8; 0x60],
    sosccr: SOSCCR,
    somcr: SOMCR,
    _reserved42: [u8; 0x0e],
    lococr: LOCOCR,
    _reserved43: [u8; 0x01],
    locoutcr: LOCOUTCR,
    _reserved44: [u8; 0x1d],
    vbtcr2: VBTCR2,
    vbtsr: VBTSR,
    vbtcmpcr: VBTCMPCR,
    _reserved47: [u8; 0x01],
    vbtlvdicr: VBTLVDICR,
    _reserved48: [u8; 0x01],
    vbtwctlr: VBTWCTLR,
    _reserved49: [u8; 0x01],
    vbtwch0otsr: VBTWCH0OTSR,
    vbtwch1otsr: VBTWCH1OTSR,
    vbtwch2otsr: VBTWCH2OTSR,
    vbtictlr: VBTICTLR,
    vbtoctlr: VBTOCTLR,
    vbtwter: VBTWTER,
    vbtwegr: VBTWEGR,
    vbtwfr: VBTWFR,
    _reserved57: [u8; 0x40],
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
    ///0x26 - System Clock Source Control Register
    #[inline(always)]
    pub const fn sckscr(&self) -> &SCKSCR {
        &self.sckscr
    }
    ///0x2a - PLL Control Register
    #[inline(always)]
    pub const fn pllcr(&self) -> &PLLCR {
        &self.pllcr
    }
    ///0x2b - PLL Clock Control Register2
    #[inline(always)]
    pub const fn pllccr2(&self) -> &PLLCCR2 {
        &self.pllccr2
    }
    ///0x31 - Memory Wait Cycle Control Register
    #[inline(always)]
    pub const fn memwait(&self) -> &MEMWAIT {
        &self.memwait
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
    ///0x50 - Segment LCD Source Clock Control Register
    #[inline(always)]
    pub const fn slcdsckcr(&self) -> &SLCDSCKCR {
        &self.slcdsckcr
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
    ///0x9e - Flash Operation Control Register
    #[inline(always)]
    pub const fn flstop(&self) -> &FLSTOP {
        &self.flstop
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
    ///0xa5 - High-Speed On-Chip Oscillator Wait Control Register
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
    ///0xc6 - Backup Register Access Control Register
    #[inline(always)]
    pub const fn bkracr(&self) -> &BKRACR {
        &self.bkracr
    }
    ///0xd0 - USB Clock Control register
    #[inline(always)]
    pub const fn usbckcr(&self) -> &USBCKCR {
        &self.usbckcr
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
    ///0x40e - System Control OCD Control Register
    #[inline(always)]
    pub const fn syocdcr(&self) -> &SYOCDCR {
        &self.syocdcr
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
    ///0x41f - VBATT Control Register1
    #[inline(always)]
    pub const fn vbtcr1(&self) -> &VBTCR1 {
        &self.vbtcr1
    }
    ///0x480 - Sub-Clock Oscillator Control Register
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
    ///0x4b0 - VBATT Control Register2
    #[inline(always)]
    pub const fn vbtcr2(&self) -> &VBTCR2 {
        &self.vbtcr2
    }
    ///0x4b1 - VBATT Status Register
    #[inline(always)]
    pub const fn vbtsr(&self) -> &VBTSR {
        &self.vbtsr
    }
    ///0x4b2 - VBATT Comparator Control Register
    #[inline(always)]
    pub const fn vbtcmpcr(&self) -> &VBTCMPCR {
        &self.vbtcmpcr
    }
    ///0x4b4 - VBATT Pin Low Voltage Detect Interrupt Control Register
    #[inline(always)]
    pub const fn vbtlvdicr(&self) -> &VBTLVDICR {
        &self.vbtlvdicr
    }
    ///0x4b6 - VBATT Wakeup function Control Register
    #[inline(always)]
    pub const fn vbtwctlr(&self) -> &VBTWCTLR {
        &self.vbtwctlr
    }
    ///0x4b8 - VBATT Wakeup I/O 0 Output Trigger Select Register
    #[inline(always)]
    pub const fn vbtwch0otsr(&self) -> &VBTWCH0OTSR {
        &self.vbtwch0otsr
    }
    ///0x4b9 - VBATT Wakeup I/O 1 Output Trigger Select Register
    #[inline(always)]
    pub const fn vbtwch1otsr(&self) -> &VBTWCH1OTSR {
        &self.vbtwch1otsr
    }
    ///0x4ba - VBATT Wakeup I/O 2 Output Trigger Select Register
    #[inline(always)]
    pub const fn vbtwch2otsr(&self) -> &VBTWCH2OTSR {
        &self.vbtwch2otsr
    }
    ///0x4bb - VBATT Input Control Register
    #[inline(always)]
    pub const fn vbtictlr(&self) -> &VBTICTLR {
        &self.vbtictlr
    }
    ///0x4bc - VBATT Output Control Register
    #[inline(always)]
    pub const fn vbtoctlr(&self) -> &VBTOCTLR {
        &self.vbtoctlr
    }
    ///0x4bd - VBATT Wakeup Trigger source Enable Register
    #[inline(always)]
    pub const fn vbtwter(&self) -> &VBTWTER {
        &self.vbtwter
    }
    ///0x4be - VBATT Wakeup Trigger source Edge Register
    #[inline(always)]
    pub const fn vbtwegr(&self) -> &VBTWEGR {
        &self.vbtwegr
    }
    ///0x4bf - VBATT Wakeup trigger source Flag Register
    #[inline(always)]
    pub const fn vbtwfr(&self) -> &VBTWFR {
        &self.vbtwfr
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
/**VBTCR1 (rw) register accessor: VBATT Control Register1

You can [`read`](crate::Reg::read) this register and get [`vbtcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtcr1`] module*/
pub type VBTCR1 = crate::Reg<vbtcr1::VBTCR1_SPEC>;
///VBATT Control Register1
pub mod vbtcr1;
/**VBTCR2 (rw) register accessor: VBATT Control Register2

You can [`read`](crate::Reg::read) this register and get [`vbtcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtcr2`] module*/
pub type VBTCR2 = crate::Reg<vbtcr2::VBTCR2_SPEC>;
///VBATT Control Register2
pub mod vbtcr2;
/**VBTSR (rw) register accessor: VBATT Status Register

You can [`read`](crate::Reg::read) this register and get [`vbtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtsr`] module*/
pub type VBTSR = crate::Reg<vbtsr::VBTSR_SPEC>;
///VBATT Status Register
pub mod vbtsr;
/**VBTCMPCR (rw) register accessor: VBATT Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtcmpcr`] module*/
pub type VBTCMPCR = crate::Reg<vbtcmpcr::VBTCMPCR_SPEC>;
///VBATT Comparator Control Register
pub mod vbtcmpcr;
/**VBTLVDICR (rw) register accessor: VBATT Pin Low Voltage Detect Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtlvdicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtlvdicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtlvdicr`] module*/
pub type VBTLVDICR = crate::Reg<vbtlvdicr::VBTLVDICR_SPEC>;
///VBATT Pin Low Voltage Detect Interrupt Control Register
pub mod vbtlvdicr;
/**VBTWCTLR (rw) register accessor: VBATT Wakeup function Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtwctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwctlr`] module*/
pub type VBTWCTLR = crate::Reg<vbtwctlr::VBTWCTLR_SPEC>;
///VBATT Wakeup function Control Register
pub mod vbtwctlr;
/**VBTWCH0OTSR (rw) register accessor: VBATT Wakeup I/O 0 Output Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`vbtwch0otsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch0otsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwch0otsr`] module*/
pub type VBTWCH0OTSR = crate::Reg<vbtwch0otsr::VBTWCH0OTSR_SPEC>;
///VBATT Wakeup I/O 0 Output Trigger Select Register
pub mod vbtwch0otsr;
/**VBTWCH1OTSR (rw) register accessor: VBATT Wakeup I/O 1 Output Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`vbtwch1otsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch1otsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwch1otsr`] module*/
pub type VBTWCH1OTSR = crate::Reg<vbtwch1otsr::VBTWCH1OTSR_SPEC>;
///VBATT Wakeup I/O 1 Output Trigger Select Register
pub mod vbtwch1otsr;
/**VBTWCH2OTSR (rw) register accessor: VBATT Wakeup I/O 2 Output Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`vbtwch2otsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch2otsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwch2otsr`] module*/
pub type VBTWCH2OTSR = crate::Reg<vbtwch2otsr::VBTWCH2OTSR_SPEC>;
///VBATT Wakeup I/O 2 Output Trigger Select Register
pub mod vbtwch2otsr;
/**VBTICTLR (rw) register accessor: VBATT Input Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtictlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtictlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtictlr`] module*/
pub type VBTICTLR = crate::Reg<vbtictlr::VBTICTLR_SPEC>;
///VBATT Input Control Register
pub mod vbtictlr;
/**VBTOCTLR (rw) register accessor: VBATT Output Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtoctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtoctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtoctlr`] module*/
pub type VBTOCTLR = crate::Reg<vbtoctlr::VBTOCTLR_SPEC>;
///VBATT Output Control Register
pub mod vbtoctlr;
/**VBTWTER (rw) register accessor: VBATT Wakeup Trigger source Enable Register

You can [`read`](crate::Reg::read) this register and get [`vbtwter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwter`] module*/
pub type VBTWTER = crate::Reg<vbtwter::VBTWTER_SPEC>;
///VBATT Wakeup Trigger source Enable Register
pub mod vbtwter;
/**VBTWEGR (rw) register accessor: VBATT Wakeup Trigger source Edge Register

You can [`read`](crate::Reg::read) this register and get [`vbtwegr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwegr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwegr`] module*/
pub type VBTWEGR = crate::Reg<vbtwegr::VBTWEGR_SPEC>;
///VBATT Wakeup Trigger source Edge Register
pub mod vbtwegr;
/**VBTWFR (rw) register accessor: VBATT Wakeup trigger source Flag Register

You can [`read`](crate::Reg::read) this register and get [`vbtwfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtwfr`] module*/
pub type VBTWFR = crate::Reg<vbtwfr::VBTWFR_SPEC>;
///VBATT Wakeup trigger source Flag Register
pub mod vbtwfr;
/**VBTBKR (rw) register accessor: VBATT Backup Register \[%s\]

You can [`read`](crate::Reg::read) this register and get [`vbtbkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtbkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtbkr`] module*/
pub type VBTBKR = crate::Reg<vbtbkr::VBTBKR_SPEC>;
///VBATT Backup Register \[%s\]
pub mod vbtbkr;
/**SCKDIVCR (rw) register accessor: System Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckdivcr`] module*/
pub type SCKDIVCR = crate::Reg<sckdivcr::SCKDIVCR_SPEC>;
///System Clock Division Control Register
pub mod sckdivcr;
/**SCKSCR (rw) register accessor: System Clock Source Control Register

You can [`read`](crate::Reg::read) this register and get [`sckscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckscr`] module*/
pub type SCKSCR = crate::Reg<sckscr::SCKSCR_SPEC>;
///System Clock Source Control Register
pub mod sckscr;
/**PLLCR (rw) register accessor: PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`pllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllcr`] module*/
pub type PLLCR = crate::Reg<pllcr::PLLCR_SPEC>;
///PLL Control Register
pub mod pllcr;
/**PLLCCR2 (rw) register accessor: PLL Clock Control Register2

You can [`read`](crate::Reg::read) this register and get [`pllccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllccr2`] module*/
pub type PLLCCR2 = crate::Reg<pllccr2::PLLCCR2_SPEC>;
///PLL Clock Control Register2
pub mod pllccr2;
/**MEMWAIT (rw) register accessor: Memory Wait Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`memwait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memwait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@memwait`] module*/
pub type MEMWAIT = crate::Reg<memwait::MEMWAIT_SPEC>;
///Memory Wait Cycle Control Register
pub mod memwait;
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
/**SLCDSCKCR (rw) register accessor: Segment LCD Source Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`slcdsckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slcdsckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slcdsckcr`] module*/
pub type SLCDSCKCR = crate::Reg<slcdsckcr::SLCDSCKCR_SPEC>;
///Segment LCD Source Clock Control Register
pub mod slcdsckcr;
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
/**MOSCWTCR (rw) register accessor: Main Clock Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@moscwtcr`] module*/
pub type MOSCWTCR = crate::Reg<moscwtcr::MOSCWTCR_SPEC>;
///Main Clock Oscillator Wait Control Register
pub mod moscwtcr;
/**HOCOWTCR (rw) register accessor: High-Speed On-Chip Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`hocowtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocowtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hocowtcr`] module*/
pub type HOCOWTCR = crate::Reg<hocowtcr::HOCOWTCR_SPEC>;
///High-Speed On-Chip Oscillator Wait Control Register
pub mod hocowtcr;
/**USBCKCR (rw) register accessor: USB Clock Control register

You can [`read`](crate::Reg::read) this register and get [`usbckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbckcr`] module*/
pub type USBCKCR = crate::Reg<usbckcr::USBCKCR_SPEC>;
///USB Clock Control register
pub mod usbckcr;
/**MOMCR (rw) register accessor: Main Clock Oscillator Mode Oscillation Control Register

You can [`read`](crate::Reg::read) this register and get [`momcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@momcr`] module*/
pub type MOMCR = crate::Reg<momcr::MOMCR_SPEC>;
///Main Clock Oscillator Mode Oscillation Control Register
pub mod momcr;
/**SOSCCR (rw) register accessor: Sub-Clock Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`sosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sosccr`] module*/
pub type SOSCCR = crate::Reg<sosccr::SOSCCR_SPEC>;
///Sub-Clock Oscillator Control Register
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
/**FLSTOP (rw) register accessor: Flash Operation Control Register

You can [`read`](crate::Reg::read) this register and get [`flstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flstop`] module*/
pub type FLSTOP = crate::Reg<flstop::FLSTOP_SPEC>;
///Flash Operation Control Register
pub mod flstop;
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
/**SYOCDCR (rw) register accessor: System Control OCD Control Register

You can [`read`](crate::Reg::read) this register and get [`syocdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syocdcr`] module*/
pub type SYOCDCR = crate::Reg<syocdcr::SYOCDCR_SPEC>;
///System Control OCD Control Register
pub mod syocdcr;
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
/**BKRACR (rw) register accessor: Backup Register Access Control Register

You can [`read`](crate::Reg::read) this register and get [`bkracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bkracr`] module*/
pub type BKRACR = crate::Reg<bkracr::BKRACR_SPEC>;
///Backup Register Access Control Register
pub mod bkracr;
