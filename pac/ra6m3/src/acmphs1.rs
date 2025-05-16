#[repr(C)]
///Register block
pub struct RegisterBlock {
    cmpctl: CMPCTL,
    _reserved1: [u8; 0x03],
    cmpsel0: CMPSEL0,
    _reserved2: [u8; 0x03],
    cmpsel1: CMPSEL1,
    _reserved3: [u8; 0x03],
    cmpmon: CMPMON,
    _reserved4: [u8; 0x03],
    cpioc: CPIOC,
}
impl RegisterBlock {
    ///0x00 - Comparator Control Register
    #[inline(always)]
    pub const fn cmpctl(&self) -> &CMPCTL {
        &self.cmpctl
    }
    ///0x04 - Comparator Input Select Register
    #[inline(always)]
    pub const fn cmpsel0(&self) -> &CMPSEL0 {
        &self.cmpsel0
    }
    ///0x08 - Comparator Reference Voltage Select Register
    #[inline(always)]
    pub const fn cmpsel1(&self) -> &CMPSEL1 {
        &self.cmpsel1
    }
    ///0x0c - Comparator Output Monitor Register
    #[inline(always)]
    pub const fn cmpmon(&self) -> &CMPMON {
        &self.cmpmon
    }
    ///0x10 - Comparator Output Control Register
    #[inline(always)]
    pub const fn cpioc(&self) -> &CPIOC {
        &self.cpioc
    }
}
/**CMPCTL (rw) register accessor: Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`cmpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpctl`] module*/
pub type CMPCTL = crate::Reg<cmpctl::CMPCTL_SPEC>;
///Comparator Control Register
pub mod cmpctl;
/**CMPSEL0 (rw) register accessor: Comparator Input Select Register

You can [`read`](crate::Reg::read) this register and get [`cmpsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpsel0`] module*/
pub type CMPSEL0 = crate::Reg<cmpsel0::CMPSEL0_SPEC>;
///Comparator Input Select Register
pub mod cmpsel0;
/**CMPSEL1 (rw) register accessor: Comparator Reference Voltage Select Register

You can [`read`](crate::Reg::read) this register and get [`cmpsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpsel1`] module*/
pub type CMPSEL1 = crate::Reg<cmpsel1::CMPSEL1_SPEC>;
///Comparator Reference Voltage Select Register
pub mod cmpsel1;
/**CMPMON (r) register accessor: Comparator Output Monitor Register

You can [`read`](crate::Reg::read) this register and get [`cmpmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpmon`] module*/
pub type CMPMON = crate::Reg<cmpmon::CMPMON_SPEC>;
///Comparator Output Monitor Register
pub mod cmpmon;
/**CPIOC (rw) register accessor: Comparator Output Control Register

You can [`read`](crate::Reg::read) this register and get [`cpioc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpioc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpioc`] module*/
pub type CPIOC = crate::Reg<cpioc::CPIOC_SPEC>;
///Comparator Output Control Register
pub mod cpioc;
