#[repr(C)]
///Register block
pub struct RegisterBlock {
    mstpcrb: MSTPCRB,
    mstpcrc: MSTPCRC,
    mstpcrd: MSTPCRD,
}
impl RegisterBlock {
    ///0x00 - Module Stop Control Register B
    #[inline(always)]
    pub const fn mstpcrb(&self) -> &MSTPCRB {
        &self.mstpcrb
    }
    ///0x04 - Module Stop Control Register C
    #[inline(always)]
    pub const fn mstpcrc(&self) -> &MSTPCRC {
        &self.mstpcrc
    }
    ///0x08 - Module Stop Control Register D
    #[inline(always)]
    pub const fn mstpcrd(&self) -> &MSTPCRD {
        &self.mstpcrd
    }
}
/**MSTPCRB (rw) register accessor: Module Stop Control Register B

You can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcrb`] module*/
pub type MSTPCRB = crate::Reg<mstpcrb::MSTPCRB_SPEC>;
///Module Stop Control Register B
pub mod mstpcrb;
/**MSTPCRC (rw) register accessor: Module Stop Control Register C

You can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcrc`] module*/
pub type MSTPCRC = crate::Reg<mstpcrc::MSTPCRC_SPEC>;
///Module Stop Control Register C
pub mod mstpcrc;
/**MSTPCRD (rw) register accessor: Module Stop Control Register D

You can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcrd`] module*/
pub type MSTPCRD = crate::Reg<mstpcrd::MSTPCRD_SPEC>;
///Module Stop Control Register D
pub mod mstpcrd;
