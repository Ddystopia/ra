///Register `SDSR` reader
pub type R = crate::R<SDSR_SPEC>;
/**Mode Register Setting Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRSST_A {
    ///0: Mode register setting not in progress
    _0 = 0,
    ///1: Mode register setting in progress
    _1 = 1,
}
impl From<MRSST_A> for bool {
    #[inline(always)]
    fn from(variant: MRSST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MRSST` reader - Mode Register Setting Status
pub type MRSST_R = crate::BitReader<MRSST_A>;
impl MRSST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MRSST_A {
        match self.bits {
            false => MRSST_A::_0,
            true => MRSST_A::_1,
        }
    }
    ///Mode register setting not in progress
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRSST_A::_0
    }
    ///Mode register setting in progress
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRSST_A::_1
    }
}
/**Initialization Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIST_A {
    ///0: Initialization sequence not in progress
    _0 = 0,
    ///1: Initialization sequence in progress
    _1 = 1,
}
impl From<INIST_A> for bool {
    #[inline(always)]
    fn from(variant: INIST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INIST` reader - Initialization Status
pub type INIST_R = crate::BitReader<INIST_A>;
impl INIST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> INIST_A {
        match self.bits {
            false => INIST_A::_0,
            true => INIST_A::_1,
        }
    }
    ///Initialization sequence not in progress
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIST_A::_0
    }
    ///Initialization sequence in progress
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIST_A::_1
    }
}
/**Self-Refresh Transition/Recovery Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRFST_A {
    ///0: Transition/recovery not in progress
    _0 = 0,
    ///1: Transition/recovery in progress
    _1 = 1,
}
impl From<SRFST_A> for bool {
    #[inline(always)]
    fn from(variant: SRFST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SRFST` reader - Self-Refresh Transition/Recovery Status
pub type SRFST_R = crate::BitReader<SRFST_A>;
impl SRFST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRFST_A {
        match self.bits {
            false => SRFST_A::_0,
            true => SRFST_A::_1,
        }
    }
    ///Transition/recovery not in progress
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRFST_A::_0
    }
    ///Transition/recovery in progress
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRFST_A::_1
    }
}
impl R {
    ///Bit 0 - Mode Register Setting Status
    #[inline(always)]
    pub fn mrsst(&self) -> MRSST_R {
        MRSST_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Initialization Status
    #[inline(always)]
    pub fn inist(&self) -> INIST_R {
        INIST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Self-Refresh Transition/Recovery Status
    #[inline(always)]
    pub fn srfst(&self) -> SRFST_R {
        SRFST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
/**SDRAM Status Register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDSR_SPEC;
impl crate::RegisterSpec for SDSR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sdsr::R`](R) reader structure
impl crate::Readable for SDSR_SPEC {}
///`reset()` method sets SDSR to value 0
impl crate::Resettable for SDSR_SPEC {}
