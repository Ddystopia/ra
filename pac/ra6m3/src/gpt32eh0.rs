#[repr(C)]
///Register block
pub struct RegisterBlock {
    gtwp: GTWP,
    gtstr: GTSTR,
    gtstp: GTSTP,
    gtclr: GTCLR,
    gtssr: GTSSR,
    gtpsr: GTPSR,
    gtcsr: GTCSR,
    gtupsr: GTUPSR,
    gtdnsr: GTDNSR,
    gticasr: GTICASR,
    gticbsr: GTICBSR,
    gtcr: GTCR,
    gtuddtyc: GTUDDTYC,
    gtior: GTIOR,
    gtintad: GTINTAD,
    gtst: GTST,
    gtber: GTBER,
    gtitc: GTITC,
    gtcnt: GTCNT,
    gtccra: GTCCRA,
    gtccrb: GTCCRB,
    gtccrc: GTCCRC,
    gtccre: GTCCRE,
    gtccrd: GTCCRD,
    gtccrf: GTCCRF,
    gtpr: GTPR,
    gtpbr: GTPBR,
    gtpdbr: GTPDBR,
    gtadtra: GTADTRA,
    gtadtbra: GTADTBRA,
    gtadtdbra: GTADTDBRA,
    gtadtrb: GTADTRB,
    gtadtbrb: GTADTBRB,
    gtadtdbrb: GTADTDBRB,
    gtdtcr: GTDTCR,
    gtdvu: GTDVU,
    gtdvd: GTDVD,
    gtdbu: GTDBU,
    gtdbd: GTDBD,
    gtsos: GTSOS,
    gtsotr: GTSOTR,
}
impl RegisterBlock {
    ///0x00 - General PWM Timer Write-Protection Register
    #[inline(always)]
    pub const fn gtwp(&self) -> &GTWP {
        &self.gtwp
    }
    ///0x04 - General PWM Timer Software Start Register
    #[inline(always)]
    pub const fn gtstr(&self) -> &GTSTR {
        &self.gtstr
    }
    ///0x08 - General PWM Timer Software Stop Register
    #[inline(always)]
    pub const fn gtstp(&self) -> &GTSTP {
        &self.gtstp
    }
    ///0x0c - General PWM Timer Software Clear Register
    #[inline(always)]
    pub const fn gtclr(&self) -> &GTCLR {
        &self.gtclr
    }
    ///0x10 - General PWM Timer Start Source Select Register
    #[inline(always)]
    pub const fn gtssr(&self) -> &GTSSR {
        &self.gtssr
    }
    ///0x14 - General PWM Timer Stop Source Select Register
    #[inline(always)]
    pub const fn gtpsr(&self) -> &GTPSR {
        &self.gtpsr
    }
    ///0x18 - General PWM Timer Clear Source Select Register
    #[inline(always)]
    pub const fn gtcsr(&self) -> &GTCSR {
        &self.gtcsr
    }
    ///0x1c - General PWM Timer Up Count Source Select Register
    #[inline(always)]
    pub const fn gtupsr(&self) -> &GTUPSR {
        &self.gtupsr
    }
    ///0x20 - General PWM Timer Down Count Source Select Register
    #[inline(always)]
    pub const fn gtdnsr(&self) -> &GTDNSR {
        &self.gtdnsr
    }
    ///0x24 - General PWM Timer Input Capture Source Select Register A
    #[inline(always)]
    pub const fn gticasr(&self) -> &GTICASR {
        &self.gticasr
    }
    ///0x28 - General PWM Timer Input Capture Source Select Register B
    #[inline(always)]
    pub const fn gticbsr(&self) -> &GTICBSR {
        &self.gticbsr
    }
    ///0x2c - General PWM Timer Control Register
    #[inline(always)]
    pub const fn gtcr(&self) -> &GTCR {
        &self.gtcr
    }
    ///0x30 - General PWM Timer Count Direction and Duty Setting Register
    #[inline(always)]
    pub const fn gtuddtyc(&self) -> &GTUDDTYC {
        &self.gtuddtyc
    }
    ///0x34 - General PWM Timer I/O Control Register
    #[inline(always)]
    pub const fn gtior(&self) -> &GTIOR {
        &self.gtior
    }
    ///0x38 - General PWM Timer Interrupt Output Setting Register
    #[inline(always)]
    pub const fn gtintad(&self) -> &GTINTAD {
        &self.gtintad
    }
    ///0x3c - General PWM Timer Status Register
    #[inline(always)]
    pub const fn gtst(&self) -> &GTST {
        &self.gtst
    }
    ///0x40 - General PWM Timer Buffer Enable Register
    #[inline(always)]
    pub const fn gtber(&self) -> &GTBER {
        &self.gtber
    }
    ///0x44 - General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register
    #[inline(always)]
    pub const fn gtitc(&self) -> &GTITC {
        &self.gtitc
    }
    ///0x48 - General PWM Timer Counter
    #[inline(always)]
    pub const fn gtcnt(&self) -> &GTCNT {
        &self.gtcnt
    }
    ///0x4c - General PWM Timer Compare Capture Register A
    #[inline(always)]
    pub const fn gtccra(&self) -> &GTCCRA {
        &self.gtccra
    }
    ///0x50 - General PWM Timer Compare Capture Register B
    #[inline(always)]
    pub const fn gtccrb(&self) -> &GTCCRB {
        &self.gtccrb
    }
    ///0x54 - General PWM Timer Compare Capture Register C
    #[inline(always)]
    pub const fn gtccrc(&self) -> &GTCCRC {
        &self.gtccrc
    }
    ///0x58 - General PWM Timer Compare Capture Register E
    #[inline(always)]
    pub const fn gtccre(&self) -> &GTCCRE {
        &self.gtccre
    }
    ///0x5c - General PWM Timer Compare Capture Register D
    #[inline(always)]
    pub const fn gtccrd(&self) -> &GTCCRD {
        &self.gtccrd
    }
    ///0x60 - General PWM Timer Compare Capture Register F
    #[inline(always)]
    pub const fn gtccrf(&self) -> &GTCCRF {
        &self.gtccrf
    }
    ///0x64 - General PWM Timer Cycle Setting Register
    #[inline(always)]
    pub const fn gtpr(&self) -> &GTPR {
        &self.gtpr
    }
    ///0x68 - General PWM Timer Cycle Setting Buffer Register
    #[inline(always)]
    pub const fn gtpbr(&self) -> &GTPBR {
        &self.gtpbr
    }
    ///0x6c - General PWM Timer Cycle Setting Double-Buffer Register
    #[inline(always)]
    pub const fn gtpdbr(&self) -> &GTPDBR {
        &self.gtpdbr
    }
    ///0x70 - A/D Converter Start Request Timing Register A
    #[inline(always)]
    pub const fn gtadtra(&self) -> &GTADTRA {
        &self.gtadtra
    }
    ///0x74 - A/D Converter Start Request Timing Buffer Register A
    #[inline(always)]
    pub const fn gtadtbra(&self) -> &GTADTBRA {
        &self.gtadtbra
    }
    ///0x78 - A/D Converter Start Request Timing Double-Buffer Register A
    #[inline(always)]
    pub const fn gtadtdbra(&self) -> &GTADTDBRA {
        &self.gtadtdbra
    }
    ///0x7c - A/D Converter Start Request Timing Register B
    #[inline(always)]
    pub const fn gtadtrb(&self) -> &GTADTRB {
        &self.gtadtrb
    }
    ///0x80 - A/D Converter Start Request Timing Buffer Register B
    #[inline(always)]
    pub const fn gtadtbrb(&self) -> &GTADTBRB {
        &self.gtadtbrb
    }
    ///0x84 - A/D Converter Start Request Timing Double-Buffer Register B
    #[inline(always)]
    pub const fn gtadtdbrb(&self) -> &GTADTDBRB {
        &self.gtadtdbrb
    }
    ///0x88 - General PWM Timer Dead Time Control Register
    #[inline(always)]
    pub const fn gtdtcr(&self) -> &GTDTCR {
        &self.gtdtcr
    }
    ///0x8c - General PWM Timer Dead Time Value Register U
    #[inline(always)]
    pub const fn gtdvu(&self) -> &GTDVU {
        &self.gtdvu
    }
    ///0x90 - General PWM Timer Dead Time Value Register D
    #[inline(always)]
    pub const fn gtdvd(&self) -> &GTDVD {
        &self.gtdvd
    }
    ///0x94 - General PWM Timer Dead Time Buffer Register U
    #[inline(always)]
    pub const fn gtdbu(&self) -> &GTDBU {
        &self.gtdbu
    }
    ///0x98 - General PWM Timer Dead Time Buffer Register D
    #[inline(always)]
    pub const fn gtdbd(&self) -> &GTDBD {
        &self.gtdbd
    }
    ///0x9c - General PWM Timer Output Protection Function Status Register
    #[inline(always)]
    pub const fn gtsos(&self) -> &GTSOS {
        &self.gtsos
    }
    ///0xa0 - General PWM Timer Output Protection Function Temporary Release Register
    #[inline(always)]
    pub const fn gtsotr(&self) -> &GTSOTR {
        &self.gtsotr
    }
}
/**GTWP (rw) register accessor: General PWM Timer Write-Protection Register

You can [`read`](crate::Reg::read) this register and get [`gtwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtwp`] module*/
pub type GTWP = crate::Reg<gtwp::GTWP_SPEC>;
///General PWM Timer Write-Protection Register
pub mod gtwp;
/**GTSTR (rw) register accessor: General PWM Timer Software Start Register

You can [`read`](crate::Reg::read) this register and get [`gtstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtstr`] module*/
pub type GTSTR = crate::Reg<gtstr::GTSTR_SPEC>;
///General PWM Timer Software Start Register
pub mod gtstr;
/**GTSTP (rw) register accessor: General PWM Timer Software Stop Register

You can [`read`](crate::Reg::read) this register and get [`gtstp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtstp`] module*/
pub type GTSTP = crate::Reg<gtstp::GTSTP_SPEC>;
///General PWM Timer Software Stop Register
pub mod gtstp;
/**GTCLR (w) register accessor: General PWM Timer Software Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtclr`] module*/
pub type GTCLR = crate::Reg<gtclr::GTCLR_SPEC>;
///General PWM Timer Software Clear Register
pub mod gtclr;
/**GTSSR (rw) register accessor: General PWM Timer Start Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtssr`] module*/
pub type GTSSR = crate::Reg<gtssr::GTSSR_SPEC>;
///General PWM Timer Start Source Select Register
pub mod gtssr;
/**GTPSR (rw) register accessor: General PWM Timer Stop Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpsr`] module*/
pub type GTPSR = crate::Reg<gtpsr::GTPSR_SPEC>;
///General PWM Timer Stop Source Select Register
pub mod gtpsr;
/**GTCSR (rw) register accessor: General PWM Timer Clear Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtcsr`] module*/
pub type GTCSR = crate::Reg<gtcsr::GTCSR_SPEC>;
///General PWM Timer Clear Source Select Register
pub mod gtcsr;
/**GTUPSR (rw) register accessor: General PWM Timer Up Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtupsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtupsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtupsr`] module*/
pub type GTUPSR = crate::Reg<gtupsr::GTUPSR_SPEC>;
///General PWM Timer Up Count Source Select Register
pub mod gtupsr;
/**GTDNSR (rw) register accessor: General PWM Timer Down Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtdnsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdnsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdnsr`] module*/
pub type GTDNSR = crate::Reg<gtdnsr::GTDNSR_SPEC>;
///General PWM Timer Down Count Source Select Register
pub mod gtdnsr;
/**GTICASR (rw) register accessor: General PWM Timer Input Capture Source Select Register A

You can [`read`](crate::Reg::read) this register and get [`gticasr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticasr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gticasr`] module*/
pub type GTICASR = crate::Reg<gticasr::GTICASR_SPEC>;
///General PWM Timer Input Capture Source Select Register A
pub mod gticasr;
/**GTICBSR (rw) register accessor: General PWM Timer Input Capture Source Select Register B

You can [`read`](crate::Reg::read) this register and get [`gticbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gticbsr`] module*/
pub type GTICBSR = crate::Reg<gticbsr::GTICBSR_SPEC>;
///General PWM Timer Input Capture Source Select Register B
pub mod gticbsr;
/**GTCR (rw) register accessor: General PWM Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`gtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtcr`] module*/
pub type GTCR = crate::Reg<gtcr::GTCR_SPEC>;
///General PWM Timer Control Register
pub mod gtcr;
/**GTUDDTYC (rw) register accessor: General PWM Timer Count Direction and Duty Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtuddtyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtuddtyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtuddtyc`] module*/
pub type GTUDDTYC = crate::Reg<gtuddtyc::GTUDDTYC_SPEC>;
///General PWM Timer Count Direction and Duty Setting Register
pub mod gtuddtyc;
/**GTIOR (rw) register accessor: General PWM Timer I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`gtior::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtior::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtior`] module*/
pub type GTIOR = crate::Reg<gtior::GTIOR_SPEC>;
///General PWM Timer I/O Control Register
pub mod gtior;
/**GTINTAD (rw) register accessor: General PWM Timer Interrupt Output Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtintad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtintad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtintad`] module*/
pub type GTINTAD = crate::Reg<gtintad::GTINTAD_SPEC>;
///General PWM Timer Interrupt Output Setting Register
pub mod gtintad;
/**GTST (rw) register accessor: General PWM Timer Status Register

You can [`read`](crate::Reg::read) this register and get [`gtst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtst`] module*/
pub type GTST = crate::Reg<gtst::GTST_SPEC>;
///General PWM Timer Status Register
pub mod gtst;
/**GTBER (rw) register accessor: General PWM Timer Buffer Enable Register

You can [`read`](crate::Reg::read) this register and get [`gtber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtber`] module*/
pub type GTBER = crate::Reg<gtber::GTBER_SPEC>;
///General PWM Timer Buffer Enable Register
pub mod gtber;
/**GTITC (rw) register accessor: General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtitc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtitc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtitc`] module*/
pub type GTITC = crate::Reg<gtitc::GTITC_SPEC>;
///General PWM Timer Interrupt and A/D Converter Start Request Skipping Setting Register
pub mod gtitc;
/**GTCNT (rw) register accessor: General PWM Timer Counter

You can [`read`](crate::Reg::read) this register and get [`gtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtcnt`] module*/
pub type GTCNT = crate::Reg<gtcnt::GTCNT_SPEC>;
///General PWM Timer Counter
pub mod gtcnt;
/**GTCCRA (rw) register accessor: General PWM Timer Compare Capture Register A

You can [`read`](crate::Reg::read) this register and get [`gtccra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccra`] module*/
pub type GTCCRA = crate::Reg<gtccra::GTCCRA_SPEC>;
///General PWM Timer Compare Capture Register A
pub mod gtccra;
/**GTCCRB (rw) register accessor: General PWM Timer Compare Capture Register B

You can [`read`](crate::Reg::read) this register and get [`gtccrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrb`] module*/
pub type GTCCRB = crate::Reg<gtccrb::GTCCRB_SPEC>;
///General PWM Timer Compare Capture Register B
pub mod gtccrb;
/**GTCCRC (rw) register accessor: General PWM Timer Compare Capture Register C

You can [`read`](crate::Reg::read) this register and get [`gtccrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrc`] module*/
pub type GTCCRC = crate::Reg<gtccrc::GTCCRC_SPEC>;
///General PWM Timer Compare Capture Register C
pub mod gtccrc;
/**GTCCRE (rw) register accessor: General PWM Timer Compare Capture Register E

You can [`read`](crate::Reg::read) this register and get [`gtccre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccre`] module*/
pub type GTCCRE = crate::Reg<gtccre::GTCCRE_SPEC>;
///General PWM Timer Compare Capture Register E
pub mod gtccre;
/**GTCCRD (rw) register accessor: General PWM Timer Compare Capture Register D

You can [`read`](crate::Reg::read) this register and get [`gtccrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrd`] module*/
pub type GTCCRD = crate::Reg<gtccrd::GTCCRD_SPEC>;
///General PWM Timer Compare Capture Register D
pub mod gtccrd;
/**GTCCRF (rw) register accessor: General PWM Timer Compare Capture Register F

You can [`read`](crate::Reg::read) this register and get [`gtccrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrf`] module*/
pub type GTCCRF = crate::Reg<gtccrf::GTCCRF_SPEC>;
///General PWM Timer Compare Capture Register F
pub mod gtccrf;
/**GTPR (rw) register accessor: General PWM Timer Cycle Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpr`] module*/
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
///General PWM Timer Cycle Setting Register
pub mod gtpr;
/**GTPBR (rw) register accessor: General PWM Timer Cycle Setting Buffer Register

You can [`read`](crate::Reg::read) this register and get [`gtpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpbr`] module*/
pub type GTPBR = crate::Reg<gtpbr::GTPBR_SPEC>;
///General PWM Timer Cycle Setting Buffer Register
pub mod gtpbr;
/**GTPDBR (rw) register accessor: General PWM Timer Cycle Setting Double-Buffer Register

You can [`read`](crate::Reg::read) this register and get [`gtpdbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpdbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpdbr`] module*/
pub type GTPDBR = crate::Reg<gtpdbr::GTPDBR_SPEC>;
///General PWM Timer Cycle Setting Double-Buffer Register
pub mod gtpdbr;
/**GTADTRA (rw) register accessor: A/D Converter Start Request Timing Register A

You can [`read`](crate::Reg::read) this register and get [`gtadtra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtadtra`] module*/
pub type GTADTRA = crate::Reg<gtadtra::GTADTRA_SPEC>;
///A/D Converter Start Request Timing Register A
pub mod gtadtra;
/**GTADTRB (rw) register accessor: A/D Converter Start Request Timing Register B

You can [`read`](crate::Reg::read) this register and get [`gtadtrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtadtrb`] module*/
pub type GTADTRB = crate::Reg<gtadtrb::GTADTRB_SPEC>;
///A/D Converter Start Request Timing Register B
pub mod gtadtrb;
/**GTADTBRA (rw) register accessor: A/D Converter Start Request Timing Buffer Register A

You can [`read`](crate::Reg::read) this register and get [`gtadtbra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtbra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtadtbra`] module*/
pub type GTADTBRA = crate::Reg<gtadtbra::GTADTBRA_SPEC>;
///A/D Converter Start Request Timing Buffer Register A
pub mod gtadtbra;
/**GTADTBRB (rw) register accessor: A/D Converter Start Request Timing Buffer Register B

You can [`read`](crate::Reg::read) this register and get [`gtadtbrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtbrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtadtbrb`] module*/
pub type GTADTBRB = crate::Reg<gtadtbrb::GTADTBRB_SPEC>;
///A/D Converter Start Request Timing Buffer Register B
pub mod gtadtbrb;
/**GTADTDBRA (rw) register accessor: A/D Converter Start Request Timing Double-Buffer Register A

You can [`read`](crate::Reg::read) this register and get [`gtadtdbra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtdbra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtadtdbra`] module*/
pub type GTADTDBRA = crate::Reg<gtadtdbra::GTADTDBRA_SPEC>;
///A/D Converter Start Request Timing Double-Buffer Register A
pub mod gtadtdbra;
/**GTADTDBRB (rw) register accessor: A/D Converter Start Request Timing Double-Buffer Register B

You can [`read`](crate::Reg::read) this register and get [`gtadtdbrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtadtdbrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtadtdbrb`] module*/
pub type GTADTDBRB = crate::Reg<gtadtdbrb::GTADTDBRB_SPEC>;
///A/D Converter Start Request Timing Double-Buffer Register B
pub mod gtadtdbrb;
/**GTDTCR (rw) register accessor: General PWM Timer Dead Time Control Register

You can [`read`](crate::Reg::read) this register and get [`gtdtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdtcr`] module*/
pub type GTDTCR = crate::Reg<gtdtcr::GTDTCR_SPEC>;
///General PWM Timer Dead Time Control Register
pub mod gtdtcr;
/**GTDVU (rw) register accessor: General PWM Timer Dead Time Value Register U

You can [`read`](crate::Reg::read) this register and get [`gtdvu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdvu`] module*/
pub type GTDVU = crate::Reg<gtdvu::GTDVU_SPEC>;
///General PWM Timer Dead Time Value Register U
pub mod gtdvu;
/**GTDVD (rw) register accessor: General PWM Timer Dead Time Value Register D

You can [`read`](crate::Reg::read) this register and get [`gtdvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdvd`] module*/
pub type GTDVD = crate::Reg<gtdvd::GTDVD_SPEC>;
///General PWM Timer Dead Time Value Register D
pub mod gtdvd;
/**GTDBU (rw) register accessor: General PWM Timer Dead Time Buffer Register U

You can [`read`](crate::Reg::read) this register and get [`gtdbu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdbu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdbu`] module*/
pub type GTDBU = crate::Reg<gtdbu::GTDBU_SPEC>;
///General PWM Timer Dead Time Buffer Register U
pub mod gtdbu;
/**GTDBD (rw) register accessor: General PWM Timer Dead Time Buffer Register D

You can [`read`](crate::Reg::read) this register and get [`gtdbd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdbd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdbd`] module*/
pub type GTDBD = crate::Reg<gtdbd::GTDBD_SPEC>;
///General PWM Timer Dead Time Buffer Register D
pub mod gtdbd;
/**GTSOS (r) register accessor: General PWM Timer Output Protection Function Status Register

You can [`read`](crate::Reg::read) this register and get [`gtsos::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtsos`] module*/
pub type GTSOS = crate::Reg<gtsos::GTSOS_SPEC>;
///General PWM Timer Output Protection Function Status Register
pub mod gtsos;
/**GTSOTR (rw) register accessor: General PWM Timer Output Protection Function Temporary Release Register

You can [`read`](crate::Reg::read) this register and get [`gtsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtsotr`] module*/
pub type GTSOTR = crate::Reg<gtsotr::GTSOTR_SPEC>;
///General PWM Timer Output Protection Function Temporary Release Register
pub mod gtsotr;
