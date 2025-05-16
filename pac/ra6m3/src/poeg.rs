#[repr(C)]
///Register block
pub struct RegisterBlock {
    poegg: (),
}
impl RegisterBlock {
    ///0x00..0x10 - POEG Group %s Setting Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `POEGGA` register.</div>
    #[inline(always)]
    pub const fn poegg(&self, n: usize) -> &POEGG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256 * n).cast() }
    }
    ///Iterator for array of:
    ///0x00..0x10 - POEG Group %s Setting Register
    #[inline(always)]
    pub fn poegg_iter(&self) -> impl Iterator<Item = &POEGG> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256 * n).cast() })
    }
    ///0x00 - POEG Group A Setting Register
    #[inline(always)]
    pub const fn poegga(&self) -> &POEGG {
        self.poegg(0)
    }
    ///0x100 - POEG Group B Setting Register
    #[inline(always)]
    pub const fn poeggb(&self) -> &POEGG {
        self.poegg(1)
    }
    ///0x200 - POEG Group C Setting Register
    #[inline(always)]
    pub const fn poeggc(&self) -> &POEGG {
        self.poegg(2)
    }
    ///0x300 - POEG Group D Setting Register
    #[inline(always)]
    pub const fn poeggd(&self) -> &POEGG {
        self.poegg(3)
    }
}
/**POEGG (rw) register accessor: POEG Group %s Setting Register

You can [`read`](crate::Reg::read) this register and get [`poegg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poegg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@poegg`] module*/
pub type POEGG = crate::Reg<poegg::POEGG_SPEC>;
///POEG Group %s Setting Register
pub mod poegg;
