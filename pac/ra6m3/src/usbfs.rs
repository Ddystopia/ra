#[repr(C)]
///Register block
pub struct RegisterBlock {
    syscfg: SYSCFG,
    _reserved1: [u8; 0x02],
    syssts0: SYSSTS0,
    _reserved2: [u8; 0x02],
    dvstctr0: DVSTCTR0,
    _reserved3: [u8; 0x0a],
    _reserved_3_cfifo: [u8; 0x02],
    _reserved4: [u8; 0x02],
    _reserved_4_d: [u8; 0x02],
    _reserved5: [u8; 0x02],
    _reserved_5_d: [u8; 0x02],
    _reserved6: [u8; 0x02],
    cfifosel: CFIFOSEL,
    cfifoctr: CFIFOCTR,
    _reserved8: [u8; 0x04],
    d0fifosel: D0FIFOSEL,
    d0fifoctr: D0FIFOCTR,
    d1fifosel: D1FIFOSEL,
    d1fifoctr: D1FIFOCTR,
    intenb0: INTENB0,
    intenb1: INTENB1,
    _reserved14: [u8; 0x02],
    brdyenb: BRDYENB,
    nrdyenb: NRDYENB,
    bempenb: BEMPENB,
    sofcfg: SOFCFG,
    _reserved18: [u8; 0x02],
    intsts0: INTSTS0,
    intsts1: INTSTS1,
    _reserved20: [u8; 0x02],
    brdysts: BRDYSTS,
    nrdysts: NRDYSTS,
    bempsts: BEMPSTS,
    frmnum: FRMNUM,
    dvchgr: DVCHGR,
    usbaddr: USBADDR,
    _reserved26: [u8; 0x02],
    usbreq: USBREQ,
    usbval: USBVAL,
    usbindx: USBINDX,
    usbleng: USBLENG,
    dcpcfg: DCPCFG,
    dcpmaxp: DCPMAXP,
    dcpctr: DCPCTR,
    _reserved33: [u8; 0x02],
    pipesel: PIPESEL,
    _reserved34: [u8; 0x02],
    pipecfg: PIPECFG,
    _reserved35: [u8; 0x02],
    pipemaxp: PIPEMAXP,
    pipeperi: PIPEPERI,
    pipectr: [PIPECTR; 5],
    pipe6ctr: PIPE6CTR,
    pipe7ctr: PIPE6CTR,
    pipe8ctr: PIPE6CTR,
    pipe9ctr: PIPE6CTR,
    _reserved42: [u8; 0x0e],
    pipetre: (),
    _reserved43: [u8; 0x02],
    pipetrn: (),
    _reserved44: [u8; 0x3e],
    devadd: [DEVADD; 6],
    _reserved45: [u8; 0x14],
    physlew: PHYSLEW,
    _reserved46: [u8; 0x030c],
    dpusr0r: DPUSR0R,
    dpusr1r: DPUSR1R,
}
impl RegisterBlock {
    ///0x00 - System Configuration Control Register
    #[inline(always)]
    pub const fn syscfg(&self) -> &SYSCFG {
        &self.syscfg
    }
    ///0x04 - System Configuration Status Register 0
    #[inline(always)]
    pub const fn syssts0(&self) -> &SYSSTS0 {
        &self.syssts0
    }
    ///0x08 - Device State Control Register 0
    #[inline(always)]
    pub const fn dvstctr0(&self) -> &DVSTCTR0 {
        &self.dvstctr0
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
    ///0x3c - SOF Output Configuration Register
    #[inline(always)]
    pub const fn sofcfg(&self) -> &SOFCFG {
        &self.sofcfg
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
    ///0x4e - Device State Change Register
    #[inline(always)]
    pub const fn dvchgr(&self) -> &DVCHGR {
        &self.dvchgr
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
    ///0x70..0x7a - Pipe %s Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1CTR` register.</div>
    #[inline(always)]
    pub const fn pipectr(&self, n: usize) -> &PIPECTR {
        &self.pipectr[n]
    }
    ///Iterator for array of:
    ///0x70..0x7a - Pipe %s Control Register
    #[inline(always)]
    pub fn pipectr_iter(&self) -> impl Iterator<Item = &PIPECTR> {
        self.pipectr.iter()
    }
    ///0x70 - Pipe 1 Control Register
    #[inline(always)]
    pub const fn pipe1ctr(&self) -> &PIPECTR {
        self.pipectr(0)
    }
    ///0x72 - Pipe 2 Control Register
    #[inline(always)]
    pub const fn pipe2ctr(&self) -> &PIPECTR {
        self.pipectr(1)
    }
    ///0x74 - Pipe 3 Control Register
    #[inline(always)]
    pub const fn pipe3ctr(&self) -> &PIPECTR {
        self.pipectr(2)
    }
    ///0x76 - Pipe 4 Control Register
    #[inline(always)]
    pub const fn pipe4ctr(&self) -> &PIPECTR {
        self.pipectr(3)
    }
    ///0x78 - Pipe 5 Control Register
    #[inline(always)]
    pub const fn pipe5ctr(&self) -> &PIPECTR {
        self.pipectr(4)
    }
    ///0x7a - Pipe 6 Control Register
    #[inline(always)]
    pub const fn pipe6ctr(&self) -> &PIPE6CTR {
        &self.pipe6ctr
    }
    ///0x7a - Pipe 7 Control Register
    #[inline(always)]
    pub const fn pipe7ctr(&self) -> &PIPE6CTR {
        &self.pipe7ctr
    }
    ///0x7a - Pipe 8 Control Register
    #[inline(always)]
    pub const fn pipe8ctr(&self) -> &PIPE6CTR {
        &self.pipe8ctr
    }
    ///0x7a - Pipe 9 Control Register
    #[inline(always)]
    pub const fn pipe9ctr(&self) -> &PIPE6CTR {
        &self.pipe9ctr
    }
    ///0x90..0x9a - Pipe %s Transaction Counter Enable Register
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
    ///0x90..0x9a - Pipe %s Transaction Counter Enable Register
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
    ///0x90 - Pipe 1 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe1tre(&self) -> &PIPETRE {
        self.pipetre(0)
    }
    ///0x94 - Pipe 2 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe2tre(&self) -> &PIPETRE {
        self.pipetre(1)
    }
    ///0x98 - Pipe 3 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe3tre(&self) -> &PIPETRE {
        self.pipetre(2)
    }
    ///0x9c - Pipe 4 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe4tre(&self) -> &PIPETRE {
        self.pipetre(3)
    }
    ///0xa0 - Pipe 5 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe5tre(&self) -> &PIPETRE {
        self.pipetre(4)
    }
    ///0x92..0x9c - Pipe %s Transaction Counter Register
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
    ///0x92..0x9c - Pipe %s Transaction Counter Register
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
    ///0x92 - Pipe 1 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe1trn(&self) -> &PIPETRN {
        self.pipetrn(0)
    }
    ///0x96 - Pipe 2 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe2trn(&self) -> &PIPETRN {
        self.pipetrn(1)
    }
    ///0x9a - Pipe 3 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe3trn(&self) -> &PIPETRN {
        self.pipetrn(2)
    }
    ///0x9e - Pipe 4 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe4trn(&self) -> &PIPETRN {
        self.pipetrn(3)
    }
    ///0xa2 - Pipe 5 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe5trn(&self) -> &PIPETRN {
        self.pipetrn(4)
    }
    ///0xd0..0xdc - Device Address %s Configuration Register
    #[inline(always)]
    pub const fn devadd(&self, n: usize) -> &DEVADD {
        &self.devadd[n]
    }
    ///Iterator for array of:
    ///0xd0..0xdc - Device Address %s Configuration Register
    #[inline(always)]
    pub fn devadd_iter(&self) -> impl Iterator<Item = &DEVADD> {
        self.devadd.iter()
    }
    ///0xf0 - PHY Cross Point Adjustment Register
    #[inline(always)]
    pub const fn physlew(&self) -> &PHYSLEW {
        &self.physlew
    }
    ///0x400 - Deep Software Standby USB Transceiver Control/Pin Monitor Register
    #[inline(always)]
    pub const fn dpusr0r(&self) -> &DPUSR0R {
        &self.dpusr0r
    }
    ///0x404 - Deep Software Standby USB Suspend/Resume Interrupt Register
    #[inline(always)]
    pub const fn dpusr1r(&self) -> &DPUSR1R {
        &self.dpusr1r
    }
}
/**SYSCFG (rw) register accessor: System Configuration Control Register

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscfg`] module*/
pub type SYSCFG = crate::Reg<syscfg::SYSCFG_SPEC>;
///System Configuration Control Register
pub mod syscfg;
/**SYSSTS0 (r) register accessor: System Configuration Status Register 0

You can [`read`](crate::Reg::read) this register and get [`syssts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syssts0`] module*/
pub type SYSSTS0 = crate::Reg<syssts0::SYSSTS0_SPEC>;
///System Configuration Status Register 0
pub mod syssts0;
/**DVSTCTR0 (rw) register accessor: Device State Control Register 0

You can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvstctr0`] module*/
pub type DVSTCTR0 = crate::Reg<dvstctr0::DVSTCTR0_SPEC>;
///Device State Control Register 0
pub mod dvstctr0;
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
/**CFIFOSEL (rw) register accessor: CFIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`cfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifosel`] module*/
pub type CFIFOSEL = crate::Reg<cfifosel::CFIFOSEL_SPEC>;
///CFIFO Port Select Register
pub mod cfifosel;
/**CFIFOCTR (rw) register accessor: CFIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoctr`] module*/
pub type CFIFOCTR = crate::Reg<cfifoctr::CFIFOCTR_SPEC>;
///CFIFO Port Control Register
pub mod cfifoctr;
/**D0FIFOSEL (rw) register accessor: D0FIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`d0fifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifosel`] module*/
pub type D0FIFOSEL = crate::Reg<d0fifosel::D0FIFOSEL_SPEC>;
///D0FIFO Port Select Register
pub mod d0fifosel;
/**D0FIFOCTR (rw) register accessor: D0FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`d0fifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifoctr`] module*/
pub type D0FIFOCTR = crate::Reg<d0fifoctr::D0FIFOCTR_SPEC>;
///D0FIFO Port Control Register
pub mod d0fifoctr;
/**D1FIFOSEL (rw) register accessor: D1FIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`d1fifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifosel`] module*/
pub type D1FIFOSEL = crate::Reg<d1fifosel::D1FIFOSEL_SPEC>;
///D1FIFO Port Select Register
pub mod d1fifosel;
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
/**SOFCFG (rw) register accessor: SOF Output Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sofcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sofcfg`] module*/
pub type SOFCFG = crate::Reg<sofcfg::SOFCFG_SPEC>;
///SOF Output Configuration Register
pub mod sofcfg;
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
/**DVCHGR (rw) register accessor: Device State Change Register

You can [`read`](crate::Reg::read) this register and get [`dvchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvchgr`] module*/
pub type DVCHGR = crate::Reg<dvchgr::DVCHGR_SPEC>;
///Device State Change Register
pub mod dvchgr;
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
/**PIPECTR (rw) register accessor: Pipe %s Control Register

You can [`read`](crate::Reg::read) this register and get [`pipectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipectr`] module*/
pub type PIPECTR = crate::Reg<pipectr::PIPECTR_SPEC>;
///Pipe %s Control Register
pub mod pipectr;
pub use pipectr as pipe6ctr;
pub use PIPECTR as PIPE6CTR;
/**PIPETRE (rw) register accessor: Pipe %s Transaction Counter Enable Register

You can [`read`](crate::Reg::read) this register and get [`pipetre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetre`] module*/
pub type PIPETRE = crate::Reg<pipetre::PIPETRE_SPEC>;
///Pipe %s Transaction Counter Enable Register
pub mod pipetre;
/**PIPETRN (rw) register accessor: Pipe %s Transaction Counter Register

You can [`read`](crate::Reg::read) this register and get [`pipetrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetrn`] module*/
pub type PIPETRN = crate::Reg<pipetrn::PIPETRN_SPEC>;
///Pipe %s Transaction Counter Register
pub mod pipetrn;
/**DEVADD (rw) register accessor: Device Address %s Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@devadd`] module*/
pub type DEVADD = crate::Reg<devadd::DEVADD_SPEC>;
///Device Address %s Configuration Register
pub mod devadd;
/**PHYSLEW (rw) register accessor: PHY Cross Point Adjustment Register

You can [`read`](crate::Reg::read) this register and get [`physlew::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physlew::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@physlew`] module*/
pub type PHYSLEW = crate::Reg<physlew::PHYSLEW_SPEC>;
///PHY Cross Point Adjustment Register
pub mod physlew;
/**DPUSR0R (rw) register accessor: Deep Software Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr0r`] module*/
pub type DPUSR0R = crate::Reg<dpusr0r::DPUSR0R_SPEC>;
///Deep Software Standby USB Transceiver Control/Pin Monitor Register
pub mod dpusr0r;
/**DPUSR1R (rw) register accessor: Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr1r`] module*/
pub type DPUSR1R = crate::Reg<dpusr1r::DPUSR1R_SPEC>;
///Deep Software Standby USB Suspend/Resume Interrupt Register
pub mod dpusr1r;
