#[repr(C)]
///Register block
pub struct RegisterBlock {
    srcfctr: [SRCFCTR; 5552],
}
impl RegisterBlock {
    ///0x00..0x56c0 - Filter Coefficient Table \[%s\]
    #[inline(always)]
    pub const fn srcfctr(&self, n: usize) -> &SRCFCTR {
        &self.srcfctr[n]
    }
    ///Iterator for array of:
    ///0x00..0x56c0 - Filter Coefficient Table \[%s\]
    #[inline(always)]
    pub fn srcfctr_iter(&self) -> impl Iterator<Item = &SRCFCTR> {
        self.srcfctr.iter()
    }
}
/**SRCFCTR (rw) register accessor: Filter Coefficient Table \[%s\]

You can [`read`](crate::Reg::read) this register and get [`srcfctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcfctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srcfctr`] module*/
pub type SRCFCTR = crate::Reg<srcfctr::SRCFCTR_SPEC>;
///Filter Coefficient Table \[%s\]
pub mod srcfctr;
