#[repr(C)]
///Register block
pub struct RegisterBlock {
    lcdm0: LCDM0,
    lcdm1: LCDM1,
    lcdc0: LCDC0,
    vlcd: VLCD,
    _reserved4: [u8; 0xfc],
    seg: [SEG; 38],
}
impl RegisterBlock {
    ///0x00 - LCD Mode Register 0
    #[inline(always)]
    pub const fn lcdm0(&self) -> &LCDM0 {
        &self.lcdm0
    }
    ///0x01 - LCD Mode Register 1
    #[inline(always)]
    pub const fn lcdm1(&self) -> &LCDM1 {
        &self.lcdm1
    }
    ///0x02 - LCD Clock Control Register 0
    #[inline(always)]
    pub const fn lcdc0(&self) -> &LCDC0 {
        &self.lcdc0
    }
    ///0x03 - LCD Boost Level Control Register
    #[inline(always)]
    pub const fn vlcd(&self) -> &VLCD {
        &self.vlcd
    }
    ///0x100..0x126 - LCD Display Data Register %s
    #[inline(always)]
    pub const fn seg(&self, n: usize) -> &SEG {
        &self.seg[n]
    }
    ///Iterator for array of:
    ///0x100..0x126 - LCD Display Data Register %s
    #[inline(always)]
    pub fn seg_iter(&self) -> impl Iterator<Item = &SEG> {
        self.seg.iter()
    }
}
/**LCDM0 (rw) register accessor: LCD Mode Register 0

You can [`read`](crate::Reg::read) this register and get [`lcdm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcdm0`] module*/
pub type LCDM0 = crate::Reg<lcdm0::LCDM0_SPEC>;
///LCD Mode Register 0
pub mod lcdm0;
/**LCDM1 (rw) register accessor: LCD Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`lcdm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcdm1`] module*/
pub type LCDM1 = crate::Reg<lcdm1::LCDM1_SPEC>;
///LCD Mode Register 1
pub mod lcdm1;
/**LCDC0 (rw) register accessor: LCD Clock Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lcdc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcdc0`] module*/
pub type LCDC0 = crate::Reg<lcdc0::LCDC0_SPEC>;
///LCD Clock Control Register 0
pub mod lcdc0;
/**VLCD (rw) register accessor: LCD Boost Level Control Register

You can [`read`](crate::Reg::read) this register and get [`vlcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vlcd`] module*/
pub type VLCD = crate::Reg<vlcd::VLCD_SPEC>;
///LCD Boost Level Control Register
pub mod vlcd;
/**SEG (rw) register accessor: LCD Display Data Register %s

You can [`read`](crate::Reg::read) this register and get [`seg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@seg`] module*/
pub type SEG = crate::Reg<seg::SEG_SPEC>;
///LCD Display Data Register %s
pub mod seg;
