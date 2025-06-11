///Register `CTSUERRS` reader
pub type R = crate::R<CTSUERRS_SPEC>;
/**TSCAP Voltage Error Monitor

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUICOMP_A {
    ///0: Normal TSCAP voltage
    _0 = 0,
    ///1: Abnormal TSCAP voltage
    _1 = 1,
}
impl From<CTSUICOMP_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUICOMP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUICOMP` reader - TSCAP Voltage Error Monitor
pub type CTSUICOMP_R = crate::BitReader<CTSUICOMP_A>;
impl CTSUICOMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUICOMP_A {
        match self.bits {
            false => CTSUICOMP_A::_0,
            true => CTSUICOMP_A::_1,
        }
    }
    ///Normal TSCAP voltage
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUICOMP_A::_0
    }
    ///Abnormal TSCAP voltage
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUICOMP_A::_1
    }
}
impl R {
    ///Bit 15 - TSCAP Voltage Error Monitor
    #[inline(always)]
    pub fn ctsuicomp(&self) -> CTSUICOMP_R {
        CTSUICOMP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
/**CTSU Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsuerrs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUERRS_SPEC;
impl crate::RegisterSpec for CTSUERRS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctsuerrs::R`](R) reader structure
impl crate::Readable for CTSUERRS_SPEC {}
///`reset()` method sets CTSUERRS to value 0
impl crate::Resettable for CTSUERRS_SPEC {}
