///Register `PCNTR2` reader
pub type R = crate::R<PCNTR2_SPEC>;
/**Pmn Input Data

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PIDR_A {
    ///0: Low input
    _0 = 0,
    ///1: High input.
    _1 = 1,
}
impl From<PIDR_A> for u16 {
    #[inline(always)]
    fn from(variant: PIDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIDR_A {
    type Ux = u16;
}
impl crate::IsEnum for PIDR_A {}
///Field `PIDR` reader - Pmn Input Data
pub type PIDR_R = crate::FieldReader<PIDR_A>;
impl PIDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIDR_A> {
        match self.bits {
            0 => Some(PIDR_A::_0),
            1 => Some(PIDR_A::_1),
            _ => None,
        }
    }
    ///Low input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR_A::_0
    }
    ///High input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR_A::_1
    }
}
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
    ///Bits 0:15 - Pmn Input Data
    #[inline(always)]
    pub fn pidr(&self) -> PIDR_R {
        PIDR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Pmn Event Input Data
    #[inline(always)]
    pub fn eidr(&self) -> EIDR_R {
        EIDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
/**Port Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pcntr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCNTR2_SPEC;
impl crate::RegisterSpec for PCNTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcntr2::R`](R) reader structure
impl crate::Readable for PCNTR2_SPEC {}
///`reset()` method sets PCNTR2 to value 0
impl crate::Resettable for PCNTR2_SPEC {}
