#[repr(C)]
///Register block
pub struct RegisterBlock {
    pfenet: PFENET,
    _reserved1: [u8; 0x02],
    pwpr: PWPR,
}
impl RegisterBlock {
    ///0x00 - Ethernet Control Register
    #[inline(always)]
    pub const fn pfenet(&self) -> &PFENET {
        &self.pfenet
    }
    ///0x03 - Write-Protect Register
    #[inline(always)]
    pub const fn pwpr(&self) -> &PWPR {
        &self.pwpr
    }
}
/**PFENET (rw) register accessor: Ethernet Control Register

You can [`read`](crate::Reg::read) this register and get [`pfenet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfenet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pfenet`] module*/
pub type PFENET = crate::Reg<pfenet::PFENET_SPEC>;
///Ethernet Control Register
pub mod pfenet;
/**PWPR (rw) register accessor: Write-Protect Register

You can [`read`](crate::Reg::read) this register and get [`pwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwpr`] module*/
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
///Write-Protect Register
pub mod pwpr;
