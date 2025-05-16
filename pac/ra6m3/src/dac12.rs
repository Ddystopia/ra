#[repr(C)]
///Register block
pub struct RegisterBlock {
    dadr: [DADR; 2],
    dacr: DACR,
    dadpr: DADPR,
    daadscr: DAADSCR,
    _reserved4: [u8; 0x01],
    daampcr: DAAMPCR,
    _reserved5: [u8; 0x1013],
    daaswcr: DAASWCR,
}
impl RegisterBlock {
    ///0x00 - D/A Data Register %s
    #[inline(always)]
    pub const fn dadr(&self, n: usize) -> &DADR {
        &self.dadr[n]
    }
    ///Iterator for array of:
    ///0x00 - D/A Data Register %s
    #[inline(always)]
    pub fn dadr_iter(&self) -> impl Iterator<Item = &DADR> {
        self.dadr.iter()
    }
    ///0x04 - D/A Control Register
    #[inline(always)]
    pub const fn dacr(&self) -> &DACR {
        &self.dacr
    }
    ///0x05 - DADRm Format Select Register
    #[inline(always)]
    pub const fn dadpr(&self) -> &DADPR {
        &self.dadpr
    }
    ///0x06 - D/A-A/D Synchronous Start Control Register
    #[inline(always)]
    pub const fn daadscr(&self) -> &DAADSCR {
        &self.daadscr
    }
    ///0x08 - D/A Output Amplifier Control Register
    #[inline(always)]
    pub const fn daampcr(&self) -> &DAAMPCR {
        &self.daampcr
    }
    ///0x101c - D/A Amplifier Stabilization Wait Control Register
    #[inline(always)]
    pub const fn daaswcr(&self) -> &DAASWCR {
        &self.daaswcr
    }
}
/**DADR (rw) register accessor: D/A Data Register %s

You can [`read`](crate::Reg::read) this register and get [`dadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dadr`] module*/
pub type DADR = crate::Reg<dadr::DADR_SPEC>;
///D/A Data Register %s
pub mod dadr;
/**DACR (rw) register accessor: D/A Control Register

You can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dacr`] module*/
pub type DACR = crate::Reg<dacr::DACR_SPEC>;
///D/A Control Register
pub mod dacr;
/**DADPR (rw) register accessor: DADRm Format Select Register

You can [`read`](crate::Reg::read) this register and get [`dadpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dadpr`] module*/
pub type DADPR = crate::Reg<dadpr::DADPR_SPEC>;
///DADRm Format Select Register
pub mod dadpr;
/**DAADSCR (rw) register accessor: D/A-A/D Synchronous Start Control Register

You can [`read`](crate::Reg::read) this register and get [`daadscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daadscr`] module*/
pub type DAADSCR = crate::Reg<daadscr::DAADSCR_SPEC>;
///D/A-A/D Synchronous Start Control Register
pub mod daadscr;
/**DAAMPCR (rw) register accessor: D/A Output Amplifier Control Register

You can [`read`](crate::Reg::read) this register and get [`daampcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daampcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daampcr`] module*/
pub type DAAMPCR = crate::Reg<daampcr::DAAMPCR_SPEC>;
///D/A Output Amplifier Control Register
pub mod daampcr;
/**DAASWCR (rw) register accessor: D/A Amplifier Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`daaswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daaswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daaswcr`] module*/
pub type DAASWCR = crate::Reg<daaswcr::DAASWCR_SPEC>;
///D/A Amplifier Stabilization Wait Control Register
pub mod daaswcr;
