///Register `ADRD` reader
pub type R = crate::R<ADRD_SPEC>;
///Field `AD` reader - A/D-converted value (right-justified)NOTE: Unused bits in the AD bit field are fixed "0"
pub type AD_R = crate::FieldReader<u16>;
/**Self-Diagnosis Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIAGST_A {
    ///0: Self-diagnosis has never been executed since power-on.
    _00 = 0,
    ///1: Self-diagnosis using the voltage of 0 V has been executed.
    _01 = 1,
    ///2: Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed.
    _10 = 2,
    ///3: Self-diagnosis using the voltage of reference power supply(VREFH) has been executed.
    _11 = 3,
}
impl From<DIAGST_A> for u8 {
    #[inline(always)]
    fn from(variant: DIAGST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIAGST_A {
    type Ux = u8;
}
impl crate::IsEnum for DIAGST_A {}
///Field `DIAGST` reader - Self-Diagnosis Status
pub type DIAGST_R = crate::FieldReader<DIAGST_A>;
impl DIAGST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIAGST_A {
        match self.bits {
            0 => DIAGST_A::_00,
            1 => DIAGST_A::_01,
            2 => DIAGST_A::_10,
            3 => DIAGST_A::_11,
            _ => unreachable!(),
        }
    }
    ///Self-diagnosis has never been executed since power-on.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DIAGST_A::_00
    }
    ///Self-diagnosis using the voltage of 0 V has been executed.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DIAGST_A::_01
    }
    ///Self-diagnosis using the voltage of reference power supply(VREFH) x 1/2 has been executed.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DIAGST_A::_10
    }
    ///Self-diagnosis using the voltage of reference power supply(VREFH) has been executed.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DIAGST_A::_11
    }
}
impl R {
    ///Bits 0:11 - A/D-converted value (right-justified)NOTE: Unused bits in the AD bit field are fixed "0"
    #[inline(always)]
    pub fn ad(&self) -> AD_R {
        AD_R::new(self.bits & 0x0fff)
    }
    ///Bits 14:15 - Self-Diagnosis Status
    #[inline(always)]
    pub fn diagst(&self) -> DIAGST_R {
        DIAGST_R::new(((self.bits >> 14) & 3) as u8)
    }
}
/**A/D Self-Diagnosis Data Register

You can [`read`](crate::Reg::read) this register and get [`adrd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADRD_SPEC;
impl crate::RegisterSpec for ADRD_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adrd::R`](R) reader structure
impl crate::Readable for ADRD_SPEC {}
///`reset()` method sets ADRD to value 0
impl crate::Resettable for ADRD_SPEC {}
