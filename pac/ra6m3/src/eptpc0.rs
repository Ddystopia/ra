#[repr(C)]
///Register block
pub struct RegisterBlock {
    sysr: SYSR,
    syipr: SYIPR,
    _reserved2: [u8; 0x08],
    symacru: SYMACRU,
    symacrl: SYMACRL,
    syllcctlr: SYLLCCTLR,
    syipaddrr: SYIPADDRR,
    _reserved6: [u8; 0x20],
    syspvrr: SYSPVRR,
    sydomr: SYDOMR,
    _reserved8: [u8; 0x08],
    anfr: ANFR,
    synfr: SYNFR,
    dyrqfr: DYRQFR,
    dyrpfr: DYRPFR,
    sycidru: SYCIDRU,
    sycidrl: SYCIDRL,
    sypnumr: SYPNUMR,
    _reserved15: [u8; 0x14],
    syrvldr: SYRVLDR,
    _reserved16: [u8; 0x0c],
    syrfl1r: SYRFL1R,
    syrfl2r: SYRFL2R,
    sytrenr: SYTRENR,
    _reserved19: [u8; 0x04],
    mtcidu: MTCIDU,
    mtcidl: MTCIDL,
    mtpid: MTPID,
    _reserved22: [u8; 0x14],
    sytlir: SYTLIR,
    syrlir: SYRLIR,
    ofmru: OFMRU,
    ofmrl: OFMRL,
    mpdru: MPDRU,
    mpdrl: MPDRL,
    _reserved28: [u8; 0x08],
    gmpr: GMPR,
    gmcqr: GMCQR,
    gmidru: GMIDRU,
    gmidrl: GMIDRL,
    cuotsr: CUOTSR,
    srr: SRR,
    _reserved34: [u8; 0x08],
    ppmacru: PPMACRU,
    ppmacrl: PPMACRL,
    pdmacru: PDMACRU,
    pdmacrl: PDMACRL,
    petyper: PETYPER,
    _reserved39: [u8; 0x0c],
    ppipr: PPIPR,
    pdipr: PDIPR,
    petosr: PETOSR,
    pgtosr: PGTOSR,
    ppttlr: PPTTLR,
    pdttlr: PDTTLR,
    peudpr: PEUDPR,
    pgudpr: PGUDPR,
    ffltr: FFLTR,
    _reserved48: [u8; 0x1c],
    fmacru: (),
    _reserved49: [u8; 0x04],
    fmacrl: (),
    _reserved50: [u8; 0x5c],
    dasymru: DASYMRU,
    dasymrl: DASYMRL,
    tslatr: TSLATR,
    syconfr: SYCONFR,
    syformr: SYFORMR,
    rstoutr: RSTOUTR,
}
impl RegisterBlock {
    ///0x00 - SYNFP Status Register
    #[inline(always)]
    pub const fn sysr(&self) -> &SYSR {
        &self.sysr
    }
    ///0x04 - SYNFP Status Notification Permission Register
    #[inline(always)]
    pub const fn syipr(&self) -> &SYIPR {
        &self.syipr
    }
    ///0x10 - SYNFP MAC Address Registers
    #[inline(always)]
    pub const fn symacru(&self) -> &SYMACRU {
        &self.symacru
    }
    ///0x14 - SYNFP MAC Address Registers
    #[inline(always)]
    pub const fn symacrl(&self) -> &SYMACRL {
        &self.symacrl
    }
    ///0x18 - SYNFP LLC-CTL Value Register
    #[inline(always)]
    pub const fn syllcctlr(&self) -> &SYLLCCTLR {
        &self.syllcctlr
    }
    ///0x1c - SYNFP Local IP Address Register
    #[inline(always)]
    pub const fn syipaddrr(&self) -> &SYIPADDRR {
        &self.syipaddrr
    }
    ///0x40 - SYNFP Specification Version Setting Register
    #[inline(always)]
    pub const fn syspvrr(&self) -> &SYSPVRR {
        &self.syspvrr
    }
    ///0x44 - SYNFP Domain Number Setting Register
    #[inline(always)]
    pub const fn sydomr(&self) -> &SYDOMR {
        &self.sydomr
    }
    ///0x50 - Announce Message Flag Field Setting Register
    #[inline(always)]
    pub const fn anfr(&self) -> &ANFR {
        &self.anfr
    }
    ///0x54 - Sync Message Flag Field Setting Register
    #[inline(always)]
    pub const fn synfr(&self) -> &SYNFR {
        &self.synfr
    }
    ///0x58 - Delay_Req Message Flag Field Setting Register
    #[inline(always)]
    pub const fn dyrqfr(&self) -> &DYRQFR {
        &self.dyrqfr
    }
    ///0x5c - Delay_Resp Message Flag Field Setting Register
    #[inline(always)]
    pub const fn dyrpfr(&self) -> &DYRPFR {
        &self.dyrpfr
    }
    ///0x60 - SYNFP Local Clock ID Registers
    #[inline(always)]
    pub const fn sycidru(&self) -> &SYCIDRU {
        &self.sycidru
    }
    ///0x64 - SYNFP Local Clock ID Registers
    #[inline(always)]
    pub const fn sycidrl(&self) -> &SYCIDRL {
        &self.sycidrl
    }
    ///0x68 - SYNFP Local Port Number Register
    #[inline(always)]
    pub const fn sypnumr(&self) -> &SYPNUMR {
        &self.sypnumr
    }
    ///0x80 - SYNFP Register Value Load Directive Register
    #[inline(always)]
    pub const fn syrvldr(&self) -> &SYRVLDR {
        &self.syrvldr
    }
    ///0x90 - SYNFP Reception Filter Register 1
    #[inline(always)]
    pub const fn syrfl1r(&self) -> &SYRFL1R {
        &self.syrfl1r
    }
    ///0x94 - SYNFP Reception Filter Register 2
    #[inline(always)]
    pub const fn syrfl2r(&self) -> &SYRFL2R {
        &self.syrfl2r
    }
    ///0x98 - SYNFP Transmission Enable Register
    #[inline(always)]
    pub const fn sytrenr(&self) -> &SYTRENR {
        &self.sytrenr
    }
    ///0xa0 - Master Clock ID Registers
    #[inline(always)]
    pub const fn mtcidu(&self) -> &MTCIDU {
        &self.mtcidu
    }
    ///0xa4 - Master Clock ID Registers
    #[inline(always)]
    pub const fn mtcidl(&self) -> &MTCIDL {
        &self.mtcidl
    }
    ///0xa8 - Master clock port number register
    #[inline(always)]
    pub const fn mtpid(&self) -> &MTPID {
        &self.mtpid
    }
    ///0xc0 - SYNFP Transmission Interval Setting Register
    #[inline(always)]
    pub const fn sytlir(&self) -> &SYTLIR {
        &self.sytlir
    }
    ///0xc4 - SYNFP Received logMessageInterval Value Indication Register
    #[inline(always)]
    pub const fn syrlir(&self) -> &SYRLIR {
        &self.syrlir
    }
    ///0xc8 - offsetFromMaster Value Registers
    #[inline(always)]
    pub const fn ofmru(&self) -> &OFMRU {
        &self.ofmru
    }
    ///0xcc - offsetFromMaster Value Registers
    #[inline(always)]
    pub const fn ofmrl(&self) -> &OFMRL {
        &self.ofmrl
    }
    ///0xd0 - meanPathDelay Value Registers
    #[inline(always)]
    pub const fn mpdru(&self) -> &MPDRU {
        &self.mpdru
    }
    ///0xd4 - meanPathDelay Value Registers
    #[inline(always)]
    pub const fn mpdrl(&self) -> &MPDRL {
        &self.mpdrl
    }
    ///0xe0 - grandmasterPriority Field Setting Register
    #[inline(always)]
    pub const fn gmpr(&self) -> &GMPR {
        &self.gmpr
    }
    ///0xe4 - grandmasterClockQuality Field Setting Register
    #[inline(always)]
    pub const fn gmcqr(&self) -> &GMCQR {
        &self.gmcqr
    }
    ///0xe8 - grandmasterIdentity Field Setting Registers
    #[inline(always)]
    pub const fn gmidru(&self) -> &GMIDRU {
        &self.gmidru
    }
    ///0xec - grandmasterIdentity Field Setting Registers
    #[inline(always)]
    pub const fn gmidrl(&self) -> &GMIDRL {
        &self.gmidrl
    }
    ///0xf0 - currentUtcOffset/timeSource Field Setting Register
    #[inline(always)]
    pub const fn cuotsr(&self) -> &CUOTSR {
        &self.cuotsr
    }
    ///0xf4 - stepsRemoved Field Setting Register
    #[inline(always)]
    pub const fn srr(&self) -> &SRR {
        &self.srr
    }
    ///0x100 - PTP-primary Message Destination MAC Address Setting Registers
    #[inline(always)]
    pub const fn ppmacru(&self) -> &PPMACRU {
        &self.ppmacru
    }
    ///0x104 - PTP-primary Message Destination MAC Address Setting Registers
    #[inline(always)]
    pub const fn ppmacrl(&self) -> &PPMACRL {
        &self.ppmacrl
    }
    ///0x108 - PTP-pdelay Message MAC Address Setting Registers
    #[inline(always)]
    pub const fn pdmacru(&self) -> &PDMACRU {
        &self.pdmacru
    }
    ///0x10c - PTP-pdelay Message MAC Address Setting Registers
    #[inline(always)]
    pub const fn pdmacrl(&self) -> &PDMACRL {
        &self.pdmacrl
    }
    ///0x110 - PTP Message EtherType Setting Register
    #[inline(always)]
    pub const fn petyper(&self) -> &PETYPER {
        &self.petyper
    }
    ///0x120 - PTP-primary Message Destination IP Address Setting Register
    #[inline(always)]
    pub const fn ppipr(&self) -> &PPIPR {
        &self.ppipr
    }
    ///0x124 - PTP-pdelay Message Destination IP Address Setting Register
    #[inline(always)]
    pub const fn pdipr(&self) -> &PDIPR {
        &self.pdipr
    }
    ///0x128 - PTP Event Message TOS Setting Register
    #[inline(always)]
    pub const fn petosr(&self) -> &PETOSR {
        &self.petosr
    }
    ///0x12c - PTP general Message TOS Setting Register
    #[inline(always)]
    pub const fn pgtosr(&self) -> &PGTOSR {
        &self.pgtosr
    }
    ///0x130 - PTP-primary Message TTL Setting Register
    #[inline(always)]
    pub const fn ppttlr(&self) -> &PPTTLR {
        &self.ppttlr
    }
    ///0x134 - PTP-pdelay Message TTL Setting Register
    #[inline(always)]
    pub const fn pdttlr(&self) -> &PDTTLR {
        &self.pdttlr
    }
    ///0x138 - PTP Event Message UDP Destination Port Number Setting Register
    #[inline(always)]
    pub const fn peudpr(&self) -> &PEUDPR {
        &self.peudpr
    }
    ///0x13c - PTP general Message UDP Destination Port Number Setting Register
    #[inline(always)]
    pub const fn pgudpr(&self) -> &PGUDPR {
        &self.pgudpr
    }
    ///0x140 - Frame Reception Filter Setting Register
    #[inline(always)]
    pub const fn ffltr(&self) -> &FFLTR {
        &self.ffltr
    }
    ///0x160..0x168 - Frame Reception Filter MAC Address %s Setting Registers
    #[inline(always)]
    pub const fn fmacru(&self, n: usize) -> &FMACRU {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x160..0x168 - Frame Reception Filter MAC Address %s Setting Registers
    #[inline(always)]
    pub fn fmacru_iter(&self) -> impl Iterator<Item = &FMACRU> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(352)
                .add(8 * n)
                .cast()
        })
    }
    ///0x160 - Frame Reception Filter MAC Address 0 Setting Registers
    #[inline(always)]
    pub const fn fmac0ru(&self) -> &FMACRU {
        self.fmacru(0)
    }
    ///0x168 - Frame Reception Filter MAC Address 1 Setting Registers
    #[inline(always)]
    pub const fn fmac1ru(&self) -> &FMACRU {
        self.fmacru(1)
    }
    ///0x164..0x16c - Frame Reception Filter MAC Address %s Setting Registers
    #[inline(always)]
    pub const fn fmacrl(&self, n: usize) -> &FMACRL {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(356)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x164..0x16c - Frame Reception Filter MAC Address %s Setting Registers
    #[inline(always)]
    pub fn fmacrl_iter(&self) -> impl Iterator<Item = &FMACRL> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(356)
                .add(8 * n)
                .cast()
        })
    }
    ///0x164 - Frame Reception Filter MAC Address 0 Setting Registers
    #[inline(always)]
    pub const fn fmac0rl(&self) -> &FMACRL {
        self.fmacrl(0)
    }
    ///0x16c - Frame Reception Filter MAC Address 1 Setting Registers
    #[inline(always)]
    pub const fn fmac1rl(&self) -> &FMACRL {
        self.fmacrl(1)
    }
    ///0x1c0 - Asymmetric Delay Setting Registers
    #[inline(always)]
    pub const fn dasymru(&self) -> &DASYMRU {
        &self.dasymru
    }
    ///0x1c4 - Asymmetric Delay Setting Registers
    #[inline(always)]
    pub const fn dasymrl(&self) -> &DASYMRL {
        &self.dasymrl
    }
    ///0x1c8 - Timestamp Latency Setting Register
    #[inline(always)]
    pub const fn tslatr(&self) -> &TSLATR {
        &self.tslatr
    }
    ///0x1cc - SYNFP Operation Setting Register
    #[inline(always)]
    pub const fn syconfr(&self) -> &SYCONFR {
        &self.syconfr
    }
    ///0x1d0 - SYNFP Frame Format Setting Register
    #[inline(always)]
    pub const fn syformr(&self) -> &SYFORMR {
        &self.syformr
    }
    ///0x1d4 - Response Message Reception Timeout Register
    #[inline(always)]
    pub const fn rstoutr(&self) -> &RSTOUTR {
        &self.rstoutr
    }
}
/**SYSR (rw) register accessor: SYNFP Status Register

You can [`read`](crate::Reg::read) this register and get [`sysr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysr`] module*/
pub type SYSR = crate::Reg<sysr::SYSR_SPEC>;
///SYNFP Status Register
pub mod sysr;
/**SYIPR (rw) register accessor: SYNFP Status Notification Permission Register

You can [`read`](crate::Reg::read) this register and get [`syipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syipr`] module*/
pub type SYIPR = crate::Reg<syipr::SYIPR_SPEC>;
///SYNFP Status Notification Permission Register
pub mod syipr;
/**SYMACRU (rw) register accessor: SYNFP MAC Address Registers

You can [`read`](crate::Reg::read) this register and get [`symacru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`symacru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@symacru`] module*/
pub type SYMACRU = crate::Reg<symacru::SYMACRU_SPEC>;
///SYNFP MAC Address Registers
pub mod symacru;
/**SYMACRL (rw) register accessor: SYNFP MAC Address Registers

You can [`read`](crate::Reg::read) this register and get [`symacrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`symacrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@symacrl`] module*/
pub type SYMACRL = crate::Reg<symacrl::SYMACRL_SPEC>;
///SYNFP MAC Address Registers
pub mod symacrl;
/**SYLLCCTLR (rw) register accessor: SYNFP LLC-CTL Value Register

You can [`read`](crate::Reg::read) this register and get [`syllcctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syllcctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syllcctlr`] module*/
pub type SYLLCCTLR = crate::Reg<syllcctlr::SYLLCCTLR_SPEC>;
///SYNFP LLC-CTL Value Register
pub mod syllcctlr;
/**SYIPADDRR (rw) register accessor: SYNFP Local IP Address Register

You can [`read`](crate::Reg::read) this register and get [`syipaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syipaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syipaddrr`] module*/
pub type SYIPADDRR = crate::Reg<syipaddrr::SYIPADDRR_SPEC>;
///SYNFP Local IP Address Register
pub mod syipaddrr;
/**SYSPVRR (rw) register accessor: SYNFP Specification Version Setting Register

You can [`read`](crate::Reg::read) this register and get [`syspvrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspvrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syspvrr`] module*/
pub type SYSPVRR = crate::Reg<syspvrr::SYSPVRR_SPEC>;
///SYNFP Specification Version Setting Register
pub mod syspvrr;
/**SYDOMR (rw) register accessor: SYNFP Domain Number Setting Register

You can [`read`](crate::Reg::read) this register and get [`sydomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sydomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sydomr`] module*/
pub type SYDOMR = crate::Reg<sydomr::SYDOMR_SPEC>;
///SYNFP Domain Number Setting Register
pub mod sydomr;
/**ANFR (rw) register accessor: Announce Message Flag Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`anfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@anfr`] module*/
pub type ANFR = crate::Reg<anfr::ANFR_SPEC>;
///Announce Message Flag Field Setting Register
pub mod anfr;
/**SYNFR (rw) register accessor: Sync Message Flag Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`synfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@synfr`] module*/
pub type SYNFR = crate::Reg<synfr::SYNFR_SPEC>;
///Sync Message Flag Field Setting Register
pub mod synfr;
/**DYRQFR (rw) register accessor: Delay_Req Message Flag Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`dyrqfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dyrqfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dyrqfr`] module*/
pub type DYRQFR = crate::Reg<dyrqfr::DYRQFR_SPEC>;
///Delay_Req Message Flag Field Setting Register
pub mod dyrqfr;
/**DYRPFR (rw) register accessor: Delay_Resp Message Flag Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`dyrpfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dyrpfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dyrpfr`] module*/
pub type DYRPFR = crate::Reg<dyrpfr::DYRPFR_SPEC>;
///Delay_Resp Message Flag Field Setting Register
pub mod dyrpfr;
/**SYCIDRU (rw) register accessor: SYNFP Local Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`sycidru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sycidru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sycidru`] module*/
pub type SYCIDRU = crate::Reg<sycidru::SYCIDRU_SPEC>;
///SYNFP Local Clock ID Registers
pub mod sycidru;
/**SYCIDRL (rw) register accessor: SYNFP Local Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`sycidrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sycidrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sycidrl`] module*/
pub type SYCIDRL = crate::Reg<sycidrl::SYCIDRL_SPEC>;
///SYNFP Local Clock ID Registers
pub mod sycidrl;
/**SYPNUMR (rw) register accessor: SYNFP Local Port Number Register

You can [`read`](crate::Reg::read) this register and get [`sypnumr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sypnumr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sypnumr`] module*/
pub type SYPNUMR = crate::Reg<sypnumr::SYPNUMR_SPEC>;
///SYNFP Local Port Number Register
pub mod sypnumr;
/**SYRVLDR (w) register accessor: SYNFP Register Value Load Directive Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syrvldr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syrvldr`] module*/
pub type SYRVLDR = crate::Reg<syrvldr::SYRVLDR_SPEC>;
///SYNFP Register Value Load Directive Register
pub mod syrvldr;
/**SYRFL1R (rw) register accessor: SYNFP Reception Filter Register 1

You can [`read`](crate::Reg::read) this register and get [`syrfl1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syrfl1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syrfl1r`] module*/
pub type SYRFL1R = crate::Reg<syrfl1r::SYRFL1R_SPEC>;
///SYNFP Reception Filter Register 1
pub mod syrfl1r;
/**SYRFL2R (rw) register accessor: SYNFP Reception Filter Register 2

You can [`read`](crate::Reg::read) this register and get [`syrfl2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syrfl2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syrfl2r`] module*/
pub type SYRFL2R = crate::Reg<syrfl2r::SYRFL2R_SPEC>;
///SYNFP Reception Filter Register 2
pub mod syrfl2r;
/**SYTRENR (rw) register accessor: SYNFP Transmission Enable Register

You can [`read`](crate::Reg::read) this register and get [`sytrenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sytrenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sytrenr`] module*/
pub type SYTRENR = crate::Reg<sytrenr::SYTRENR_SPEC>;
///SYNFP Transmission Enable Register
pub mod sytrenr;
/**MTCIDU (rw) register accessor: Master Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`mtcidu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcidu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mtcidu`] module*/
pub type MTCIDU = crate::Reg<mtcidu::MTCIDU_SPEC>;
///Master Clock ID Registers
pub mod mtcidu;
/**MTCIDL (rw) register accessor: Master Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`mtcidl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcidl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mtcidl`] module*/
pub type MTCIDL = crate::Reg<mtcidl::MTCIDL_SPEC>;
///Master Clock ID Registers
pub mod mtcidl;
/**MTPID (rw) register accessor: Master clock port number register

You can [`read`](crate::Reg::read) this register and get [`mtpid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtpid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mtpid`] module*/
pub type MTPID = crate::Reg<mtpid::MTPID_SPEC>;
///Master clock port number register
pub mod mtpid;
/**SYTLIR (rw) register accessor: SYNFP Transmission Interval Setting Register

You can [`read`](crate::Reg::read) this register and get [`sytlir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sytlir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sytlir`] module*/
pub type SYTLIR = crate::Reg<sytlir::SYTLIR_SPEC>;
///SYNFP Transmission Interval Setting Register
pub mod sytlir;
/**SYRLIR (r) register accessor: SYNFP Received logMessageInterval Value Indication Register

You can [`read`](crate::Reg::read) this register and get [`syrlir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syrlir`] module*/
pub type SYRLIR = crate::Reg<syrlir::SYRLIR_SPEC>;
///SYNFP Received logMessageInterval Value Indication Register
pub mod syrlir;
/**OFMRU (r) register accessor: offsetFromMaster Value Registers

You can [`read`](crate::Reg::read) this register and get [`ofmru::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ofmru`] module*/
pub type OFMRU = crate::Reg<ofmru::OFMRU_SPEC>;
///offsetFromMaster Value Registers
pub mod ofmru;
/**OFMRL (r) register accessor: offsetFromMaster Value Registers

You can [`read`](crate::Reg::read) this register and get [`ofmrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ofmrl`] module*/
pub type OFMRL = crate::Reg<ofmrl::OFMRL_SPEC>;
///offsetFromMaster Value Registers
pub mod ofmrl;
/**MPDRU (r) register accessor: meanPathDelay Value Registers

You can [`read`](crate::Reg::read) this register and get [`mpdru::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mpdru`] module*/
pub type MPDRU = crate::Reg<mpdru::MPDRU_SPEC>;
///meanPathDelay Value Registers
pub mod mpdru;
/**MPDRL (r) register accessor: meanPathDelay Value Registers

You can [`read`](crate::Reg::read) this register and get [`mpdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mpdrl`] module*/
pub type MPDRL = crate::Reg<mpdrl::MPDRL_SPEC>;
///meanPathDelay Value Registers
pub mod mpdrl;
/**GMPR (rw) register accessor: grandmasterPriority Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`gmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gmpr`] module*/
pub type GMPR = crate::Reg<gmpr::GMPR_SPEC>;
///grandmasterPriority Field Setting Register
pub mod gmpr;
/**GMCQR (rw) register accessor: grandmasterClockQuality Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`gmcqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmcqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gmcqr`] module*/
pub type GMCQR = crate::Reg<gmcqr::GMCQR_SPEC>;
///grandmasterClockQuality Field Setting Register
pub mod gmcqr;
/**GMIDRU (rw) register accessor: grandmasterIdentity Field Setting Registers

You can [`read`](crate::Reg::read) this register and get [`gmidru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmidru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gmidru`] module*/
pub type GMIDRU = crate::Reg<gmidru::GMIDRU_SPEC>;
///grandmasterIdentity Field Setting Registers
pub mod gmidru;
/**GMIDRL (rw) register accessor: grandmasterIdentity Field Setting Registers

You can [`read`](crate::Reg::read) this register and get [`gmidrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmidrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gmidrl`] module*/
pub type GMIDRL = crate::Reg<gmidrl::GMIDRL_SPEC>;
///grandmasterIdentity Field Setting Registers
pub mod gmidrl;
/**CUOTSR (rw) register accessor: currentUtcOffset/timeSource Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`cuotsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cuotsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cuotsr`] module*/
pub type CUOTSR = crate::Reg<cuotsr::CUOTSR_SPEC>;
///currentUtcOffset/timeSource Field Setting Register
pub mod cuotsr;
/**SRR (rw) register accessor: stepsRemoved Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`srr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@srr`] module*/
pub type SRR = crate::Reg<srr::SRR_SPEC>;
///stepsRemoved Field Setting Register
pub mod srr;
/**PPMACRU (rw) register accessor: PTP-primary Message Destination MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`ppmacru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppmacru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ppmacru`] module*/
pub type PPMACRU = crate::Reg<ppmacru::PPMACRU_SPEC>;
///PTP-primary Message Destination MAC Address Setting Registers
pub mod ppmacru;
/**PPMACRL (rw) register accessor: PTP-primary Message Destination MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`ppmacrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppmacrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ppmacrl`] module*/
pub type PPMACRL = crate::Reg<ppmacrl::PPMACRL_SPEC>;
///PTP-primary Message Destination MAC Address Setting Registers
pub mod ppmacrl;
/**PDMACRU (rw) register accessor: PTP-pdelay Message MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`pdmacru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmacru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pdmacru`] module*/
pub type PDMACRU = crate::Reg<pdmacru::PDMACRU_SPEC>;
///PTP-pdelay Message MAC Address Setting Registers
pub mod pdmacru;
/**PDMACRL (rw) register accessor: PTP-pdelay Message MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`pdmacrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmacrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pdmacrl`] module*/
pub type PDMACRL = crate::Reg<pdmacrl::PDMACRL_SPEC>;
///PTP-pdelay Message MAC Address Setting Registers
pub mod pdmacrl;
/**PETYPER (rw) register accessor: PTP Message EtherType Setting Register

You can [`read`](crate::Reg::read) this register and get [`petyper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`petyper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@petyper`] module*/
pub type PETYPER = crate::Reg<petyper::PETYPER_SPEC>;
///PTP Message EtherType Setting Register
pub mod petyper;
/**PPIPR (rw) register accessor: PTP-primary Message Destination IP Address Setting Register

You can [`read`](crate::Reg::read) this register and get [`ppipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ppipr`] module*/
pub type PPIPR = crate::Reg<ppipr::PPIPR_SPEC>;
///PTP-primary Message Destination IP Address Setting Register
pub mod ppipr;
/**PDIPR (rw) register accessor: PTP-pdelay Message Destination IP Address Setting Register

You can [`read`](crate::Reg::read) this register and get [`pdipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pdipr`] module*/
pub type PDIPR = crate::Reg<pdipr::PDIPR_SPEC>;
///PTP-pdelay Message Destination IP Address Setting Register
pub mod pdipr;
/**PETOSR (rw) register accessor: PTP Event Message TOS Setting Register

You can [`read`](crate::Reg::read) this register and get [`petosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`petosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@petosr`] module*/
pub type PETOSR = crate::Reg<petosr::PETOSR_SPEC>;
///PTP Event Message TOS Setting Register
pub mod petosr;
/**PGTOSR (rw) register accessor: PTP general Message TOS Setting Register

You can [`read`](crate::Reg::read) this register and get [`pgtosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgtosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgtosr`] module*/
pub type PGTOSR = crate::Reg<pgtosr::PGTOSR_SPEC>;
///PTP general Message TOS Setting Register
pub mod pgtosr;
/**PPTTLR (rw) register accessor: PTP-primary Message TTL Setting Register

You can [`read`](crate::Reg::read) this register and get [`ppttlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppttlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ppttlr`] module*/
pub type PPTTLR = crate::Reg<ppttlr::PPTTLR_SPEC>;
///PTP-primary Message TTL Setting Register
pub mod ppttlr;
/**PDTTLR (rw) register accessor: PTP-pdelay Message TTL Setting Register

You can [`read`](crate::Reg::read) this register and get [`pdttlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdttlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pdttlr`] module*/
pub type PDTTLR = crate::Reg<pdttlr::PDTTLR_SPEC>;
///PTP-pdelay Message TTL Setting Register
pub mod pdttlr;
/**PEUDPR (rw) register accessor: PTP Event Message UDP Destination Port Number Setting Register

You can [`read`](crate::Reg::read) this register and get [`peudpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peudpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@peudpr`] module*/
pub type PEUDPR = crate::Reg<peudpr::PEUDPR_SPEC>;
///PTP Event Message UDP Destination Port Number Setting Register
pub mod peudpr;
/**PGUDPR (rw) register accessor: PTP general Message UDP Destination Port Number Setting Register

You can [`read`](crate::Reg::read) this register and get [`pgudpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgudpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pgudpr`] module*/
pub type PGUDPR = crate::Reg<pgudpr::PGUDPR_SPEC>;
///PTP general Message UDP Destination Port Number Setting Register
pub mod pgudpr;
/**FFLTR (rw) register accessor: Frame Reception Filter Setting Register

You can [`read`](crate::Reg::read) this register and get [`ffltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ffltr`] module*/
pub type FFLTR = crate::Reg<ffltr::FFLTR_SPEC>;
///Frame Reception Filter Setting Register
pub mod ffltr;
/**FMACRU (rw) register accessor: Frame Reception Filter MAC Address %s Setting Registers

You can [`read`](crate::Reg::read) this register and get [`fmacru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmacru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fmacru`] module*/
pub type FMACRU = crate::Reg<fmacru::FMACRU_SPEC>;
///Frame Reception Filter MAC Address %s Setting Registers
pub mod fmacru;
/**FMACRL (rw) register accessor: Frame Reception Filter MAC Address %s Setting Registers

You can [`read`](crate::Reg::read) this register and get [`fmacrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmacrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fmacrl`] module*/
pub type FMACRL = crate::Reg<fmacrl::FMACRL_SPEC>;
///Frame Reception Filter MAC Address %s Setting Registers
pub mod fmacrl;
/**DASYMRU (rw) register accessor: Asymmetric Delay Setting Registers

You can [`read`](crate::Reg::read) this register and get [`dasymru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dasymru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dasymru`] module*/
pub type DASYMRU = crate::Reg<dasymru::DASYMRU_SPEC>;
///Asymmetric Delay Setting Registers
pub mod dasymru;
/**DASYMRL (rw) register accessor: Asymmetric Delay Setting Registers

You can [`read`](crate::Reg::read) this register and get [`dasymrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dasymrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dasymrl`] module*/
pub type DASYMRL = crate::Reg<dasymrl::DASYMRL_SPEC>;
///Asymmetric Delay Setting Registers
pub mod dasymrl;
/**TSLATR (rw) register accessor: Timestamp Latency Setting Register

You can [`read`](crate::Reg::read) this register and get [`tslatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tslatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tslatr`] module*/
pub type TSLATR = crate::Reg<tslatr::TSLATR_SPEC>;
///Timestamp Latency Setting Register
pub mod tslatr;
/**SYCONFR (rw) register accessor: SYNFP Operation Setting Register

You can [`read`](crate::Reg::read) this register and get [`syconfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syconfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syconfr`] module*/
pub type SYCONFR = crate::Reg<syconfr::SYCONFR_SPEC>;
///SYNFP Operation Setting Register
pub mod syconfr;
/**SYFORMR (rw) register accessor: SYNFP Frame Format Setting Register

You can [`read`](crate::Reg::read) this register and get [`syformr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syformr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syformr`] module*/
pub type SYFORMR = crate::Reg<syformr::SYFORMR_SPEC>;
///SYNFP Frame Format Setting Register
pub mod syformr;
/**RSTOUTR (rw) register accessor: Response Message Reception Timeout Register

You can [`read`](crate::Reg::read) this register and get [`rstoutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstoutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstoutr`] module*/
pub type RSTOUTR = crate::Reg<rstoutr::RSTOUTR_SPEC>;
///Response Message Reception Timeout Register
pub mod rstoutr;
