#[repr(C)]
///Register block
pub struct RegisterBlock {
    miesr: MIESR,
    mieipr: MIEIPR,
    _reserved2: [u8; 0x08],
    elippr: ELIPPR,
    elipacr: ELIPACR,
    _reserved4: [u8; 0x28],
    stsr: STSR,
    stipr: STIPR,
    _reserved6: [u8; 0x08],
    stcfr: STCFR,
    stmr: STMR,
    syntor: SYNTOR,
    _reserved9: [u8; 0x04],
    iptselr: IPTSELR,
    mitselr: MITSELR,
    eltselr: ELTSELR,
    stchselr: STCHSELR,
    _reserved13: [u8; 0x10],
    synstartr: SYNSTARTR,
    lcivldr: LCIVLDR,
    _reserved15: [u8; 0x08],
    syntdaru: SYNTDARU,
    syntdarl: SYNTDARL,
    syntdbru: SYNTDBRU,
    syntdbrl: SYNTDBRL,
    _reserved19: [u8; 0x10],
    lcivru: LCIVRU,
    lcivrm: LCIVRM,
    lcivrl: LCIVRL,
    _reserved22: [u8; 0x68],
    getw10r: GETW10R,
    plimitru: PLIMITRU,
    plimitrm: PLIMITRM,
    plimitrl: PLIMITRL,
    mlimitru: MLIMITRU,
    mlimitrm: MLIMITRM,
    mlimitrl: MLIMITRL,
    getinfor: GETINFOR,
    _reserved30: [u8; 0x2c],
    lccvru: LCCVRU,
    lccvrm: LCCVRM,
    lccvrl: LCCVRL,
    _reserved33: [u8; 0x94],
    pw10vru: PW10VRU,
    pw10vrm: PW10VRM,
    pw10vrl: PW10VRL,
    _reserved36: [u8; 0xb4],
    mw10ru: MW10RU,
    mw10rm: MW10RM,
    mw10rl: MW10RL,
    _reserved39: [u8; 0x24],
    tmsttru: (),
    _reserved40: [u8; 0x04],
    tmsttrl: (),
    _reserved41: [u8; 0x04],
    tmcycr: (),
    _reserved42: [u8; 0x04],
    tmplsr: (),
    _reserved43: [u8; 0x70],
    tmstartr: TMSTARTR,
}
impl RegisterBlock {
    ///0x00 - MINT Interrupt Source Status Register
    #[inline(always)]
    pub const fn miesr(&self) -> &MIESR {
        &self.miesr
    }
    ///0x04 - MINT Interrupt Request Permission Register
    #[inline(always)]
    pub const fn mieipr(&self) -> &MIEIPR {
        &self.mieipr
    }
    ///0x10 - ELC Output/IPLS Interrupt Request Permission Register
    #[inline(always)]
    pub const fn elippr(&self) -> &ELIPPR {
        &self.elippr
    }
    ///0x14 - ELC Output/IPLS Interrupt Permission Automatic Clearing Register
    #[inline(always)]
    pub const fn elipacr(&self) -> &ELIPACR {
        &self.elipacr
    }
    ///0x40 - STCA Status Register
    #[inline(always)]
    pub const fn stsr(&self) -> &STSR {
        &self.stsr
    }
    ///0x44 - STCA Status Notification Permission Register
    #[inline(always)]
    pub const fn stipr(&self) -> &STIPR {
        &self.stipr
    }
    ///0x50 - STCA Clock Frequency Setting Register
    #[inline(always)]
    pub const fn stcfr(&self) -> &STCFR {
        &self.stcfr
    }
    ///0x54 - STCA Operating Mode Register
    #[inline(always)]
    pub const fn stmr(&self) -> &STMR {
        &self.stmr
    }
    ///0x58 - Sync Message Reception Timeout Register
    #[inline(always)]
    pub const fn syntor(&self) -> &SYNTOR {
        &self.syntor
    }
    ///0x60 - IPLS Interrupt Request Timer Select Register
    #[inline(always)]
    pub const fn iptselr(&self) -> &IPTSELR {
        &self.iptselr
    }
    ///0x64 - MINT Interrupt Request Timer Select Register
    #[inline(always)]
    pub const fn mitselr(&self) -> &MITSELR {
        &self.mitselr
    }
    ///0x68 - ELC Output Timer Select Register
    #[inline(always)]
    pub const fn eltselr(&self) -> &ELTSELR {
        &self.eltselr
    }
    ///0x6c - Time Synchronization Channel Select Register
    #[inline(always)]
    pub const fn stchselr(&self) -> &STCHSELR {
        &self.stchselr
    }
    ///0x80 - Slave Time Synchronization Start Register
    #[inline(always)]
    pub const fn synstartr(&self) -> &SYNSTARTR {
        &self.synstartr
    }
    ///0x84 - Local Time Counter Initial Value Load Directive Register
    #[inline(always)]
    pub const fn lcivldr(&self) -> &LCIVLDR {
        &self.lcivldr
    }
    ///0x90 - Synchronization Loss Detection Threshold Registers
    #[inline(always)]
    pub const fn syntdaru(&self) -> &SYNTDARU {
        &self.syntdaru
    }
    ///0x94 - Synchronization Loss Detection Threshold Registers
    #[inline(always)]
    pub const fn syntdarl(&self) -> &SYNTDARL {
        &self.syntdarl
    }
    ///0x98 - Synchronization Detection Threshold Registers
    #[inline(always)]
    pub const fn syntdbru(&self) -> &SYNTDBRU {
        &self.syntdbru
    }
    ///0x9c - Synchronization Detection Threshold Registers
    #[inline(always)]
    pub const fn syntdbrl(&self) -> &SYNTDBRL {
        &self.syntdbrl
    }
    ///0xb0 - Local Time Counter Initial Value Registers
    #[inline(always)]
    pub const fn lcivru(&self) -> &LCIVRU {
        &self.lcivru
    }
    ///0xb4 - Local Time Counter Initial Value Registers
    #[inline(always)]
    pub const fn lcivrm(&self) -> &LCIVRM {
        &self.lcivrm
    }
    ///0xb8 - Local Time Counter Initial Value Registers
    #[inline(always)]
    pub const fn lcivrl(&self) -> &LCIVRL {
        &self.lcivrl
    }
    ///0x124 - Worst 10 Acquisition Directive Register
    #[inline(always)]
    pub const fn getw10r(&self) -> &GETW10R {
        &self.getw10r
    }
    ///0x128 - Positive Gradient Limit Registers
    #[inline(always)]
    pub const fn plimitru(&self) -> &PLIMITRU {
        &self.plimitru
    }
    ///0x12c - Positive Gradient Limit Registers
    #[inline(always)]
    pub const fn plimitrm(&self) -> &PLIMITRM {
        &self.plimitrm
    }
    ///0x130 - Positive Gradient Limit Registers
    #[inline(always)]
    pub const fn plimitrl(&self) -> &PLIMITRL {
        &self.plimitrl
    }
    ///0x134 - Negative Gradient Limit Registers
    #[inline(always)]
    pub const fn mlimitru(&self) -> &MLIMITRU {
        &self.mlimitru
    }
    ///0x138 - Negative Gradient Limit Registers
    #[inline(always)]
    pub const fn mlimitrm(&self) -> &MLIMITRM {
        &self.mlimitrm
    }
    ///0x13c - Negative Gradient Limit Registers
    #[inline(always)]
    pub const fn mlimitrl(&self) -> &MLIMITRL {
        &self.mlimitrl
    }
    ///0x140 - Statistical Information Retention Control Register
    #[inline(always)]
    pub const fn getinfor(&self) -> &GETINFOR {
        &self.getinfor
    }
    ///0x170 - Local Time Counters
    #[inline(always)]
    pub const fn lccvru(&self) -> &LCCVRU {
        &self.lccvru
    }
    ///0x174 - Local Time Counters
    #[inline(always)]
    pub const fn lccvrm(&self) -> &LCCVRM {
        &self.lccvrm
    }
    ///0x178 - Local Time Counters
    #[inline(always)]
    pub const fn lccvrl(&self) -> &LCCVRL {
        &self.lccvrl
    }
    ///0x210 - Positive Gradient Worst 10 Value Registers
    #[inline(always)]
    pub const fn pw10vru(&self) -> &PW10VRU {
        &self.pw10vru
    }
    ///0x214 - Positive Gradient Worst 10 Value Registers
    #[inline(always)]
    pub const fn pw10vrm(&self) -> &PW10VRM {
        &self.pw10vrm
    }
    ///0x218 - Positive Gradient Worst 10 Value Registers
    #[inline(always)]
    pub const fn pw10vrl(&self) -> &PW10VRL {
        &self.pw10vrl
    }
    ///0x2d0 - Negative Gradient Worst 10 Value Registers
    #[inline(always)]
    pub const fn mw10ru(&self) -> &MW10RU {
        &self.mw10ru
    }
    ///0x2d4 - Negative Gradient Worst 10 Value Registers
    #[inline(always)]
    pub const fn mw10rm(&self) -> &MW10RM {
        &self.mw10rm
    }
    ///0x2d8 - Negative Gradient Worst 10 Value Registers
    #[inline(always)]
    pub const fn mw10rl(&self) -> &MW10RL {
        &self.mw10rl
    }
    ///0x300..0x318 - Timer Start Time Setting Register %s
    #[inline(always)]
    pub const fn tmsttru(&self, n: usize) -> &TMSTTRU {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(768)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x300..0x318 - Timer Start Time Setting Register %s
    #[inline(always)]
    pub fn tmsttru_iter(&self) -> impl Iterator<Item = &TMSTTRU> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(768)
                .add(16 * n)
                .cast()
        })
    }
    ///0x304..0x31c - Timer Start Time Setting Register %s
    #[inline(always)]
    pub const fn tmsttrl(&self, n: usize) -> &TMSTTRL {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(772)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x304..0x31c - Timer Start Time Setting Register %s
    #[inline(always)]
    pub fn tmsttrl_iter(&self) -> impl Iterator<Item = &TMSTTRL> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(772)
                .add(16 * n)
                .cast()
        })
    }
    ///0x308..0x320 - Timer Cycle Setting Registers %s
    #[inline(always)]
    pub const fn tmcycr(&self, n: usize) -> &TMCYCR {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(776)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x308..0x320 - Timer Cycle Setting Registers %s
    #[inline(always)]
    pub fn tmcycr_iter(&self) -> impl Iterator<Item = &TMCYCR> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(776)
                .add(16 * n)
                .cast()
        })
    }
    ///0x30c..0x324 - Timer Pulse Width Setting Register %s
    #[inline(always)]
    pub const fn tmplsr(&self, n: usize) -> &TMPLSR {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(780)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x30c..0x324 - Timer Pulse Width Setting Register %s
    #[inline(always)]
    pub fn tmplsr_iter(&self) -> impl Iterator<Item = &TMPLSR> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(780)
                .add(16 * n)
                .cast()
        })
    }
    ///0x37c - Timer Start Register
    #[inline(always)]
    pub const fn tmstartr(&self) -> &TMSTARTR {
        &self.tmstartr
    }
}
/**MIESR (rw) register accessor: MINT Interrupt Source Status Register

You can [`read`](crate::Reg::read) this register and get [`miesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@miesr`] module*/
pub type MIESR = crate::Reg<miesr::MIESR_SPEC>;
///MINT Interrupt Source Status Register
pub mod miesr;
/**MIEIPR (rw) register accessor: MINT Interrupt Request Permission Register

You can [`read`](crate::Reg::read) this register and get [`mieipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mieipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mieipr`] module*/
pub type MIEIPR = crate::Reg<mieipr::MIEIPR_SPEC>;
///MINT Interrupt Request Permission Register
pub mod mieipr;
/**ELIPPR (rw) register accessor: ELC Output/IPLS Interrupt Request Permission Register

You can [`read`](crate::Reg::read) this register and get [`elippr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elippr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elippr`] module*/
pub type ELIPPR = crate::Reg<elippr::ELIPPR_SPEC>;
///ELC Output/IPLS Interrupt Request Permission Register
pub mod elippr;
/**ELIPACR (rw) register accessor: ELC Output/IPLS Interrupt Permission Automatic Clearing Register

You can [`read`](crate::Reg::read) this register and get [`elipacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elipacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@elipacr`] module*/
pub type ELIPACR = crate::Reg<elipacr::ELIPACR_SPEC>;
///ELC Output/IPLS Interrupt Permission Automatic Clearing Register
pub mod elipacr;
/**STSR (rw) register accessor: STCA Status Register

You can [`read`](crate::Reg::read) this register and get [`stsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stsr`] module*/
pub type STSR = crate::Reg<stsr::STSR_SPEC>;
///STCA Status Register
pub mod stsr;
/**STIPR (rw) register accessor: STCA Status Notification Permission Register

You can [`read`](crate::Reg::read) this register and get [`stipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stipr`] module*/
pub type STIPR = crate::Reg<stipr::STIPR_SPEC>;
///STCA Status Notification Permission Register
pub mod stipr;
/**STCFR (rw) register accessor: STCA Clock Frequency Setting Register

You can [`read`](crate::Reg::read) this register and get [`stcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stcfr`] module*/
pub type STCFR = crate::Reg<stcfr::STCFR_SPEC>;
///STCA Clock Frequency Setting Register
pub mod stcfr;
/**STMR (rw) register accessor: STCA Operating Mode Register

You can [`read`](crate::Reg::read) this register and get [`stmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stmr`] module*/
pub type STMR = crate::Reg<stmr::STMR_SPEC>;
///STCA Operating Mode Register
pub mod stmr;
/**SYNTOR (rw) register accessor: Sync Message Reception Timeout Register

You can [`read`](crate::Reg::read) this register and get [`syntor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syntor`] module*/
pub type SYNTOR = crate::Reg<syntor::SYNTOR_SPEC>;
///Sync Message Reception Timeout Register
pub mod syntor;
/**IPTSELR (rw) register accessor: IPLS Interrupt Request Timer Select Register

You can [`read`](crate::Reg::read) this register and get [`iptselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iptselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iptselr`] module*/
pub type IPTSELR = crate::Reg<iptselr::IPTSELR_SPEC>;
///IPLS Interrupt Request Timer Select Register
pub mod iptselr;
/**MITSELR (rw) register accessor: MINT Interrupt Request Timer Select Register

You can [`read`](crate::Reg::read) this register and get [`mitselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mitselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mitselr`] module*/
pub type MITSELR = crate::Reg<mitselr::MITSELR_SPEC>;
///MINT Interrupt Request Timer Select Register
pub mod mitselr;
/**ELTSELR (rw) register accessor: ELC Output Timer Select Register

You can [`read`](crate::Reg::read) this register and get [`eltselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eltselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eltselr`] module*/
pub type ELTSELR = crate::Reg<eltselr::ELTSELR_SPEC>;
///ELC Output Timer Select Register
pub mod eltselr;
/**STCHSELR (rw) register accessor: Time Synchronization Channel Select Register

You can [`read`](crate::Reg::read) this register and get [`stchselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stchselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stchselr`] module*/
pub type STCHSELR = crate::Reg<stchselr::STCHSELR_SPEC>;
///Time Synchronization Channel Select Register
pub mod stchselr;
/**SYNSTARTR (rw) register accessor: Slave Time Synchronization Start Register

You can [`read`](crate::Reg::read) this register and get [`synstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@synstartr`] module*/
pub type SYNSTARTR = crate::Reg<synstartr::SYNSTARTR_SPEC>;
///Slave Time Synchronization Start Register
pub mod synstartr;
/**LCIVLDR (w) register accessor: Local Time Counter Initial Value Load Directive Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivldr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcivldr`] module*/
pub type LCIVLDR = crate::Reg<lcivldr::LCIVLDR_SPEC>;
///Local Time Counter Initial Value Load Directive Register
pub mod lcivldr;
/**SYNTDARU (rw) register accessor: Synchronization Loss Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdaru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdaru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syntdaru`] module*/
pub type SYNTDARU = crate::Reg<syntdaru::SYNTDARU_SPEC>;
///Synchronization Loss Detection Threshold Registers
pub mod syntdaru;
/**SYNTDARL (rw) register accessor: Synchronization Loss Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdarl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdarl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syntdarl`] module*/
pub type SYNTDARL = crate::Reg<syntdarl::SYNTDARL_SPEC>;
///Synchronization Loss Detection Threshold Registers
pub mod syntdarl;
/**SYNTDBRU (rw) register accessor: Synchronization Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdbru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdbru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syntdbru`] module*/
pub type SYNTDBRU = crate::Reg<syntdbru::SYNTDBRU_SPEC>;
///Synchronization Detection Threshold Registers
pub mod syntdbru;
/**SYNTDBRL (rw) register accessor: Synchronization Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdbrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdbrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syntdbrl`] module*/
pub type SYNTDBRL = crate::Reg<syntdbrl::SYNTDBRL_SPEC>;
///Synchronization Detection Threshold Registers
pub mod syntdbrl;
/**LCIVRU (rw) register accessor: Local Time Counter Initial Value Registers

You can [`read`](crate::Reg::read) this register and get [`lcivru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcivru`] module*/
pub type LCIVRU = crate::Reg<lcivru::LCIVRU_SPEC>;
///Local Time Counter Initial Value Registers
pub mod lcivru;
/**LCIVRM (rw) register accessor: Local Time Counter Initial Value Registers

You can [`read`](crate::Reg::read) this register and get [`lcivrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcivrm`] module*/
pub type LCIVRM = crate::Reg<lcivrm::LCIVRM_SPEC>;
///Local Time Counter Initial Value Registers
pub mod lcivrm;
/**LCIVRL (rw) register accessor: Local Time Counter Initial Value Registers

You can [`read`](crate::Reg::read) this register and get [`lcivrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcivrl`] module*/
pub type LCIVRL = crate::Reg<lcivrl::LCIVRL_SPEC>;
///Local Time Counter Initial Value Registers
pub mod lcivrl;
/**GETW10R (rw) register accessor: Worst 10 Acquisition Directive Register

You can [`read`](crate::Reg::read) this register and get [`getw10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getw10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@getw10r`] module*/
pub type GETW10R = crate::Reg<getw10r::GETW10R_SPEC>;
///Worst 10 Acquisition Directive Register
pub mod getw10r;
/**PLIMITRU (rw) register accessor: Positive Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`plimitru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plimitru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@plimitru`] module*/
pub type PLIMITRU = crate::Reg<plimitru::PLIMITRU_SPEC>;
///Positive Gradient Limit Registers
pub mod plimitru;
/**PLIMITRM (rw) register accessor: Positive Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`plimitrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plimitrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@plimitrm`] module*/
pub type PLIMITRM = crate::Reg<plimitrm::PLIMITRM_SPEC>;
///Positive Gradient Limit Registers
pub mod plimitrm;
/**PLIMITRL (rw) register accessor: Positive Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`plimitrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plimitrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@plimitrl`] module*/
pub type PLIMITRL = crate::Reg<plimitrl::PLIMITRL_SPEC>;
///Positive Gradient Limit Registers
pub mod plimitrl;
/**MLIMITRU (rw) register accessor: Negative Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`mlimitru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlimitru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mlimitru`] module*/
pub type MLIMITRU = crate::Reg<mlimitru::MLIMITRU_SPEC>;
///Negative Gradient Limit Registers
pub mod mlimitru;
/**MLIMITRM (rw) register accessor: Negative Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`mlimitrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlimitrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mlimitrm`] module*/
pub type MLIMITRM = crate::Reg<mlimitrm::MLIMITRM_SPEC>;
///Negative Gradient Limit Registers
pub mod mlimitrm;
/**MLIMITRL (rw) register accessor: Negative Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`mlimitrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlimitrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mlimitrl`] module*/
pub type MLIMITRL = crate::Reg<mlimitrl::MLIMITRL_SPEC>;
///Negative Gradient Limit Registers
pub mod mlimitrl;
/**GETINFOR (rw) register accessor: Statistical Information Retention Control Register

You can [`read`](crate::Reg::read) this register and get [`getinfor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`getinfor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@getinfor`] module*/
pub type GETINFOR = crate::Reg<getinfor::GETINFOR_SPEC>;
///Statistical Information Retention Control Register
pub mod getinfor;
/**LCCVRU (r) register accessor: Local Time Counters

You can [`read`](crate::Reg::read) this register and get [`lccvru::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lccvru`] module*/
pub type LCCVRU = crate::Reg<lccvru::LCCVRU_SPEC>;
///Local Time Counters
pub mod lccvru;
/**LCCVRM (r) register accessor: Local Time Counters

You can [`read`](crate::Reg::read) this register and get [`lccvrm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lccvrm`] module*/
pub type LCCVRM = crate::Reg<lccvrm::LCCVRM_SPEC>;
///Local Time Counters
pub mod lccvrm;
/**LCCVRL (r) register accessor: Local Time Counters

You can [`read`](crate::Reg::read) this register and get [`lccvrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lccvrl`] module*/
pub type LCCVRL = crate::Reg<lccvrl::LCCVRL_SPEC>;
///Local Time Counters
pub mod lccvrl;
/**PW10VRU (r) register accessor: Positive Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`pw10vru::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pw10vru`] module*/
pub type PW10VRU = crate::Reg<pw10vru::PW10VRU_SPEC>;
///Positive Gradient Worst 10 Value Registers
pub mod pw10vru;
/**PW10VRM (r) register accessor: Positive Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`pw10vrm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pw10vrm`] module*/
pub type PW10VRM = crate::Reg<pw10vrm::PW10VRM_SPEC>;
///Positive Gradient Worst 10 Value Registers
pub mod pw10vrm;
/**PW10VRL (r) register accessor: Positive Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`pw10vrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pw10vrl`] module*/
pub type PW10VRL = crate::Reg<pw10vrl::PW10VRL_SPEC>;
///Positive Gradient Worst 10 Value Registers
pub mod pw10vrl;
/**MW10RU (r) register accessor: Negative Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`mw10ru::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mw10ru`] module*/
pub type MW10RU = crate::Reg<mw10ru::MW10RU_SPEC>;
///Negative Gradient Worst 10 Value Registers
pub mod mw10ru;
/**MW10RM (r) register accessor: Negative Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`mw10rm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mw10rm`] module*/
pub type MW10RM = crate::Reg<mw10rm::MW10RM_SPEC>;
///Negative Gradient Worst 10 Value Registers
pub mod mw10rm;
/**MW10RL (r) register accessor: Negative Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`mw10rl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mw10rl`] module*/
pub type MW10RL = crate::Reg<mw10rl::MW10RL_SPEC>;
///Negative Gradient Worst 10 Value Registers
pub mod mw10rl;
/**TMSTTRU (rw) register accessor: Timer Start Time Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`tmsttru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsttru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmsttru`] module*/
pub type TMSTTRU = crate::Reg<tmsttru::TMSTTRU_SPEC>;
///Timer Start Time Setting Register %s
pub mod tmsttru;
/**TMSTTRL (rw) register accessor: Timer Start Time Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`tmsttrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsttrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmsttrl`] module*/
pub type TMSTTRL = crate::Reg<tmsttrl::TMSTTRL_SPEC>;
///Timer Start Time Setting Register %s
pub mod tmsttrl;
/**TMCYCR (rw) register accessor: Timer Cycle Setting Registers %s

You can [`read`](crate::Reg::read) this register and get [`tmcycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmcycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmcycr`] module*/
pub type TMCYCR = crate::Reg<tmcycr::TMCYCR_SPEC>;
///Timer Cycle Setting Registers %s
pub mod tmcycr;
/**TMPLSR (rw) register accessor: Timer Pulse Width Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`tmplsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmplsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmplsr`] module*/
pub type TMPLSR = crate::Reg<tmplsr::TMPLSR_SPEC>;
///Timer Pulse Width Setting Register %s
pub mod tmplsr;
/**TMSTARTR (rw) register accessor: Timer Start Register

You can [`read`](crate::Reg::read) this register and get [`tmstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmstartr`] module*/
pub type TMSTARTR = crate::Reg<tmstartr::TMSTARTR_SPEC>;
///Timer Start Register
pub mod tmstartr;
