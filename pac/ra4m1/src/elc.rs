#[repr(C)]
///Register block
pub struct RegisterBlock {
    elcr: ELCR,
    _reserved1: [u8; 0x01],
    elsegr: (),
    _reserved2: [u8; 0x0e],
    elsr: (),
    _reserved3: [u8; 0x30],
    elsr12: ELSR12,
    _reserved4: [u8; 0x06],
    elsr14: ELSR14,
    _reserved5: [u8; 0x02],
    elsr15: ELSR14,
    _reserved6: [u8; 0x02],
    elsr16: ELSR14,
    _reserved7: [u8; 0x02],
    elsr17: ELSR14,
    _reserved8: [u8; 0x02],
    elsr18: ELSR14,
}
impl RegisterBlock {
    ///0x00 - Event Link Controller Register
    #[inline(always)]
    pub const fn elcr(&self) -> &ELCR {
        &self.elcr
    }
    ///0x02 - Event Link Software Event Generation Register %s
    #[inline(always)]
    pub const fn elsegr(&self, n: usize) -> &ELSEGR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(2 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x02 - Event Link Software Event Generation Register %s
    #[inline(always)]
    pub fn elsegr_iter(&self) -> impl Iterator<Item = &ELSEGR> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(2 * n)
                .cast()
        })
    }
    ///0x10..0x24 - Event Link Setting Register %s
    #[inline(always)]
    pub const fn elsr(&self, n: usize) -> &ELSR {
        #[allow(clippy::no_effect)]
        [(); 10][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x10..0x24 - Event Link Setting Register %s
    #[inline(always)]
    pub fn elsr_iter(&self) -> impl Iterator<Item = &ELSR> {
        (0..10).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(4 * n)
                .cast()
        })
    }
    ///0x40 - Event Link Setting Register 12
    #[inline(always)]
    pub const fn elsr12(&self) -> &ELSR12 {
        &self.elsr12
    }
    ///0x48 - Event Link Setting Register 14
    #[inline(always)]
    pub const fn elsr14(&self) -> &ELSR14 {
        &self.elsr14
    }
    ///0x48 - Event Link Setting Register 15
    #[inline(always)]
    pub const fn elsr15(&self) -> &ELSR14 {
        &self.elsr15
    }
    ///0x48 - Event Link Setting Register 16
    #[inline(always)]
    pub const fn elsr16(&self) -> &ELSR14 {
        &self.elsr16
    }
    ///0x48 - Event Link Setting Register 17
    #[inline(always)]
    pub const fn elsr17(&self) -> &ELSR14 {
        &self.elsr17
    }
    ///0x48 - Event Link Setting Register 18
    #[inline(always)]
    pub const fn elsr18(&self) -> &ELSR14 {
        &self.elsr18
    }
}
/**ELCR (rw) register accessor: Event Link Controller Register

You can [`read`](crate::Reg::read) this register and get [`elcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elcr`] module*/
pub type ELCR = crate::Reg<elcr::ELCR_SPEC>;
///Event Link Controller Register
pub mod elcr;
/**ELSEGR (rw) register accessor: Event Link Software Event Generation Register %s

You can [`read`](crate::Reg::read) this register and get [`elsegr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsegr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elsegr`] module*/
pub type ELSEGR = crate::Reg<elsegr::ELSEGR_SPEC>;
///Event Link Software Event Generation Register %s
pub mod elsegr;
/**ELSR (rw) register accessor: Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`elsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elsr`] module*/
pub type ELSR = crate::Reg<elsr::ELSR_SPEC>;
///Event Link Setting Register %s
pub mod elsr;
pub use elsr as elsr12;
pub use elsr as elsr14;
pub use ELSR as ELSR12;
pub use ELSR as ELSR14;
