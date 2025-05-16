#[repr(C)]
///Register block
pub struct RegisterBlock {
    srcid: SRCID,
    srcod: SRCOD,
    srcidctrl: SRCIDCTRL,
    srcodctrl: SRCODCTRL,
    srcctrl: SRCCTRL,
    srcstat: SRCSTAT,
}
impl RegisterBlock {
    ///0x00 - Input Data Register
    #[inline(always)]
    pub const fn srcid(&self) -> &SRCID {
        &self.srcid
    }
    ///0x04 - Output Data Register
    #[inline(always)]
    pub const fn srcod(&self) -> &SRCOD {
        &self.srcod
    }
    ///0x08 - Input Data Control Register
    #[inline(always)]
    pub const fn srcidctrl(&self) -> &SRCIDCTRL {
        &self.srcidctrl
    }
    ///0x0a - Output Data Control Register
    #[inline(always)]
    pub const fn srcodctrl(&self) -> &SRCODCTRL {
        &self.srcodctrl
    }
    ///0x0c - Control Register
    #[inline(always)]
    pub const fn srcctrl(&self) -> &SRCCTRL {
        &self.srcctrl
    }
    ///0x0e - Status Register
    #[inline(always)]
    pub const fn srcstat(&self) -> &SRCSTAT {
        &self.srcstat
    }
}
/**SRCID (w) register accessor: Input Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcid::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcid`] module*/
pub type SRCID = crate::Reg<srcid::SRCID_SPEC>;
///Input Data Register
pub mod srcid;
/**SRCOD (r) register accessor: Output Data Register

You can [`read`](crate::Reg::read) this register and get [`srcod::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcod`] module*/
pub type SRCOD = crate::Reg<srcod::SRCOD_SPEC>;
///Output Data Register
pub mod srcod;
/**SRCIDCTRL (rw) register accessor: Input Data Control Register

You can [`read`](crate::Reg::read) this register and get [`srcidctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcidctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcidctrl`] module*/
pub type SRCIDCTRL = crate::Reg<srcidctrl::SRCIDCTRL_SPEC>;
///Input Data Control Register
pub mod srcidctrl;
/**SRCODCTRL (rw) register accessor: Output Data Control Register

You can [`read`](crate::Reg::read) this register and get [`srcodctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcodctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcodctrl`] module*/
pub type SRCODCTRL = crate::Reg<srcodctrl::SRCODCTRL_SPEC>;
///Output Data Control Register
pub mod srcodctrl;
/**SRCCTRL (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`srcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcctrl`] module*/
pub type SRCCTRL = crate::Reg<srcctrl::SRCCTRL_SPEC>;
///Control Register
pub mod srcctrl;
/**SRCSTAT (rw) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`srcstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcstat`] module*/
pub type SRCSTAT = crate::Reg<srcstat::SRCSTAT_SPEC>;
///Status Register
pub mod srcstat;
