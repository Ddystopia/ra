#[repr(C)]
///Register block
pub struct RegisterBlock {
    dadr0: DADR0,
    _reserved1: [u8; 0x02],
    dacr: DACR,
    dadpr: DADPR,
    daadscr: DAADSCR,
    davrefcr: DAVREFCR,
}
impl RegisterBlock {
    ///0x00 - D/A Data Register 0
    #[inline(always)]
    pub const fn dadr0(&self) -> &DADR0 {
        &self.dadr0
    }
    ///0x04 - D/A Control Register
    #[inline(always)]
    pub const fn dacr(&self) -> &DACR {
        &self.dacr
    }
    ///0x05 - DADR0 Format Select Register
    #[inline(always)]
    pub const fn dadpr(&self) -> &DADPR {
        &self.dadpr
    }
    ///0x06 - D/A-A/D Synchronous Start Control Register
    #[inline(always)]
    pub const fn daadscr(&self) -> &DAADSCR {
        &self.daadscr
    }
    ///0x07 - D/A VREF Control Register
    #[inline(always)]
    pub const fn davrefcr(&self) -> &DAVREFCR {
        &self.davrefcr
    }
}
/**DADR0 (rw) register accessor: D/A Data Register 0

You can [`read`](crate::Reg::read) this register and get [`dadr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dadr0`] module*/
pub type DADR0 = crate::Reg<dadr0::DADR0_SPEC>;
///D/A Data Register 0
pub mod dadr0;
/**DACR (rw) register accessor: D/A Control Register

You can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dacr`] module*/
pub type DACR = crate::Reg<dacr::DACR_SPEC>;
///D/A Control Register
pub mod dacr;
/**DADPR (rw) register accessor: DADR0 Format Select Register

You can [`read`](crate::Reg::read) this register and get [`dadpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dadpr`] module*/
pub type DADPR = crate::Reg<dadpr::DADPR_SPEC>;
///DADR0 Format Select Register
pub mod dadpr;
/**DAADSCR (rw) register accessor: D/A-A/D Synchronous Start Control Register

You can [`read`](crate::Reg::read) this register and get [`daadscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daadscr`] module*/
pub type DAADSCR = crate::Reg<daadscr::DAADSCR_SPEC>;
///D/A-A/D Synchronous Start Control Register
pub mod daadscr;
/**DAVREFCR (rw) register accessor: D/A VREF Control Register

You can [`read`](crate::Reg::read) this register and get [`davrefcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`davrefcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@davrefcr`] module*/
pub type DAVREFCR = crate::Reg<davrefcr::DAVREFCR_SPEC>;
///D/A VREF Control Register
pub mod davrefcr;
