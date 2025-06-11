#[repr(C)]
///Register block
pub struct RegisterBlock {
    smpuctl: SMPUCTL,
    _reserved1: [u8; 0x0e],
    smpumbiu: SMPUMBIU,
    _reserved2: [u8; 0x02],
    smpufbiu: SMPUFBIU,
    _reserved3: [u8; 0x02],
    smpusram0: SMPUSRAM0,
    _reserved4: [u8; 0x06],
    smpupbiu: (),
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
    ///0x18 - Access Control Register for SRAM0
    #[inline(always)]
    pub const fn smpusram0(&self) -> &SMPUSRAM0 {
        &self.smpusram0
    }
    ///0x20..0x26 - Access Control Register for P%sBIU
    #[inline(always)]
    pub const fn smpupbiu(&self, n: usize) -> &SMPUPBIU {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(4 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x20..0x26 - Access Control Register for P%sBIU
    #[inline(always)]
    pub fn smpupbiu_iter(&self) -> impl Iterator<Item = &SMPUPBIU> {
        (0..3).map(move |n| unsafe {
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
/**SMPUSRAM0 (rw) register accessor: Access Control Register for SRAM0

You can [`read`](crate::Reg::read) this register and get [`smpusram0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpusram0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpusram0`] module*/
pub type SMPUSRAM0 = crate::Reg<smpusram0::SMPUSRAM0_SPEC>;
///Access Control Register for SRAM0
pub mod smpusram0;
/**SMPUPBIU (rw) register accessor: Access Control Register for P%sBIU

You can [`read`](crate::Reg::read) this register and get [`smpupbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpupbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smpupbiu`] module*/
pub type SMPUPBIU = crate::Reg<smpupbiu::SMPUPBIU_SPEC>;
///Access Control Register for P%sBIU
pub mod smpupbiu;
