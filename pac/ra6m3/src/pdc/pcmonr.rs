///Register `PCMONR` reader
pub type R = crate::R<PCMONR_SPEC>;
/**VSYNC Signal Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_A {
    ///0: VSYNC signal is at the low level.
    _0 = 0,
    ///1: VSYNC signal is at the high level.
    _1 = 1,
}
impl From<VSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VSYNC` reader - VSYNC Signal Status Flag
pub type VSYNC_R = crate::BitReader<VSYNC_A>;
impl VSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSYNC_A {
        match self.bits {
            false => VSYNC_A::_0,
            true => VSYNC_A::_1,
        }
    }
    ///VSYNC signal is at the low level.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VSYNC_A::_0
    }
    ///VSYNC signal is at the high level.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VSYNC_A::_1
    }
}
/**HSYNC Signal Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNC_A {
    ///0: HSYNC signal is at the low level.
    _0 = 0,
    ///1: HSYNC signal is at the high level.
    _1 = 1,
}
impl From<HSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HSYNC` reader - HSYNC Signal Status Flag
pub type HSYNC_R = crate::BitReader<HSYNC_A>;
impl HSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSYNC_A {
        match self.bits {
            false => HSYNC_A::_0,
            true => HSYNC_A::_1,
        }
    }
    ///HSYNC signal is at the low level.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSYNC_A::_0
    }
    ///HSYNC signal is at the high level.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSYNC_A::_1
    }
}
impl R {
    ///Bit 0 - VSYNC Signal Status Flag
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSYNC Signal Status Flag
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
/**PDC Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`pcmonr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCMONR_SPEC;
impl crate::RegisterSpec for PCMONR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcmonr::R`](R) reader structure
impl crate::Readable for PCMONR_SPEC {}
///`reset()` method sets PCMONR to value 0
impl crate::Resettable for PCMONR_SPEC {}
