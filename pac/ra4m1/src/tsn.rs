#[repr(C)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0228],
    tscdrl: TSCDRL,
    tscdrh: TSCDRH,
}
impl RegisterBlock {
    ///0x228 - Temperature Sensor Calibration Data Register L
    #[inline(always)]
    pub const fn tscdrl(&self) -> &TSCDRL {
        &self.tscdrl
    }
    ///0x229 - Temperature Sensor Calibration Data Register H
    #[inline(always)]
    pub const fn tscdrh(&self) -> &TSCDRH {
        &self.tscdrh
    }
}
/**TSCDRH (r) register accessor: Temperature Sensor Calibration Data Register H

You can [`read`](crate::Reg::read) this register and get [`tscdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tscdrh`] module*/
pub type TSCDRH = crate::Reg<tscdrh::TSCDRH_SPEC>;
///Temperature Sensor Calibration Data Register H
pub mod tscdrh;
/**TSCDRL (r) register accessor: Temperature Sensor Calibration Data Register L

You can [`read`](crate::Reg::read) this register and get [`tscdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tscdrl`] module*/
pub type TSCDRL = crate::Reg<tscdrl::TSCDRL_SPEC>;
///Temperature Sensor Calibration Data Register L
pub mod tscdrl;
