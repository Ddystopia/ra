///Register `CTSUCHAC0` reader
pub type R = crate::R<CTSUCHAC0_SPEC>;
///Register `CTSUCHAC0` writer
pub type W = crate::W<CTSUCHAC0_SPEC>;
/**CTSU Channel Enable Control 0.0: Not measurement target1: Measurement targetNote: CTSUCHAC0\[0\] corresponds to TS00 and CTSUCHAC0\[7\] corresponds to TS07.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC0_A {
    ///0: TS pin which correspond to the bit number of CTSUCHAC0 register set whether the measurement target.
    CTSUCHAC0 = 0,
}
impl From<CTSUCHAC0_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC0_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHAC0_A {}
///Field `CTSUCHAC0` reader - CTSU Channel Enable Control 0.0: Not measurement target1: Measurement targetNote: CTSUCHAC0\[0\] corresponds to TS00 and CTSUCHAC0\[7\] corresponds to TS07.
pub type CTSUCHAC0_R = crate::FieldReader<CTSUCHAC0_A>;
impl CTSUCHAC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCHAC0_A {
        match self.bits {
            _ => CTSUCHAC0_A::CTSUCHAC0,
        }
    }
    ///TS pin which correspond to the bit number of CTSUCHAC0 register set whether the measurement target.
    #[inline(always)]
    pub fn is_ctsuchac0(&self) -> bool {
        matches!(self.variant(), CTSUCHAC0_A::CTSUCHAC0)
    }
}
///Field `CTSUCHAC0` writer - CTSU Channel Enable Control 0.0: Not measurement target1: Measurement targetNote: CTSUCHAC0\[0\] corresponds to TS00 and CTSUCHAC0\[7\] corresponds to TS07.
pub type CTSUCHAC0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHAC0_A, crate::Safe>;
impl<'a, REG> CTSUCHAC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TS pin which correspond to the bit number of CTSUCHAC0 register set whether the measurement target.
    #[inline(always)]
    pub fn ctsuchac0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC0_A::CTSUCHAC0)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Enable Control 0.0: Not measurement target1: Measurement targetNote: CTSUCHAC0\[0\] corresponds to TS00 and CTSUCHAC0\[7\] corresponds to TS07.
    #[inline(always)]
    pub fn ctsuchac0(&self) -> CTSUCHAC0_R {
        CTSUCHAC0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Enable Control 0.0: Not measurement target1: Measurement targetNote: CTSUCHAC0\[0\] corresponds to TS00 and CTSUCHAC0\[7\] corresponds to TS07.
    #[inline(always)]
    pub fn ctsuchac0(&mut self) -> CTSUCHAC0_W<CTSUCHAC0_SPEC> {
        CTSUCHAC0_W::new(self, 0)
    }
}
/**CTSU Channel Enable Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchac0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHAC0_SPEC;
impl crate::RegisterSpec for CTSUCHAC0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac0::R`](R) reader structure
impl crate::Readable for CTSUCHAC0_SPEC {}
///`write(|w| ..)` method takes [`ctsuchac0::W`](W) writer structure
impl crate::Writable for CTSUCHAC0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC0 to value 0
impl crate::Resettable for CTSUCHAC0_SPEC {}
