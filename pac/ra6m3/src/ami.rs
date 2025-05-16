#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0xc0],
    daadusr: DAADUSR,
}
impl RegisterBlock {
    ///0xc0 - D/A A/D Synchronous Unit Select Register
    #[inline(always)]
    pub const fn daadusr(&self) -> &DAADUSR {
        &self.daadusr
    }
}
/**DAADUSR (rw) register accessor: D/A A/D Synchronous Unit Select Register

You can [`read`](crate::Reg::read) this register and get [`daadusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daadusr`] module*/
pub type DAADUSR = crate::Reg<daadusr::DAADUSR_SPEC>;
///D/A A/D Synchronous Unit Select Register
pub mod daadusr;
