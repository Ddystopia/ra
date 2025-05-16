#[repr(C)]
///Register block
pub struct RegisterBlock {
    syscfg: SYSCFG,
    buswait: BUSWAIT,
    syssts0: SYSSTS0,
    pllsta: PLLSTA,
    dvstctr0: DVSTCTR0,
    _reserved5: [u8; 0x02],
    testmode: TESTMODE,
    _reserved6: [u8; 0x06],
    _reserved_6_cfifo: [u8; 0x04],
    _reserved_7_d: [u8; 0x04],
    _reserved_8_d: [u8; 0x04],
    cfifosel: CFIFOSEL,
    cfifoctr: CFIFOCTR,
    _reserved11: [u8; 0x04],
    d0fifosel: D0FIFOSEL,
    d0fifoctr: D0FIFOCTR,
    d1fifosel: D1FIFOSEL,
    d1fifoctr: D1FIFOCTR,
    intenb0: INTENB0,
    intenb1: INTENB1,
    _reserved17: [u8; 0x02],
    brdyenb: BRDYENB,
    nrdyenb: NRDYENB,
    bempenb: BEMPENB,
    sofcfg: SOFCFG,
    physet: PHYSET,
    intsts0: INTSTS0,
    intsts1: INTSTS1,
    _reserved24: [u8; 0x02],
    brdysts: BRDYSTS,
    nrdysts: NRDYSTS,
    bempsts: BEMPSTS,
    frmnum: FRMNUM,
    ufrmnum: UFRMNUM,
    usbaddr: USBADDR,
    _reserved30: [u8; 0x02],
    usbreq: USBREQ,
    usbval: USBVAL,
    usbindx: USBINDX,
    usbleng: USBLENG,
    dcpcfg: DCPCFG,
    dcpmaxp: DCPMAXP,
    dcpctr: DCPCTR,
    _reserved37: [u8; 0x02],
    pipesel: PIPESEL,
    _reserved38: [u8; 0x02],
    pipecfg: PIPECFG,
    pipebuf: PIPEBUF,
    pipemaxp: PIPEMAXP,
    pipeperi: PIPEPERI,
    pipectr: [PIPECTR; 9],
    _reserved43: [u8; 0x0e],
    pipetre: (),
    _reserved44: [u8; 0x02],
    pipetrn: (),
    _reserved45: [u8; 0x3e],
    devadd: [DEVADD; 10],
    devadda: DEVADDA,
    _reserved47: [u8; 0x1a],
    lpctrl: LPCTRL,
    lpsts: LPSTS,
    _reserved49: [u8; 0x3c],
    bcctrl: BCCTRL,
    _reserved50: [u8; 0x02],
    pl1ctrl1: PL1CTRL1,
    pl1ctrl2: PL1CTRL2,
    hl1ctrl1: HL1CTRL1,
    hl1ctrl2: HL1CTRL2,
    _reserved54: [u8; 0x14],
    dpusr0r: DPUSR0R,
    dpusr1r: DPUSR1R,
    dpusr2r: DPUSR2R,
    dpusrcr: DPUSRCR,
}
impl RegisterBlock {
    ///0x00 - System Configuration Control Register
    #[inline(always)]
    pub const fn syscfg(&self) -> &SYSCFG {
        &self.syscfg
    }
    ///0x02 - CPU Bus Wait Register
    #[inline(always)]
    pub const fn buswait(&self) -> &BUSWAIT {
        &self.buswait
    }
    ///0x04 - System Configuration Status Register
    #[inline(always)]
    pub const fn syssts0(&self) -> &SYSSTS0 {
        &self.syssts0
    }
    ///0x06 - PLL Status Register
    #[inline(always)]
    pub const fn pllsta(&self) -> &PLLSTA {
        &self.pllsta
    }
    ///0x08 - Device State Control Register 0
    #[inline(always)]
    pub const fn dvstctr0(&self) -> &DVSTCTR0 {
        &self.dvstctr0
    }
    ///0x0c - USB Test Mode Register
    #[inline(always)]
    pub const fn testmode(&self) -> &TESTMODE {
        &self.testmode
    }
    ///0x14 - CFIFO Port Register LL
    #[inline(always)]
    pub const fn cfifoll(&self) -> &CFIFOLL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - CFIFO Port Register L
    #[inline(always)]
    pub const fn cfifol(&self) -> &CFIFOL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - CFIFO Port Register
    #[inline(always)]
    pub const fn cfifo(&self) -> &CFIFO {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x16 - CFIFO Port Register H
    #[inline(always)]
    pub const fn cfifoh(&self) -> &CFIFOH {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    ///0x17 - CFIFO Port Register HH
    #[inline(always)]
    pub const fn cfifohh(&self) -> &CFIFOHH {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(23).cast() }
    }
    ///0x18 - D0FIFO Port Register LL
    #[inline(always)]
    pub const fn d0fifoll(&self) -> &D0FIFOLL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - D0FIFO Port Register L
    #[inline(always)]
    pub const fn d0fifol(&self) -> &D0FIFOL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - D0FIFO Port Register
    #[inline(always)]
    pub const fn d0fifo(&self) -> &D0FIFO {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1a - D0FIFO Port Register H
    #[inline(always)]
    pub const fn d0fifoh(&self) -> &D0FIFOH {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    ///0x1b - D0FIFO Port Register HH
    #[inline(always)]
    pub const fn d0fifohh(&self) -> &D0FIFOHH {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(27).cast() }
    }
    ///0x1c - D1FIFO Port Register LL
    #[inline(always)]
    pub const fn d1fifoll(&self) -> &D1FIFOLL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - D1FIFO Port Register L
    #[inline(always)]
    pub const fn d1fifol(&self) -> &D1FIFOL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - D1FIFO Port Register
    #[inline(always)]
    pub const fn d1fifo(&self) -> &D1FIFO {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1e - D1FIFO Port Register H
    #[inline(always)]
    pub const fn d1fifoh(&self) -> &D1FIFOH {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    ///0x1f - D1FIFO Port Register HH
    #[inline(always)]
    pub const fn d1fifohh(&self) -> &D1FIFOHH {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(31).cast() }
    }
    ///0x20 - CFIFO Port Select Register
    #[inline(always)]
    pub const fn cfifosel(&self) -> &CFIFOSEL {
        &self.cfifosel
    }
    ///0x22 - CFIFO Port Control Register
    #[inline(always)]
    pub const fn cfifoctr(&self) -> &CFIFOCTR {
        &self.cfifoctr
    }
    ///0x28 - D0FIFO Port Select Register
    #[inline(always)]
    pub const fn d0fifosel(&self) -> &D0FIFOSEL {
        &self.d0fifosel
    }
    ///0x2a - D0FIFO Port Control Register
    #[inline(always)]
    pub const fn d0fifoctr(&self) -> &D0FIFOCTR {
        &self.d0fifoctr
    }
    ///0x2c - D1FIFO Port Select Register
    #[inline(always)]
    pub const fn d1fifosel(&self) -> &D1FIFOSEL {
        &self.d1fifosel
    }
    ///0x2e - D1FIFO Port Control Register
    #[inline(always)]
    pub const fn d1fifoctr(&self) -> &D1FIFOCTR {
        &self.d1fifoctr
    }
    ///0x30 - Interrupt Enable Register 0
    #[inline(always)]
    pub const fn intenb0(&self) -> &INTENB0 {
        &self.intenb0
    }
    ///0x32 - Interrupt Enable Register 1
    #[inline(always)]
    pub const fn intenb1(&self) -> &INTENB1 {
        &self.intenb1
    }
    ///0x36 - BRDY Interrupt Enable Register
    #[inline(always)]
    pub const fn brdyenb(&self) -> &BRDYENB {
        &self.brdyenb
    }
    ///0x38 - NRDY Interrupt Enable Register
    #[inline(always)]
    pub const fn nrdyenb(&self) -> &NRDYENB {
        &self.nrdyenb
    }
    ///0x3a - BEMP Interrupt Enable Register
    #[inline(always)]
    pub const fn bempenb(&self) -> &BEMPENB {
        &self.bempenb
    }
    ///0x3c - SOF Pin Configuration Register
    #[inline(always)]
    pub const fn sofcfg(&self) -> &SOFCFG {
        &self.sofcfg
    }
    ///0x3e - PHY Setting Register
    #[inline(always)]
    pub const fn physet(&self) -> &PHYSET {
        &self.physet
    }
    ///0x40 - Interrupt Status Register 0
    #[inline(always)]
    pub const fn intsts0(&self) -> &INTSTS0 {
        &self.intsts0
    }
    ///0x42 - Interrupt Status Register 1
    #[inline(always)]
    pub const fn intsts1(&self) -> &INTSTS1 {
        &self.intsts1
    }
    ///0x46 - BRDY Interrupt Status Register
    #[inline(always)]
    pub const fn brdysts(&self) -> &BRDYSTS {
        &self.brdysts
    }
    ///0x48 - NRDY Interrupt Status Register
    #[inline(always)]
    pub const fn nrdysts(&self) -> &NRDYSTS {
        &self.nrdysts
    }
    ///0x4a - BEMP Interrupt Status Register
    #[inline(always)]
    pub const fn bempsts(&self) -> &BEMPSTS {
        &self.bempsts
    }
    ///0x4c - Frame Number Register
    #[inline(always)]
    pub const fn frmnum(&self) -> &FRMNUM {
        &self.frmnum
    }
    ///0x4e - uFrame Number Register
    #[inline(always)]
    pub const fn ufrmnum(&self) -> &UFRMNUM {
        &self.ufrmnum
    }
    ///0x50 - USB Address Register
    #[inline(always)]
    pub const fn usbaddr(&self) -> &USBADDR {
        &self.usbaddr
    }
    ///0x54 - USB Request Type Register
    #[inline(always)]
    pub const fn usbreq(&self) -> &USBREQ {
        &self.usbreq
    }
    ///0x56 - USB Request Value Register
    #[inline(always)]
    pub const fn usbval(&self) -> &USBVAL {
        &self.usbval
    }
    ///0x58 - USB Request Index Register
    #[inline(always)]
    pub const fn usbindx(&self) -> &USBINDX {
        &self.usbindx
    }
    ///0x5a - USB Request Length Register
    #[inline(always)]
    pub const fn usbleng(&self) -> &USBLENG {
        &self.usbleng
    }
    ///0x5c - DCP Configuration Register
    #[inline(always)]
    pub const fn dcpcfg(&self) -> &DCPCFG {
        &self.dcpcfg
    }
    ///0x5e - DCP Maximum Packet Size Register
    #[inline(always)]
    pub const fn dcpmaxp(&self) -> &DCPMAXP {
        &self.dcpmaxp
    }
    ///0x60 - DCP Control Register
    #[inline(always)]
    pub const fn dcpctr(&self) -> &DCPCTR {
        &self.dcpctr
    }
    ///0x64 - Pipe Window Select Register
    #[inline(always)]
    pub const fn pipesel(&self) -> &PIPESEL {
        &self.pipesel
    }
    ///0x68 - Pipe Configuration Register
    #[inline(always)]
    pub const fn pipecfg(&self) -> &PIPECFG {
        &self.pipecfg
    }
    ///0x6a - Pipe Buffer Register
    #[inline(always)]
    pub const fn pipebuf(&self) -> &PIPEBUF {
        &self.pipebuf
    }
    ///0x6c - Pipe Maximum Packet Size Register
    #[inline(always)]
    pub const fn pipemaxp(&self) -> &PIPEMAXP {
        &self.pipemaxp
    }
    ///0x6e - Pipe Cycle Control Register
    #[inline(always)]
    pub const fn pipeperi(&self) -> &PIPEPERI {
        &self.pipeperi
    }
    ///0x70..0x82 - PIPE Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1CTR` register.</div>
    #[inline(always)]
    pub const fn pipectr(&self, n: usize) -> &PIPECTR {
        &self.pipectr[n]
    }
    ///Iterator for array of:
    ///0x70..0x82 - PIPE Control Register
    #[inline(always)]
    pub fn pipectr_iter(&self) -> impl Iterator<Item = &PIPECTR> {
        self.pipectr.iter()
    }
    ///0x70 - PIPE Control Register
    #[inline(always)]
    pub const fn pipe1ctr(&self) -> &PIPECTR {
        self.pipectr(0)
    }
    ///0x72 - PIPE Control Register
    #[inline(always)]
    pub const fn pipe2ctr(&self) -> &PIPECTR {
        self.pipectr(1)
    }
    ///0x74 - PIPE Control Register
    #[inline(always)]
    pub const fn pipe3ctr(&self) -> &PIPECTR {
        self.pipectr(2)
    }
    ///0x76 - PIPE Control Register
    #[inline(always)]
    pub const fn pipe4ctr(&self) -> &PIPECTR {
        self.pipectr(3)
    }
    ///0x78 - PIPE Control Register
    #[inline(always)]
    pub const fn pipe5ctr(&self) -> &PIPECTR {
        self.pipectr(4)
    }
    ///0x7a - PIPE Control Register
    #[inline(always)]
    pub const fn pipe6ctr(&self) -> &PIPECTR {
        self.pipectr(5)
    }
    ///0x7c - PIPE Control Register
    #[inline(always)]
    pub const fn pipe7ctr(&self) -> &PIPECTR {
        self.pipectr(6)
    }
    ///0x7e - PIPE Control Register
    #[inline(always)]
    pub const fn pipe8ctr(&self) -> &PIPECTR {
        self.pipectr(7)
    }
    ///0x80 - PIPE Control Register
    #[inline(always)]
    pub const fn pipe9ctr(&self) -> &PIPECTR {
        self.pipectr(8)
    }
    ///0x90..0x9a - PIPE Transaction Counter Enable Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRE` register.</div>
    #[inline(always)]
    pub const fn pipetre(&self, n: usize) -> &PIPETRE {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(144)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x90..0x9a - PIPE Transaction Counter Enable Register
    #[inline(always)]
    pub fn pipetre_iter(&self) -> impl Iterator<Item = &PIPETRE> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(144)
                .add(4 * n)
                .cast()
        })
    }
    ///0x90 - PIPE Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe1tre(&self) -> &PIPETRE {
        self.pipetre(0)
    }
    ///0x94 - PIPE Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe2tre(&self) -> &PIPETRE {
        self.pipetre(1)
    }
    ///0x98 - PIPE Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe3tre(&self) -> &PIPETRE {
        self.pipetre(2)
    }
    ///0x9c - PIPE Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe4tre(&self) -> &PIPETRE {
        self.pipetre(3)
    }
    ///0xa0 - PIPE Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe5tre(&self) -> &PIPETRE {
        self.pipetre(4)
    }
    ///0x92..0x9c - PIPE Transaction Counter Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRN` register.</div>
    #[inline(always)]
    pub const fn pipetrn(&self, n: usize) -> &PIPETRN {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(146)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x92..0x9c - PIPE Transaction Counter Register
    #[inline(always)]
    pub fn pipetrn_iter(&self) -> impl Iterator<Item = &PIPETRN> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(146)
                .add(4 * n)
                .cast()
        })
    }
    ///0x92 - PIPE Transaction Counter Register
    #[inline(always)]
    pub const fn pipe1trn(&self) -> &PIPETRN {
        self.pipetrn(0)
    }
    ///0x96 - PIPE Transaction Counter Register
    #[inline(always)]
    pub const fn pipe2trn(&self) -> &PIPETRN {
        self.pipetrn(1)
    }
    ///0x9a - PIPE Transaction Counter Register
    #[inline(always)]
    pub const fn pipe3trn(&self) -> &PIPETRN {
        self.pipetrn(2)
    }
    ///0x9e - PIPE Transaction Counter Register
    #[inline(always)]
    pub const fn pipe4trn(&self) -> &PIPETRN {
        self.pipetrn(3)
    }
    ///0xa2 - PIPE Transaction Counter Register
    #[inline(always)]
    pub const fn pipe5trn(&self) -> &PIPETRN {
        self.pipetrn(4)
    }
    ///0xd0..0xe4 - Device Address Configuration Register
    #[inline(always)]
    pub const fn devadd(&self, n: usize) -> &DEVADD {
        &self.devadd[n]
    }
    ///Iterator for array of:
    ///0xd0..0xe4 - Device Address Configuration Register
    #[inline(always)]
    pub fn devadd_iter(&self) -> impl Iterator<Item = &DEVADD> {
        self.devadd.iter()
    }
    ///0xe4 - Device Address Configuration Register A
    #[inline(always)]
    pub const fn devadda(&self) -> &DEVADDA {
        &self.devadda
    }
    ///0x100 - Low Power Control Register
    #[inline(always)]
    pub const fn lpctrl(&self) -> &LPCTRL {
        &self.lpctrl
    }
    ///0x102 - Low Power Status Register
    #[inline(always)]
    pub const fn lpsts(&self) -> &LPSTS {
        &self.lpsts
    }
    ///0x140 - Battery Charging Control Register
    #[inline(always)]
    pub const fn bcctrl(&self) -> &BCCTRL {
        &self.bcctrl
    }
    ///0x144 - Function L1 Control Register 1
    #[inline(always)]
    pub const fn pl1ctrl1(&self) -> &PL1CTRL1 {
        &self.pl1ctrl1
    }
    ///0x146 - Function L1 Control Register 2
    #[inline(always)]
    pub const fn pl1ctrl2(&self) -> &PL1CTRL2 {
        &self.pl1ctrl2
    }
    ///0x148 - Host L1 Control Register 1
    #[inline(always)]
    pub const fn hl1ctrl1(&self) -> &HL1CTRL1 {
        &self.hl1ctrl1
    }
    ///0x14a - Host L1 Control Register 2
    #[inline(always)]
    pub const fn hl1ctrl2(&self) -> &HL1CTRL2 {
        &self.hl1ctrl2
    }
    ///0x160 - Deep Standby USB Transceiver Control/Pin Monitor Register
    #[inline(always)]
    pub const fn dpusr0r(&self) -> &DPUSR0R {
        &self.dpusr0r
    }
    ///0x164 - Deep Standby USB Suspend/Resume Interrupt Register
    #[inline(always)]
    pub const fn dpusr1r(&self) -> &DPUSR1R {
        &self.dpusr1r
    }
    ///0x168 - Deep Standby USB Suspend/Resume Interrupt Register
    #[inline(always)]
    pub const fn dpusr2r(&self) -> &DPUSR2R {
        &self.dpusr2r
    }
    ///0x16a - Deep Standby USB Suspend/Resume Command Register
    #[inline(always)]
    pub const fn dpusrcr(&self) -> &DPUSRCR {
        &self.dpusrcr
    }
}
/**SYSCFG (rw) register accessor: System Configuration Control Register

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscfg`] module*/
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
///System Configuration Control Register
pub mod syscfg;
/**BUSWAIT (rw) register accessor: CPU Bus Wait Register

You can [`read`](crate::Reg::read) this register and get [`buswait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buswait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buswait`] module*/
pub type BUSWAIT = crate::Reg<buswait::BUSWAIT_SPEC>;
///CPU Bus Wait Register
pub mod buswait;
/**SYSSTS0 (r) register accessor: System Configuration Status Register

You can [`read`](crate::Reg::read) this register and get [`syssts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syssts0`] module*/
pub type SYSSTS0 = crate::Reg<syssts0::SYSSTS0_SPEC>;
///System Configuration Status Register
pub mod syssts0;
/**PLLSTA (r) register accessor: PLL Status Register

You can [`read`](crate::Reg::read) this register and get [`pllsta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllsta`] module*/
pub type PLLSTA = crate::Reg<pllsta::PLLSTA_SPEC>;
///PLL Status Register
pub mod pllsta;
/**DVSTCTR0 (rw) register accessor: Device State Control Register 0

You can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvstctr0`] module*/
pub type DVSTCTR0 = crate::Reg<dvstctr0::DVSTCTR0_SPEC>;
///Device State Control Register 0
pub mod dvstctr0;
/**TESTMODE (rw) register accessor: USB Test Mode Register

You can [`read`](crate::Reg::read) this register and get [`testmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@testmode`] module*/
pub type TESTMODE = crate::Reg<testmode::TESTMODE_SPEC>;
///USB Test Mode Register
pub mod testmode;
/**CFIFO (rw) register accessor: CFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifo`] module*/
pub type CFIFO = crate::Reg<cfifo::CFIFO_SPEC>;
///CFIFO Port Register
pub mod cfifo;
/**CFIFOL (rw) register accessor: CFIFO Port Register L

You can [`read`](crate::Reg::read) this register and get [`cfifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifol`] module*/
pub type CFIFOL = crate::Reg<cfifol::CFIFOL_SPEC>;
///CFIFO Port Register L
pub mod cfifol;
/**CFIFOH (rw) register accessor: CFIFO Port Register H

You can [`read`](crate::Reg::read) this register and get [`cfifoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoh`] module*/
pub type CFIFOH = crate::Reg<cfifoh::CFIFOH_SPEC>;
///CFIFO Port Register H
pub mod cfifoh;
/**CFIFOLL (rw) register accessor: CFIFO Port Register LL

You can [`read`](crate::Reg::read) this register and get [`cfifoll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoll`] module*/
pub type CFIFOLL = crate::Reg<cfifoll::CFIFOLL_SPEC>;
///CFIFO Port Register LL
pub mod cfifoll;
/**CFIFOHH (rw) register accessor: CFIFO Port Register HH

You can [`read`](crate::Reg::read) this register and get [`cfifohh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifohh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifohh`] module*/
pub type CFIFOHH = crate::Reg<cfifohh::CFIFOHH_SPEC>;
///CFIFO Port Register HH
pub mod cfifohh;
/**D0FIFO (rw) register accessor: D0FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifo`] module*/
pub type D0FIFO = crate::Reg<d0fifo::D0FIFO_SPEC>;
///D0FIFO Port Register
pub mod d0fifo;
/**D0FIFOL (rw) register accessor: D0FIFO Port Register L

You can [`read`](crate::Reg::read) this register and get [`d0fifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifol`] module*/
pub type D0FIFOL = crate::Reg<d0fifol::D0FIFOL_SPEC>;
///D0FIFO Port Register L
pub mod d0fifol;
/**D0FIFOH (rw) register accessor: D0FIFO Port Register H

You can [`read`](crate::Reg::read) this register and get [`d0fifoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifoh`] module*/
pub type D0FIFOH = crate::Reg<d0fifoh::D0FIFOH_SPEC>;
///D0FIFO Port Register H
pub mod d0fifoh;
/**D0FIFOLL (rw) register accessor: D0FIFO Port Register LL

You can [`read`](crate::Reg::read) this register and get [`d0fifoll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifoll`] module*/
pub type D0FIFOLL = crate::Reg<d0fifoll::D0FIFOLL_SPEC>;
///D0FIFO Port Register LL
pub mod d0fifoll;
/**D0FIFOHH (rw) register accessor: D0FIFO Port Register HH

You can [`read`](crate::Reg::read) this register and get [`d0fifohh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifohh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifohh`] module*/
pub type D0FIFOHH = crate::Reg<d0fifohh::D0FIFOHH_SPEC>;
///D0FIFO Port Register HH
pub mod d0fifohh;
/**D1FIFO (rw) register accessor: D1FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifo`] module*/
pub type D1FIFO = crate::Reg<d1fifo::D1FIFO_SPEC>;
///D1FIFO Port Register
pub mod d1fifo;
/**D1FIFOL (rw) register accessor: D1FIFO Port Register L

You can [`read`](crate::Reg::read) this register and get [`d1fifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifol`] module*/
pub type D1FIFOL = crate::Reg<d1fifol::D1FIFOL_SPEC>;
///D1FIFO Port Register L
pub mod d1fifol;
/**D1FIFOH (rw) register accessor: D1FIFO Port Register H

You can [`read`](crate::Reg::read) this register and get [`d1fifoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifoh`] module*/
pub type D1FIFOH = crate::Reg<d1fifoh::D1FIFOH_SPEC>;
///D1FIFO Port Register H
pub mod d1fifoh;
/**D1FIFOLL (rw) register accessor: D1FIFO Port Register LL

You can [`read`](crate::Reg::read) this register and get [`d1fifoll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifoll`] module*/
pub type D1FIFOLL = crate::Reg<d1fifoll::D1FIFOLL_SPEC>;
///D1FIFO Port Register LL
pub mod d1fifoll;
/**D1FIFOHH (rw) register accessor: D1FIFO Port Register HH

You can [`read`](crate::Reg::read) this register and get [`d1fifohh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifohh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifohh`] module*/
pub type D1FIFOHH = crate::Reg<d1fifohh::D1FIFOHH_SPEC>;
///D1FIFO Port Register HH
pub mod d1fifohh;
/**CFIFOSEL (rw) register accessor: CFIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`cfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifosel`] module*/
pub type CFIFOSEL = crate::Reg<cfifosel::CFIFOSEL_SPEC>;
///CFIFO Port Select Register
pub mod cfifosel;
/**D0FIFOSEL (rw) register accessor: D0FIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`d0fifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifosel`] module*/
pub type D0FIFOSEL = crate::Reg<d0fifosel::D0FIFOSEL_SPEC>;
///D0FIFO Port Select Register
pub mod d0fifosel;
/**D1FIFOSEL (rw) register accessor: D1FIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`d1fifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifosel`] module*/
pub type D1FIFOSEL = crate::Reg<d1fifosel::D1FIFOSEL_SPEC>;
///D1FIFO Port Select Register
pub mod d1fifosel;
/**CFIFOCTR (rw) register accessor: CFIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoctr`] module*/
pub type CFIFOCTR = crate::Reg<cfifoctr::CFIFOCTR_SPEC>;
///CFIFO Port Control Register
pub mod cfifoctr;
/**D0FIFOCTR (rw) register accessor: D0FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`d0fifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifoctr`] module*/
pub type D0FIFOCTR = crate::Reg<d0fifoctr::D0FIFOCTR_SPEC>;
///D0FIFO Port Control Register
pub mod d0fifoctr;
/**D1FIFOCTR (rw) register accessor: D1FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`d1fifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifoctr`] module*/
pub type D1FIFOCTR = crate::Reg<d1fifoctr::D1FIFOCTR_SPEC>;
///D1FIFO Port Control Register
pub mod d1fifoctr;
/**INTENB0 (rw) register accessor: Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`intenb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intenb0`] module*/
pub type INTENB0 = crate::Reg<intenb0::INTENB0_SPEC>;
///Interrupt Enable Register 0
pub mod intenb0;
/**INTENB1 (rw) register accessor: Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`intenb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intenb1`] module*/
pub type INTENB1 = crate::Reg<intenb1::INTENB1_SPEC>;
///Interrupt Enable Register 1
pub mod intenb1;
/**BRDYENB (rw) register accessor: BRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`brdyenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brdyenb`] module*/
pub type BRDYENB = crate::Reg<brdyenb::BRDYENB_SPEC>;
///BRDY Interrupt Enable Register
pub mod brdyenb;
/**NRDYENB (rw) register accessor: NRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nrdyenb`] module*/
pub type NRDYENB = crate::Reg<nrdyenb::NRDYENB_SPEC>;
///NRDY Interrupt Enable Register
pub mod nrdyenb;
/**BEMPENB (rw) register accessor: BEMP Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`bempenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bempenb`] module*/
pub type BEMPENB = crate::Reg<bempenb::BEMPENB_SPEC>;
///BEMP Interrupt Enable Register
pub mod bempenb;
/**SOFCFG (rw) register accessor: SOF Pin Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sofcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sofcfg`] module*/
pub type SOFCFG = crate::Reg<sofcfg::SOFCFG_SPEC>;
///SOF Pin Configuration Register
pub mod sofcfg;
/**PHYSET (rw) register accessor: PHY Setting Register

You can [`read`](crate::Reg::read) this register and get [`physet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@physet`] module*/
pub type PHYSET = crate::Reg<physet::PHYSET_SPEC>;
///PHY Setting Register
pub mod physet;
/**INTSTS0 (rw) register accessor: Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`intsts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intsts0`] module*/
pub type INTSTS0 = crate::Reg<intsts0::INTSTS0_SPEC>;
///Interrupt Status Register 0
pub mod intsts0;
/**INTSTS1 (rw) register accessor: Interrupt Status Register 1

You can [`read`](crate::Reg::read) this register and get [`intsts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intsts1`] module*/
pub type INTSTS1 = crate::Reg<intsts1::INTSTS1_SPEC>;
///Interrupt Status Register 1
pub mod intsts1;
/**BRDYSTS (rw) register accessor: BRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`brdysts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brdysts`] module*/
pub type BRDYSTS = crate::Reg<brdysts::BRDYSTS_SPEC>;
///BRDY Interrupt Status Register
pub mod brdysts;
/**NRDYSTS (rw) register accessor: NRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nrdysts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nrdysts`] module*/
pub type NRDYSTS = crate::Reg<nrdysts::NRDYSTS_SPEC>;
///NRDY Interrupt Status Register
pub mod nrdysts;
/**BEMPSTS (rw) register accessor: BEMP Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`bempsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bempsts`] module*/
pub type BEMPSTS = crate::Reg<bempsts::BEMPSTS_SPEC>;
///BEMP Interrupt Status Register
pub mod bempsts;
/**FRMNUM (rw) register accessor: Frame Number Register

You can [`read`](crate::Reg::read) this register and get [`frmnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frmnum`] module*/
pub type FRMNUM = crate::Reg<frmnum::FRMNUM_SPEC>;
///Frame Number Register
pub mod frmnum;
/**UFRMNUM (rw) register accessor: uFrame Number Register

You can [`read`](crate::Reg::read) this register and get [`ufrmnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrmnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ufrmnum`] module*/
pub type UFRMNUM = crate::Reg<ufrmnum::UFRMNUM_SPEC>;
///uFrame Number Register
pub mod ufrmnum;
/**USBADDR (rw) register accessor: USB Address Register

You can [`read`](crate::Reg::read) this register and get [`usbaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbaddr`] module*/
pub type USBADDR = crate::Reg<usbaddr::USBADDR_SPEC>;
///USB Address Register
pub mod usbaddr;
/**USBREQ (rw) register accessor: USB Request Type Register

You can [`read`](crate::Reg::read) this register and get [`usbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbreq`] module*/
pub type USBREQ = crate::Reg<usbreq::USBREQ_SPEC>;
///USB Request Type Register
pub mod usbreq;
/**USBVAL (rw) register accessor: USB Request Value Register

You can [`read`](crate::Reg::read) this register and get [`usbval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbval`] module*/
pub type USBVAL = crate::Reg<usbval::USBVAL_SPEC>;
///USB Request Value Register
pub mod usbval;
/**USBINDX (rw) register accessor: USB Request Index Register

You can [`read`](crate::Reg::read) this register and get [`usbindx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbindx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbindx`] module*/
pub type USBINDX = crate::Reg<usbindx::USBINDX_SPEC>;
///USB Request Index Register
pub mod usbindx;
/**USBLENG (rw) register accessor: USB Request Length Register

You can [`read`](crate::Reg::read) this register and get [`usbleng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbleng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbleng`] module*/
pub type USBLENG = crate::Reg<usbleng::USBLENG_SPEC>;
///USB Request Length Register
pub mod usbleng;
/**DCPCFG (rw) register accessor: DCP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dcpcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcpcfg`] module*/
pub type DCPCFG = crate::Reg<dcpcfg::DCPCFG_SPEC>;
///DCP Configuration Register
pub mod dcpcfg;
/**DCPMAXP (rw) register accessor: DCP Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`dcpmaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpmaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcpmaxp`] module*/
pub type DCPMAXP = crate::Reg<dcpmaxp::DCPMAXP_SPEC>;
///DCP Maximum Packet Size Register
pub mod dcpmaxp;
/**DCPCTR (rw) register accessor: DCP Control Register

You can [`read`](crate::Reg::read) this register and get [`dcpctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcpctr`] module*/
pub type DCPCTR = crate::Reg<dcpctr::DCPCTR_SPEC>;
///DCP Control Register
pub mod dcpctr;
/**PIPESEL (rw) register accessor: Pipe Window Select Register

You can [`read`](crate::Reg::read) this register and get [`pipesel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipesel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipesel`] module*/
pub type PIPESEL = crate::Reg<pipesel::PIPESEL_SPEC>;
///Pipe Window Select Register
pub mod pipesel;
/**PIPECFG (rw) register accessor: Pipe Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pipecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipecfg`] module*/
pub type PIPECFG = crate::Reg<pipecfg::PIPECFG_SPEC>;
///Pipe Configuration Register
pub mod pipecfg;
/**PIPEBUF (rw) register accessor: Pipe Buffer Register

You can [`read`](crate::Reg::read) this register and get [`pipebuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipebuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipebuf`] module*/
pub type PIPEBUF = crate::Reg<pipebuf::PIPEBUF_SPEC>;
///Pipe Buffer Register
pub mod pipebuf;
/**PIPEMAXP (rw) register accessor: Pipe Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`pipemaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipemaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipemaxp`] module*/
pub type PIPEMAXP = crate::Reg<pipemaxp::PIPEMAXP_SPEC>;
///Pipe Maximum Packet Size Register
pub mod pipemaxp;
/**PIPEPERI (rw) register accessor: Pipe Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`pipeperi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipeperi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipeperi`] module*/
pub type PIPEPERI = crate::Reg<pipeperi::PIPEPERI_SPEC>;
///Pipe Cycle Control Register
pub mod pipeperi;
/**PIPECTR (rw) register accessor: PIPE Control Register

You can [`read`](crate::Reg::read) this register and get [`pipectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipectr`] module*/
pub type PIPECTR = crate::Reg<pipectr::PIPECTR_SPEC>;
///PIPE Control Register
pub mod pipectr;
/**PIPETRE (rw) register accessor: PIPE Transaction Counter Enable Register

You can [`read`](crate::Reg::read) this register and get [`pipetre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetre`] module*/
pub type PIPETRE = crate::Reg<pipetre::PIPETRE_SPEC>;
///PIPE Transaction Counter Enable Register
pub mod pipetre;
/**PIPETRN (rw) register accessor: PIPE Transaction Counter Register

You can [`read`](crate::Reg::read) this register and get [`pipetrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetrn`] module*/
pub type PIPETRN = crate::Reg<pipetrn::PIPETRN_SPEC>;
///PIPE Transaction Counter Register
pub mod pipetrn;
/**DEVADD (rw) register accessor: Device Address Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@devadd`] module*/
pub type DEVADD = crate::Reg<devadd::DEVADD_SPEC>;
///Device Address Configuration Register
pub mod devadd;
/**DEVADDA (rw) register accessor: Device Address Configuration Register A

You can [`read`](crate::Reg::read) this register and get [`devadda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@devadda`] module*/
pub type DEVADDA = crate::Reg<devadda::DEVADDA_SPEC>;
///Device Address Configuration Register A
pub mod devadda;
/**LPCTRL (rw) register accessor: Low Power Control Register

You can [`read`](crate::Reg::read) this register and get [`lpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpctrl`] module*/
pub type LPCTRL = crate::Reg<lpctrl::LPCTRL_SPEC>;
///Low Power Control Register
pub mod lpctrl;
/**LPSTS (rw) register accessor: Low Power Status Register

You can [`read`](crate::Reg::read) this register and get [`lpsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpsts`] module*/
pub type LPSTS = crate::Reg<lpsts::LPSTS_SPEC>;
///Low Power Status Register
pub mod lpsts;
/**BCCTRL (rw) register accessor: Battery Charging Control Register

You can [`read`](crate::Reg::read) this register and get [`bcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcctrl`] module*/
pub type BCCTRL = crate::Reg<bcctrl::BCCTRL_SPEC>;
///Battery Charging Control Register
pub mod bcctrl;
/**PL1CTRL1 (rw) register accessor: Function L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pl1ctrl1`] module*/
pub type PL1CTRL1 = crate::Reg<pl1ctrl1::PL1CTRL1_SPEC>;
///Function L1 Control Register 1
pub mod pl1ctrl1;
/**PL1CTRL2 (rw) register accessor: Function L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pl1ctrl2`] module*/
pub type PL1CTRL2 = crate::Reg<pl1ctrl2::PL1CTRL2_SPEC>;
///Function L1 Control Register 2
pub mod pl1ctrl2;
/**HL1CTRL1 (rw) register accessor: Host L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hl1ctrl1`] module*/
pub type HL1CTRL1 = crate::Reg<hl1ctrl1::HL1CTRL1_SPEC>;
///Host L1 Control Register 1
pub mod hl1ctrl1;
/**HL1CTRL2 (rw) register accessor: Host L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hl1ctrl2`] module*/
pub type HL1CTRL2 = crate::Reg<hl1ctrl2::HL1CTRL2_SPEC>;
///Host L1 Control Register 2
pub mod hl1ctrl2;
/**DPUSR0R (r) register accessor: Deep Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr0r`] module*/
pub type DPUSR0R = crate::Reg<dpusr0r::DPUSR0R_SPEC>;
///Deep Standby USB Transceiver Control/Pin Monitor Register
pub mod dpusr0r;
/**DPUSR1R (rw) register accessor: Deep Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr1r`] module*/
pub type DPUSR1R = crate::Reg<dpusr1r::DPUSR1R_SPEC>;
///Deep Standby USB Suspend/Resume Interrupt Register
pub mod dpusr1r;
/**DPUSR2R (rw) register accessor: Deep Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr2r`] module*/
pub type DPUSR2R = crate::Reg<dpusr2r::DPUSR2R_SPEC>;
///Deep Standby USB Suspend/Resume Interrupt Register
pub mod dpusr2r;
/**DPUSRCR (rw) register accessor: Deep Standby USB Suspend/Resume Command Register

You can [`read`](crate::Reg::read) this register and get [`dpusrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusrcr`] module*/
pub type DPUSRCR = crate::Reg<dpusrcr::DPUSRCR_SPEC>;
///Deep Standby USB Suspend/Resume Command Register
pub mod dpusrcr;
