///Register `CTSUMCH1` reader
pub type R = crate::R<CTSUMCH1_SPEC>;
///Register `CTSUMCH1` writer
pub type W = crate::W<CTSUMCH1_SPEC>;
/**CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped.

Value on reset: 63*/
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
///Field `CTSUMCH1` reader - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped.
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
    ///Bits 0:5 - CTSU Measurement Channel 1 Note1: If the value of CTSUMCH1 was set to b'111111, the measurement is stopped.
    #[inline(always)]
    pub fn ctsumch1(&self) -> CTSUMCH1_R {
        CTSUMCH1_R::new(self.bits & 0x3f)
    }
}
impl W {}
/**CTSU Measurement Channel Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsumch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUMCH1_SPEC;
impl crate::RegisterSpec for CTSUMCH1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsumch1::R`](R) reader structure
impl crate::Readable for CTSUMCH1_SPEC {}
///`write(|w| ..)` method takes [`ctsumch1::W`](W) writer structure
impl crate::Writable for CTSUMCH1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUMCH1 to value 0x3f
impl crate::Resettable for CTSUMCH1_SPEC {
    const RESET_VALUE: u8 = 0x3f;
}
