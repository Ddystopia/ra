#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x03],
    pwpr: PWPR,
}
impl RegisterBlock {
    ///0x03 - Write-Protect Register
    #[inline(always)]
    pub const fn pwpr(&self) -> &PWPR {
        &self.pwpr
    }
}
/**PWPR (rw) register accessor: Write-Protect Register

You can [`read`](crate::Reg::read) this register and get [`pwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwpr`] module*/
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
///Write-Protect Register
pub mod pwpr;
