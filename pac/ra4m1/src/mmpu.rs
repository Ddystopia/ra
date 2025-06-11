#[repr(C)]
///Register block
pub struct RegisterBlock {
    mmpuctla: MMPUCTLA,
    _reserved1: [u8; 0x0100],
    mmpupta: MMPUPTA,
    _reserved2: [u8; 0xfc],
    mmpuaca: (),
    _reserved3: [u8; 0x04],
    mmpusa: (),
    _reserved4: [u8; 0x04],
    mmpuea: (),
}
impl RegisterBlock {
    ///0x00 - Bus Master MPU Control Register A
    #[inline(always)]
    pub const fn mmpuctla(&self) -> &MMPUCTLA {
        &self.mmpuctla
    }
    ///0x102 - Group A Protection of Register
    #[inline(always)]
    pub const fn mmpupta(&self) -> &MMPUPTA {
        &self.mmpupta
    }
    ///0x200..0x220 - Group A Region %s Access Control Register
    #[inline(always)]
    pub const fn mmpuaca(&self, n: usize) -> &MMPUACA {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x200..0x220 - Group A Region %s Access Control Register
    #[inline(always)]
    pub fn mmpuaca_iter(&self) -> impl Iterator<Item = &MMPUACA> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(16 * n)
                .cast()
        })
    }
    ///0x204..0x244 - Group A Region %s Start Address Register
    #[inline(always)]
    pub const fn mmpusa(&self, n: usize) -> &MMPUSA {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x204..0x244 - Group A Region %s Start Address Register
    #[inline(always)]
    pub fn mmpusa_iter(&self) -> impl Iterator<Item = &MMPUSA> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(516)
                .add(16 * n)
                .cast()
        })
    }
    ///0x208..0x248 - Group A Region %s End Address Register
    #[inline(always)]
    pub const fn mmpuea(&self, n: usize) -> &MMPUEA {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x208..0x248 - Group A Region %s End Address Register
    #[inline(always)]
    pub fn mmpuea_iter(&self) -> impl Iterator<Item = &MMPUEA> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(520)
                .add(16 * n)
                .cast()
        })
    }
}
/**MMPUCTLA (rw) register accessor: Bus Master MPU Control Register A

You can [`read`](crate::Reg::read) this register and get [`mmpuctla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuctla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuctla`] module*/
pub type MMPUCTLA = crate::Reg<mmpuctla::MMPUCTLA_SPEC>;
///Bus Master MPU Control Register A
pub mod mmpuctla;
/**MMPUACA (rw) register accessor: Group A Region %s Access Control Register

You can [`read`](crate::Reg::read) this register and get [`mmpuaca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuaca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuaca`] module*/
pub type MMPUACA = crate::Reg<mmpuaca::MMPUACA_SPEC>;
///Group A Region %s Access Control Register
pub mod mmpuaca;
/**MMPUSA (rw) register accessor: Group A Region %s Start Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpusa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusa`] module*/
pub type MMPUSA = crate::Reg<mmpusa::MMPUSA_SPEC>;
///Group A Region %s Start Address Register
pub mod mmpusa;
/**MMPUEA (rw) register accessor: Group A Region %s End Address Register

You can [`read`](crate::Reg::read) this register and get [`mmpuea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuea`] module*/
pub type MMPUEA = crate::Reg<mmpuea::MMPUEA_SPEC>;
///Group A Region %s End Address Register
pub mod mmpuea;
/**MMPUPTA (rw) register accessor: Group A Protection of Register

You can [`read`](crate::Reg::read) this register and get [`mmpupta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpupta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpupta`] module*/
pub type MMPUPTA = crate::Reg<mmpupta::MMPUPTA_SPEC>;
///Group A Protection of Register
pub mod mmpupta;
