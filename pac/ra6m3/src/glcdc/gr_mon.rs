///Register `GR%s_MON` reader
pub type R = crate::R<GR_MON_SPEC>;
/**Status monitor for alpha blending in rectangular area

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCST_A {
    ///1: Fade-in/fade-out is in progress.
    _1 = 1,
    ///0: Fade-in/fade-out is not in progress.
    _0 = 0,
}
impl From<ARCST_A> for bool {
    #[inline(always)]
    fn from(variant: ARCST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARCST` reader - Status monitor for alpha blending in rectangular area
pub type ARCST_R = crate::BitReader<ARCST_A>;
impl ARCST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCST_A {
        match self.bits {
            true => ARCST_A::_1,
            false => ARCST_A::_0,
        }
    }
    ///Fade-in/fade-out is in progress.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARCST_A::_1
    }
    ///Fade-in/fade-out is not in progress.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARCST_A::_0
    }
}
/**Status monitor for underflow

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDFLST_A {
    ///1: An underflow occurs in internal operations.
    _1 = 1,
    ///0: No underflow occurs in internal operations.
    _0 = 0,
}
impl From<UNDFLST_A> for bool {
    #[inline(always)]
    fn from(variant: UNDFLST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UNDFLST` reader - Status monitor for underflow
pub type UNDFLST_R = crate::BitReader<UNDFLST_A>;
impl UNDFLST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UNDFLST_A {
        match self.bits {
            true => UNDFLST_A::_1,
            false => UNDFLST_A::_0,
        }
    }
    ///An underflow occurs in internal operations.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNDFLST_A::_1
    }
    ///No underflow occurs in internal operations.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNDFLST_A::_0
    }
}
impl R {
    ///Bit 0 - Status monitor for alpha blending in rectangular area
    #[inline(always)]
    pub fn arcst(&self) -> ARCST_R {
        ARCST_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Status monitor for underflow
    #[inline(always)]
    pub fn undflst(&self) -> UNDFLST_R {
        UNDFLST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
/**Graphics %s Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`gr_mon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_MON_SPEC;
impl crate::RegisterSpec for GR_MON_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_mon::R`](R) reader structure
impl crate::Readable for GR_MON_SPEC {}
///`reset()` method sets GR%s_MON to value 0
impl crate::Resettable for GR_MON_SPEC {}
