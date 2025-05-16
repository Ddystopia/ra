#[repr(C)]
///Register block
pub struct RegisterBlock {
    dmast: DMAST,
}
impl RegisterBlock {
    ///0x00 - DMA Module Activation Register
    #[inline(always)]
    pub const fn dmast(&self) -> &DMAST {
        &self.dmast
    }
}
/**DMAST (rw) register accessor: DMA Module Activation Register

You can [`read`](crate::Reg::read) this register and get [`dmast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmast`] module*/
pub type DMAST = crate::Reg<dmast::DMAST_SPEC>;
///DMA Module Activation Register
pub mod dmast;
