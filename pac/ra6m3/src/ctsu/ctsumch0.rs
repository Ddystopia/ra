///Register `CTSUMCH0` reader
pub type R = crate::R<CTSUMCH0_SPEC>;
///Register `CTSUMCH0` writer
pub type W = crate::W<CTSUMCH0_SPEC>;
/**CTSU Measurement Channel 0.Note1: Writing to these bits is only enabled in self-capacitance single scan mode (CTSUCR1.CTSUMD\[1:0\] bits = 00b).Note2: If the value of CTSUMCH0 was set to b'11111 in mode other than self-capacitor single scan mode, the measurement is stopped.

Value on reset: 31*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUMCH0_A {
    ///0: The value of CTSUMCH0 indicate to channel to be measured.
    CTSUMCH0 = 0,
}
impl From<CTSUMCH0_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUMCH0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUMCH0_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUMCH0_A {}
///Field `CTSUMCH0` reader - CTSU Measurement Channel 0.Note1: Writing to these bits is only enabled in self-capacitance single scan mode (CTSUCR1.CTSUMD\[1:0\] bits = 00b).Note2: If the value of CTSUMCH0 was set to b'11111 in mode other than self-capacitor single scan mode, the measurement is stopped.
pub type CTSUMCH0_R = crate::FieldReader<CTSUMCH0_A>;
impl CTSUMCH0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUMCH0_A {
        match self.bits {
            _ => CTSUMCH0_A::CTSUMCH0,
        }
    }
    ///The value of CTSUMCH0 indicate to channel to be measured.
    #[inline(always)]
    pub fn is_ctsumch0(&self) -> bool {
        matches!(self.variant(), CTSUMCH0_A::CTSUMCH0)
    }
}
///Field `CTSUMCH0` writer - CTSU Measurement Channel 0.Note1: Writing to these bits is only enabled in self-capacitance single scan mode (CTSUCR1.CTSUMD\[1:0\] bits = 00b).Note2: If the value of CTSUMCH0 was set to b'11111 in mode other than self-capacitor single scan mode, the measurement is stopped.
pub type CTSUMCH0_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CTSUMCH0_A, crate::Safe>;
impl<'a, REG> CTSUMCH0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The value of CTSUMCH0 indicate to channel to be measured.
    #[inline(always)]
    pub fn ctsumch0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUMCH0_A::CTSUMCH0)
    }
}
impl R {
    ///Bits 0:4 - CTSU Measurement Channel 0.Note1: Writing to these bits is only enabled in self-capacitance single scan mode (CTSUCR1.CTSUMD\[1:0\] bits = 00b).Note2: If the value of CTSUMCH0 was set to b'11111 in mode other than self-capacitor single scan mode, the measurement is stopped.
    #[inline(always)]
    pub fn ctsumch0(&self) -> CTSUMCH0_R {
        CTSUMCH0_R::new(self.bits & 0x1f)
    }
}
impl W {
    ///Bits 0:4 - CTSU Measurement Channel 0.Note1: Writing to these bits is only enabled in self-capacitance single scan mode (CTSUCR1.CTSUMD\[1:0\] bits = 00b).Note2: If the value of CTSUMCH0 was set to b'11111 in mode other than self-capacitor single scan mode, the measurement is stopped.
    #[inline(always)]
    pub fn ctsumch0(&mut self) -> CTSUMCH0_W<CTSUMCH0_SPEC> {
        CTSUMCH0_W::new(self, 0)
    }
}
/**CTSU Measurement Channel Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsumch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUMCH0_SPEC;
impl crate::RegisterSpec for CTSUMCH0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsumch0::R`](R) reader structure
impl crate::Readable for CTSUMCH0_SPEC {}
///`write(|w| ..)` method takes [`ctsumch0::W`](W) writer structure
impl crate::Writable for CTSUMCH0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUMCH0 to value 0x1f
impl crate::Resettable for CTSUMCH0_SPEC {
    const RESET_VALUE: u8 = 0x1f;
}
