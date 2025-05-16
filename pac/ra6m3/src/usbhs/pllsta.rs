///Register `PLLSTA` reader
pub type R = crate::R<PLLSTA_SPEC>;
/**PLL Lock Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLOCK_A {
    ///0: PLL is not locked.
    _0 = 0,
    ///1: PLL is locked.
    _1 = 1,
}
impl From<PLLLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLLOCK` reader - PLL Lock Flag
pub type PLLLOCK_R = crate::BitReader<PLLLOCK_A>;
impl PLLLOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLLOCK_A {
        match self.bits {
            false => PLLLOCK_A::_0,
            true => PLLLOCK_A::_1,
        }
    }
    ///PLL is not locked.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLLOCK_A::_0
    }
    ///PLL is locked.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLLOCK_A::_1
    }
}
impl R {
    ///Bit 0 - PLL Lock Flag
    #[inline(always)]
    pub fn plllock(&self) -> PLLLOCK_R {
        PLLLOCK_R::new((self.bits & 1) != 0)
    }
}
/**PLL Status Register

You can [`read`](crate::Reg::read) this register and get [`pllsta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLLSTA_SPEC;
impl crate::RegisterSpec for PLLSTA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pllsta::R`](R) reader structure
impl crate::Readable for PLLSTA_SPEC {}
///`reset()` method sets PLLSTA to value 0
impl crate::Resettable for PLLSTA_SPEC {}
