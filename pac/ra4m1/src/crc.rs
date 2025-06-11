#[repr(C)]
///Register block
pub struct RegisterBlock {
    crccr0: CRCCR0,
    crccr1: CRCCR1,
    _reserved2: [u8; 0x02],
    _reserved_2_crcdir: [u8; 0x04],
    _reserved_3_crcdor: [u8; 0x04],
    crcsar: CRCSAR,
}
impl RegisterBlock {
    ///0x00 - CRC Control Register0
    #[inline(always)]
    pub const fn crccr0(&self) -> &CRCCR0 {
        &self.crccr0
    }
    ///0x01 - CRC Control Register1
    #[inline(always)]
    pub const fn crccr1(&self) -> &CRCCR1 {
        &self.crccr1
    }
    ///0x04 - CRC Data Input Register (byte access)
    #[inline(always)]
    pub const fn crcdir_by(&self) -> &CRCDIR_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - CRC Data Input Register
    #[inline(always)]
    pub const fn crcdir(&self) -> &CRCDIR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - CRC Data Output Register(byte access)
    #[inline(always)]
    pub const fn crcdor_by(&self) -> &CRCDOR_BY {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - CRC Data Output Register (halfword access)
    #[inline(always)]
    pub const fn crcdor_ha(&self) -> &CRCDOR_HA {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - CRC Data Output Register
    #[inline(always)]
    pub const fn crcdor(&self) -> &CRCDOR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0c - Snoop Address Register
    #[inline(always)]
    pub const fn crcsar(&self) -> &CRCSAR {
        &self.crcsar
    }
}
/**CRCCR0 (rw) register accessor: CRC Control Register0

You can [`read`](crate::Reg::read) this register and get [`crccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crccr0`] module*/
pub type CRCCR0 = crate::Reg<crccr0::CRCCR0_SPEC>;
///CRC Control Register0
pub mod crccr0;
/**CRCCR1 (rw) register accessor: CRC Control Register1

You can [`read`](crate::Reg::read) this register and get [`crccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crccr1`] module*/
pub type CRCCR1 = crate::Reg<crccr1::CRCCR1_SPEC>;
///CRC Control Register1
pub mod crccr1;
/**CRCDIR (rw) register accessor: CRC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`crcdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdir`] module*/
pub type CRCDIR = crate::Reg<crcdir::CRCDIR_SPEC>;
///CRC Data Input Register
pub mod crcdir;
/**CRCDIR_BY (rw) register accessor: CRC Data Input Register (byte access)

You can [`read`](crate::Reg::read) this register and get [`crcdir_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdir_by`] module*/
pub type CRCDIR_BY = crate::Reg<crcdir_by::CRCDIR_BY_SPEC>;
///CRC Data Input Register (byte access)
pub mod crcdir_by;
/**CRCDOR (rw) register accessor: CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdor`] module*/
pub type CRCDOR = crate::Reg<crcdor::CRCDOR_SPEC>;
///CRC Data Output Register
pub mod crcdor;
/**CRCDOR_HA (rw) register accessor: CRC Data Output Register (halfword access)

You can [`read`](crate::Reg::read) this register and get [`crcdor_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdor_ha`] module*/
pub type CRCDOR_HA = crate::Reg<crcdor_ha::CRCDOR_HA_SPEC>;
///CRC Data Output Register (halfword access)
pub mod crcdor_ha;
/**CRCDOR_BY (rw) register accessor: CRC Data Output Register(byte access)

You can [`read`](crate::Reg::read) this register and get [`crcdor_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdor_by`] module*/
pub type CRCDOR_BY = crate::Reg<crcdor_by::CRCDOR_BY_SPEC>;
///CRC Data Output Register(byte access)
pub mod crcdor_by;
/**CRCSAR (rw) register accessor: Snoop Address Register

You can [`read`](crate::Reg::read) this register and get [`crcsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcsar`] module*/
pub type CRCSAR = crate::Reg<crcsar::CRCSAR_SPEC>;
///Snoop Address Register
pub mod crcsar;
