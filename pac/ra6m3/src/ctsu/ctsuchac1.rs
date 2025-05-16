///Register `CTSUCHAC1` reader
pub type R = crate::R<CTSUCHAC1_SPEC>;
///Register `CTSUCHAC1` writer
pub type W = crate::W<CTSUCHAC1_SPEC>;
/**CTSU Channel Enable Control 1.0: Not measurement target1: Measurement targetNote: CTSUCHAC1\[0\] corresponds to TS08 and CTSUCHAC1\[7\] corresponds to TS15.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC1_A {
    ///0: TS pin which correspond to the bit number of CTSUCHAC1 register set whether the measurement target.
    CTSUCHAC1 = 0,
}
impl From<CTSUCHAC1_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC1_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHAC1_A {}
///Field `CTSUCHAC1` reader - CTSU Channel Enable Control 1.0: Not measurement target1: Measurement targetNote: CTSUCHAC1\[0\] corresponds to TS08 and CTSUCHAC1\[7\] corresponds to TS15.
pub type CTSUCHAC1_R = crate::FieldReader<CTSUCHAC1_A>;
impl CTSUCHAC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCHAC1_A {
        match self.bits {
            _ => CTSUCHAC1_A::CTSUCHAC1,
        }
    }
    ///TS pin which correspond to the bit number of CTSUCHAC1 register set whether the measurement target.
    #[inline(always)]
    pub fn is_ctsuchac1(&self) -> bool {
        matches!(self.variant(), CTSUCHAC1_A::CTSUCHAC1)
    }
}
///Field `CTSUCHAC1` writer - CTSU Channel Enable Control 1.0: Not measurement target1: Measurement targetNote: CTSUCHAC1\[0\] corresponds to TS08 and CTSUCHAC1\[7\] corresponds to TS15.
pub type CTSUCHAC1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHAC1_A, crate::Safe>;
impl<'a, REG> CTSUCHAC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TS pin which correspond to the bit number of CTSUCHAC1 register set whether the measurement target.
    #[inline(always)]
    pub fn ctsuchac1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC1_A::CTSUCHAC1)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Enable Control 1.0: Not measurement target1: Measurement targetNote: CTSUCHAC1\[0\] corresponds to TS08 and CTSUCHAC1\[7\] corresponds to TS15.
    #[inline(always)]
    pub fn ctsuchac1(&self) -> CTSUCHAC1_R {
        CTSUCHAC1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Enable Control 1.0: Not measurement target1: Measurement targetNote: CTSUCHAC1\[0\] corresponds to TS08 and CTSUCHAC1\[7\] corresponds to TS15.
    #[inline(always)]
    pub fn ctsuchac1(&mut self) -> CTSUCHAC1_W<CTSUCHAC1_SPEC> {
        CTSUCHAC1_W::new(self, 0)
    }
}
/**CTSU Channel Enable Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchac1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHAC1_SPEC;
impl crate::RegisterSpec for CTSUCHAC1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac1::R`](R) reader structure
impl crate::Readable for CTSUCHAC1_SPEC {}
///`write(|w| ..)` method takes [`ctsuchac1::W`](W) writer structure
impl crate::Writable for CTSUCHAC1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC1 to value 0
impl crate::Resettable for CTSUCHAC1_SPEC {}
