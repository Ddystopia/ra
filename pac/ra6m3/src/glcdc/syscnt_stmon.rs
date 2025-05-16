///Register `SYSCNT_STMON` reader
pub type R = crate::R<SYSCNT_STMON_SPEC>;
/**Graphics 2 specified line detection flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VPOS_A {
    ///1: A specified line notification has been detected in graphics 2.
    _1 = 1,
    ///0: No specified line notification has been detected in graphics 2.
    _0 = 0,
}
impl From<VPOS_A> for bool {
    #[inline(always)]
    fn from(variant: VPOS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VPOS` reader - Graphics 2 specified line detection flag
pub type VPOS_R = crate::BitReader<VPOS_A>;
impl VPOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VPOS_A {
        match self.bits {
            true => VPOS_A::_1,
            false => VPOS_A::_0,
        }
    }
    ///A specified line notification has been detected in graphics 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VPOS_A::_1
    }
    ///No specified line notification has been detected in graphics 2.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VPOS_A::_0
    }
}
/**Graphics 1 underflow detection flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1UNDF_A {
    ///1: An underflow has been detected in graphics 1.
    _1 = 1,
    ///0: No underflow has been detected in graphics 1.
    _0 = 0,
}
impl From<L1UNDF_A> for bool {
    #[inline(always)]
    fn from(variant: L1UNDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L1UNDF` reader - Graphics 1 underflow detection flag
pub type L1UNDF_R = crate::BitReader<L1UNDF_A>;
impl L1UNDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L1UNDF_A {
        match self.bits {
            true => L1UNDF_A::_1,
            false => L1UNDF_A::_0,
        }
    }
    ///An underflow has been detected in graphics 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L1UNDF_A::_1
    }
    ///No underflow has been detected in graphics 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L1UNDF_A::_0
    }
}
/**Graphics 2 underflow detection flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L2UNDF_A {
    ///1: An underflow has been detected in graphics 2.
    _1 = 1,
    ///0: No underflow has been detected in graphics 2.
    _0 = 0,
}
impl From<L2UNDF_A> for bool {
    #[inline(always)]
    fn from(variant: L2UNDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `L2UNDF` reader - Graphics 2 underflow detection flag
pub type L2UNDF_R = crate::BitReader<L2UNDF_A>;
impl L2UNDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> L2UNDF_A {
        match self.bits {
            true => L2UNDF_A::_1,
            false => L2UNDF_A::_0,
        }
    }
    ///An underflow has been detected in graphics 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == L2UNDF_A::_1
    }
    ///No underflow has been detected in graphics 2.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == L2UNDF_A::_0
    }
}
impl R {
    ///Bit 0 - Graphics 2 specified line detection flag
    #[inline(always)]
    pub fn vpos(&self) -> VPOS_R {
        VPOS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Graphics 1 underflow detection flag
    #[inline(always)]
    pub fn l1undf(&self) -> L1UNDF_R {
        L1UNDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Graphics 2 underflow detection flag
    #[inline(always)]
    pub fn l2undf(&self) -> L2UNDF_R {
        L2UNDF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
/**System Control Block Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`syscnt_stmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCNT_STMON_SPEC;
impl crate::RegisterSpec for SYSCNT_STMON_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syscnt_stmon::R`](R) reader structure
impl crate::Readable for SYSCNT_STMON_SPEC {}
///`reset()` method sets SYSCNT_STMON to value 0
impl crate::Resettable for SYSCNT_STMON_SPEC {}
