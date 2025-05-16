#[repr(C)]
///Register block
pub struct RegisterBlock {
    edmr: EDMR,
    _reserved1: [u8; 0x04],
    edtrr: EDTRR,
    _reserved2: [u8; 0x04],
    edrrr: EDRRR,
    _reserved3: [u8; 0x04],
    tdlar: TDLAR,
    _reserved4: [u8; 0x04],
    rdlar: RDLAR,
    _reserved5: [u8; 0x04],
    eesr: EESR,
    _reserved6: [u8; 0x04],
    eesipr: EESIPR,
    _reserved7: [u8; 0x04],
    trscer: TRSCER,
    _reserved8: [u8; 0x04],
    rmfcr: RMFCR,
    _reserved9: [u8; 0x04],
    tftr: TFTR,
    _reserved10: [u8; 0x04],
    fdr: FDR,
    _reserved11: [u8; 0x04],
    rmcr: RMCR,
    _reserved12: [u8; 0x08],
    tfucr: TFUCR,
    rfocr: RFOCR,
    iosr: IOSR,
    fcftr: FCFTR,
    _reserved16: [u8; 0x04],
    rpadir: RPADIR,
    trimd: TRIMD,
    _reserved18: [u8; 0x48],
    rbwar: RBWAR,
    rdfar: RDFAR,
    _reserved20: [u8; 0x04],
    tbrar: TBRAR,
    tdfar: TDFAR,
}
impl RegisterBlock {
    ///0x00 - EDMAC Mode Register
    #[inline(always)]
    pub const fn edmr(&self) -> &EDMR {
        &self.edmr
    }
    ///0x08 - EDMAC Transmit Request Register
    #[inline(always)]
    pub const fn edtrr(&self) -> &EDTRR {
        &self.edtrr
    }
    ///0x10 - EDMAC Receive Request Register
    #[inline(always)]
    pub const fn edrrr(&self) -> &EDRRR {
        &self.edrrr
    }
    ///0x18 - Transmit Descriptor List Start Address Register
    #[inline(always)]
    pub const fn tdlar(&self) -> &TDLAR {
        &self.tdlar
    }
    ///0x20 - Receive Descriptor List Start Address Register
    #[inline(always)]
    pub const fn rdlar(&self) -> &RDLAR {
        &self.rdlar
    }
    ///0x28 - ETHERC/EDMAC Status Register
    #[inline(always)]
    pub const fn eesr(&self) -> &EESR {
        &self.eesr
    }
    ///0x30 - ETHERC/EDMAC Status Interrupt Enable Register
    #[inline(always)]
    pub const fn eesipr(&self) -> &EESIPR {
        &self.eesipr
    }
    ///0x38 - ETHERC/EDMAC Transmit/Receive Status Copy Enable Register
    #[inline(always)]
    pub const fn trscer(&self) -> &TRSCER {
        &self.trscer
    }
    ///0x40 - Missed-Frame Counter Register
    #[inline(always)]
    pub const fn rmfcr(&self) -> &RMFCR {
        &self.rmfcr
    }
    ///0x48 - Transmit FIFO Threshold Register
    #[inline(always)]
    pub const fn tftr(&self) -> &TFTR {
        &self.tftr
    }
    ///0x50 - Transmit FIFO Threshold Register
    #[inline(always)]
    pub const fn fdr(&self) -> &FDR {
        &self.fdr
    }
    ///0x58 - Receive Method Control Register
    #[inline(always)]
    pub const fn rmcr(&self) -> &RMCR {
        &self.rmcr
    }
    ///0x64 - Transmit FIFO Underflow Counter
    #[inline(always)]
    pub const fn tfucr(&self) -> &TFUCR {
        &self.tfucr
    }
    ///0x68 - Receive FIFO Overflow Counter
    #[inline(always)]
    pub const fn rfocr(&self) -> &RFOCR {
        &self.rfocr
    }
    ///0x6c - Independent Output Signal Setting Register
    #[inline(always)]
    pub const fn iosr(&self) -> &IOSR {
        &self.iosr
    }
    ///0x70 - Flow Control Start FIFO Threshold Setting Register
    #[inline(always)]
    pub const fn fcftr(&self) -> &FCFTR {
        &self.fcftr
    }
    ///0x78 - Receive Data Padding Insert Register
    #[inline(always)]
    pub const fn rpadir(&self) -> &RPADIR {
        &self.rpadir
    }
    ///0x7c - Transmit Interrupt Setting Register
    #[inline(always)]
    pub const fn trimd(&self) -> &TRIMD {
        &self.trimd
    }
    ///0xc8 - Receive Buffer Write Address Register
    #[inline(always)]
    pub const fn rbwar(&self) -> &RBWAR {
        &self.rbwar
    }
    ///0xcc - Receive Descriptor Fetch Address Register
    #[inline(always)]
    pub const fn rdfar(&self) -> &RDFAR {
        &self.rdfar
    }
    ///0xd4 - Transmit Buffer Read Address Register
    #[inline(always)]
    pub const fn tbrar(&self) -> &TBRAR {
        &self.tbrar
    }
    ///0xd8 - Transmit Descriptor Fetch Address Register
    #[inline(always)]
    pub const fn tdfar(&self) -> &TDFAR {
        &self.tdfar
    }
}
/**EDMR (rw) register accessor: EDMAC Mode Register

You can [`read`](crate::Reg::read) this register and get [`edmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edmr`] module*/
pub type EDMR = crate::Reg<edmr::EDMR_SPEC>;
///EDMAC Mode Register
pub mod edmr;
/**EDTRR (rw) register accessor: EDMAC Transmit Request Register

You can [`read`](crate::Reg::read) this register and get [`edtrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edtrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edtrr`] module*/
pub type EDTRR = crate::Reg<edtrr::EDTRR_SPEC>;
///EDMAC Transmit Request Register
pub mod edtrr;
/**EDRRR (rw) register accessor: EDMAC Receive Request Register

You can [`read`](crate::Reg::read) this register and get [`edrrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edrrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edrrr`] module*/
pub type EDRRR = crate::Reg<edrrr::EDRRR_SPEC>;
///EDMAC Receive Request Register
pub mod edrrr;
/**TDLAR (rw) register accessor: Transmit Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`tdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdlar`] module*/
pub type TDLAR = crate::Reg<tdlar::TDLAR_SPEC>;
///Transmit Descriptor List Start Address Register
pub mod tdlar;
/**RDLAR (rw) register accessor: Receive Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`rdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdlar`] module*/
pub type RDLAR = crate::Reg<rdlar::RDLAR_SPEC>;
///Receive Descriptor List Start Address Register
pub mod rdlar;
/**EESR (rw) register accessor: ETHERC/EDMAC Status Register

You can [`read`](crate::Reg::read) this register and get [`eesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eesr`] module*/
pub type EESR = crate::Reg<eesr::EESR_SPEC>;
///ETHERC/EDMAC Status Register
pub mod eesr;
/**EESIPR (rw) register accessor: ETHERC/EDMAC Status Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`eesipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eesipr`] module*/
pub type EESIPR = crate::Reg<eesipr::EESIPR_SPEC>;
///ETHERC/EDMAC Status Interrupt Enable Register
pub mod eesipr;
/**TRSCER (rw) register accessor: ETHERC/EDMAC Transmit/Receive Status Copy Enable Register

You can [`read`](crate::Reg::read) this register and get [`trscer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trscer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trscer`] module*/
pub type TRSCER = crate::Reg<trscer::TRSCER_SPEC>;
///ETHERC/EDMAC Transmit/Receive Status Copy Enable Register
pub mod trscer;
/**RMFCR (rw) register accessor: Missed-Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rmfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmfcr`] module*/
pub type RMFCR = crate::Reg<rmfcr::RMFCR_SPEC>;
///Missed-Frame Counter Register
pub mod rmfcr;
/**TFTR (rw) register accessor: Transmit FIFO Threshold Register

You can [`read`](crate::Reg::read) this register and get [`tftr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tftr`] module*/
pub type TFTR = crate::Reg<tftr::TFTR_SPEC>;
///Transmit FIFO Threshold Register
pub mod tftr;
/**FDR (rw) register accessor: Transmit FIFO Threshold Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fdr`] module*/
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
///Transmit FIFO Threshold Register
pub mod fdr;
/**RMCR (rw) register accessor: Receive Method Control Register

You can [`read`](crate::Reg::read) this register and get [`rmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmcr`] module*/
pub type RMCR = crate::Reg<rmcr::RMCR_SPEC>;
///Receive Method Control Register
pub mod rmcr;
/**TFUCR (rw) register accessor: Transmit FIFO Underflow Counter

You can [`read`](crate::Reg::read) this register and get [`tfucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tfucr`] module*/
pub type TFUCR = crate::Reg<tfucr::TFUCR_SPEC>;
///Transmit FIFO Underflow Counter
pub mod tfucr;
/**RFOCR (rw) register accessor: Receive FIFO Overflow Counter

You can [`read`](crate::Reg::read) this register and get [`rfocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfocr`] module*/
pub type RFOCR = crate::Reg<rfocr::RFOCR_SPEC>;
///Receive FIFO Overflow Counter
pub mod rfocr;
/**IOSR (rw) register accessor: Independent Output Signal Setting Register

You can [`read`](crate::Reg::read) this register and get [`iosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iosr`] module*/
pub type IOSR = crate::Reg<iosr::IOSR_SPEC>;
///Independent Output Signal Setting Register
pub mod iosr;
/**FCFTR (rw) register accessor: Flow Control Start FIFO Threshold Setting Register

You can [`read`](crate::Reg::read) this register and get [`fcftr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcftr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcftr`] module*/
pub type FCFTR = crate::Reg<fcftr::FCFTR_SPEC>;
///Flow Control Start FIFO Threshold Setting Register
pub mod fcftr;
/**RPADIR (rw) register accessor: Receive Data Padding Insert Register

You can [`read`](crate::Reg::read) this register and get [`rpadir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpadir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rpadir`] module*/
pub type RPADIR = crate::Reg<rpadir::RPADIR_SPEC>;
///Receive Data Padding Insert Register
pub mod rpadir;
/**TRIMD (rw) register accessor: Transmit Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`trimd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trimd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trimd`] module*/
pub type TRIMD = crate::Reg<trimd::TRIMD_SPEC>;
///Transmit Interrupt Setting Register
pub mod trimd;
/**RBWAR (r) register accessor: Receive Buffer Write Address Register

You can [`read`](crate::Reg::read) this register and get [`rbwar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rbwar`] module*/
pub type RBWAR = crate::Reg<rbwar::RBWAR_SPEC>;
///Receive Buffer Write Address Register
pub mod rbwar;
/**RDFAR (r) register accessor: Receive Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`rdfar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdfar`] module*/
pub type RDFAR = crate::Reg<rdfar::RDFAR_SPEC>;
///Receive Descriptor Fetch Address Register
pub mod rdfar;
/**TBRAR (r) register accessor: Transmit Buffer Read Address Register

You can [`read`](crate::Reg::read) this register and get [`tbrar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tbrar`] module*/
pub type TBRAR = crate::Reg<tbrar::TBRAR_SPEC>;
///Transmit Buffer Read Address Register
pub mod tbrar;
/**TDFAR (r) register accessor: Transmit Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`tdfar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdfar`] module*/
pub type TDFAR = crate::Reg<tdfar::TDFAR_SPEC>;
///Transmit Descriptor Fetch Address Register
pub mod tdfar;
