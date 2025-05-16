#[repr(C)]
///Register block
pub struct RegisterBlock {
    mmpuctl: (),
    _reserved1: [u8; 0x0102],
    mmpupta: MMPUPTA,
    _reserved2: [u8; 0xfc],
    mmpuaca: (),
    _reserved3: [u8; 0x04],
    mmpusa: (),
    _reserved4: [u8; 0x04],
    mmpuea: (),
    _reserved5: [u8; 0x02fa],
    mmpuptb: MMPUPTB,
    _reserved6: [u8; 0xfc],
    mmpuacb: (),
    _reserved7: [u8; 0x04],
    mmpusb: (),
    _reserved8: [u8; 0x04],
    mmpueb: (),
    _reserved9: [u8; 0x02fa],
    mmpuptc: MMPUPTC,
    _reserved10: [u8; 0xfc],
    mmpuacc: (),
    _reserved11: [u8; 0x04],
    mmpusc: (),
    _reserved12: [u8; 0x04],
    mmpuec: (),
}
impl RegisterBlock {
    ///0x00..0x06 - Bus Master MPU Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `MMPUCTLA` register.</div>
    #[inline(always)]
    pub const fn mmpuctl(&self, n: usize) -> &MMPUCTL {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1024 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x06 - Bus Master MPU Control Register
    #[inline(always)]
    pub fn mmpuctl_iter(&self) -> impl Iterator<Item = &MMPUCTL> {
        (0..3)
            .map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1024 * n).cast() })
    }
    ///0x00 - Bus Master MPU Control Register
    #[inline(always)]
    pub const fn mmpuctla(&self) -> &MMPUCTL {
        self.mmpuctl(0)
    }
    ///0x400 - Bus Master MPU Control Register
    #[inline(always)]
    pub const fn mmpuctlb(&self) -> &MMPUCTL {
        self.mmpuctl(1)
    }
    ///0x800 - Bus Master MPU Control Register
    #[inline(always)]
    pub const fn mmpuctlc(&self) -> &MMPUCTL {
        self.mmpuctl(2)
    }
    ///0x102 - Group A Protection of Register
    #[inline(always)]
    pub const fn mmpupta(&self) -> &MMPUPTA {
        &self.mmpupta
    }
    ///0x200..0x240 - Group A Region %s Access Control Register
    #[inline(always)]
    pub const fn mmpuaca(&self, n: usize) -> &MMPUACA {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x200..0x240 - Group A Region %s Access Control Register
    #[inline(always)]
    pub fn mmpuaca_iter(&self) -> impl Iterator<Item = &MMPUACA> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        })
    }
    ///0x204..0x284 - Group A Region %s Start Address Register
    #[inline(always)]
    pub const fn mmpusa(&self, n: usize) -> &MMPUSA {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x204..0x284 - Group A Region %s Start Address Register
    #[inline(always)]
    pub fn mmpusa_iter(&self) -> impl Iterator<Item = &MMPUSA> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        })
    }
    ///0x208..0x288 - Group A Region %s End Address Register
    #[inline(always)]
    pub const fn mmpuea(&self, n: usize) -> &MMPUEA {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x208..0x288 - Group A Region %s End Address Register
    #[inline(always)]
    pub fn mmpuea_iter(&self) -> impl Iterator<Item = &MMPUEA> {
        (0..32).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        })
    }
    ///0x502 - Group B Protection of Register
    #[inline(always)]
    pub const fn mmpuptb(&self) -> &MMPUPTB {
        &self.mmpuptb
    }
    ///0x600..0x610 - Group B Region %s Access Control Register
    #[inline(always)]
    pub const fn mmpuacb(&self, n: usize) -> &MMPUACB {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x600..0x610 - Group B Region %s Access Control Register
    #[inline(always)]
    pub fn mmpuacb_iter(&self) -> impl Iterator<Item = &MMPUACB> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1536)
                .add(16 * n)
                .cast()
        })
    }
    ///0x604..0x624 - Group B Region %s Start Address Register
    #[inline(always)]
    pub const fn mmpusb(&self, n: usize) -> &MMPUSB {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1540)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x604..0x624 - Group B Region %s Start Address Register
    #[inline(always)]
    pub fn mmpusb_iter(&self) -> impl Iterator<Item = &MMPUSB> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1540)
                .add(16 * n)
                .cast()
        })
    }
    ///0x608..0x628 - Group B Region %s End Address Register
    #[inline(always)]
    pub const fn mmpueb(&self, n: usize) -> &MMPUEB {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1544)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x608..0x628 - Group B Region %s End Address Register
    #[inline(always)]
    pub fn mmpueb_iter(&self) -> impl Iterator<Item = &MMPUEB> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1544)
                .add(16 * n)
                .cast()
        })
    }
    ///0x902 - Group C protection of register
    #[inline(always)]
    pub const fn mmpuptc(&self) -> &MMPUPTC {
        &self.mmpuptc
    }
    ///0xa00..0xa10 - Group C Region %s Access Control Register
    #[inline(always)]
    pub const fn mmpuacc(&self, n: usize) -> &MMPUACC {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2560)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0xa00..0xa10 - Group C Region %s Access Control Register
    #[inline(always)]
    pub fn mmpuacc_iter(&self) -> impl Iterator<Item = &MMPUACC> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2560)
                .add(16 * n)
                .cast()
        })
    }
    ///0xa04..0xa24 - Group C Region %s Start Address Register
    #[inline(always)]
    pub const fn mmpusc(&self, n: usize) -> &MMPUSC {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2564)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0xa04..0xa24 - Group C Region %s Start Address Register
    #[inline(always)]
    pub fn mmpusc_iter(&self) -> impl Iterator<Item = &MMPUSC> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2564)
                .add(16 * n)
                .cast()
        })
    }
    ///0xa08..0xa28 - Group C Region %s End Address Register
    #[inline(always)]
    pub const fn mmpuec(&self, n: usize) -> &MMPUEC {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2568)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0xa08..0xa28 - Group C Region %s End Address Register
    #[inline(always)]
    pub fn mmpuec_iter(&self) -> impl Iterator<Item = &MMPUEC> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2568)
                .add(16 * n)
                .cast()
        })
    }
}
/**MMPUCTL (rw) register accessor: Bus Master MPU Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuctl`] module*/
pub type MMPUCTL = crate::Reg<mmpuctl::MMPUCTL_SPEC>;
///Bus Master MPU Control Register
pub mod mmpuctl;
/**MMPUACA (rw) register accessor: Group A Region %s Access Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuaca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuaca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuaca`] module*/
pub type MMPUACA = crate::Reg<mmpuaca::MMPUACA_SPEC>;
///Group A Region %s Access Control Register
pub mod mmpuaca;
/**MMPUACB (rw) register accessor: Group B Region %s Access Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuacb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuacb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuacb`] module*/
pub type MMPUACB = crate::Reg<mmpuacb::MMPUACB_SPEC>;
///Group B Region %s Access Control Register
pub mod mmpuacb;
/**MMPUACC (rw) register accessor: Group C Region %s Access Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuacc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuacc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuacc`] module*/
pub type MMPUACC = crate::Reg<mmpuacc::MMPUACC_SPEC>;
///Group C Region %s Access Control Register
pub mod mmpuacc;
/**MMPUSA (rw) register accessor: Group A Region %s Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusa`] module*/
pub type MMPUSA = crate::Reg<mmpusa::MMPUSA_SPEC>;
///Group A Region %s Start Address Register
pub mod mmpusa;
/**MMPUSB (rw) register accessor: Group B Region %s Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpusb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusb`] module*/
pub type MMPUSB = crate::Reg<mmpusb::MMPUSB_SPEC>;
///Group B Region %s Start Address Register
pub mod mmpusb;
/**MMPUSC (rw) register accessor: Group C Region %s Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpusc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusc`] module*/
pub type MMPUSC = crate::Reg<mmpusc::MMPUSC_SPEC>;
///Group C Region %s Start Address Register
pub mod mmpusc;
/**MMPUEA (rw) register accessor: Group A Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuea`] module*/
pub type MMPUEA = crate::Reg<mmpuea::MMPUEA_SPEC>;
///Group A Region %s End Address Register
pub mod mmpuea;
/**MMPUEB (rw) register accessor: Group B Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpueb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpueb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpueb`] module*/
pub type MMPUEB = crate::Reg<mmpueb::MMPUEB_SPEC>;
///Group B Region %s End Address Register
pub mod mmpueb;
/**MMPUEC (rw) register accessor: Group C Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpuec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuec`] module*/
pub type MMPUEC = crate::Reg<mmpuec::MMPUEC_SPEC>;
///Group C Region %s End Address Register
pub mod mmpuec;
/**MMPUPTA (rw) register accessor: Group A Protection of Register

You can [`read`](crate::Reg::read) this register and get [`mmpupta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpupta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpupta`] module*/
pub type MMPUPTA = crate::Reg<mmpupta::MMPUPTA_SPEC>;
///Group A Protection of Register
pub mod mmpupta;
/**MMPUPTB (rw) register accessor: Group B Protection of Register

You can [`read`](crate::Reg::read) this register and get [`mmpuptb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuptb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuptb`] module*/
pub type MMPUPTB = crate::Reg<mmpuptb::MMPUPTB_SPEC>;
///Group B Protection of Register
pub mod mmpuptb;
/**MMPUPTC (rw) register accessor: Group C protection of register

You can [`read`](crate::Reg::read) this register and get [`mmpuptc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuptc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuptc`] module*/
pub type MMPUPTC = crate::Reg<mmpuptc::MMPUPTC_SPEC>;
///Group C protection of register
pub mod mmpuptc;
