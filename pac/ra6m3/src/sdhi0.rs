#[repr(C)]
///Register block
pub struct RegisterBlock {
    sd_cmd: SD_CMD,
    _reserved1: [u8; 0x04],
    sd_arg: SD_ARG,
    sd_arg1: SD_ARG1,
    sd_stop: SD_STOP,
    sd_seccnt: SD_SECCNT,
    sd_rsp10: SD_RSP10,
    sd_rsp1: SD_RSP1,
    sd_rsp32: SD_RSP32,
    sd_rsp3: SD_RSP3,
    sd_rsp54: SD_RSP54,
    sd_rsp5: SD_RSP5,
    sd_rsp76: SD_RSP76,
    sd_rsp7: SD_RSP7,
    sd_info1: SD_INFO1,
    sd_info2: SD_INFO2,
    sd_info1_mask: SD_INFO1_MASK,
    sd_info2_mask: SD_INFO2_MASK,
    sd_clk_ctrl: SD_CLK_CTRL,
    sd_size: SD_SIZE,
    sd_option: SD_OPTION,
    _reserved20: [u8; 0x04],
    sd_err_sts1: SD_ERR_STS1,
    sd_err_sts2: SD_ERR_STS2,
    sd_buf0: SD_BUF0,
    _reserved23: [u8; 0x04],
    sdio_mode: SDIO_MODE,
    sdio_info1: SDIO_INFO1,
    sdio_info1_mask: SDIO_INFO1_MASK,
    _reserved26: [u8; 0x013c],
    sd_dmaen: SD_DMAEN,
    _reserved27: [u8; 0x0c],
    soft_rst: SOFT_RST,
    _reserved28: [u8; 0x08],
    sdif_mode: SDIF_MODE,
    _reserved29: [u8; 0x10],
    ext_swap: EXT_SWAP,
}
impl RegisterBlock {
    ///0x00 - Command Type Register
    #[inline(always)]
    pub const fn sd_cmd(&self) -> &SD_CMD {
        &self.sd_cmd
    }
    ///0x08 - SD Command Argument Register
    #[inline(always)]
    pub const fn sd_arg(&self) -> &SD_ARG {
        &self.sd_arg
    }
    ///0x0c - SD Command Argument Register 1
    #[inline(always)]
    pub const fn sd_arg1(&self) -> &SD_ARG1 {
        &self.sd_arg1
    }
    ///0x10 - Data Stop Register
    #[inline(always)]
    pub const fn sd_stop(&self) -> &SD_STOP {
        &self.sd_stop
    }
    ///0x14 - Block Count Register
    #[inline(always)]
    pub const fn sd_seccnt(&self) -> &SD_SECCNT {
        &self.sd_seccnt
    }
    ///0x18 - SD Card Response Register 10
    #[inline(always)]
    pub const fn sd_rsp10(&self) -> &SD_RSP10 {
        &self.sd_rsp10
    }
    ///0x1c - SD Card Response Register 1
    #[inline(always)]
    pub const fn sd_rsp1(&self) -> &SD_RSP1 {
        &self.sd_rsp1
    }
    ///0x20 - SD Card Response Register 32
    #[inline(always)]
    pub const fn sd_rsp32(&self) -> &SD_RSP32 {
        &self.sd_rsp32
    }
    ///0x24 - SD Card Response Register 3
    #[inline(always)]
    pub const fn sd_rsp3(&self) -> &SD_RSP3 {
        &self.sd_rsp3
    }
    ///0x28 - SD Card Response Register 54
    #[inline(always)]
    pub const fn sd_rsp54(&self) -> &SD_RSP54 {
        &self.sd_rsp54
    }
    ///0x2c - SD Card Response Register 5
    #[inline(always)]
    pub const fn sd_rsp5(&self) -> &SD_RSP5 {
        &self.sd_rsp5
    }
    ///0x30 - SD Card Response Register 76
    #[inline(always)]
    pub const fn sd_rsp76(&self) -> &SD_RSP76 {
        &self.sd_rsp76
    }
    ///0x34 - SD Card Response Register 7
    #[inline(always)]
    pub const fn sd_rsp7(&self) -> &SD_RSP7 {
        &self.sd_rsp7
    }
    ///0x38 - SD Card Interrupt Flag Register 1
    #[inline(always)]
    pub const fn sd_info1(&self) -> &SD_INFO1 {
        &self.sd_info1
    }
    ///0x3c - SD Card Interrupt Flag Register 2
    #[inline(always)]
    pub const fn sd_info2(&self) -> &SD_INFO2 {
        &self.sd_info2
    }
    ///0x40 - SD_INFO1 Interrupt Mask Register
    #[inline(always)]
    pub const fn sd_info1_mask(&self) -> &SD_INFO1_MASK {
        &self.sd_info1_mask
    }
    ///0x44 - SD_INFO2 Interrupt Mask Register
    #[inline(always)]
    pub const fn sd_info2_mask(&self) -> &SD_INFO2_MASK {
        &self.sd_info2_mask
    }
    ///0x48 - SD Clock Control Register
    #[inline(always)]
    pub const fn sd_clk_ctrl(&self) -> &SD_CLK_CTRL {
        &self.sd_clk_ctrl
    }
    ///0x4c - Transfer Data Length Register
    #[inline(always)]
    pub const fn sd_size(&self) -> &SD_SIZE {
        &self.sd_size
    }
    ///0x50 - SD Card Access Control Option Register
    #[inline(always)]
    pub const fn sd_option(&self) -> &SD_OPTION {
        &self.sd_option
    }
    ///0x58 - SD Error Status Register 1
    #[inline(always)]
    pub const fn sd_err_sts1(&self) -> &SD_ERR_STS1 {
        &self.sd_err_sts1
    }
    ///0x5c - SD Error Status Register 2
    #[inline(always)]
    pub const fn sd_err_sts2(&self) -> &SD_ERR_STS2 {
        &self.sd_err_sts2
    }
    ///0x60 - SD Buffer Register
    #[inline(always)]
    pub const fn sd_buf0(&self) -> &SD_BUF0 {
        &self.sd_buf0
    }
    ///0x68 - SDIO Mode Control Register
    #[inline(always)]
    pub const fn sdio_mode(&self) -> &SDIO_MODE {
        &self.sdio_mode
    }
    ///0x6c - SDIO Interrupt Flag Register 1
    #[inline(always)]
    pub const fn sdio_info1(&self) -> &SDIO_INFO1 {
        &self.sdio_info1
    }
    ///0x70 - SDIO_INFO1 Interrupt Mask Register
    #[inline(always)]
    pub const fn sdio_info1_mask(&self) -> &SDIO_INFO1_MASK {
        &self.sdio_info1_mask
    }
    ///0x1b0 - DMA Mode Enable Register
    #[inline(always)]
    pub const fn sd_dmaen(&self) -> &SD_DMAEN {
        &self.sd_dmaen
    }
    ///0x1c0 - Software Reset Register
    #[inline(always)]
    pub const fn soft_rst(&self) -> &SOFT_RST {
        &self.soft_rst
    }
    ///0x1cc - SD Interface Mode Setting Register
    #[inline(always)]
    pub const fn sdif_mode(&self) -> &SDIF_MODE {
        &self.sdif_mode
    }
    ///0x1e0 - Swap Control Register
    #[inline(always)]
    pub const fn ext_swap(&self) -> &EXT_SWAP {
        &self.ext_swap
    }
}
/**SD_CMD (rw) register accessor: Command Type Register

You can [`read`](crate::Reg::read) this register and get [`sd_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_cmd`] module*/
pub type SD_CMD = crate::Reg<sd_cmd::SD_CMD_SPEC>;
///Command Type Register
pub mod sd_cmd;
/**SD_ARG (rw) register accessor: SD Command Argument Register

You can [`read`](crate::Reg::read) this register and get [`sd_arg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_arg`] module*/
pub type SD_ARG = crate::Reg<sd_arg::SD_ARG_SPEC>;
///SD Command Argument Register
pub mod sd_arg;
/**SD_ARG1 (rw) register accessor: SD Command Argument Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_arg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_arg1`] module*/
pub type SD_ARG1 = crate::Reg<sd_arg1::SD_ARG1_SPEC>;
///SD Command Argument Register 1
pub mod sd_arg1;
/**SD_STOP (rw) register accessor: Data Stop Register

You can [`read`](crate::Reg::read) this register and get [`sd_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_stop`] module*/
pub type SD_STOP = crate::Reg<sd_stop::SD_STOP_SPEC>;
///Data Stop Register
pub mod sd_stop;
/**SD_SECCNT (rw) register accessor: Block Count Register

You can [`read`](crate::Reg::read) this register and get [`sd_seccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_seccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_seccnt`] module*/
pub type SD_SECCNT = crate::Reg<sd_seccnt::SD_SECCNT_SPEC>;
///Block Count Register
pub mod sd_seccnt;
/**SD_RSP10 (r) register accessor: SD Card Response Register 10

You can [`read`](crate::Reg::read) this register and get [`sd_rsp10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp10`] module*/
pub type SD_RSP10 = crate::Reg<sd_rsp10::SD_RSP10_SPEC>;
///SD Card Response Register 10
pub mod sd_rsp10;
/**SD_RSP1 (r) register accessor: SD Card Response Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_rsp1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp1`] module*/
pub type SD_RSP1 = crate::Reg<sd_rsp1::SD_RSP1_SPEC>;
///SD Card Response Register 1
pub mod sd_rsp1;
/**SD_RSP32 (r) register accessor: SD Card Response Register 32

You can [`read`](crate::Reg::read) this register and get [`sd_rsp32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp32`] module*/
pub type SD_RSP32 = crate::Reg<sd_rsp32::SD_RSP32_SPEC>;
///SD Card Response Register 32
pub mod sd_rsp32;
/**SD_RSP3 (r) register accessor: SD Card Response Register 3

You can [`read`](crate::Reg::read) this register and get [`sd_rsp3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp3`] module*/
pub type SD_RSP3 = crate::Reg<sd_rsp3::SD_RSP3_SPEC>;
///SD Card Response Register 3
pub mod sd_rsp3;
/**SD_RSP54 (r) register accessor: SD Card Response Register 54

You can [`read`](crate::Reg::read) this register and get [`sd_rsp54::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp54`] module*/
pub type SD_RSP54 = crate::Reg<sd_rsp54::SD_RSP54_SPEC>;
///SD Card Response Register 54
pub mod sd_rsp54;
/**SD_RSP5 (r) register accessor: SD Card Response Register 5

You can [`read`](crate::Reg::read) this register and get [`sd_rsp5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp5`] module*/
pub type SD_RSP5 = crate::Reg<sd_rsp5::SD_RSP5_SPEC>;
///SD Card Response Register 5
pub mod sd_rsp5;
/**SD_RSP76 (r) register accessor: SD Card Response Register 76

You can [`read`](crate::Reg::read) this register and get [`sd_rsp76::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp76`] module*/
pub type SD_RSP76 = crate::Reg<sd_rsp76::SD_RSP76_SPEC>;
///SD Card Response Register 76
pub mod sd_rsp76;
/**SD_RSP7 (r) register accessor: SD Card Response Register 7

You can [`read`](crate::Reg::read) this register and get [`sd_rsp7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_rsp7`] module*/
pub type SD_RSP7 = crate::Reg<sd_rsp7::SD_RSP7_SPEC>;
///SD Card Response Register 7
pub mod sd_rsp7;
/**SD_INFO1 (rw) register accessor: SD Card Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_info1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info1`] module*/
pub type SD_INFO1 = crate::Reg<sd_info1::SD_INFO1_SPEC>;
///SD Card Interrupt Flag Register 1
pub mod sd_info1;
/**SD_INFO2 (rw) register accessor: SD Card Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_info2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info2`] module*/
pub type SD_INFO2 = crate::Reg<sd_info2::SD_INFO2_SPEC>;
///SD Card Interrupt Flag Register 2
pub mod sd_info2;
/**SD_INFO1_MASK (rw) register accessor: SD_INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info1_mask`] module*/
pub type SD_INFO1_MASK = crate::Reg<sd_info1_mask::SD_INFO1_MASK_SPEC>;
///SD_INFO1 Interrupt Mask Register
pub mod sd_info1_mask;
/**SD_INFO2_MASK (rw) register accessor: SD_INFO2 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sd_info2_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_info2_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_info2_mask`] module*/
pub type SD_INFO2_MASK = crate::Reg<sd_info2_mask::SD_INFO2_MASK_SPEC>;
///SD_INFO2 Interrupt Mask Register
pub mod sd_info2_mask;
/**SD_CLK_CTRL (rw) register accessor: SD Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sd_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_clk_ctrl`] module*/
pub type SD_CLK_CTRL = crate::Reg<sd_clk_ctrl::SD_CLK_CTRL_SPEC>;
///SD Clock Control Register
pub mod sd_clk_ctrl;
/**SD_SIZE (rw) register accessor: Transfer Data Length Register

You can [`read`](crate::Reg::read) this register and get [`sd_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_size`] module*/
pub type SD_SIZE = crate::Reg<sd_size::SD_SIZE_SPEC>;
///Transfer Data Length Register
pub mod sd_size;
/**SD_OPTION (rw) register accessor: SD Card Access Control Option Register

You can [`read`](crate::Reg::read) this register and get [`sd_option::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_option::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_option`] module*/
pub type SD_OPTION = crate::Reg<sd_option::SD_OPTION_SPEC>;
///SD Card Access Control Option Register
pub mod sd_option;
/**SD_ERR_STS1 (r) register accessor: SD Error Status Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_err_sts1`] module*/
pub type SD_ERR_STS1 = crate::Reg<sd_err_sts1::SD_ERR_STS1_SPEC>;
///SD Error Status Register 1
pub mod sd_err_sts1;
/**SD_ERR_STS2 (r) register accessor: SD Error Status Register 2

You can [`read`](crate::Reg::read) this register and get [`sd_err_sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_err_sts2`] module*/
pub type SD_ERR_STS2 = crate::Reg<sd_err_sts2::SD_ERR_STS2_SPEC>;
///SD Error Status Register 2
pub mod sd_err_sts2;
/**SD_BUF0 (rw) register accessor: SD Buffer Register

You can [`read`](crate::Reg::read) this register and get [`sd_buf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_buf0`] module*/
pub type SD_BUF0 = crate::Reg<sd_buf0::SD_BUF0_SPEC>;
///SD Buffer Register
pub mod sd_buf0;
/**SDIO_MODE (rw) register accessor: SDIO Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sdio_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_mode`] module*/
pub type SDIO_MODE = crate::Reg<sdio_mode::SDIO_MODE_SPEC>;
///SDIO Mode Control Register
pub mod sdio_mode;
/**SDIO_INFO1 (rw) register accessor: SDIO Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`sdio_info1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_info1`] module*/
pub type SDIO_INFO1 = crate::Reg<sdio_info1::SDIO_INFO1_SPEC>;
///SDIO Interrupt Flag Register 1
pub mod sdio_info1;
/**SDIO_INFO1_MASK (rw) register accessor: SDIO_INFO1 Interrupt Mask Register

You can [`read`](crate::Reg::read) this register and get [`sdio_info1_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_info1_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_info1_mask`] module*/
pub type SDIO_INFO1_MASK = crate::Reg<sdio_info1_mask::SDIO_INFO1_MASK_SPEC>;
///SDIO_INFO1 Interrupt Mask Register
pub mod sdio_info1_mask;
/**SD_DMAEN (rw) register accessor: DMA Mode Enable Register

You can [`read`](crate::Reg::read) this register and get [`sd_dmaen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dmaen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sd_dmaen`] module*/
pub type SD_DMAEN = crate::Reg<sd_dmaen::SD_DMAEN_SPEC>;
///DMA Mode Enable Register
pub mod sd_dmaen;
/**SOFT_RST (rw) register accessor: Software Reset Register

You can [`read`](crate::Reg::read) this register and get [`soft_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@soft_rst`] module*/
pub type SOFT_RST = crate::Reg<soft_rst::SOFT_RST_SPEC>;
///Software Reset Register
pub mod soft_rst;
/**SDIF_MODE (rw) register accessor: SD Interface Mode Setting Register

You can [`read`](crate::Reg::read) this register and get [`sdif_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdif_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdif_mode`] module*/
pub type SDIF_MODE = crate::Reg<sdif_mode::SDIF_MODE_SPEC>;
///SD Interface Mode Setting Register
pub mod sdif_mode;
/**EXT_SWAP (rw) register accessor: Swap Control Register

You can [`read`](crate::Reg::read) this register and get [`ext_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_swap`] module*/
pub type EXT_SWAP = crate::Reg<ext_swap::EXT_SWAP_SPEC>;
///Swap Control Register
pub mod ext_swap;
