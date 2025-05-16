#[repr(C)]
///Register block
pub struct RegisterBlock {
    r64cnt: R64CNT,
    _reserved1: [u8; 0x01],
    _reserved_1_bcnt0: [u8; 0x01],
    _reserved2: [u8; 0x01],
    _reserved_2_bcnt1: [u8; 0x01],
    _reserved3: [u8; 0x01],
    _reserved_3_bcnt2: [u8; 0x01],
    _reserved4: [u8; 0x01],
    _reserved_4_bcnt3: [u8; 0x01],
    _reserved5: [u8; 0x01],
    rdaycnt: RDAYCNT,
    _reserved6: [u8; 0x01],
    rmoncnt: RMONCNT,
    _reserved7: [u8; 0x01],
    ryrcnt: RYRCNT,
    _reserved_8_rsecar: [u8; 0x01],
    _reserved9: [u8; 0x01],
    _reserved_9_rminar: [u8; 0x01],
    _reserved10: [u8; 0x01],
    _reserved_10_rhrar: [u8; 0x01],
    _reserved11: [u8; 0x01],
    _reserved_11_rwkar: [u8; 0x01],
    _reserved12: [u8; 0x01],
    _reserved_12_rdayar: [u8; 0x01],
    _reserved13: [u8; 0x01],
    _reserved_13_rmonar: [u8; 0x01],
    _reserved14: [u8; 0x01],
    _reserved_14_ryrar: [u8; 0x02],
    _reserved_15_ryraren: [u8; 0x01],
    _reserved16: [u8; 0x03],
    rcr1: RCR1,
    _reserved17: [u8; 0x01],
    rcr2: RCR2,
    _reserved18: [u8; 0x03],
    rcr4: RCR4,
    _reserved19: [u8; 0x01],
    rfrh: RFRH,
    rfrl: RFRL,
    radj: RADJ,
    _reserved22: [u8; 0x11],
    rtccr: (),
    _reserved23: [u8; 0x12],
    bcnt0cp: (),
    rseccp: (),
    _reserved25: [u8; 0x02],
    bcnt1cp: (),
    rmincp: (),
    _reserved27: [u8; 0x02],
    bcnt2cp: (),
    rhrcp: (),
    _reserved29: [u8; 0x04],
    bcnt3cp: (),
    rdaycp: (),
    _reserved31: [u8; 0x02],
    rmoncp: (),
}
impl RegisterBlock {
    ///0x00 - 64-Hz Counter
    #[inline(always)]
    pub const fn r64cnt(&self) -> &R64CNT {
        &self.r64cnt
    }
    ///0x02 - Binary Counter 0
    #[inline(always)]
    pub const fn bcnt0(&self) -> &BCNT0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x02 - Second Counter
    #[inline(always)]
    pub const fn rseccnt(&self) -> &RSECCNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x04 - Binary Counter 1
    #[inline(always)]
    pub const fn bcnt1(&self) -> &BCNT1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Minute Counter
    #[inline(always)]
    pub const fn rmincnt(&self) -> &RMINCNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - Binary Counter 2
    #[inline(always)]
    pub const fn bcnt2(&self) -> &BCNT2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x06 - Hour Counter
    #[inline(always)]
    pub const fn rhrcnt(&self) -> &RHRCNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x08 - Binary Counter 3
    #[inline(always)]
    pub const fn bcnt3(&self) -> &BCNT3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - Day-of-Week Counter
    #[inline(always)]
    pub const fn rwkcnt(&self) -> &RWKCNT {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0a - Day Counter
    #[inline(always)]
    pub const fn rdaycnt(&self) -> &RDAYCNT {
        &self.rdaycnt
    }
    ///0x0c - Month Counter
    #[inline(always)]
    pub const fn rmoncnt(&self) -> &RMONCNT {
        &self.rmoncnt
    }
    ///0x0e - Year Counter
    #[inline(always)]
    pub const fn ryrcnt(&self) -> &RYRCNT {
        &self.ryrcnt
    }
    ///0x10 - Binary Counter 0 Alarm Register
    #[inline(always)]
    pub const fn bcnt0ar(&self) -> &BCNT0AR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Second Alarm Register
    #[inline(always)]
    pub const fn rsecar(&self) -> &RSECAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x12 - Binary Counter 1 Alarm Register
    #[inline(always)]
    pub const fn bcnt1ar(&self) -> &BCNT1AR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    ///0x12 - Minute Alarm Register
    #[inline(always)]
    pub const fn rminar(&self) -> &RMINAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    ///0x14 - Binary Counter 2 Alarm Register
    #[inline(always)]
    pub const fn bcnt2ar(&self) -> &BCNT2AR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - Hour Alarm Register
    #[inline(always)]
    pub const fn rhrar(&self) -> &RHRAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x16 - Binary Counter 3 Alarm Register
    #[inline(always)]
    pub const fn bcnt3ar(&self) -> &BCNT3AR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    ///0x16 - Day-of-Week Alarm Register
    #[inline(always)]
    pub const fn rwkar(&self) -> &RWKAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    ///0x18 - Binary Counter 0 Alarm Enable Register
    #[inline(always)]
    pub const fn bcnt0aer(&self) -> &BCNT0AER {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - Date Alarm Register
    #[inline(always)]
    pub const fn rdayar(&self) -> &RDAYAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1a - Binary Counter 1 Alarm Enable Register
    #[inline(always)]
    pub const fn bcnt1aer(&self) -> &BCNT1AER {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    ///0x1a - Month Alarm Register
    #[inline(always)]
    pub const fn rmonar(&self) -> &RMONAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    ///0x1c - Binary Counter 2 Alarm Enable Register
    #[inline(always)]
    pub const fn bcnt2aer(&self) -> &BCNT2AER {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - Year Alarm Register
    #[inline(always)]
    pub const fn ryrar(&self) -> &RYRAR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1e - Binary Counter 3 Alarm Enable Register
    #[inline(always)]
    pub const fn bcnt3aer(&self) -> &BCNT3AER {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    ///0x1e - Year Alarm Enable Register
    #[inline(always)]
    pub const fn ryraren(&self) -> &RYRAREN {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(30).cast() }
    }
    ///0x22 - RTC Control Register 1
    #[inline(always)]
    pub const fn rcr1(&self) -> &RCR1 {
        &self.rcr1
    }
    ///0x24 - RTC Control Register 2
    #[inline(always)]
    pub const fn rcr2(&self) -> &RCR2 {
        &self.rcr2
    }
    ///0x28 - RTC Control Register 4
    #[inline(always)]
    pub const fn rcr4(&self) -> &RCR4 {
        &self.rcr4
    }
    ///0x2a - Frequency Register H
    #[inline(always)]
    pub const fn rfrh(&self) -> &RFRH {
        &self.rfrh
    }
    ///0x2c - Frequency Register L
    #[inline(always)]
    pub const fn rfrl(&self) -> &RFRL {
        &self.rfrl
    }
    ///0x2e - Time Error Adjustment Register
    #[inline(always)]
    pub const fn radj(&self) -> &RADJ {
        &self.radj
    }
    ///0x40 - Time Capture Control Register %s
    #[inline(always)]
    pub const fn rtccr(&self, n: usize) -> &RTCCR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(2 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x40 - Time Capture Control Register %s
    #[inline(always)]
    pub fn rtccr_iter(&self) -> impl Iterator<Item = &RTCCR> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(2 * n)
                .cast()
        })
    }
    ///0x52 - BCNT0 Capture Register %s
    #[inline(always)]
    pub const fn bcnt0cp(&self, n: usize) -> &BCNT0CP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x52 - BCNT0 Capture Register %s
    #[inline(always)]
    pub fn bcnt0cp_iter(&self) -> impl Iterator<Item = &BCNT0CP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        })
    }
    ///0x52 - Second Capture Register %s
    #[inline(always)]
    pub const fn rseccp(&self, n: usize) -> &RSECCP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x52 - Second Capture Register %s
    #[inline(always)]
    pub fn rseccp_iter(&self) -> impl Iterator<Item = &RSECCP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(82)
                .add(16 * n)
                .cast()
        })
    }
    ///0x54 - BCNT1 Capture Register %s
    #[inline(always)]
    pub const fn bcnt1cp(&self, n: usize) -> &BCNT1CP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x54 - BCNT1 Capture Register %s
    #[inline(always)]
    pub fn bcnt1cp_iter(&self) -> impl Iterator<Item = &BCNT1CP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        })
    }
    ///0x54 - Minute Capture Register %s
    #[inline(always)]
    pub const fn rmincp(&self, n: usize) -> &RMINCP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x54 - Minute Capture Register %s
    #[inline(always)]
    pub fn rmincp_iter(&self) -> impl Iterator<Item = &RMINCP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        })
    }
    ///0x56 - BCNT2 Capture Register %s
    #[inline(always)]
    pub const fn bcnt2cp(&self, n: usize) -> &BCNT2CP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x56 - BCNT2 Capture Register %s
    #[inline(always)]
    pub fn bcnt2cp_iter(&self) -> impl Iterator<Item = &BCNT2CP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        })
    }
    ///0x56 - Hour Capture Register %s
    #[inline(always)]
    pub const fn rhrcp(&self, n: usize) -> &RHRCP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x56 - Hour Capture Register %s
    #[inline(always)]
    pub fn rhrcp_iter(&self) -> impl Iterator<Item = &RHRCP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(86)
                .add(16 * n)
                .cast()
        })
    }
    ///0x5a - BCNT3 Capture Register %s
    #[inline(always)]
    pub const fn bcnt3cp(&self, n: usize) -> &BCNT3CP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x5a - BCNT3 Capture Register %s
    #[inline(always)]
    pub fn bcnt3cp_iter(&self) -> impl Iterator<Item = &BCNT3CP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        })
    }
    ///0x5a - Date Capture Register %s
    #[inline(always)]
    pub const fn rdaycp(&self, n: usize) -> &RDAYCP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x5a - Date Capture Register %s
    #[inline(always)]
    pub fn rdaycp_iter(&self) -> impl Iterator<Item = &RDAYCP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(90)
                .add(16 * n)
                .cast()
        })
    }
    ///0x5c - Month Capture Register %s
    #[inline(always)]
    pub const fn rmoncp(&self, n: usize) -> &RMONCP {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x5c - Month Capture Register %s
    #[inline(always)]
    pub fn rmoncp_iter(&self) -> impl Iterator<Item = &RMONCP> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        })
    }
}
/**R64CNT (r) register accessor: 64-Hz Counter

You can [`read`](crate::Reg::read) this register and get [`r64cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@r64cnt`] module*/
pub type R64CNT = crate::Reg<r64cnt::R64CNT_SPEC>;
///64-Hz Counter
pub mod r64cnt;
/**RSECCNT (rw) register accessor: Second Counter

You can [`read`](crate::Reg::read) this register and get [`rseccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rseccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rseccnt`] module*/
pub type RSECCNT = crate::Reg<rseccnt::RSECCNT_SPEC>;
///Second Counter
pub mod rseccnt;
/**BCNT0 (rw) register accessor: Binary Counter 0

You can [`read`](crate::Reg::read) this register and get [`bcnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt0`] module*/
pub type BCNT0 = crate::Reg<bcnt0::BCNT0_SPEC>;
///Binary Counter 0
pub mod bcnt0;
/**RMINCNT (rw) register accessor: Minute Counter

You can [`read`](crate::Reg::read) this register and get [`rmincnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmincnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmincnt`] module*/
pub type RMINCNT = crate::Reg<rmincnt::RMINCNT_SPEC>;
///Minute Counter
pub mod rmincnt;
/**BCNT1 (rw) register accessor: Binary Counter 1

You can [`read`](crate::Reg::read) this register and get [`bcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt1`] module*/
pub type BCNT1 = crate::Reg<bcnt1::BCNT1_SPEC>;
///Binary Counter 1
pub mod bcnt1;
/**RHRCNT (rw) register accessor: Hour Counter

You can [`read`](crate::Reg::read) this register and get [`rhrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rhrcnt`] module*/
pub type RHRCNT = crate::Reg<rhrcnt::RHRCNT_SPEC>;
///Hour Counter
pub mod rhrcnt;
/**BCNT2 (rw) register accessor: Binary Counter 2

You can [`read`](crate::Reg::read) this register and get [`bcnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt2`] module*/
pub type BCNT2 = crate::Reg<bcnt2::BCNT2_SPEC>;
///Binary Counter 2
pub mod bcnt2;
/**RWKCNT (rw) register accessor: Day-of-Week Counter

You can [`read`](crate::Reg::read) this register and get [`rwkcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwkcnt`] module*/
pub type RWKCNT = crate::Reg<rwkcnt::RWKCNT_SPEC>;
///Day-of-Week Counter
pub mod rwkcnt;
/**BCNT3 (rw) register accessor: Binary Counter 3

You can [`read`](crate::Reg::read) this register and get [`bcnt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt3`] module*/
pub type BCNT3 = crate::Reg<bcnt3::BCNT3_SPEC>;
///Binary Counter 3
pub mod bcnt3;
/**RDAYCNT (rw) register accessor: Day Counter

You can [`read`](crate::Reg::read) this register and get [`rdaycnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdaycnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdaycnt`] module*/
pub type RDAYCNT = crate::Reg<rdaycnt::RDAYCNT_SPEC>;
///Day Counter
pub mod rdaycnt;
/**RMONCNT (rw) register accessor: Month Counter

You can [`read`](crate::Reg::read) this register and get [`rmoncnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmoncnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmoncnt`] module*/
pub type RMONCNT = crate::Reg<rmoncnt::RMONCNT_SPEC>;
///Month Counter
pub mod rmoncnt;
/**RYRCNT (rw) register accessor: Year Counter

You can [`read`](crate::Reg::read) this register and get [`ryrcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ryrcnt`] module*/
pub type RYRCNT = crate::Reg<ryrcnt::RYRCNT_SPEC>;
///Year Counter
pub mod ryrcnt;
/**RSECAR (rw) register accessor: Second Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rsecar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsecar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsecar`] module*/
pub type RSECAR = crate::Reg<rsecar::RSECAR_SPEC>;
///Second Alarm Register
pub mod rsecar;
/**BCNT0AR (rw) register accessor: Binary Counter 0 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt0ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt0ar`] module*/
pub type BCNT0AR = crate::Reg<bcnt0ar::BCNT0AR_SPEC>;
///Binary Counter 0 Alarm Register
pub mod bcnt0ar;
/**RMINAR (rw) register accessor: Minute Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rminar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rminar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rminar`] module*/
pub type RMINAR = crate::Reg<rminar::RMINAR_SPEC>;
///Minute Alarm Register
pub mod rminar;
/**BCNT1AR (rw) register accessor: Binary Counter 1 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt1ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt1ar`] module*/
pub type BCNT1AR = crate::Reg<bcnt1ar::BCNT1AR_SPEC>;
///Binary Counter 1 Alarm Register
pub mod bcnt1ar;
/**RHRAR (rw) register accessor: Hour Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rhrar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rhrar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rhrar`] module*/
pub type RHRAR = crate::Reg<rhrar::RHRAR_SPEC>;
///Hour Alarm Register
pub mod rhrar;
/**BCNT2AR (rw) register accessor: Binary Counter 2 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt2ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt2ar`] module*/
pub type BCNT2AR = crate::Reg<bcnt2ar::BCNT2AR_SPEC>;
///Binary Counter 2 Alarm Register
pub mod bcnt2ar;
/**RWKAR (rw) register accessor: Day-of-Week Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rwkar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwkar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rwkar`] module*/
pub type RWKAR = crate::Reg<rwkar::RWKAR_SPEC>;
///Day-of-Week Alarm Register
pub mod rwkar;
/**BCNT3AR (rw) register accessor: Binary Counter 3 Alarm Register

You can [`read`](crate::Reg::read) this register and get [`bcnt3ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt3ar`] module*/
pub type BCNT3AR = crate::Reg<bcnt3ar::BCNT3AR_SPEC>;
///Binary Counter 3 Alarm Register
pub mod bcnt3ar;
/**RDAYAR (rw) register accessor: Date Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rdayar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdayar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdayar`] module*/
pub type RDAYAR = crate::Reg<rdayar::RDAYAR_SPEC>;
///Date Alarm Register
pub mod rdayar;
/**BCNT0AER (rw) register accessor: Binary Counter 0 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt0aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt0aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt0aer`] module*/
pub type BCNT0AER = crate::Reg<bcnt0aer::BCNT0AER_SPEC>;
///Binary Counter 0 Alarm Enable Register
pub mod bcnt0aer;
/**RMONAR (rw) register accessor: Month Alarm Register

You can [`read`](crate::Reg::read) this register and get [`rmonar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmonar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmonar`] module*/
pub type RMONAR = crate::Reg<rmonar::RMONAR_SPEC>;
///Month Alarm Register
pub mod rmonar;
/**BCNT1AER (rw) register accessor: Binary Counter 1 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt1aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt1aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt1aer`] module*/
pub type BCNT1AER = crate::Reg<bcnt1aer::BCNT1AER_SPEC>;
///Binary Counter 1 Alarm Enable Register
pub mod bcnt1aer;
/**RYRAR (rw) register accessor: Year Alarm Register

You can [`read`](crate::Reg::read) this register and get [`ryrar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryrar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ryrar`] module*/
pub type RYRAR = crate::Reg<ryrar::RYRAR_SPEC>;
///Year Alarm Register
pub mod ryrar;
/**BCNT2AER (rw) register accessor: Binary Counter 2 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt2aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt2aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt2aer`] module*/
pub type BCNT2AER = crate::Reg<bcnt2aer::BCNT2AER_SPEC>;
///Binary Counter 2 Alarm Enable Register
pub mod bcnt2aer;
/**RYRAREN (rw) register accessor: Year Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`ryraren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ryraren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ryraren`] module*/
pub type RYRAREN = crate::Reg<ryraren::RYRAREN_SPEC>;
///Year Alarm Enable Register
pub mod ryraren;
/**BCNT3AER (rw) register accessor: Binary Counter 3 Alarm Enable Register

You can [`read`](crate::Reg::read) this register and get [`bcnt3aer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt3aer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt3aer`] module*/
pub type BCNT3AER = crate::Reg<bcnt3aer::BCNT3AER_SPEC>;
///Binary Counter 3 Alarm Enable Register
pub mod bcnt3aer;
/**RCR1 (rw) register accessor: RTC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`rcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcr1`] module*/
pub type RCR1 = crate::Reg<rcr1::RCR1_SPEC>;
///RTC Control Register 1
pub mod rcr1;
/**RCR2 (rw) register accessor: RTC Control Register 2

You can [`read`](crate::Reg::read) this register and get [`rcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcr2`] module*/
pub type RCR2 = crate::Reg<rcr2::RCR2_SPEC>;
///RTC Control Register 2
pub mod rcr2;
/**RCR4 (rw) register accessor: RTC Control Register 4

You can [`read`](crate::Reg::read) this register and get [`rcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rcr4`] module*/
pub type RCR4 = crate::Reg<rcr4::RCR4_SPEC>;
///RTC Control Register 4
pub mod rcr4;
/**RFRH (rw) register accessor: Frequency Register H

You can [`read`](crate::Reg::read) this register and get [`rfrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfrh`] module*/
pub type RFRH = crate::Reg<rfrh::RFRH_SPEC>;
///Frequency Register H
pub mod rfrh;
/**RFRL (rw) register accessor: Frequency Register L

You can [`read`](crate::Reg::read) this register and get [`rfrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfrl`] module*/
pub type RFRL = crate::Reg<rfrl::RFRL_SPEC>;
///Frequency Register L
pub mod rfrl;
/**RADJ (rw) register accessor: Time Error Adjustment Register

You can [`read`](crate::Reg::read) this register and get [`radj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@radj`] module*/
pub type RADJ = crate::Reg<radj::RADJ_SPEC>;
///Time Error Adjustment Register
pub mod radj;
/**RTCCR (rw) register accessor: Time Capture Control Register %s

You can [`read`](crate::Reg::read) this register and get [`rtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtccr`] module*/
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
///Time Capture Control Register %s
pub mod rtccr;
/**RSECCP (r) register accessor: Second Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rseccp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rseccp`] module*/
pub type RSECCP = crate::Reg<rseccp::RSECCP_SPEC>;
///Second Capture Register %s
pub mod rseccp;
/**BCNT0CP (r) register accessor: BCNT0 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt0cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt0cp`] module*/
pub type BCNT0CP = crate::Reg<bcnt0cp::BCNT0CP_SPEC>;
///BCNT0 Capture Register %s
pub mod bcnt0cp;
/**RMINCP (r) register accessor: Minute Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmincp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmincp`] module*/
pub type RMINCP = crate::Reg<rmincp::RMINCP_SPEC>;
///Minute Capture Register %s
pub mod rmincp;
/**BCNT1CP (r) register accessor: BCNT1 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt1cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt1cp`] module*/
pub type BCNT1CP = crate::Reg<bcnt1cp::BCNT1CP_SPEC>;
///BCNT1 Capture Register %s
pub mod bcnt1cp;
/**RHRCP (r) register accessor: Hour Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rhrcp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rhrcp`] module*/
pub type RHRCP = crate::Reg<rhrcp::RHRCP_SPEC>;
///Hour Capture Register %s
pub mod rhrcp;
/**BCNT2CP (r) register accessor: BCNT2 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt2cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt2cp`] module*/
pub type BCNT2CP = crate::Reg<bcnt2cp::BCNT2CP_SPEC>;
///BCNT2 Capture Register %s
pub mod bcnt2cp;
/**RDAYCP (r) register accessor: Date Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rdaycp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdaycp`] module*/
pub type RDAYCP = crate::Reg<rdaycp::RDAYCP_SPEC>;
///Date Capture Register %s
pub mod rdaycp;
/**BCNT3CP (r) register accessor: BCNT3 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt3cp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcnt3cp`] module*/
pub type BCNT3CP = crate::Reg<bcnt3cp::BCNT3CP_SPEC>;
///BCNT3 Capture Register %s
pub mod bcnt3cp;
/**RMONCP (r) register accessor: Month Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmoncp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmoncp`] module*/
pub type RMONCP = crate::Reg<rmoncp::RMONCP_SPEC>;
///Month Capture Register %s
pub mod rmoncp;
