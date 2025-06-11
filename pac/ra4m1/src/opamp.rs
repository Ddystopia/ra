#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ampmc: AMPMC,
    amptrm: AMPTRM,
    amptrs: AMPTRS,
    ampc: AMPC,
    ampmon: AMPMON,
}
impl RegisterBlock {
    ///0x08 - Operational amplifier mode control register
    #[inline(always)]
    pub const fn ampmc(&self) -> &AMPMC {
        &self.ampmc
    }
    ///0x09 - Operational amplifier trigger mode control register
    #[inline(always)]
    pub const fn amptrm(&self) -> &AMPTRM {
        &self.amptrm
    }
    ///0x0a - Operational Amplifier Activation Trigger Select Register
    #[inline(always)]
    pub const fn amptrs(&self) -> &AMPTRS {
        &self.amptrs
    }
    ///0x0b - Operational amplifier control register
    #[inline(always)]
    pub const fn ampc(&self) -> &AMPC {
        &self.ampc
    }
    ///0x0c - Operational amplifier monitor register
    #[inline(always)]
    pub const fn ampmon(&self) -> &AMPMON {
        &self.ampmon
    }
}
/**AMPMC (rw) register accessor: Operational amplifier mode control register

You can [`read`](crate::Reg::read) this register and get [`ampmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ampmc`] module*/
pub type AMPMC = crate::Reg<ampmc::AMPMC_SPEC>;
///Operational amplifier mode control register
pub mod ampmc;
/**AMPTRM (rw) register accessor: Operational amplifier trigger mode control register

You can [`read`](crate::Reg::read) this register and get [`amptrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@amptrm`] module*/
pub type AMPTRM = crate::Reg<amptrm::AMPTRM_SPEC>;
///Operational amplifier trigger mode control register
pub mod amptrm;
/**AMPTRS (rw) register accessor: Operational Amplifier Activation Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`amptrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amptrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@amptrs`] module*/
pub type AMPTRS = crate::Reg<amptrs::AMPTRS_SPEC>;
///Operational Amplifier Activation Trigger Select Register
pub mod amptrs;
/**AMPC (rw) register accessor: Operational amplifier control register

You can [`read`](crate::Reg::read) this register and get [`ampc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ampc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ampc`] module*/
pub type AMPC = crate::Reg<ampc::AMPC_SPEC>;
///Operational amplifier control register
pub mod ampc;
/**AMPMON (r) register accessor: Operational amplifier monitor register

You can [`read`](crate::Reg::read) this register and get [`ampmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ampmon`] module*/
pub type AMPMON = crate::Reg<ampmon::AMPMON_SPEC>;
///Operational amplifier monitor register
pub mod ampmon;
