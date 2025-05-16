///Register `CTSUMCH1` reader
pub type R = crate::R<CTSUMCH1_SPEC>;
/**CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'11111, the measurement is stopped.

Value on reset: 31*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUMCH1_A {
    ///0: The value of CTSUMCH1 indicate to channel to be measured.
    CTSUMCH1 = 0,
}
impl From<CTSUMCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUMCH1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUMCH1_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUMCH1_A {}
///Field `CTSUMCH1` reader - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'11111, the measurement is stopped.
pub type CTSUMCH1_R = crate::FieldReader<CTSUMCH1_A>;
impl CTSUMCH1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUMCH1_A {
        match self.bits {
            _ => CTSUMCH1_A::CTSUMCH1,
        }
    }
    ///The value of CTSUMCH1 indicate to channel to be measured.
    #[inline(always)]
    pub fn is_ctsumch1(&self) -> bool {
        matches!(self.variant(), CTSUMCH1_A::CTSUMCH1)
    }
}
impl R {
    ///Bits 0:4 - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'11111, the measurement is stopped.
    #[inline(always)]
    pub fn ctsumch1(&self) -> CTSUMCH1_R {
        CTSUMCH1_R::new(self.bits & 0x1f)
    }
}
/**CTSU Measurement Channel Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsumch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUMCH1_SPEC;
impl crate::RegisterSpec for CTSUMCH1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsumch1::R`](R) reader structure
impl crate::Readable for CTSUMCH1_SPEC {}
///`reset()` method sets CTSUMCH1 to value 0x1f
impl crate::Resettable for CTSUMCH1_SPEC {
    const RESET_VALUE: u8 = 0x1f;
}
