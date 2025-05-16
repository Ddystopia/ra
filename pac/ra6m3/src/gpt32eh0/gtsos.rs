///Register `GTSOS` reader
pub type R = crate::R<GTSOS_SPEC>;
/**Output Protection Function Status

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOS_A {
    ///0: Normal operation
    _00 = 0,
    ///1: Protected state (GTCCRA = 0 is set during transfer at trough or crest)
    _01 = 1,
    ///2: Protected state (GTCCRA >= GTPR is set during transfer at trough)
    _10 = 2,
    ///3: Protected state (GTCCRA >= GTPR is set during transfer at crest)
    _11 = 3,
}
impl From<SOS_A> for u8 {
    #[inline(always)]
    fn from(variant: SOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SOS_A {
    type Ux = u8;
}
impl crate::IsEnum for SOS_A {}
///Field `SOS` reader - Output Protection Function Status
pub type SOS_R = crate::FieldReader<SOS_A>;
impl SOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOS_A {
        match self.bits {
            0 => SOS_A::_00,
            1 => SOS_A::_01,
            2 => SOS_A::_10,
            3 => SOS_A::_11,
            _ => unreachable!(),
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SOS_A::_00
    }
    ///Protected state (GTCCRA = 0 is set during transfer at trough or crest)
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SOS_A::_01
    }
    ///Protected state (GTCCRA >= GTPR is set during transfer at trough)
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SOS_A::_10
    }
    ///Protected state (GTCCRA >= GTPR is set during transfer at crest)
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SOS_A::_11
    }
}
impl R {
    ///Bits 0:1 - Output Protection Function Status
    #[inline(always)]
    pub fn sos(&self) -> SOS_R {
        SOS_R::new((self.bits & 3) as u8)
    }
}
/**General PWM Timer Output Protection Function Status Register

You can [`read`](crate::Reg::read) this register and get [`gtsos::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTSOS_SPEC;
impl crate::RegisterSpec for GTSOS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtsos::R`](R) reader structure
impl crate::Readable for GTSOS_SPEC {}
///`reset()` method sets GTSOS to value 0
impl crate::Resettable for GTSOS_SPEC {}
