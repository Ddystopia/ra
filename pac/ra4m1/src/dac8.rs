#[repr(C)]
///Register block
pub struct RegisterBlock {
    dacs: [DACS; 2],
    _reserved1: [u8; 0x01],
    dam: DAM,
}
impl RegisterBlock {
    ///0x00 - D/A Conversion Value Setting Register %s
    #[inline(always)]
    pub const fn dacs(&self, n: usize) -> &DACS {
        &self.dacs[n]
    }
    ///Iterator for array of:
    ///0x00 - D/A Conversion Value Setting Register %s
    #[inline(always)]
    pub fn dacs_iter(&self) -> impl Iterator<Item = &DACS> {
        self.dacs.iter()
    }
    ///0x03 - D/A Converter Mode Register
    #[inline(always)]
    pub const fn dam(&self) -> &DAM {
        &self.dam
    }
}
/**DACS (rw) register accessor: D/A Conversion Value Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`dacs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dacs`] module*/
pub type DACS = crate::Reg<dacs::DACS_SPEC>;
///D/A Conversion Value Setting Register %s
pub mod dacs;
/**DAM (rw) register accessor: D/A Converter Mode Register

You can [`read`](crate::Reg::read) this register and get [`dam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dam`] module*/
pub type DAM = crate::Reg<dam::DAM_SPEC>;
///D/A Converter Mode Register
pub mod dam;
