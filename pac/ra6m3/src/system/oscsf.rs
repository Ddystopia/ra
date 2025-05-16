///Register `OSCSF` reader
pub type R = crate::R<OSCSF_SPEC>;
/**HOCO Clock Oscillation Stabilization FlagNOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOCOSF_A {
    ///0: HOCO clock is stopped or is not yet stable
    _0 = 0,
    ///1: HOCO clock is stable, so is available for use as the system clock
    _1 = 1,
}
impl From<HOCOSF_A> for bool {
    #[inline(always)]
    fn from(variant: HOCOSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HOCOSF` reader - HOCO Clock Oscillation Stabilization FlagNOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1.
pub type HOCOSF_R = crate::BitReader<HOCOSF_A>;
impl HOCOSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HOCOSF_A {
        match self.bits {
            false => HOCOSF_A::_0,
            true => HOCOSF_A::_1,
        }
    }
    ///HOCO clock is stopped or is not yet stable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOCOSF_A::_0
    }
    ///HOCO clock is stable, so is available for use as the system clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOCOSF_A::_1
    }
}
/**Main Clock Oscillation Stabilization Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSCSF_A {
    ///0: Main clock oscillator is stopped (MOSTP = 1) or is not yet stable
    _0 = 0,
    ///1: Main clock oscillator is stable, so is available for use as the system clock
    _1 = 1,
}
impl From<MOSCSF_A> for bool {
    #[inline(always)]
    fn from(variant: MOSCSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MOSCSF` reader - Main Clock Oscillation Stabilization Flag
pub type MOSCSF_R = crate::BitReader<MOSCSF_A>;
impl MOSCSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOSCSF_A {
        match self.bits {
            false => MOSCSF_A::_0,
            true => MOSCSF_A::_1,
        }
    }
    ///Main clock oscillator is stopped (MOSTP = 1) or is not yet stable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSCSF_A::_0
    }
    ///Main clock oscillator is stable, so is available for use as the system clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSCSF_A::_1
    }
}
/**PLL Clock Oscillation Stabilization Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSF_A {
    ///0: PLL clock is stopped or is not yet stable
    _0 = 0,
    ///1: PLL clock is stable, so is available for use as the system clock
    _1 = 1,
}
impl From<PLLSF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSF` reader - PLL Clock Oscillation Stabilization Flag
pub type PLLSF_R = crate::BitReader<PLLSF_A>;
impl PLLSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSF_A {
        match self.bits {
            false => PLLSF_A::_0,
            true => PLLSF_A::_1,
        }
    }
    ///PLL clock is stopped or is not yet stable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSF_A::_0
    }
    ///PLL clock is stable, so is available for use as the system clock
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSF_A::_1
    }
}
impl R {
    ///Bit 0 - HOCO Clock Oscillation Stabilization FlagNOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1.
    #[inline(always)]
    pub fn hocosf(&self) -> HOCOSF_R {
        HOCOSF_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - Main Clock Oscillation Stabilization Flag
    #[inline(always)]
    pub fn moscsf(&self) -> MOSCSF_R {
        MOSCSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - PLL Clock Oscillation Stabilization Flag
    #[inline(always)]
    pub fn pllsf(&self) -> PLLSF_R {
        PLLSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
/**Oscillation Stabilization Flag Register

You can [`read`](crate::Reg::read) this register and get [`oscsf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OSCSF_SPEC;
impl crate::RegisterSpec for OSCSF_SPEC {
    type Ux = u8;
}
///`read()` method returns [`oscsf::R`](R) reader structure
impl crate::Readable for OSCSF_SPEC {}
///`reset()` method sets OSCSF to value 0
impl crate::Resettable for OSCSF_SPEC {}
