///Register `EIDR` reader
pub type R = crate::R<EIDR_SPEC>;
/**Pmn Event Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EIDR_A {
    ///0: Low input
    _0 = 0,
    ///1: High input.
    _1 = 1,
}
impl From<EIDR_A> for u16 {
    #[inline(always)]
    fn from(variant: EIDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EIDR_A {
    type Ux = u16;
}
impl crate::IsEnum for EIDR_A {}
///Field `EIDR` reader - Pmn Event Input Data
pub type EIDR_R = crate::FieldReader<EIDR_A>;
impl EIDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EIDR_A> {
        match self.bits {
            0 => Some(EIDR_A::_0),
            1 => Some(EIDR_A::_1),
            _ => None,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EIDR_A::_0
    }
    ///High input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EIDR_A::_1
    }
}
impl R {
    ///Bits 0:15 - Pmn Event Input Data
    #[inline(always)]
    pub fn eidr(&self) -> EIDR_R {
        EIDR_R::new(self.bits)
    }
}
/**Event input data register

You can [`read`](crate::Reg::read) this register and get [`eidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EIDR_SPEC;
impl crate::RegisterSpec for EIDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`eidr::R`](R) reader structure
impl crate::Readable for EIDR_SPEC {}
///`reset()` method sets EIDR to value 0
impl crate::Resettable for EIDR_SPEC {}
