#[repr(C)]
///Register block
pub struct RegisterBlock {
    jcmod: JCMOD,
    jccmd: JCCMD,
    _reserved2: [u8; 0x01],
    jcqtn: JCQTN,
    jchtn: JCHTN,
    jcdriu: JCDRIU,
    jcdrid: JCDRID,
    jcvszu: JCVSZU,
    jcvszd: JCVSZD,
    jchszu: JCHSZU,
    jchszd: JCHSZD,
    jcdtcu: JCDTCU,
    jcdtcm: JCDTCM,
    jcdtcd: JCDTCD,
    jinte0: JINTE0,
    jints0: JINTS0,
    jcderr: JCDERR,
    jcrst: JCRST,
    _reserved17: [u8; 0x2e],
    jifecnt: JIFECNT,
    jifesa: JIFESA,
    jifesofst: JIFESOFST,
    jifeda: JIFEDA,
    jifeslc: JIFESLC,
    _reserved22: [u8; 0x04],
    jifdcnt: JIFDCNT,
    jifdsa: JIFDSA,
    jifddofst: JIFDDOFST,
    jifdda: JIFDDA,
    jifdsdc: JIFDSDC,
    jifddlc: JIFDDLC,
    jifdadt: JIFDADT,
    _reserved29: [u8; 0x18],
    jinte1: JINTE1,
    jints1: JINTS1,
}
impl RegisterBlock {
    ///0x00 - JPEG Code Mode Register
    #[inline(always)]
    pub const fn jcmod(&self) -> &JCMOD {
        &self.jcmod
    }
    ///0x01 - JPEG Code Command Register
    #[inline(always)]
    pub const fn jccmd(&self) -> &JCCMD {
        &self.jccmd
    }
    ///0x03 - JPEG Code Quantization Table Number Register
    #[inline(always)]
    pub const fn jcqtn(&self) -> &JCQTN {
        &self.jcqtn
    }
    ///0x04 - JPEG Code Huffman Table Number Register
    #[inline(always)]
    pub const fn jchtn(&self) -> &JCHTN {
        &self.jchtn
    }
    ///0x05 - JPEG Code DRI Upper Register
    #[inline(always)]
    pub const fn jcdriu(&self) -> &JCDRIU {
        &self.jcdriu
    }
    ///0x06 - JPEG Code DRI Lower Register
    #[inline(always)]
    pub const fn jcdrid(&self) -> &JCDRID {
        &self.jcdrid
    }
    ///0x07 - JPEG Code Vertical Size Upper Register
    #[inline(always)]
    pub const fn jcvszu(&self) -> &JCVSZU {
        &self.jcvszu
    }
    ///0x08 - JPEG Code Vertical Size Lower Register
    #[inline(always)]
    pub const fn jcvszd(&self) -> &JCVSZD {
        &self.jcvszd
    }
    ///0x09 - JPEG Code Horizontal Size Upper Register
    #[inline(always)]
    pub const fn jchszu(&self) -> &JCHSZU {
        &self.jchszu
    }
    ///0x0a - JPEG Coded Horizontal Size Lower Register
    #[inline(always)]
    pub const fn jchszd(&self) -> &JCHSZD {
        &self.jchszd
    }
    ///0x0b - JPEG Code Data Count Upper Register
    #[inline(always)]
    pub const fn jcdtcu(&self) -> &JCDTCU {
        &self.jcdtcu
    }
    ///0x0c - JPEG Code Data Count Middle Register
    #[inline(always)]
    pub const fn jcdtcm(&self) -> &JCDTCM {
        &self.jcdtcm
    }
    ///0x0d - JPEG Code Data Count Lower Register
    #[inline(always)]
    pub const fn jcdtcd(&self) -> &JCDTCD {
        &self.jcdtcd
    }
    ///0x0e - JPEG Interrupt Enable Register 0
    #[inline(always)]
    pub const fn jinte0(&self) -> &JINTE0 {
        &self.jinte0
    }
    ///0x0f - JPEG Interrupt Status Register 0
    #[inline(always)]
    pub const fn jints0(&self) -> &JINTS0 {
        &self.jints0
    }
    ///0x10 - JPEG Code Decode Error Register
    #[inline(always)]
    pub const fn jcderr(&self) -> &JCDERR {
        &self.jcderr
    }
    ///0x11 - JPEG Code Reset Register
    #[inline(always)]
    pub const fn jcrst(&self) -> &JCRST {
        &self.jcrst
    }
    ///0x40 - JPEG Interface Compression Control Register
    #[inline(always)]
    pub const fn jifecnt(&self) -> &JIFECNT {
        &self.jifecnt
    }
    ///0x44 - JPEG Interface Compression Source Address Register
    #[inline(always)]
    pub const fn jifesa(&self) -> &JIFESA {
        &self.jifesa
    }
    ///0x48 - JPEG Interface Compression Line Offset Register
    #[inline(always)]
    pub const fn jifesofst(&self) -> &JIFESOFST {
        &self.jifesofst
    }
    ///0x4c - JPEG Interface Compression Destination Address Register
    #[inline(always)]
    pub const fn jifeda(&self) -> &JIFEDA {
        &self.jifeda
    }
    ///0x50 - JPEG Interface Compression Source Line Count Register
    #[inline(always)]
    pub const fn jifeslc(&self) -> &JIFESLC {
        &self.jifeslc
    }
    ///0x58 - JPEG Interface Decompression Control Register
    #[inline(always)]
    pub const fn jifdcnt(&self) -> &JIFDCNT {
        &self.jifdcnt
    }
    ///0x5c - JPEG Interface Decompression Source Address Register
    #[inline(always)]
    pub const fn jifdsa(&self) -> &JIFDSA {
        &self.jifdsa
    }
    ///0x60 - JPEG Interface Decompression Line Offset Register
    #[inline(always)]
    pub const fn jifddofst(&self) -> &JIFDDOFST {
        &self.jifddofst
    }
    ///0x64 - JPEG Interface Decompression Destination Address Register
    #[inline(always)]
    pub const fn jifdda(&self) -> &JIFDDA {
        &self.jifdda
    }
    ///0x68 - JPEG Interface Decompression Source Data Count Register
    #[inline(always)]
    pub const fn jifdsdc(&self) -> &JIFDSDC {
        &self.jifdsdc
    }
    ///0x6c - JPEG Interface Decompression Destination Line Count Register
    #[inline(always)]
    pub const fn jifddlc(&self) -> &JIFDDLC {
        &self.jifddlc
    }
    ///0x70 - JPEG Interface Decompression alpha Set Register
    #[inline(always)]
    pub const fn jifdadt(&self) -> &JIFDADT {
        &self.jifdadt
    }
    ///0x8c - JPEG Interrupt Enable Register 1
    #[inline(always)]
    pub const fn jinte1(&self) -> &JINTE1 {
        &self.jinte1
    }
    ///0x90 - JPEG Interrupt Status Register 1
    #[inline(always)]
    pub const fn jints1(&self) -> &JINTS1 {
        &self.jints1
    }
}
/**JCMOD (rw) register accessor: JPEG Code Mode Register

You can [`read`](crate::Reg::read) this register and get [`jcmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcmod`] module*/
pub type JCMOD = crate::Reg<jcmod::JCMOD_SPEC>;
///JPEG Code Mode Register
pub mod jcmod;
/**JCCMD (w) register accessor: JPEG Code Command Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jccmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jccmd`] module*/
pub type JCCMD = crate::Reg<jccmd::JCCMD_SPEC>;
///JPEG Code Command Register
pub mod jccmd;
/**JCQTN (rw) register accessor: JPEG Code Quantization Table Number Register

You can [`read`](crate::Reg::read) this register and get [`jcqtn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcqtn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcqtn`] module*/
pub type JCQTN = crate::Reg<jcqtn::JCQTN_SPEC>;
///JPEG Code Quantization Table Number Register
pub mod jcqtn;
/**JCHTN (rw) register accessor: JPEG Code Huffman Table Number Register

You can [`read`](crate::Reg::read) this register and get [`jchtn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchtn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jchtn`] module*/
pub type JCHTN = crate::Reg<jchtn::JCHTN_SPEC>;
///JPEG Code Huffman Table Number Register
pub mod jchtn;
/**JCDRIU (rw) register accessor: JPEG Code DRI Upper Register

You can [`read`](crate::Reg::read) this register and get [`jcdriu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcdriu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcdriu`] module*/
pub type JCDRIU = crate::Reg<jcdriu::JCDRIU_SPEC>;
///JPEG Code DRI Upper Register
pub mod jcdriu;
/**JCDRID (rw) register accessor: JPEG Code DRI Lower Register

You can [`read`](crate::Reg::read) this register and get [`jcdrid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcdrid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcdrid`] module*/
pub type JCDRID = crate::Reg<jcdrid::JCDRID_SPEC>;
///JPEG Code DRI Lower Register
pub mod jcdrid;
/**JCVSZU (rw) register accessor: JPEG Code Vertical Size Upper Register

You can [`read`](crate::Reg::read) this register and get [`jcvszu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcvszu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcvszu`] module*/
pub type JCVSZU = crate::Reg<jcvszu::JCVSZU_SPEC>;
///JPEG Code Vertical Size Upper Register
pub mod jcvszu;
/**JCVSZD (rw) register accessor: JPEG Code Vertical Size Lower Register

You can [`read`](crate::Reg::read) this register and get [`jcvszd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcvszd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcvszd`] module*/
pub type JCVSZD = crate::Reg<jcvszd::JCVSZD_SPEC>;
///JPEG Code Vertical Size Lower Register
pub mod jcvszd;
/**JCHSZU (rw) register accessor: JPEG Code Horizontal Size Upper Register

You can [`read`](crate::Reg::read) this register and get [`jchszu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchszu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jchszu`] module*/
pub type JCHSZU = crate::Reg<jchszu::JCHSZU_SPEC>;
///JPEG Code Horizontal Size Upper Register
pub mod jchszu;
/**JCHSZD (rw) register accessor: JPEG Coded Horizontal Size Lower Register

You can [`read`](crate::Reg::read) this register and get [`jchszd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchszd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jchszd`] module*/
pub type JCHSZD = crate::Reg<jchszd::JCHSZD_SPEC>;
///JPEG Coded Horizontal Size Lower Register
pub mod jchszd;
/**JCDTCU (r) register accessor: JPEG Code Data Count Upper Register

You can [`read`](crate::Reg::read) this register and get [`jcdtcu::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcdtcu`] module*/
pub type JCDTCU = crate::Reg<jcdtcu::JCDTCU_SPEC>;
///JPEG Code Data Count Upper Register
pub mod jcdtcu;
/**JCDTCM (r) register accessor: JPEG Code Data Count Middle Register

You can [`read`](crate::Reg::read) this register and get [`jcdtcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcdtcm`] module*/
pub type JCDTCM = crate::Reg<jcdtcm::JCDTCM_SPEC>;
///JPEG Code Data Count Middle Register
pub mod jcdtcm;
/**JCDTCD (r) register accessor: JPEG Code Data Count Lower Register

You can [`read`](crate::Reg::read) this register and get [`jcdtcd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcdtcd`] module*/
pub type JCDTCD = crate::Reg<jcdtcd::JCDTCD_SPEC>;
///JPEG Code Data Count Lower Register
pub mod jcdtcd;
/**JINTE0 (rw) register accessor: JPEG Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`jinte0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jinte0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jinte0`] module*/
pub type JINTE0 = crate::Reg<jinte0::JINTE0_SPEC>;
///JPEG Interrupt Enable Register 0
pub mod jinte0;
/**JINTS0 (rw) register accessor: JPEG Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`jints0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jints0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jints0`] module*/
pub type JINTS0 = crate::Reg<jints0::JINTS0_SPEC>;
///JPEG Interrupt Status Register 0
pub mod jints0;
/**JCDERR (rw) register accessor: JPEG Code Decode Error Register

You can [`read`](crate::Reg::read) this register and get [`jcderr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcderr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcderr`] module*/
pub type JCDERR = crate::Reg<jcderr::JCDERR_SPEC>;
///JPEG Code Decode Error Register
pub mod jcderr;
/**JCRST (r) register accessor: JPEG Code Reset Register

You can [`read`](crate::Reg::read) this register and get [`jcrst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jcrst`] module*/
pub type JCRST = crate::Reg<jcrst::JCRST_SPEC>;
///JPEG Code Reset Register
pub mod jcrst;
/**JIFECNT (rw) register accessor: JPEG Interface Compression Control Register

You can [`read`](crate::Reg::read) this register and get [`jifecnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifecnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifecnt`] module*/
pub type JIFECNT = crate::Reg<jifecnt::JIFECNT_SPEC>;
///JPEG Interface Compression Control Register
pub mod jifecnt;
/**JIFESA (rw) register accessor: JPEG Interface Compression Source Address Register

You can [`read`](crate::Reg::read) this register and get [`jifesa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifesa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifesa`] module*/
pub type JIFESA = crate::Reg<jifesa::JIFESA_SPEC>;
///JPEG Interface Compression Source Address Register
pub mod jifesa;
/**JIFESOFST (rw) register accessor: JPEG Interface Compression Line Offset Register

You can [`read`](crate::Reg::read) this register and get [`jifesofst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifesofst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifesofst`] module*/
pub type JIFESOFST = crate::Reg<jifesofst::JIFESOFST_SPEC>;
///JPEG Interface Compression Line Offset Register
pub mod jifesofst;
/**JIFEDA (rw) register accessor: JPEG Interface Compression Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`jifeda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifeda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifeda`] module*/
pub type JIFEDA = crate::Reg<jifeda::JIFEDA_SPEC>;
///JPEG Interface Compression Destination Address Register
pub mod jifeda;
/**JIFESLC (rw) register accessor: JPEG Interface Compression Source Line Count Register

You can [`read`](crate::Reg::read) this register and get [`jifeslc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifeslc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifeslc`] module*/
pub type JIFESLC = crate::Reg<jifeslc::JIFESLC_SPEC>;
///JPEG Interface Compression Source Line Count Register
pub mod jifeslc;
/**JIFDCNT (rw) register accessor: JPEG Interface Decompression Control Register

You can [`read`](crate::Reg::read) this register and get [`jifdcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifdcnt`] module*/
pub type JIFDCNT = crate::Reg<jifdcnt::JIFDCNT_SPEC>;
///JPEG Interface Decompression Control Register
pub mod jifdcnt;
/**JIFDSA (rw) register accessor: JPEG Interface Decompression Source Address Register

You can [`read`](crate::Reg::read) this register and get [`jifdsa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdsa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifdsa`] module*/
pub type JIFDSA = crate::Reg<jifdsa::JIFDSA_SPEC>;
///JPEG Interface Decompression Source Address Register
pub mod jifdsa;
/**JIFDDOFST (rw) register accessor: JPEG Interface Decompression Line Offset Register

You can [`read`](crate::Reg::read) this register and get [`jifddofst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifddofst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifddofst`] module*/
pub type JIFDDOFST = crate::Reg<jifddofst::JIFDDOFST_SPEC>;
///JPEG Interface Decompression Line Offset Register
pub mod jifddofst;
/**JIFDDA (rw) register accessor: JPEG Interface Decompression Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`jifdda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifdda`] module*/
pub type JIFDDA = crate::Reg<jifdda::JIFDDA_SPEC>;
///JPEG Interface Decompression Destination Address Register
pub mod jifdda;
/**JIFDSDC (rw) register accessor: JPEG Interface Decompression Source Data Count Register

You can [`read`](crate::Reg::read) this register and get [`jifdsdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdsdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifdsdc`] module*/
pub type JIFDSDC = crate::Reg<jifdsdc::JIFDSDC_SPEC>;
///JPEG Interface Decompression Source Data Count Register
pub mod jifdsdc;
/**JIFDDLC (rw) register accessor: JPEG Interface Decompression Destination Line Count Register

You can [`read`](crate::Reg::read) this register and get [`jifddlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifddlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifddlc`] module*/
pub type JIFDDLC = crate::Reg<jifddlc::JIFDDLC_SPEC>;
///JPEG Interface Decompression Destination Line Count Register
pub mod jifddlc;
/**JIFDADT (rw) register accessor: JPEG Interface Decompression alpha Set Register

You can [`read`](crate::Reg::read) this register and get [`jifdadt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdadt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jifdadt`] module*/
pub type JIFDADT = crate::Reg<jifdadt::JIFDADT_SPEC>;
///JPEG Interface Decompression alpha Set Register
pub mod jifdadt;
/**JINTE1 (rw) register accessor: JPEG Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`jinte1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jinte1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jinte1`] module*/
pub type JINTE1 = crate::Reg<jinte1::JINTE1_SPEC>;
///JPEG Interrupt Enable Register 1
pub mod jinte1;
/**JINTS1 (rw) register accessor: JPEG Interrupt Status Register 1

You can [`read`](crate::Reg::read) this register and get [`jints1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jints1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jints1`] module*/
pub type JINTS1 = crate::Reg<jints1::JINTS1_SPEC>;
///JPEG Interrupt Status Register 1
pub mod jints1;
