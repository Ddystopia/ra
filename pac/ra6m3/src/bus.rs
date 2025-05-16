#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    csmod: (),
    _reserved1: [u8; 0x02],
    cswcr1: (),
    _reserved2: [u8; 0x04],
    cswcr2: (),
    _reserved3: [u8; 0x07fa],
    cs0cr: CS0CR,
    _reserved4: [u8; 0x06],
    csrec: (),
    _reserved5: [u8; 0x08],
    cscr: (),
    _reserved6: [u8; 0x6e],
    csrecen: CSRECEN,
    _reserved7: [u8; 0x037e],
    sdccr: SDCCR,
    sdcmod: SDCMOD,
    sdamod: SDAMOD,
    _reserved10: [u8; 0x0d],
    sdself: SDSELF,
    _reserved11: [u8; 0x03],
    sdrfcr: SDRFCR,
    sdrfen: SDRFEN,
    _reserved13: [u8; 0x09],
    sdicr: SDICR,
    _reserved14: [u8; 0x03],
    sdir: SDIR,
    _reserved15: [u8; 0x1a],
    sdadr: SDADR,
    _reserved16: [u8; 0x03],
    sdtr: SDTR,
    sdmod: SDMOD,
    _reserved18: [u8; 0x06],
    sdsr: SDSR,
    _reserved19: [u8; 0x03af],
    busmcnt: (),
    _reserved20: [u8; 0x08],
    busmcntsys: BUSMCNTSYS,
    _reserved21: [u8; 0x02],
    busmcntdma: BUSMCNTDMA,
    _reserved22: [u8; 0x02],
    busmcntedm: BUSMCNTEDM,
    _reserved23: [u8; 0x02],
    busmcntgpx: BUSMCNTEDM,
    _reserved24: [u8; 0xea],
    busscnt: (),
    _reserved25: [u8; 0x08],
    busscntmbiu: BUSSCNTMBIU,
    _reserved26: [u8; 0x02],
    busscntram0: BUSSCNTRAM0,
    _reserved27: [u8; 0x02],
    busscntram1: BUSSCNTRAM0,
    _reserved28: [u8; 0x02],
    busscntp0b: BUSSCNTP0B,
    _reserved29: [u8; 0x02],
    busscntp2b: BUSSCNTP0B,
    _reserved30: [u8; 0x02],
    busscntp3b: BUSSCNTP0B,
    _reserved31: [u8; 0x02],
    busscntp4b: BUSSCNTP0B,
    _reserved32: [u8; 0x06],
    busscntp6b: BUSSCNTP6B,
    _reserved33: [u8; 0x02],
    busscntp7b: BUSSCNTP6B,
    _reserved34: [u8; 0x02],
    busscntfbu: BUSSCNTFBU,
    _reserved35: [u8; 0x02],
    busscntext: BUSSCNTFBU,
    _reserved36: [u8; 0x02],
    busscntext2: BUSSCNTFBU,
    _reserved37: [u8; 0x02],
    busscntgpx: BUSSCNTFBU,
    _reserved38: [u8; 0x06c2],
    buserradd: (),
    _reserved39: [u8; 0x04],
    buserrstat: (),
}
impl RegisterBlock {
    ///0x02..0x12 - CS%s Mode Register
    #[inline(always)]
    pub const fn csmod(&self, n: usize) -> &CSMOD {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x02..0x12 - CS%s Mode Register
    #[inline(always)]
    pub fn csmod_iter(&self) -> impl Iterator<Item = &CSMOD> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(16 * n)
                .cast()
        })
    }
    ///0x02 - CS0 Mode Register
    #[inline(always)]
    pub const fn cs0mod(&self) -> &CSMOD {
        self.csmod(0)
    }
    ///0x12 - CS1 Mode Register
    #[inline(always)]
    pub const fn cs1mod(&self) -> &CSMOD {
        self.csmod(1)
    }
    ///0x22 - CS2 Mode Register
    #[inline(always)]
    pub const fn cs2mod(&self) -> &CSMOD {
        self.csmod(2)
    }
    ///0x32 - CS3 Mode Register
    #[inline(always)]
    pub const fn cs3mod(&self) -> &CSMOD {
        self.csmod(3)
    }
    ///0x42 - CS4 Mode Register
    #[inline(always)]
    pub const fn cs4mod(&self) -> &CSMOD {
        self.csmod(4)
    }
    ///0x52 - CS5 Mode Register
    #[inline(always)]
    pub const fn cs5mod(&self) -> &CSMOD {
        self.csmod(5)
    }
    ///0x62 - CS6 Mode Register
    #[inline(always)]
    pub const fn cs6mod(&self) -> &CSMOD {
        self.csmod(6)
    }
    ///0x72 - CS7 Mode Register
    #[inline(always)]
    pub const fn cs7mod(&self) -> &CSMOD {
        self.csmod(7)
    }
    ///0x04..0x24 - CS%s Wait Control Register 1
    #[inline(always)]
    pub const fn cswcr1(&self, n: usize) -> &CSWCR1 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x04..0x24 - CS%s Wait Control Register 1
    #[inline(always)]
    pub fn cswcr1_iter(&self) -> impl Iterator<Item = &CSWCR1> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        })
    }
    ///0x04 - CS0 Wait Control Register 1
    #[inline(always)]
    pub const fn cs0wcr1(&self) -> &CSWCR1 {
        self.cswcr1(0)
    }
    ///0x14 - CS1 Wait Control Register 1
    #[inline(always)]
    pub const fn cs1wcr1(&self) -> &CSWCR1 {
        self.cswcr1(1)
    }
    ///0x24 - CS2 Wait Control Register 1
    #[inline(always)]
    pub const fn cs2wcr1(&self) -> &CSWCR1 {
        self.cswcr1(2)
    }
    ///0x34 - CS3 Wait Control Register 1
    #[inline(always)]
    pub const fn cs3wcr1(&self) -> &CSWCR1 {
        self.cswcr1(3)
    }
    ///0x44 - CS4 Wait Control Register 1
    #[inline(always)]
    pub const fn cs4wcr1(&self) -> &CSWCR1 {
        self.cswcr1(4)
    }
    ///0x54 - CS5 Wait Control Register 1
    #[inline(always)]
    pub const fn cs5wcr1(&self) -> &CSWCR1 {
        self.cswcr1(5)
    }
    ///0x64 - CS6 Wait Control Register 1
    #[inline(always)]
    pub const fn cs6wcr1(&self) -> &CSWCR1 {
        self.cswcr1(6)
    }
    ///0x74 - CS7 Wait Control Register 1
    #[inline(always)]
    pub const fn cs7wcr1(&self) -> &CSWCR1 {
        self.cswcr1(7)
    }
    ///0x08..0x28 - CS%s Wait Control Register 2
    #[inline(always)]
    pub const fn cswcr2(&self, n: usize) -> &CSWCR2 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x08..0x28 - CS%s Wait Control Register 2
    #[inline(always)]
    pub fn cswcr2_iter(&self) -> impl Iterator<Item = &CSWCR2> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        })
    }
    ///0x08 - CS0 Wait Control Register 2
    #[inline(always)]
    pub const fn cs0wcr2(&self) -> &CSWCR2 {
        self.cswcr2(0)
    }
    ///0x18 - CS1 Wait Control Register 2
    #[inline(always)]
    pub const fn cs1wcr2(&self) -> &CSWCR2 {
        self.cswcr2(1)
    }
    ///0x28 - CS2 Wait Control Register 2
    #[inline(always)]
    pub const fn cs2wcr2(&self) -> &CSWCR2 {
        self.cswcr2(2)
    }
    ///0x38 - CS3 Wait Control Register 2
    #[inline(always)]
    pub const fn cs3wcr2(&self) -> &CSWCR2 {
        self.cswcr2(3)
    }
    ///0x48 - CS4 Wait Control Register 2
    #[inline(always)]
    pub const fn cs4wcr2(&self) -> &CSWCR2 {
        self.cswcr2(4)
    }
    ///0x58 - CS5 Wait Control Register 2
    #[inline(always)]
    pub const fn cs5wcr2(&self) -> &CSWCR2 {
        self.cswcr2(5)
    }
    ///0x68 - CS6 Wait Control Register 2
    #[inline(always)]
    pub const fn cs6wcr2(&self) -> &CSWCR2 {
        self.cswcr2(6)
    }
    ///0x78 - CS7 Wait Control Register 2
    #[inline(always)]
    pub const fn cs7wcr2(&self) -> &CSWCR2 {
        self.cswcr2(7)
    }
    ///0x802 - CS0 Control Register
    #[inline(always)]
    pub const fn cs0cr(&self) -> &CS0CR {
        &self.cs0cr
    }
    ///0x80a..0x81a - CS%s Recovery Cycle Register
    #[inline(always)]
    pub const fn csrec(&self, n: usize) -> &CSREC {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2058)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x80a..0x81a - CS%s Recovery Cycle Register
    #[inline(always)]
    pub fn csrec_iter(&self) -> impl Iterator<Item = &CSREC> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2058)
                .add(16 * n)
                .cast()
        })
    }
    ///0x80a - CS0 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs0rec(&self) -> &CSREC {
        self.csrec(0)
    }
    ///0x81a - CS1 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs1rec(&self) -> &CSREC {
        self.csrec(1)
    }
    ///0x82a - CS2 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs2rec(&self) -> &CSREC {
        self.csrec(2)
    }
    ///0x83a - CS3 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs3rec(&self) -> &CSREC {
        self.csrec(3)
    }
    ///0x84a - CS4 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs4rec(&self) -> &CSREC {
        self.csrec(4)
    }
    ///0x85a - CS5 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs5rec(&self) -> &CSREC {
        self.csrec(5)
    }
    ///0x86a - CS6 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs6rec(&self) -> &CSREC {
        self.csrec(6)
    }
    ///0x87a - CS7 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs7rec(&self) -> &CSREC {
        self.csrec(7)
    }
    ///0x812..0x820 - CS%s Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `CS1CR` register.</div>
    #[inline(always)]
    pub const fn cscr(&self, n: usize) -> &CSCR {
        #[allow(clippy::no_effect)]
        [(); 7][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2066)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x812..0x820 - CS%s Control Register
    #[inline(always)]
    pub fn cscr_iter(&self) -> impl Iterator<Item = &CSCR> {
        (0..7).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2066)
                .add(16 * n)
                .cast()
        })
    }
    ///0x812 - CS1 Control Register
    #[inline(always)]
    pub const fn cs1cr(&self) -> &CSCR {
        self.cscr(0)
    }
    ///0x822 - CS2 Control Register
    #[inline(always)]
    pub const fn cs2cr(&self) -> &CSCR {
        self.cscr(1)
    }
    ///0x832 - CS3 Control Register
    #[inline(always)]
    pub const fn cs3cr(&self) -> &CSCR {
        self.cscr(2)
    }
    ///0x842 - CS4 Control Register
    #[inline(always)]
    pub const fn cs4cr(&self) -> &CSCR {
        self.cscr(3)
    }
    ///0x852 - CS5 Control Register
    #[inline(always)]
    pub const fn cs5cr(&self) -> &CSCR {
        self.cscr(4)
    }
    ///0x862 - CS6 Control Register
    #[inline(always)]
    pub const fn cs6cr(&self) -> &CSCR {
        self.cscr(5)
    }
    ///0x872 - CS7 Control Register
    #[inline(always)]
    pub const fn cs7cr(&self) -> &CSCR {
        self.cscr(6)
    }
    ///0x880 - CS Recovery Cycle Insertion Enable Register
    #[inline(always)]
    pub const fn csrecen(&self) -> &CSRECEN {
        &self.csrecen
    }
    ///0xc00 - SDC Control Register
    #[inline(always)]
    pub const fn sdccr(&self) -> &SDCCR {
        &self.sdccr
    }
    ///0xc01 - SDC Mode Register
    #[inline(always)]
    pub const fn sdcmod(&self) -> &SDCMOD {
        &self.sdcmod
    }
    ///0xc02 - SDRAM Access Mode Register
    #[inline(always)]
    pub const fn sdamod(&self) -> &SDAMOD {
        &self.sdamod
    }
    ///0xc10 - SDRAM Self-Refresh Control Register
    #[inline(always)]
    pub const fn sdself(&self) -> &SDSELF {
        &self.sdself
    }
    ///0xc14 - SDRAM Refresh Control Register
    #[inline(always)]
    pub const fn sdrfcr(&self) -> &SDRFCR {
        &self.sdrfcr
    }
    ///0xc16 - SDRAM Auto-Refresh Control Register
    #[inline(always)]
    pub const fn sdrfen(&self) -> &SDRFEN {
        &self.sdrfen
    }
    ///0xc20 - SDRAM Initialization Sequence Control Register
    #[inline(always)]
    pub const fn sdicr(&self) -> &SDICR {
        &self.sdicr
    }
    ///0xc24 - SDRAM Initialization Register
    #[inline(always)]
    pub const fn sdir(&self) -> &SDIR {
        &self.sdir
    }
    ///0xc40 - SDRAM Address Register
    #[inline(always)]
    pub const fn sdadr(&self) -> &SDADR {
        &self.sdadr
    }
    ///0xc44 - SDRAM Timing Register
    #[inline(always)]
    pub const fn sdtr(&self) -> &SDTR {
        &self.sdtr
    }
    ///0xc48 - SDRAM Mode Register
    #[inline(always)]
    pub const fn sdmod(&self) -> &SDMOD {
        &self.sdmod
    }
    ///0xc50 - SDRAM Status Register
    #[inline(always)]
    pub const fn sdsr(&self) -> &SDSR {
        &self.sdsr
    }
    ///0x1000 - Master Bus Control Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUSMCNTM4I` register.</div>
    #[inline(always)]
    pub const fn busmcnt(&self, n: usize) -> &BUSMCNT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1000 - Master Bus Control Register %s
    #[inline(always)]
    pub fn busmcnt_iter(&self) -> impl Iterator<Item = &BUSMCNT> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        })
    }
    ///0x1000 - Master Bus Control Register M4I
    #[inline(always)]
    pub const fn busmcntm4i(&self) -> &BUSMCNT {
        self.busmcnt(0)
    }
    ///0x1004 - Master Bus Control Register M4D
    #[inline(always)]
    pub const fn busmcntm4d(&self) -> &BUSMCNT {
        self.busmcnt(1)
    }
    ///0x1008 - Master Bus Control Register SYS
    #[inline(always)]
    pub const fn busmcntsys(&self) -> &BUSMCNTSYS {
        &self.busmcntsys
    }
    ///0x100c - Master Bus Control Register DMA
    #[inline(always)]
    pub const fn busmcntdma(&self) -> &BUSMCNTDMA {
        &self.busmcntdma
    }
    ///0x1010 - Master Bus Control Register EDM
    #[inline(always)]
    pub const fn busmcntedm(&self) -> &BUSMCNTEDM {
        &self.busmcntedm
    }
    ///0x1010 - Master Bus Control Register GPX
    #[inline(always)]
    pub const fn busmcntgpx(&self) -> &BUSMCNTEDM {
        &self.busmcntgpx
    }
    ///0x1100 - Slave Bus Control Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUSSCNTFLI` register.</div>
    #[inline(always)]
    pub const fn busscnt(&self, n: usize) -> &BUSSCNT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4352)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1100 - Slave Bus Control Register %s
    #[inline(always)]
    pub fn busscnt_iter(&self) -> impl Iterator<Item = &BUSSCNT> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4352)
                .add(4 * n)
                .cast()
        })
    }
    ///0x1100 - Slave Bus Control Register FLI
    #[inline(always)]
    pub const fn busscntfli(&self) -> &BUSSCNT {
        self.busscnt(0)
    }
    ///0x1104 - Slave Bus Control Register RAMH
    #[inline(always)]
    pub const fn busscntramh(&self) -> &BUSSCNT {
        self.busscnt(1)
    }
    ///0x1108 - Slave Bus Control Register MBIU
    #[inline(always)]
    pub const fn busscntmbiu(&self) -> &BUSSCNTMBIU {
        &self.busscntmbiu
    }
    ///0x110c - Slave Bus Control Register RAM0
    #[inline(always)]
    pub const fn busscntram0(&self) -> &BUSSCNTRAM0 {
        &self.busscntram0
    }
    ///0x110c - Slave Bus Control Register RAM1
    #[inline(always)]
    pub const fn busscntram1(&self) -> &BUSSCNTRAM0 {
        &self.busscntram1
    }
    ///0x1114 - Slave Bus Control Register P0B
    #[inline(always)]
    pub const fn busscntp0b(&self) -> &BUSSCNTP0B {
        &self.busscntp0b
    }
    ///0x1114 - Slave Bus Control Register P2B
    #[inline(always)]
    pub const fn busscntp2b(&self) -> &BUSSCNTP0B {
        &self.busscntp2b
    }
    ///0x1114 - Slave Bus Control Register P3B
    #[inline(always)]
    pub const fn busscntp3b(&self) -> &BUSSCNTP0B {
        &self.busscntp3b
    }
    ///0x1114 - Slave Bus Control Register P4B
    #[inline(always)]
    pub const fn busscntp4b(&self) -> &BUSSCNTP0B {
        &self.busscntp4b
    }
    ///0x1128 - Slave Bus Control Register P6B
    #[inline(always)]
    pub const fn busscntp6b(&self) -> &BUSSCNTP6B {
        &self.busscntp6b
    }
    ///0x1128 - Slave Bus Control Register P7B
    #[inline(always)]
    pub const fn busscntp7b(&self) -> &BUSSCNTP6B {
        &self.busscntp7b
    }
    ///0x1130 - Slave Bus Control Register FBU
    #[inline(always)]
    pub const fn busscntfbu(&self) -> &BUSSCNTFBU {
        &self.busscntfbu
    }
    ///0x1130 - Slave Bus Control Register EXT
    #[inline(always)]
    pub const fn busscntext(&self) -> &BUSSCNTFBU {
        &self.busscntext
    }
    ///0x1130 - Slave Bus Control Register EXT2
    #[inline(always)]
    pub const fn busscntext2(&self) -> &BUSSCNTFBU {
        &self.busscntext2
    }
    ///0x1130 - Slave Bus Control Register GPX
    #[inline(always)]
    pub const fn busscntgpx(&self) -> &BUSSCNTFBU {
        &self.busscntgpx
    }
    ///0x1800..0x182c - Bus Error Address Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRADD` register.</div>
    #[inline(always)]
    pub const fn buserradd(&self, n: usize) -> &BUSERRADD {
        #[allow(clippy::no_effect)]
        [(); 11][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6144)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1800..0x182c - Bus Error Address Register %s
    #[inline(always)]
    pub fn buserradd_iter(&self) -> impl Iterator<Item = &BUSERRADD> {
        (0..11).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6144)
                .add(16 * n)
                .cast()
        })
    }
    ///0x1800 - Bus Error Address Register 1
    #[inline(always)]
    pub const fn bus1erradd(&self) -> &BUSERRADD {
        self.buserradd(0)
    }
    ///0x1810 - Bus Error Address Register 2
    #[inline(always)]
    pub const fn bus2erradd(&self) -> &BUSERRADD {
        self.buserradd(1)
    }
    ///0x1820 - Bus Error Address Register 3
    #[inline(always)]
    pub const fn bus3erradd(&self) -> &BUSERRADD {
        self.buserradd(2)
    }
    ///0x1830 - Bus Error Address Register 4
    #[inline(always)]
    pub const fn bus4erradd(&self) -> &BUSERRADD {
        self.buserradd(3)
    }
    ///0x1840 - Bus Error Address Register 5
    #[inline(always)]
    pub const fn bus5erradd(&self) -> &BUSERRADD {
        self.buserradd(4)
    }
    ///0x1850 - Bus Error Address Register 6
    #[inline(always)]
    pub const fn bus6erradd(&self) -> &BUSERRADD {
        self.buserradd(5)
    }
    ///0x1860 - Bus Error Address Register 7
    #[inline(always)]
    pub const fn bus7erradd(&self) -> &BUSERRADD {
        self.buserradd(6)
    }
    ///0x1870 - Bus Error Address Register 8
    #[inline(always)]
    pub const fn bus8erradd(&self) -> &BUSERRADD {
        self.buserradd(7)
    }
    ///0x1880 - Bus Error Address Register 9
    #[inline(always)]
    pub const fn bus9erradd(&self) -> &BUSERRADD {
        self.buserradd(8)
    }
    ///0x1890 - Bus Error Address Register 10
    #[inline(always)]
    pub const fn bus10erradd(&self) -> &BUSERRADD {
        self.buserradd(9)
    }
    ///0x18a0 - Bus Error Address Register 11
    #[inline(always)]
    pub const fn bus11erradd(&self) -> &BUSERRADD {
        self.buserradd(10)
    }
    ///0x1804..0x180f - Bus Error Status Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRSTAT` register.</div>
    #[inline(always)]
    pub const fn buserrstat(&self, n: usize) -> &BUSERRSTAT {
        #[allow(clippy::no_effect)]
        [(); 11][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6148)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1804..0x180f - Bus Error Status Register %s
    #[inline(always)]
    pub fn buserrstat_iter(&self) -> impl Iterator<Item = &BUSERRSTAT> {
        (0..11).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6148)
                .add(16 * n)
                .cast()
        })
    }
    ///0x1804 - Bus Error Status Register 1
    #[inline(always)]
    pub const fn bus1errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(0)
    }
    ///0x1814 - Bus Error Status Register 2
    #[inline(always)]
    pub const fn bus2errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(1)
    }
    ///0x1824 - Bus Error Status Register 3
    #[inline(always)]
    pub const fn bus3errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(2)
    }
    ///0x1834 - Bus Error Status Register 4
    #[inline(always)]
    pub const fn bus4errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(3)
    }
    ///0x1844 - Bus Error Status Register 5
    #[inline(always)]
    pub const fn bus5errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(4)
    }
    ///0x1854 - Bus Error Status Register 6
    #[inline(always)]
    pub const fn bus6errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(5)
    }
    ///0x1864 - Bus Error Status Register 7
    #[inline(always)]
    pub const fn bus7errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(6)
    }
    ///0x1874 - Bus Error Status Register 8
    #[inline(always)]
    pub const fn bus8errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(7)
    }
    ///0x1884 - Bus Error Status Register 9
    #[inline(always)]
    pub const fn bus9errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(8)
    }
    ///0x1894 - Bus Error Status Register 10
    #[inline(always)]
    pub const fn bus10errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(9)
    }
    ///0x18a4 - Bus Error Status Register 11
    #[inline(always)]
    pub const fn bus11errstat(&self) -> &BUSERRSTAT {
        self.buserrstat(10)
    }
}
/**CS0CR (rw) register accessor: CS0 Control Register

You can [`read`](crate::Reg::read) this register and get [`cs0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cs0cr`] module*/
pub type CS0CR = crate::Reg<cs0cr::CS0CR_SPEC>;
///CS0 Control Register
pub mod cs0cr;
/**CSCR (rw) register accessor: CS%s Control Register

You can [`read`](crate::Reg::read) this register and get [`cscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cscr`] module*/
pub type CSCR = crate::Reg<cscr::CSCR_SPEC>;
///CS%s Control Register
pub mod cscr;
/**CSREC (rw) register accessor: CS%s Recovery Cycle Register

You can [`read`](crate::Reg::read) this register and get [`csrec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csrec`] module*/
pub type CSREC = crate::Reg<csrec::CSREC_SPEC>;
///CS%s Recovery Cycle Register
pub mod csrec;
/**CSRECEN (rw) register accessor: CS Recovery Cycle Insertion Enable Register

You can [`read`](crate::Reg::read) this register and get [`csrecen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrecen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csrecen`] module*/
pub type CSRECEN = crate::Reg<csrecen::CSRECEN_SPEC>;
///CS Recovery Cycle Insertion Enable Register
pub mod csrecen;
/**CSMOD (rw) register accessor: CS%s Mode Register

You can [`read`](crate::Reg::read) this register and get [`csmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csmod`] module*/
pub type CSMOD = crate::Reg<csmod::CSMOD_SPEC>;
///CS%s Mode Register
pub mod csmod;
/**CSWCR1 (rw) register accessor: CS%s Wait Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cswcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cswcr1`] module*/
pub type CSWCR1 = crate::Reg<cswcr1::CSWCR1_SPEC>;
///CS%s Wait Control Register 1
pub mod cswcr1;
/**CSWCR2 (rw) register accessor: CS%s Wait Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cswcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cswcr2`] module*/
pub type CSWCR2 = crate::Reg<cswcr2::CSWCR2_SPEC>;
///CS%s Wait Control Register 2
pub mod cswcr2;
/**SDCCR (rw) register accessor: SDC Control Register

You can [`read`](crate::Reg::read) this register and get [`sdccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdccr`] module*/
pub type SDCCR = crate::Reg<sdccr::SDCCR_SPEC>;
///SDC Control Register
pub mod sdccr;
/**SDCMOD (rw) register accessor: SDC Mode Register

You can [`read`](crate::Reg::read) this register and get [`sdcmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdcmod`] module*/
pub type SDCMOD = crate::Reg<sdcmod::SDCMOD_SPEC>;
///SDC Mode Register
pub mod sdcmod;
/**SDAMOD (rw) register accessor: SDRAM Access Mode Register

You can [`read`](crate::Reg::read) this register and get [`sdamod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdamod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdamod`] module*/
pub type SDAMOD = crate::Reg<sdamod::SDAMOD_SPEC>;
///SDRAM Access Mode Register
pub mod sdamod;
/**SDSELF (rw) register accessor: SDRAM Self-Refresh Control Register

You can [`read`](crate::Reg::read) this register and get [`sdself::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdself::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdself`] module*/
pub type SDSELF = crate::Reg<sdself::SDSELF_SPEC>;
///SDRAM Self-Refresh Control Register
pub mod sdself;
/**SDRFCR (rw) register accessor: SDRAM Refresh Control Register

You can [`read`](crate::Reg::read) this register and get [`sdrfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdrfcr`] module*/
pub type SDRFCR = crate::Reg<sdrfcr::SDRFCR_SPEC>;
///SDRAM Refresh Control Register
pub mod sdrfcr;
/**SDRFEN (rw) register accessor: SDRAM Auto-Refresh Control Register

You can [`read`](crate::Reg::read) this register and get [`sdrfen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrfen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdrfen`] module*/
pub type SDRFEN = crate::Reg<sdrfen::SDRFEN_SPEC>;
///SDRAM Auto-Refresh Control Register
pub mod sdrfen;
/**SDICR (rw) register accessor: SDRAM Initialization Sequence Control Register

You can [`read`](crate::Reg::read) this register and get [`sdicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdicr`] module*/
pub type SDICR = crate::Reg<sdicr::SDICR_SPEC>;
///SDRAM Initialization Sequence Control Register
pub mod sdicr;
/**SDIR (rw) register accessor: SDRAM Initialization Register

You can [`read`](crate::Reg::read) this register and get [`sdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdir`] module*/
pub type SDIR = crate::Reg<sdir::SDIR_SPEC>;
///SDRAM Initialization Register
pub mod sdir;
/**SDADR (rw) register accessor: SDRAM Address Register

You can [`read`](crate::Reg::read) this register and get [`sdadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdadr`] module*/
pub type SDADR = crate::Reg<sdadr::SDADR_SPEC>;
///SDRAM Address Register
pub mod sdadr;
/**SDTR (rw) register accessor: SDRAM Timing Register

You can [`read`](crate::Reg::read) this register and get [`sdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdtr`] module*/
pub type SDTR = crate::Reg<sdtr::SDTR_SPEC>;
///SDRAM Timing Register
pub mod sdtr;
/**SDMOD (rw) register accessor: SDRAM Mode Register

You can [`read`](crate::Reg::read) this register and get [`sdmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdmod`] module*/
pub type SDMOD = crate::Reg<sdmod::SDMOD_SPEC>;
///SDRAM Mode Register
pub mod sdmod;
/**SDSR (r) register accessor: SDRAM Status Register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdsr`] module*/
pub type SDSR = crate::Reg<sdsr::SDSR_SPEC>;
///SDRAM Status Register
pub mod sdsr;
/**BUSERRADD (r) register accessor: Bus Error Address Register %s

You can [`read`](crate::Reg::read) this register and get [`buserradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buserradd`] module*/
pub type BUSERRADD = crate::Reg<buserradd::BUSERRADD_SPEC>;
///Bus Error Address Register %s
pub mod buserradd;
/**BUSERRSTAT (r) register accessor: Bus Error Status Register %s

You can [`read`](crate::Reg::read) this register and get [`buserrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buserrstat`] module*/
pub type BUSERRSTAT = crate::Reg<buserrstat::BUSERRSTAT_SPEC>;
///Bus Error Status Register %s
pub mod buserrstat;
/**BUSMCNT (rw) register accessor: Master Bus Control Register %s

You can [`read`](crate::Reg::read) this register and get [`busmcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busmcnt`] module*/
pub type BUSMCNT = crate::Reg<busmcnt::BUSMCNT_SPEC>;
///Master Bus Control Register %s
pub mod busmcnt;
/**BUSMCNTSYS (rw) register accessor: Master Bus Control Register SYS

You can [`read`](crate::Reg::read) this register and get [`busmcntsys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntsys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busmcntsys`] module*/
pub type BUSMCNTSYS = crate::Reg<busmcntsys::BUSMCNTSYS_SPEC>;
///Master Bus Control Register SYS
pub mod busmcntsys;
/**BUSMCNTDMA (rw) register accessor: Master Bus Control Register DMA

You can [`read`](crate::Reg::read) this register and get [`busmcntdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busmcntdma`] module*/
pub type BUSMCNTDMA = crate::Reg<busmcntdma::BUSMCNTDMA_SPEC>;
///Master Bus Control Register DMA
pub mod busmcntdma;
pub use busmcnt as busmcntedm;
pub use BUSMCNT as BUSMCNTEDM;
/**BUSSCNT (rw) register accessor: Slave Bus Control Register %s

You can [`read`](crate::Reg::read) this register and get [`busscnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscnt`] module*/
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
///Slave Bus Control Register %s
pub mod busscnt;
/**BUSSCNTMBIU (rw) register accessor: Slave Bus Control Register MBIU

You can [`read`](crate::Reg::read) this register and get [`busscntmbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntmbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntmbiu`] module*/
pub type BUSSCNTMBIU = crate::Reg<busscntmbiu::BUSSCNTMBIU_SPEC>;
///Slave Bus Control Register MBIU
pub mod busscntmbiu;
pub use busscnt as busscntram0;
pub use busscnt as busscntp0b;
pub use busscnt as busscntp6b;
pub use busscnt as busscntfbu;
pub use BUSSCNT as BUSSCNTRAM0;
pub use BUSSCNT as BUSSCNTP0B;
pub use BUSSCNT as BUSSCNTP6B;
pub use BUSSCNT as BUSSCNTFBU;
