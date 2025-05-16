///Register `CMPMON` reader
pub type R = crate::R<CMPMON_SPEC>;
/**Comparator output monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMON_A {
    ///0: Comparator output Low
    _0 = 0,
    ///1: Comparator output High
    _1 = 1,
}
impl From<CMPMON_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPMON` reader - Comparator output monitor
pub type CMPMON_R = crate::BitReader<CMPMON_A>;
impl CMPMON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPMON_A {
        match self.bits {
            false => CMPMON_A::_0,
            true => CMPMON_A::_1,
        }
    }
    ///Comparator output Low
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPMON_A::_0
    }
    ///Comparator output High
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPMON_A::_1
    }
}
impl R {
    ///Bit 0 - Comparator output monitor
    #[inline(always)]
    pub fn cmpmon(&self) -> CMPMON_R {
        CMPMON_R::new((self.bits & 1) != 0)
    }
}
/**Comparator Output Monitor Register

You can [`read`](crate::Reg::read) this register and get [`cmpmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMPMON_SPEC;
impl crate::RegisterSpec for CMPMON_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cmpmon::R`](R) reader structure
impl crate::Readable for CMPMON_SPEC {}
///`reset()` method sets CMPMON to value 0
impl crate::Resettable for CMPMON_SPEC {}
