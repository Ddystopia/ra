#[repr(C)]
///Register block
pub struct RegisterBlock {
    adcsr: ADCSR,
    _reserved1: [u8; 0x02],
    adansa0: ADANSA0,
    adansa1: ADANSA1,
    adads0: ADADS0,
    adads1: ADADS1,
    adadc: ADADC,
    _reserved6: [u8; 0x01],
    adcer: ADCER,
    adstrgr: ADSTRGR,
    adexicr: ADEXICR,
    adansb0: ADANSB0,
    adansb1: ADANSB1,
    addbldr: ADDBLDR,
    adtsdr: ADTSDR,
    adocdr: ADOCDR,
    adrd: ADRD,
    addr: [ADDR; 4],
    _reserved16: [u8; 0x02],
    addr5: ADDR5,
    addr6: ADDR5,
    addr7: ADDR5,
    _reserved19: [u8; 0x10],
    addr16: ADDR16,
    addr17: ADDR16,
    addr18: ADDR16,
    addr19: ADDR16,
    _reserved23: [u8; 0x1e],
    adshcr: ADSHCR,
    _reserved24: [u8; 0x12],
    addiscr: ADDISCR,
    _reserved25: [u8; 0x01],
    adshmsr: ADSHMSR,
    _reserved26: [u8; 0x03],
    adgspcr: ADGSPCR,
    _reserved27: [u8; 0x02],
    addbldra: ADDBLDRA,
    addbldrb: ADDBLDRB,
    _reserved29: [u8; 0x04],
    adwinmon: ADWINMON,
    _reserved30: [u8; 0x03],
    adcmpcr: ADCMPCR,
    adcmpanser: ADCMPANSER,
    adcmpler: ADCMPLER,
    adcmpansr0: ADCMPANSR0,
    adcmpansr1: ADCMPANSR1,
    adcmplr0: ADCMPLR0,
    adcmplr1: ADCMPLR1,
    adcmpdr0: ADCMPDR0,
    adcmpdr1: ADCMPDR1,
    adcmpsr0: ADCMPSR0,
    adcmpsr1: ADCMPSR1,
    adcmpser: ADCMPSER,
    _reserved42: [u8; 0x01],
    adcmpbnsr: ADCMPBNSR,
    _reserved43: [u8; 0x01],
    adwinllb: ADWINLLB,
    adwinulb: ADWINULB,
    adcmpbsr: ADCMPBSR,
    _reserved46: [u8; 0x30],
    adsstrl: ADSSTRL,
    adsstrt: ADSSTRT,
    adsstro: ADSSTRO,
    adsstr0: [ADSSTR0; 4],
    _reserved50: [u8; 0x01],
    adsstr05: ADSSTR05,
    adsstr06: ADSSTR05,
    adsstr07: ADSSTR05,
    _reserved53: [u8; 0xb8],
    adpgacr: ADPGACR,
    adpgags0: ADPGAGS0,
    _reserved55: [u8; 0x0c],
    adpgadcr0: ADPGADCR0,
}
impl RegisterBlock {
    ///0x00 - A/D Control Register
    #[inline(always)]
    pub const fn adcsr(&self) -> &ADCSR {
        &self.adcsr
    }
    ///0x04 - A/D Channel Select Register A0
    #[inline(always)]
    pub const fn adansa0(&self) -> &ADANSA0 {
        &self.adansa0
    }
    ///0x06 - A/D Channel Select Register A1
    #[inline(always)]
    pub const fn adansa1(&self) -> &ADANSA1 {
        &self.adansa1
    }
    ///0x08 - A/D-Converted Value Addition/Average Channel Select Register 0
    #[inline(always)]
    pub const fn adads0(&self) -> &ADADS0 {
        &self.adads0
    }
    ///0x0a - A/D-Converted Value Addition/Average Channel Select Register 1
    #[inline(always)]
    pub const fn adads1(&self) -> &ADADS1 {
        &self.adads1
    }
    ///0x0c - A/D-Converted Value Addition/Average Count Select Register
    #[inline(always)]
    pub const fn adadc(&self) -> &ADADC {
        &self.adadc
    }
    ///0x0e - A/D Control Extended Register
    #[inline(always)]
    pub const fn adcer(&self) -> &ADCER {
        &self.adcer
    }
    ///0x10 - A/D Conversion Start Trigger Select Register
    #[inline(always)]
    pub const fn adstrgr(&self) -> &ADSTRGR {
        &self.adstrgr
    }
    ///0x12 - A/D Conversion Extended Input Control Register
    #[inline(always)]
    pub const fn adexicr(&self) -> &ADEXICR {
        &self.adexicr
    }
    ///0x14 - A/D Channel Select Register B0
    #[inline(always)]
    pub const fn adansb0(&self) -> &ADANSB0 {
        &self.adansb0
    }
    ///0x16 - A/D Channel Select Register B1
    #[inline(always)]
    pub const fn adansb1(&self) -> &ADANSB1 {
        &self.adansb1
    }
    ///0x18 - A/D Data Duplication Register
    #[inline(always)]
    pub const fn addbldr(&self) -> &ADDBLDR {
        &self.addbldr
    }
    ///0x1a - A/D Temperature Sensor Data Register
    #[inline(always)]
    pub const fn adtsdr(&self) -> &ADTSDR {
        &self.adtsdr
    }
    ///0x1c - A/D Internal Reference Voltage Data Register
    #[inline(always)]
    pub const fn adocdr(&self) -> &ADOCDR {
        &self.adocdr
    }
    ///0x1e - A/D Self-Diagnosis Data Register
    #[inline(always)]
    pub const fn adrd(&self) -> &ADRD {
        &self.adrd
    }
    ///0x20..0x28 - A/D Data Register %s
    #[inline(always)]
    pub const fn addr(&self, n: usize) -> &ADDR {
        &self.addr[n]
    }
    ///Iterator for array of:
    ///0x20..0x28 - A/D Data Register %s
    #[inline(always)]
    pub fn addr_iter(&self) -> impl Iterator<Item = &ADDR> {
        self.addr.iter()
    }
    ///0x2a - A/D Data Register 5
    #[inline(always)]
    pub const fn addr5(&self) -> &ADDR5 {
        &self.addr5
    }
    ///0x2a - A/D Data Register 6
    #[inline(always)]
    pub const fn addr6(&self) -> &ADDR5 {
        &self.addr6
    }
    ///0x2a - A/D Data Register 7
    #[inline(always)]
    pub const fn addr7(&self) -> &ADDR5 {
        &self.addr7
    }
    ///0x40 - A/D Data Register 16
    #[inline(always)]
    pub const fn addr16(&self) -> &ADDR16 {
        &self.addr16
    }
    ///0x40 - A/D Data Register 17
    #[inline(always)]
    pub const fn addr17(&self) -> &ADDR16 {
        &self.addr17
    }
    ///0x40 - A/D Data Register 18
    #[inline(always)]
    pub const fn addr18(&self) -> &ADDR16 {
        &self.addr18
    }
    ///0x40 - A/D Data Register 19
    #[inline(always)]
    pub const fn addr19(&self) -> &ADDR16 {
        &self.addr19
    }
    ///0x66 - A/D Sample and Hold Circuit Control Register
    #[inline(always)]
    pub const fn adshcr(&self) -> &ADSHCR {
        &self.adshcr
    }
    ///0x7a - A/D Disconnection Detection Control Register
    #[inline(always)]
    pub const fn addiscr(&self) -> &ADDISCR {
        &self.addiscr
    }
    ///0x7c - A/D Sample and Hold Operation Mode Select Register
    #[inline(always)]
    pub const fn adshmsr(&self) -> &ADSHMSR {
        &self.adshmsr
    }
    ///0x80 - A/D Group Scan Priority Control Register
    #[inline(always)]
    pub const fn adgspcr(&self) -> &ADGSPCR {
        &self.adgspcr
    }
    ///0x84 - A/D Data Duplication Register A
    #[inline(always)]
    pub const fn addbldra(&self) -> &ADDBLDRA {
        &self.addbldra
    }
    ///0x86 - A/D Data Duplication Register B
    #[inline(always)]
    pub const fn addbldrb(&self) -> &ADDBLDRB {
        &self.addbldrb
    }
    ///0x8c - A/D Compare Function Window A/B Status Monitor Register
    #[inline(always)]
    pub const fn adwinmon(&self) -> &ADWINMON {
        &self.adwinmon
    }
    ///0x90 - A/D Compare Function Control Register
    #[inline(always)]
    pub const fn adcmpcr(&self) -> &ADCMPCR {
        &self.adcmpcr
    }
    ///0x92 - A/D Compare Function Window A Extended Input Select Register
    #[inline(always)]
    pub const fn adcmpanser(&self) -> &ADCMPANSER {
        &self.adcmpanser
    }
    ///0x93 - A/D Compare Function Window A Extended Input Comparison Condition Setting Register
    #[inline(always)]
    pub const fn adcmpler(&self) -> &ADCMPLER {
        &self.adcmpler
    }
    ///0x94 - A/D Compare Function Window A Channel Select Register 0
    #[inline(always)]
    pub const fn adcmpansr0(&self) -> &ADCMPANSR0 {
        &self.adcmpansr0
    }
    ///0x96 - A/D Compare Function Window A Channel Select Register 1
    #[inline(always)]
    pub const fn adcmpansr1(&self) -> &ADCMPANSR1 {
        &self.adcmpansr1
    }
    ///0x98 - A/D Compare Function Window A Comparison Condition Setting Register 0
    #[inline(always)]
    pub const fn adcmplr0(&self) -> &ADCMPLR0 {
        &self.adcmplr0
    }
    ///0x9a - A/D Compare Function Window A Comparison Condition Setting Register 1
    #[inline(always)]
    pub const fn adcmplr1(&self) -> &ADCMPLR1 {
        &self.adcmplr1
    }
    ///0x9c - A/D Compare Function Window A Lower-Side Level Setting Register
    #[inline(always)]
    pub const fn adcmpdr0(&self) -> &ADCMPDR0 {
        &self.adcmpdr0
    }
    ///0x9e - A/D Compare Function Window A Upper-Side Level Setting Register
    #[inline(always)]
    pub const fn adcmpdr1(&self) -> &ADCMPDR1 {
        &self.adcmpdr1
    }
    ///0xa0 - A/D Compare Function Window A Channel Status Register 0
    #[inline(always)]
    pub const fn adcmpsr0(&self) -> &ADCMPSR0 {
        &self.adcmpsr0
    }
    ///0xa2 - A/D Compare Function Window A Channel Status Register 1
    #[inline(always)]
    pub const fn adcmpsr1(&self) -> &ADCMPSR1 {
        &self.adcmpsr1
    }
    ///0xa4 - A/D Compare Function Window A Extended Input Channel Status Register
    #[inline(always)]
    pub const fn adcmpser(&self) -> &ADCMPSER {
        &self.adcmpser
    }
    ///0xa6 - A/D Compare Function Window B Channel Selection Register
    #[inline(always)]
    pub const fn adcmpbnsr(&self) -> &ADCMPBNSR {
        &self.adcmpbnsr
    }
    ///0xa8 - A/D Compare Function Window B Lower-Side Level Setting Register
    #[inline(always)]
    pub const fn adwinllb(&self) -> &ADWINLLB {
        &self.adwinllb
    }
    ///0xaa - A/D Compare Function Window B Upper-Side Level Setting Register
    #[inline(always)]
    pub const fn adwinulb(&self) -> &ADWINULB {
        &self.adwinulb
    }
    ///0xac - A/D Compare Function Window B Status Register
    #[inline(always)]
    pub const fn adcmpbsr(&self) -> &ADCMPBSR {
        &self.adcmpbsr
    }
    ///0xdd - A/D Sampling State Register L
    #[inline(always)]
    pub const fn adsstrl(&self) -> &ADSSTRL {
        &self.adsstrl
    }
    ///0xde - A/D Sampling State Register T
    #[inline(always)]
    pub const fn adsstrt(&self) -> &ADSSTRT {
        &self.adsstrt
    }
    ///0xdf - A/D Sampling State Register O
    #[inline(always)]
    pub const fn adsstro(&self) -> &ADSSTRO {
        &self.adsstro
    }
    ///0xe0 - A/D Sampling State Register %s (Corresponding Channel is AN10%s )
    #[inline(always)]
    pub const fn adsstr0(&self, n: usize) -> &ADSSTR0 {
        &self.adsstr0[n]
    }
    ///Iterator for array of:
    ///0xe0 - A/D Sampling State Register %s (Corresponding Channel is AN10%s )
    #[inline(always)]
    pub fn adsstr0_iter(&self) -> impl Iterator<Item = &ADSSTR0> {
        self.adsstr0.iter()
    }
    ///0xe5 - A/D Sampling State Register 5 (Corresponding Channel is AN105 )
    #[inline(always)]
    pub const fn adsstr05(&self) -> &ADSSTR05 {
        &self.adsstr05
    }
    ///0xe5 - A/D Sampling State Register 6 (Corresponding Channel is AN106 )
    #[inline(always)]
    pub const fn adsstr06(&self) -> &ADSSTR05 {
        &self.adsstr06
    }
    ///0xe5 - A/D Sampling State Register 7 (Corresponding Channel is AN107 )
    #[inline(always)]
    pub const fn adsstr07(&self) -> &ADSSTR05 {
        &self.adsstr07
    }
    ///0x1a0 - A/D Programmable Gain Amplifier Control Register
    #[inline(always)]
    pub const fn adpgacr(&self) -> &ADPGACR {
        &self.adpgacr
    }
    ///0x1a2 - A/D Programmable Gain Amplifier Gain Setting Register 0
    #[inline(always)]
    pub const fn adpgags0(&self) -> &ADPGAGS0 {
        &self.adpgags0
    }
    ///0x1b0 - A/D Programmable Gain Amplifier Differential Input Control Register
    #[inline(always)]
    pub const fn adpgadcr0(&self) -> &ADPGADCR0 {
        &self.adpgadcr0
    }
}
/**ADCSR (rw) register accessor: A/D Control Register

You can [`read`](crate::Reg::read) this register and get [`adcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcsr`] module*/
pub type ADCSR = crate::Reg<adcsr::ADCSR_SPEC>;
///A/D Control Register
pub mod adcsr;
/**ADANSA0 (rw) register accessor: A/D Channel Select Register A0

You can [`read`](crate::Reg::read) this register and get [`adansa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansa0`] module*/
pub type ADANSA0 = crate::Reg<adansa0::ADANSA0_SPEC>;
///A/D Channel Select Register A0
pub mod adansa0;
/**ADANSA1 (rw) register accessor: A/D Channel Select Register A1

You can [`read`](crate::Reg::read) this register and get [`adansa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansa1`] module*/
pub type ADANSA1 = crate::Reg<adansa1::ADANSA1_SPEC>;
///A/D Channel Select Register A1
pub mod adansa1;
/**ADADS0 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adads0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adads0`] module*/
pub type ADADS0 = crate::Reg<adads0::ADADS0_SPEC>;
///A/D-Converted Value Addition/Average Channel Select Register 0
pub mod adads0;
/**ADADS1 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adads1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adads1`] module*/
pub type ADADS1 = crate::Reg<adads1::ADADS1_SPEC>;
///A/D-Converted Value Addition/Average Channel Select Register 1
pub mod adads1;
/**ADADC (rw) register accessor: A/D-Converted Value Addition/Average Count Select Register

You can [`read`](crate::Reg::read) this register and get [`adadc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adadc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adadc`] module*/
pub type ADADC = crate::Reg<adadc::ADADC_SPEC>;
///A/D-Converted Value Addition/Average Count Select Register
pub mod adadc;
/**ADCER (rw) register accessor: A/D Control Extended Register

You can [`read`](crate::Reg::read) this register and get [`adcer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcer`] module*/
pub type ADCER = crate::Reg<adcer::ADCER_SPEC>;
///A/D Control Extended Register
pub mod adcer;
/**ADSTRGR (rw) register accessor: A/D Conversion Start Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`adstrgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adstrgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adstrgr`] module*/
pub type ADSTRGR = crate::Reg<adstrgr::ADSTRGR_SPEC>;
///A/D Conversion Start Trigger Select Register
pub mod adstrgr;
/**ADEXICR (rw) register accessor: A/D Conversion Extended Input Control Register

You can [`read`](crate::Reg::read) this register and get [`adexicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adexicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adexicr`] module*/
pub type ADEXICR = crate::Reg<adexicr::ADEXICR_SPEC>;
///A/D Conversion Extended Input Control Register
pub mod adexicr;
/**ADANSB0 (rw) register accessor: A/D Channel Select Register B0

You can [`read`](crate::Reg::read) this register and get [`adansb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansb0`] module*/
pub type ADANSB0 = crate::Reg<adansb0::ADANSB0_SPEC>;
///A/D Channel Select Register B0
pub mod adansb0;
/**ADANSB1 (rw) register accessor: A/D Channel Select Register B1

You can [`read`](crate::Reg::read) this register and get [`adansb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansb1`] module*/
pub type ADANSB1 = crate::Reg<adansb1::ADANSB1_SPEC>;
///A/D Channel Select Register B1
pub mod adansb1;
/**ADDBLDR (r) register accessor: A/D Data Duplication Register

You can [`read`](crate::Reg::read) this register and get [`addbldr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addbldr`] module*/
pub type ADDBLDR = crate::Reg<addbldr::ADDBLDR_SPEC>;
///A/D Data Duplication Register
pub mod addbldr;
/**ADTSDR (r) register accessor: A/D Temperature Sensor Data Register

You can [`read`](crate::Reg::read) this register and get [`adtsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adtsdr`] module*/
pub type ADTSDR = crate::Reg<adtsdr::ADTSDR_SPEC>;
///A/D Temperature Sensor Data Register
pub mod adtsdr;
/**ADOCDR (r) register accessor: A/D Internal Reference Voltage Data Register

You can [`read`](crate::Reg::read) this register and get [`adocdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adocdr`] module*/
pub type ADOCDR = crate::Reg<adocdr::ADOCDR_SPEC>;
///A/D Internal Reference Voltage Data Register
pub mod adocdr;
/**ADRD (r) register accessor: A/D Self-Diagnosis Data Register

You can [`read`](crate::Reg::read) this register and get [`adrd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adrd`] module*/
pub type ADRD = crate::Reg<adrd::ADRD_SPEC>;
///A/D Self-Diagnosis Data Register
pub mod adrd;
/**ADDR (r) register accessor: A/D Data Register %s

You can [`read`](crate::Reg::read) this register and get [`addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
///A/D Data Register %s
pub mod addr;
pub use addr as addr5;
pub use addr as addr16;
pub use ADDR as ADDR5;
pub use ADDR as ADDR16;
/**ADSHCR (rw) register accessor: A/D Sample and Hold Circuit Control Register

You can [`read`](crate::Reg::read) this register and get [`adshcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adshcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adshcr`] module*/
pub type ADSHCR = crate::Reg<adshcr::ADSHCR_SPEC>;
///A/D Sample and Hold Circuit Control Register
pub mod adshcr;
/**ADDISCR (rw) register accessor: A/D Disconnection Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`addiscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addiscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addiscr`] module*/
pub type ADDISCR = crate::Reg<addiscr::ADDISCR_SPEC>;
///A/D Disconnection Detection Control Register
pub mod addiscr;
/**ADSHMSR (rw) register accessor: A/D Sample and Hold Operation Mode Select Register

You can [`read`](crate::Reg::read) this register and get [`adshmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adshmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adshmsr`] module*/
pub type ADSHMSR = crate::Reg<adshmsr::ADSHMSR_SPEC>;
///A/D Sample and Hold Operation Mode Select Register
pub mod adshmsr;
/**ADGSPCR (rw) register accessor: A/D Group Scan Priority Control Register

You can [`read`](crate::Reg::read) this register and get [`adgspcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgspcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adgspcr`] module*/
pub type ADGSPCR = crate::Reg<adgspcr::ADGSPCR_SPEC>;
///A/D Group Scan Priority Control Register
pub mod adgspcr;
/**ADDBLDRA (r) register accessor: A/D Data Duplication Register A

You can [`read`](crate::Reg::read) this register and get [`addbldra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addbldra`] module*/
pub type ADDBLDRA = crate::Reg<addbldra::ADDBLDRA_SPEC>;
///A/D Data Duplication Register A
pub mod addbldra;
/**ADDBLDRB (r) register accessor: A/D Data Duplication Register B

You can [`read`](crate::Reg::read) this register and get [`addbldrb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addbldrb`] module*/
pub type ADDBLDRB = crate::Reg<addbldrb::ADDBLDRB_SPEC>;
///A/D Data Duplication Register B
pub mod addbldrb;
/**ADWINMON (r) register accessor: A/D Compare Function Window A/B Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`adwinmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adwinmon`] module*/
pub type ADWINMON = crate::Reg<adwinmon::ADWINMON_SPEC>;
///A/D Compare Function Window A/B Status Monitor Register
pub mod adwinmon;
/**ADCMPCR (rw) register accessor: A/D Compare Function Control Register

You can [`read`](crate::Reg::read) this register and get [`adcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpcr`] module*/
pub type ADCMPCR = crate::Reg<adcmpcr::ADCMPCR_SPEC>;
///A/D Compare Function Control Register
pub mod adcmpcr;
/**ADCMPANSER (rw) register accessor: A/D Compare Function Window A Extended Input Select Register

You can [`read`](crate::Reg::read) this register and get [`adcmpanser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpanser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpanser`] module*/
pub type ADCMPANSER = crate::Reg<adcmpanser::ADCMPANSER_SPEC>;
///A/D Compare Function Window A Extended Input Select Register
pub mod adcmpanser;
/**ADCMPLER (rw) register accessor: A/D Compare Function Window A Extended Input Comparison Condition Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpler`] module*/
pub type ADCMPLER = crate::Reg<adcmpler::ADCMPLER_SPEC>;
///A/D Compare Function Window A Extended Input Comparison Condition Setting Register
pub mod adcmpler;
/**ADCMPANSR0 (rw) register accessor: A/D Compare Function Window A Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpansr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpansr0`] module*/
pub type ADCMPANSR0 = crate::Reg<adcmpansr0::ADCMPANSR0_SPEC>;
///A/D Compare Function Window A Channel Select Register 0
pub mod adcmpansr0;
/**ADCMPANSR1 (rw) register accessor: A/D Compare Function Window A Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmpansr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpansr1`] module*/
pub type ADCMPANSR1 = crate::Reg<adcmpansr1::ADCMPANSR1_SPEC>;
///A/D Compare Function Window A Channel Select Register 1
pub mod adcmpansr1;
/**ADCMPLR0 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmplr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmplr0`] module*/
pub type ADCMPLR0 = crate::Reg<adcmplr0::ADCMPLR0_SPEC>;
///A/D Compare Function Window A Comparison Condition Setting Register 0
pub mod adcmplr0;
/**ADCMPLR1 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmplr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmplr1`] module*/
pub type ADCMPLR1 = crate::Reg<adcmplr1::ADCMPLR1_SPEC>;
///A/D Compare Function Window A Comparison Condition Setting Register 1
pub mod adcmplr1;
/**ADCMPDR0 (rw) register accessor: A/D Compare Function Window A Lower-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpdr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpdr0`] module*/
pub type ADCMPDR0 = crate::Reg<adcmpdr0::ADCMPDR0_SPEC>;
///A/D Compare Function Window A Lower-Side Level Setting Register
pub mod adcmpdr0;
/**ADCMPDR1 (rw) register accessor: A/D Compare Function Window A Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpdr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpdr1`] module*/
pub type ADCMPDR1 = crate::Reg<adcmpdr1::ADCMPDR1_SPEC>;
///A/D Compare Function Window A Upper-Side Level Setting Register
pub mod adcmpdr1;
/**ADCMPSR0 (rw) register accessor: A/D Compare Function Window A Channel Status Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpsr0`] module*/
pub type ADCMPSR0 = crate::Reg<adcmpsr0::ADCMPSR0_SPEC>;
///A/D Compare Function Window A Channel Status Register 0
pub mod adcmpsr0;
/**ADCMPSR1 (rw) register accessor: A/D Compare Function Window A Channel Status Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmpsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpsr1`] module*/
pub type ADCMPSR1 = crate::Reg<adcmpsr1::ADCMPSR1_SPEC>;
///A/D Compare Function Window A Channel Status Register 1
pub mod adcmpsr1;
/**ADCMPSER (rw) register accessor: A/D Compare Function Window A Extended Input Channel Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpser`] module*/
pub type ADCMPSER = crate::Reg<adcmpser::ADCMPSER_SPEC>;
///A/D Compare Function Window A Extended Input Channel Status Register
pub mod adcmpser;
/**ADCMPBNSR (rw) register accessor: A/D Compare Function Window B Channel Selection Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbnsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbnsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpbnsr`] module*/
pub type ADCMPBNSR = crate::Reg<adcmpbnsr::ADCMPBNSR_SPEC>;
///A/D Compare Function Window B Channel Selection Register
pub mod adcmpbnsr;
/**ADWINLLB (rw) register accessor: A/D Compare Function Window B Lower-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinllb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinllb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adwinllb`] module*/
pub type ADWINLLB = crate::Reg<adwinllb::ADWINLLB_SPEC>;
///A/D Compare Function Window B Lower-Side Level Setting Register
pub mod adwinllb;
/**ADWINULB (rw) register accessor: A/D Compare Function Window B Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinulb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinulb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adwinulb`] module*/
pub type ADWINULB = crate::Reg<adwinulb::ADWINULB_SPEC>;
///A/D Compare Function Window B Upper-Side Level Setting Register
pub mod adwinulb;
/**ADCMPBSR (rw) register accessor: A/D Compare Function Window B Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpbsr`] module*/
pub type ADCMPBSR = crate::Reg<adcmpbsr::ADCMPBSR_SPEC>;
///A/D Compare Function Window B Status Register
pub mod adcmpbsr;
/**ADSSTRL (rw) register accessor: A/D Sampling State Register L

You can [`read`](crate::Reg::read) this register and get [`adsstrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstrl`] module*/
pub type ADSSTRL = crate::Reg<adsstrl::ADSSTRL_SPEC>;
///A/D Sampling State Register L
pub mod adsstrl;
/**ADSSTRT (rw) register accessor: A/D Sampling State Register T

You can [`read`](crate::Reg::read) this register and get [`adsstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstrt`] module*/
pub type ADSSTRT = crate::Reg<adsstrt::ADSSTRT_SPEC>;
///A/D Sampling State Register T
pub mod adsstrt;
/**ADSSTRO (rw) register accessor: A/D Sampling State Register O

You can [`read`](crate::Reg::read) this register and get [`adsstro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstro`] module*/
pub type ADSSTRO = crate::Reg<adsstro::ADSSTRO_SPEC>;
///A/D Sampling State Register O
pub mod adsstro;
/**ADSSTR0 (rw) register accessor: A/D Sampling State Register %s (Corresponding Channel is AN10%s )

You can [`read`](crate::Reg::read) this register and get [`adsstr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstr0`] module*/
pub type ADSSTR0 = crate::Reg<adsstr0::ADSSTR0_SPEC>;
///A/D Sampling State Register %s (Corresponding Channel is AN10%s )
pub mod adsstr0;
pub use adsstr0 as adsstr05;
pub use ADSSTR0 as ADSSTR05;
/**ADPGACR (rw) register accessor: A/D Programmable Gain Amplifier Control Register

You can [`read`](crate::Reg::read) this register and get [`adpgacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adpgacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adpgacr`] module*/
pub type ADPGACR = crate::Reg<adpgacr::ADPGACR_SPEC>;
///A/D Programmable Gain Amplifier Control Register
pub mod adpgacr;
/**ADPGAGS0 (rw) register accessor: A/D Programmable Gain Amplifier Gain Setting Register 0

You can [`read`](crate::Reg::read) this register and get [`adpgags0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adpgags0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adpgags0`] module*/
pub type ADPGAGS0 = crate::Reg<adpgags0::ADPGAGS0_SPEC>;
///A/D Programmable Gain Amplifier Gain Setting Register 0
pub mod adpgags0;
/**ADPGADCR0 (rw) register accessor: A/D Programmable Gain Amplifier Differential Input Control Register

You can [`read`](crate::Reg::read) this register and get [`adpgadcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adpgadcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adpgadcr0`] module*/
pub type ADPGADCR0 = crate::Reg<adpgadcr0::ADPGADCR0_SPEC>;
///A/D Programmable Gain Amplifier Differential Input Control Register
pub mod adpgadcr0;
