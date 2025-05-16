#[repr(C)]
///Register block
pub struct RegisterBlock {
    gtdlycr: GTDLYCR,
    gtdlycr2: GTDLYCR2,
    _reserved2: [u8; 0x14],
    gtdlyra: (),
    _reserved3: [u8; 0x02],
    gtdlyrb: (),
    _reserved4: [u8; 0x0e],
    gtdlyfa: (),
    _reserved5: [u8; 0x02],
    gtdlyfb: (),
}
impl RegisterBlock {
    ///0x00 - PWM Output Delay Control Register
    #[inline(always)]
    pub const fn gtdlycr(&self) -> &GTDLYCR {
        &self.gtdlycr
    }
    ///0x02 - PWM Output Delay Control Register2
    #[inline(always)]
    pub const fn gtdlycr2(&self) -> &GTDLYCR2 {
        &self.gtdlycr2
    }
    ///0x18..0x20 - GTIOC%sA Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyra(&self, n: usize) -> &GTDLYRA {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x18..0x20 - GTIOC%sA Rising Output Delay Register
    #[inline(always)]
    pub fn gtdlyra_iter(&self) -> impl Iterator<Item = &GTDLYRA> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(4 * n)
                .cast()
        })
    }
    ///0x18 - GTIOC0A Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr0a(&self) -> &GTDLYRA {
        self.gtdlyra(0)
    }
    ///0x1c - GTIOC1A Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr1a(&self) -> &GTDLYRA {
        self.gtdlyra(1)
    }
    ///0x20 - GTIOC2A Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr2a(&self) -> &GTDLYRA {
        self.gtdlyra(2)
    }
    ///0x24 - GTIOC3A Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr3a(&self) -> &GTDLYRA {
        self.gtdlyra(3)
    }
    ///0x1a..0x22 - GTIOC%sB Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyrb(&self, n: usize) -> &GTDLYRB {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(26)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x1a..0x22 - GTIOC%sB Rising Output Delay Register
    #[inline(always)]
    pub fn gtdlyrb_iter(&self) -> impl Iterator<Item = &GTDLYRB> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(26)
                .add(4 * n)
                .cast()
        })
    }
    ///0x1a - GTIOC0B Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr0b(&self) -> &GTDLYRB {
        self.gtdlyrb(0)
    }
    ///0x1e - GTIOC1B Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr1b(&self) -> &GTDLYRB {
        self.gtdlyrb(1)
    }
    ///0x22 - GTIOC2B Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr2b(&self) -> &GTDLYRB {
        self.gtdlyrb(2)
    }
    ///0x26 - GTIOC3B Rising Output Delay Register
    #[inline(always)]
    pub const fn gtdlyr3b(&self) -> &GTDLYRB {
        self.gtdlyrb(3)
    }
    ///0x28..0x30 - GTIOC%sA Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyfa(&self, n: usize) -> &GTDLYFA {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(40)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x28..0x30 - GTIOC%sA Falling Output Delay Register
    #[inline(always)]
    pub fn gtdlyfa_iter(&self) -> impl Iterator<Item = &GTDLYFA> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(40)
                .add(4 * n)
                .cast()
        })
    }
    ///0x28 - GTIOC0A Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf0a(&self) -> &GTDLYFA {
        self.gtdlyfa(0)
    }
    ///0x2c - GTIOC1A Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf1a(&self) -> &GTDLYFA {
        self.gtdlyfa(1)
    }
    ///0x30 - GTIOC2A Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf2a(&self) -> &GTDLYFA {
        self.gtdlyfa(2)
    }
    ///0x34 - GTIOC3A Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf3a(&self) -> &GTDLYFA {
        self.gtdlyfa(3)
    }
    ///0x2a..0x32 - GTIOC%sB Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyfb(&self, n: usize) -> &GTDLYFB {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(42)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x2a..0x32 - GTIOC%sB Falling Output Delay Register
    #[inline(always)]
    pub fn gtdlyfb_iter(&self) -> impl Iterator<Item = &GTDLYFB> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(42)
                .add(4 * n)
                .cast()
        })
    }
    ///0x2a - GTIOC0B Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf0b(&self) -> &GTDLYFB {
        self.gtdlyfb(0)
    }
    ///0x2e - GTIOC1B Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf1b(&self) -> &GTDLYFB {
        self.gtdlyfb(1)
    }
    ///0x32 - GTIOC2B Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf2b(&self) -> &GTDLYFB {
        self.gtdlyfb(2)
    }
    ///0x36 - GTIOC3B Falling Output Delay Register
    #[inline(always)]
    pub const fn gtdlyf3b(&self) -> &GTDLYFB {
        self.gtdlyfb(3)
    }
}
/**GTDLYCR (rw) register accessor: PWM Output Delay Control Register

You can [`read`](crate::Reg::read) this register and get [`gtdlycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdlycr`] module*/
pub type GTDLYCR = crate::Reg<gtdlycr::GTDLYCR_SPEC>;
///PWM Output Delay Control Register
pub mod gtdlycr;
/**GTDLYCR2 (rw) register accessor: PWM Output Delay Control Register2

You can [`read`](crate::Reg::read) this register and get [`gtdlycr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlycr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdlycr2`] module*/
pub type GTDLYCR2 = crate::Reg<gtdlycr2::GTDLYCR2_SPEC>;
///PWM Output Delay Control Register2
pub mod gtdlycr2;
/**GTDLYRA (rw) register accessor: GTIOC%sA Rising Output Delay Register

You can [`read`](crate::Reg::read) this register and get [`gtdlyra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlyra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdlyra`] module*/
pub type GTDLYRA = crate::Reg<gtdlyra::GTDLYRA_SPEC>;
///GTIOC%sA Rising Output Delay Register
pub mod gtdlyra;
/**GTDLYRB (rw) register accessor: GTIOC%sB Rising Output Delay Register

You can [`read`](crate::Reg::read) this register and get [`gtdlyrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlyrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdlyrb`] module*/
pub type GTDLYRB = crate::Reg<gtdlyrb::GTDLYRB_SPEC>;
///GTIOC%sB Rising Output Delay Register
pub mod gtdlyrb;
/**GTDLYFA (rw) register accessor: GTIOC%sA Falling Output Delay Register

You can [`read`](crate::Reg::read) this register and get [`gtdlyfa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlyfa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdlyfa`] module*/
pub type GTDLYFA = crate::Reg<gtdlyfa::GTDLYFA_SPEC>;
///GTIOC%sA Falling Output Delay Register
pub mod gtdlyfa;
/**GTDLYFB (rw) register accessor: GTIOC%sB Falling Output Delay Register

You can [`read`](crate::Reg::read) this register and get [`gtdlyfb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlyfb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdlyfb`] module*/
pub type GTDLYFB = crate::Reg<gtdlyfb::GTDLYFB_SPEC>;
///GTIOC%sB Falling Output Delay Register
pub mod gtdlyfb;
