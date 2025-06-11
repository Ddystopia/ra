#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    busmcnt: (),
    _reserved1: [u8; 0x0100],
    busscntfli: BUSSCNTFLI,
    _reserved2: [u8; 0x06],
    busscnt: (),
    _reserved3: [u8; 0x0c],
    busscntp0b: BUSSCNTP0B,
    _reserved4: [u8; 0x02],
    busscntp2b: BUSSCNTP0B,
    _reserved5: [u8; 0x02],
    busscntp3b: BUSSCNTP0B,
    _reserved6: [u8; 0x02],
    busscntp4b: BUSSCNTP0B,
    _reserved7: [u8; 0x06],
    busscntp6b: BUSSCNTP6B,
    _reserved8: [u8; 0x06],
    busscntfbu: BUSSCNTFBU,
    _reserved9: [u8; 0x06ce],
    buserradd: (),
    _reserved10: [u8; 0x04],
    buserrstat: (),
}
impl RegisterBlock {
    ///0x1000..0x1008 - Master Bus Control Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUSMCNTM4I` register.</div>
    #[inline(always)]
    pub const fn busmcnt(&self, n: usize) -> &BUSMCNT {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4096)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1000..0x1008 - Master Bus Control Register %s
    #[inline(always)]
    pub fn busmcnt_iter(&self) -> impl Iterator<Item = &BUSMCNT> {
        (0..4).map(move |n| unsafe {
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
    pub const fn busmcntsys(&self) -> &BUSMCNT {
        self.busmcnt(2)
    }
    ///0x100c - Master Bus Control Register DMA
    #[inline(always)]
    pub const fn busmcntdma(&self) -> &BUSMCNT {
        self.busmcnt(3)
    }
    ///0x1100 - Slave Bus Control Register FLI
    #[inline(always)]
    pub const fn busscntfli(&self) -> &BUSSCNTFLI {
        &self.busscntfli
    }
    ///0x1108 - Slave Bus Control Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUSSCNTMBIU` register.</div>
    #[inline(always)]
    pub const fn busscnt(&self, n: usize) -> &BUSSCNT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4360)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1108 - Slave Bus Control Register %s
    #[inline(always)]
    pub fn busscnt_iter(&self) -> impl Iterator<Item = &BUSSCNT> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4360)
                .add(4 * n)
                .cast()
        })
    }
    ///0x1108 - Slave Bus Control Register MBIU
    #[inline(always)]
    pub const fn busscntmbiu(&self) -> &BUSSCNT {
        self.busscnt(0)
    }
    ///0x110c - Slave Bus Control Register RAM0
    #[inline(always)]
    pub const fn busscntram0(&self) -> &BUSSCNT {
        self.busscnt(1)
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
    ///0x1130 - Slave Bus Control Register FBU
    #[inline(always)]
    pub const fn busscntfbu(&self) -> &BUSSCNTFBU {
        &self.busscntfbu
    }
    ///0x1800..0x1810 - Bus Error Address Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRADD` register.</div>
    #[inline(always)]
    pub const fn buserradd(&self, n: usize) -> &BUSERRADD {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6144)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1800..0x1810 - Bus Error Address Register %s
    #[inline(always)]
    pub fn buserradd_iter(&self) -> impl Iterator<Item = &BUSERRADD> {
        (0..4).map(move |n| unsafe {
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
    ///0x1804 - Bus Error Status Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRSTAT` register.</div>
    #[inline(always)]
    pub const fn buserrstat(&self, n: usize) -> &BUSERRSTAT {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6148)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1804 - Bus Error Status Register %s
    #[inline(always)]
    pub fn buserrstat_iter(&self) -> impl Iterator<Item = &BUSERRSTAT> {
        (0..4).map(move |n| unsafe {
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
}
/**BUSMCNT (rw) register accessor: Master Bus Control Register %s

You can [`read`](crate::Reg::read) this register and get [`busmcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busmcnt`] module*/
pub type BUSMCNT = crate::Reg<busmcnt::BUSMCNT_SPEC>;
///Master Bus Control Register %s
pub mod busmcnt;
/**BUSSCNTFLI (rw) register accessor: Slave Bus Control Register FLI

You can [`read`](crate::Reg::read) this register and get [`busscntfli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntfli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntfli`] module*/
pub type BUSSCNTFLI = crate::Reg<busscntfli::BUSSCNTFLI_SPEC>;
///Slave Bus Control Register FLI
pub mod busscntfli;
/**BUSSCNT (rw) register accessor: Slave Bus Control Register %s

You can [`read`](crate::Reg::read) this register and get [`busscnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscnt`] module*/
pub type BUSSCNT = crate::Reg<busscnt::BUSSCNT_SPEC>;
///Slave Bus Control Register %s
pub mod busscnt;
pub use busscnt as busscntp0b;
pub use BUSSCNT as BUSSCNTP0B;
/**BUSSCNTP6B (rw) register accessor: Slave Bus Control Register P6B

You can [`read`](crate::Reg::read) this register and get [`busscntp6b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntp6b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntp6b`] module*/
pub type BUSSCNTP6B = crate::Reg<busscntp6b::BUSSCNTP6B_SPEC>;
///Slave Bus Control Register P6B
pub mod busscntp6b;
/**BUSSCNTFBU (rw) register accessor: Slave Bus Control Register FBU

You can [`read`](crate::Reg::read) this register and get [`busscntfbu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntfbu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntfbu`] module*/
pub type BUSSCNTFBU = crate::Reg<busscntfbu::BUSSCNTFBU_SPEC>;
///Slave Bus Control Register FBU
pub mod busscntfbu;
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
