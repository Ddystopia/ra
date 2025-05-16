#[repr(C)]
///Register block
pub struct RegisterBlock {
    iccr1: ICCR1,
    iccr2: ICCR2,
    icmr1: ICMR1,
    icmr2: ICMR2,
    icmr3: ICMR3,
    icfer: ICFER,
    icser: ICSER,
    icier: ICIER,
    icsr1: ICSR1,
    icsr2: ICSR2,
    sarl: (),
    _reserved11: [u8; 0x01],
    saru: (),
    _reserved12: [u8; 0x05],
    icbrl: ICBRL,
    icbrh: ICBRH,
    icdrt: ICDRT,
    icdrr: ICDRR,
    _reserved16: [u8; 0x02],
    icwur: ICWUR,
    icwur2: ICWUR2,
}
impl RegisterBlock {
    ///0x00 - I2C Bus Control Register 1
    #[inline(always)]
    pub const fn iccr1(&self) -> &ICCR1 {
        &self.iccr1
    }
    ///0x01 - I2C Bus Control Register 2
    #[inline(always)]
    pub const fn iccr2(&self) -> &ICCR2 {
        &self.iccr2
    }
    ///0x02 - I2C Bus Mode Register 1
    #[inline(always)]
    pub const fn icmr1(&self) -> &ICMR1 {
        &self.icmr1
    }
    ///0x03 - I2C Bus Mode Register 2
    #[inline(always)]
    pub const fn icmr2(&self) -> &ICMR2 {
        &self.icmr2
    }
    ///0x04 - I2C Bus Mode Register 3
    #[inline(always)]
    pub const fn icmr3(&self) -> &ICMR3 {
        &self.icmr3
    }
    ///0x05 - I2C Bus Function Enable Register
    #[inline(always)]
    pub const fn icfer(&self) -> &ICFER {
        &self.icfer
    }
    ///0x06 - I2C Bus Status Enable Register
    #[inline(always)]
    pub const fn icser(&self) -> &ICSER {
        &self.icser
    }
    ///0x07 - I2C Bus Interrupt Enable Register
    #[inline(always)]
    pub const fn icier(&self) -> &ICIER {
        &self.icier
    }
    ///0x08 - I2C Bus Status Register 1
    #[inline(always)]
    pub const fn icsr1(&self) -> &ICSR1 {
        &self.icsr1
    }
    ///0x09 - I2C Bus Status Register 2
    #[inline(always)]
    pub const fn icsr2(&self) -> &ICSR2 {
        &self.icsr2
    }
    ///0x0a - Slave Address Register L%s
    #[inline(always)]
    pub const fn sarl(&self, n: usize) -> &SARL {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(10)
                .add(2 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x0a - Slave Address Register L%s
    #[inline(always)]
    pub fn sarl_iter(&self) -> impl Iterator<Item = &SARL> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(10)
                .add(2 * n)
                .cast()
        })
    }
    ///0x0b - Slave Address Register U%s
    #[inline(always)]
    pub const fn saru(&self, n: usize) -> &SARU {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(11)
                .add(2 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x0b - Slave Address Register U%s
    #[inline(always)]
    pub fn saru_iter(&self) -> impl Iterator<Item = &SARU> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(11)
                .add(2 * n)
                .cast()
        })
    }
    ///0x10 - I2C Bus Bit Rate Low-Level Register
    #[inline(always)]
    pub const fn icbrl(&self) -> &ICBRL {
        &self.icbrl
    }
    ///0x11 - I2C Bus Bit Rate High-Level Register
    #[inline(always)]
    pub const fn icbrh(&self) -> &ICBRH {
        &self.icbrh
    }
    ///0x12 - I2C Bus Transmit Data Register
    #[inline(always)]
    pub const fn icdrt(&self) -> &ICDRT {
        &self.icdrt
    }
    ///0x13 - I2C Bus Receive Data Register
    #[inline(always)]
    pub const fn icdrr(&self) -> &ICDRR {
        &self.icdrr
    }
    ///0x16 - I2C Bus Wake Up Unit Register
    #[inline(always)]
    pub const fn icwur(&self) -> &ICWUR {
        &self.icwur
    }
    ///0x17 - I2C Bus Wake Up Unit Register 2
    #[inline(always)]
    pub const fn icwur2(&self) -> &ICWUR2 {
        &self.icwur2
    }
}
/**ICCR1 (rw) register accessor: I2C Bus Control Register 1

You can [`read`](crate::Reg::read) this register and get [`iccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iccr1`] module*/
pub type ICCR1 = crate::Reg<iccr1::ICCR1_SPEC>;
///I2C Bus Control Register 1
pub mod iccr1;
/**ICCR2 (rw) register accessor: I2C Bus Control Register 2

You can [`read`](crate::Reg::read) this register and get [`iccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iccr2`] module*/
pub type ICCR2 = crate::Reg<iccr2::ICCR2_SPEC>;
///I2C Bus Control Register 2
pub mod iccr2;
/**ICMR1 (rw) register accessor: I2C Bus Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`icmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icmr1`] module*/
pub type ICMR1 = crate::Reg<icmr1::ICMR1_SPEC>;
///I2C Bus Mode Register 1
pub mod icmr1;
/**ICMR2 (rw) register accessor: I2C Bus Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`icmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icmr2`] module*/
pub type ICMR2 = crate::Reg<icmr2::ICMR2_SPEC>;
///I2C Bus Mode Register 2
pub mod icmr2;
/**ICMR3 (rw) register accessor: I2C Bus Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`icmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icmr3`] module*/
pub type ICMR3 = crate::Reg<icmr3::ICMR3_SPEC>;
///I2C Bus Mode Register 3
pub mod icmr3;
/**ICFER (rw) register accessor: I2C Bus Function Enable Register

You can [`read`](crate::Reg::read) this register and get [`icfer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icfer`] module*/
pub type ICFER = crate::Reg<icfer::ICFER_SPEC>;
///I2C Bus Function Enable Register
pub mod icfer;
/**ICSER (rw) register accessor: I2C Bus Status Enable Register

You can [`read`](crate::Reg::read) this register and get [`icser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icser`] module*/
pub type ICSER = crate::Reg<icser::ICSER_SPEC>;
///I2C Bus Status Enable Register
pub mod icser;
/**ICIER (rw) register accessor: I2C Bus Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`icier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icier`] module*/
pub type ICIER = crate::Reg<icier::ICIER_SPEC>;
///I2C Bus Interrupt Enable Register
pub mod icier;
/**ICSR1 (rw) register accessor: I2C Bus Status Register 1

You can [`read`](crate::Reg::read) this register and get [`icsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icsr1`] module*/
pub type ICSR1 = crate::Reg<icsr1::ICSR1_SPEC>;
///I2C Bus Status Register 1
pub mod icsr1;
/**ICSR2 (rw) register accessor: I2C Bus Status Register 2

You can [`read`](crate::Reg::read) this register and get [`icsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icsr2`] module*/
pub type ICSR2 = crate::Reg<icsr2::ICSR2_SPEC>;
///I2C Bus Status Register 2
pub mod icsr2;
/**SARL (rw) register accessor: Slave Address Register L%s

You can [`read`](crate::Reg::read) this register and get [`sarl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sarl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sarl`] module*/
pub type SARL = crate::Reg<sarl::SARL_SPEC>;
///Slave Address Register L%s
pub mod sarl;
/**SARU (rw) register accessor: Slave Address Register U%s

You can [`read`](crate::Reg::read) this register and get [`saru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@saru`] module*/
pub type SARU = crate::Reg<saru::SARU_SPEC>;
///Slave Address Register U%s
pub mod saru;
/**ICBRL (rw) register accessor: I2C Bus Bit Rate Low-Level Register

You can [`read`](crate::Reg::read) this register and get [`icbrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icbrl`] module*/
pub type ICBRL = crate::Reg<icbrl::ICBRL_SPEC>;
///I2C Bus Bit Rate Low-Level Register
pub mod icbrl;
/**ICBRH (rw) register accessor: I2C Bus Bit Rate High-Level Register

You can [`read`](crate::Reg::read) this register and get [`icbrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icbrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icbrh`] module*/
pub type ICBRH = crate::Reg<icbrh::ICBRH_SPEC>;
///I2C Bus Bit Rate High-Level Register
pub mod icbrh;
/**ICDRT (rw) register accessor: I2C Bus Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`icdrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icdrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icdrt`] module*/
pub type ICDRT = crate::Reg<icdrt::ICDRT_SPEC>;
///I2C Bus Transmit Data Register
pub mod icdrt;
/**ICDRR (r) register accessor: I2C Bus Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`icdrr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icdrr`] module*/
pub type ICDRR = crate::Reg<icdrr::ICDRR_SPEC>;
///I2C Bus Receive Data Register
pub mod icdrr;
/**ICWUR (rw) register accessor: I2C Bus Wake Up Unit Register

You can [`read`](crate::Reg::read) this register and get [`icwur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icwur`] module*/
pub type ICWUR = crate::Reg<icwur::ICWUR_SPEC>;
///I2C Bus Wake Up Unit Register
pub mod icwur;
/**ICWUR2 (r) register accessor: I2C Bus Wake Up Unit Register 2

You can [`read`](crate::Reg::read) this register and get [`icwur2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icwur2`] module*/
pub type ICWUR2 = crate::Reg<icwur2::ICWUR2_SPEC>;
///I2C Bus Wake Up Unit Register 2
pub mod icwur2;
