#[repr(C)]
///Register block
pub struct RegisterBlock {
    smpuctl: SMPUCTL,
    _reserved1: [u8; 0x0e],
    smpumbiu: SMPUMBIU,
    _reserved2: [u8; 0x02],
    smpufbiu: SMPUFBIU,
    _reserved3: [u8; 0x02],
    smpusram: (),
    _reserved4: [u8; 0x08],
    smpupbiu: (),
    _reserved5: [u8; 0x10],
    smpuexbiu: SMPUEXBIU,
    _reserved6: [u8; 0x02],
    smpuexbiu2: SMPUEXBIU2,
}
impl RegisterBlock {
    ///0x00 - Slave MPU Control Register
    #[inline(always)]
    pub const fn smpuctl(&self) -> &SMPUCTL {
        &self.smpuctl
    }
    ///0x10 - Access Control Register for MBIU
    #[inline(always)]
    pub const fn smpumbiu(&self) -> &SMPUMBIU {
        &self.smpumbiu
    }
    ///0x14 - Access Control Register for FBIU
    #[inline(always)]
    pub const fn smpufbiu(&self) -> &SMPUFBIU {
        &self.smpufbiu
    }
    ///0x18 - Access Control Register for SRAM%s
    #[inline(always)]
    pub const fn smpusram(&self, n: usize) -> &SMPUSRAM {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x18 - Access Control Register for SRAM%s
    #[inline(always)]
    pub fn smpusram_iter(&self) -> impl Iterator<Item = &SMPUSRAM> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(4 * n)
                .cast()
        })
    }
    ///0x20..0x28 - Access Control Register for P%sBIU
    #[inline(always)]
    pub const fn smpupbiu(&self, n: usize) -> &SMPUPBIU {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x20..0x28 - Access Control Register for P%sBIU
    #[inline(always)]
    pub fn smpupbiu_iter(&self) -> impl Iterator<Item = &SMPUPBIU> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        })
    }
    ///0x20 - Access Control Register for P0BIU
    #[inline(always)]
    pub const fn smpup0biu(&self) -> &SMPUPBIU {
        self.smpupbiu(0)
    }
    ///0x24 - Access Control Register for P2BIU
    #[inline(always)]
    pub const fn smpup2biu(&self) -> &SMPUPBIU {
        self.smpupbiu(1)
    }
    ///0x28 - Access Control Register for P6BIU
    #[inline(always)]
    pub const fn smpup6biu(&self) -> &SMPUPBIU {
        self.smpupbiu(2)
    }
    ///0x2c - Access Control Register for P7BIU
    #[inline(always)]
    pub const fn smpup7biu(&self) -> &SMPUPBIU {
        self.smpupbiu(3)
    }
    ///0x30 - Access Control Register for EXBIU
    #[inline(always)]
    pub const fn smpuexbiu(&self) -> &SMPUEXBIU {
        &self.smpuexbiu
    }
    ///0x34 - Access Control Register for EXBIU2
    #[inline(always)]
    pub const fn smpuexbiu2(&self) -> &SMPUEXBIU2 {
        &self.smpuexbiu2
    }
}
/**SMPUCTL (rw) register accessor: Slave MPU Control Register

You can [`read`](crate::Reg::read) this register and get [`smpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpuctl`] module*/
pub type SMPUCTL = crate::Reg<smpuctl::SMPUCTL_SPEC>;
///Slave MPU Control Register
pub mod smpuctl;
/**SMPUMBIU (rw) register accessor: Access Control Register for MBIU

You can [`read`](crate::Reg::read) this register and get [`smpumbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpumbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpumbiu`] module*/
pub type SMPUMBIU = crate::Reg<smpumbiu::SMPUMBIU_SPEC>;
///Access Control Register for MBIU
pub mod smpumbiu;
/**SMPUFBIU (rw) register accessor: Access Control Register for FBIU

You can [`read`](crate::Reg::read) this register and get [`smpufbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpufbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpufbiu`] module*/
pub type SMPUFBIU = crate::Reg<smpufbiu::SMPUFBIU_SPEC>;
///Access Control Register for FBIU
pub mod smpufbiu;
/**SMPUSRAM (rw) register accessor: Access Control Register for SRAM%s

You can [`read`](crate::Reg::read) this register and get [`smpusram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpusram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpusram`] module*/
pub type SMPUSRAM = crate::Reg<smpusram::SMPUSRAM_SPEC>;
///Access Control Register for SRAM%s
pub mod smpusram;
/**SMPUPBIU (rw) register accessor: Access Control Register for P%sBIU

You can [`read`](crate::Reg::read) this register and get [`smpupbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpupbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpupbiu`] module*/
pub type SMPUPBIU = crate::Reg<smpupbiu::SMPUPBIU_SPEC>;
///Access Control Register for P%sBIU
pub mod smpupbiu;
/**SMPUEXBIU (rw) register accessor: Access Control Register for EXBIU

You can [`read`](crate::Reg::read) this register and get [`smpuexbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpuexbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpuexbiu`] module*/
pub type SMPUEXBIU = crate::Reg<smpuexbiu::SMPUEXBIU_SPEC>;
///Access Control Register for EXBIU
pub mod smpuexbiu;
/**SMPUEXBIU2 (rw) register accessor: Access Control Register for EXBIU2

You can [`read`](crate::Reg::read) this register and get [`smpuexbiu2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpuexbiu2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpuexbiu2`] module*/
pub type SMPUEXBIU2 = crate::Reg<smpuexbiu2::SMPUEXBIU2_SPEC>;
///Access Control Register for EXBIU2
pub mod smpuexbiu2;
