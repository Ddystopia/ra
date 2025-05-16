///Register `CASTR` reader
pub type R = crate::R<CASTR_SPEC>;
/**Frequency Error Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FERRF_A {
    ///0: The clock frequency is within the range corresponding to the settings.
    _0 = 0,
    ///1: The clock frequency has deviated beyond the range corresponding to the settings (frequency error).
    _1 = 1,
}
impl From<FERRF_A> for bool {
    #[inline(always)]
    fn from(variant: FERRF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FERRF` reader - Frequency Error Flag
pub type FERRF_R = crate::BitReader<FERRF_A>;
impl FERRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FERRF_A {
        match self.bits {
            false => FERRF_A::_0,
            true => FERRF_A::_1,
        }
    }
    ///The clock frequency is within the range corresponding to the settings.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FERRF_A::_0
    }
    ///The clock frequency has deviated beyond the range corresponding to the settings (frequency error).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FERRF_A::_1
    }
}
/**Measurement End Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MENDF_A {
    ///0: Measurement is in progress.
    _0 = 0,
    ///1: Measurement has ended.
    _1 = 1,
}
impl From<MENDF_A> for bool {
    #[inline(always)]
    fn from(variant: MENDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MENDF` reader - Measurement End Flag
pub type MENDF_R = crate::BitReader<MENDF_A>;
impl MENDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MENDF_A {
        match self.bits {
            false => MENDF_A::_0,
            true => MENDF_A::_1,
        }
    }
    ///Measurement is in progress.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MENDF_A::_0
    }
    ///Measurement has ended.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MENDF_A::_1
    }
}
/**Counter Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFF_A {
    ///0: The counter has not overflowed.
    _0 = 0,
    ///1: The counter has overflowed.
    _1 = 1,
}
impl From<OVFF_A> for bool {
    #[inline(always)]
    fn from(variant: OVFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVFF` reader - Counter Overflow Flag
pub type OVFF_R = crate::BitReader<OVFF_A>;
impl OVFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVFF_A {
        match self.bits {
            false => OVFF_A::_0,
            true => OVFF_A::_1,
        }
    }
    ///The counter has not overflowed.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFF_A::_0
    }
    ///The counter has overflowed.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFF_A::_1
    }
}
impl R {
    ///Bit 0 - Frequency Error Flag
    #[inline(always)]
    pub fn ferrf(&self) -> FERRF_R {
        FERRF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Measurement End Flag
    #[inline(always)]
    pub fn mendf(&self) -> MENDF_R {
        MENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Counter Overflow Flag
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
/**CAC Status Register

You can [`read`](crate::Reg::read) this register and get [`castr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CASTR_SPEC;
impl crate::RegisterSpec for CASTR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`castr::R`](R) reader structure
impl crate::Readable for CASTR_SPEC {}
///`reset()` method sets CASTR to value 0
impl crate::Resettable for CASTR_SPEC {}
