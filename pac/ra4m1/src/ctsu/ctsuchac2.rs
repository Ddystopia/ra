///Register `CTSUCHAC2` reader
pub type R = crate::R<CTSUCHAC2_SPEC>;
///Register `CTSUCHAC2` writer
pub type W = crate::W<CTSUCHAC2_SPEC>;
/**CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\[0\] corresponds to TS16 and CTSUCHAC2\[7\] corresponds to TS23.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC2_A {
    ///0: TS pin which correspond to the bit number of CTSUCHAC2 register set whether the measurement target.
    CTSUCHAC2 = 0,
}
impl From<CTSUCHAC2_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC2_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHAC2_A {}
///Field `CTSUCHAC2` reader - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\[0\] corresponds to TS16 and CTSUCHAC2\[7\] corresponds to TS23.
pub type CTSUCHAC2_R = crate::FieldReader<CTSUCHAC2_A>;
impl CTSUCHAC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCHAC2_A {
        match self.bits {
            _ => CTSUCHAC2_A::CTSUCHAC2,
        }
    }
    ///TS pin which correspond to the bit number of CTSUCHAC2 register set whether the measurement target.
    #[inline(always)]
    pub fn is_ctsuchac2(&self) -> bool {
        matches!(self.variant(), CTSUCHAC2_A::CTSUCHAC2)
    }
}
///Field `CTSUCHAC2` writer - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\[0\] corresponds to TS16 and CTSUCHAC2\[7\] corresponds to TS23.
pub type CTSUCHAC2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHAC2_A, crate::Safe>;
impl<'a, REG> CTSUCHAC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TS pin which correspond to the bit number of CTSUCHAC2 register set whether the measurement target.
    #[inline(always)]
    pub fn ctsuchac2(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC2_A::CTSUCHAC2)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\[0\] corresponds to TS16 and CTSUCHAC2\[7\] corresponds to TS23.
    #[inline(always)]
    pub fn ctsuchac2(&self) -> CTSUCHAC2_R {
        CTSUCHAC2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Enable Control 2. 0: Not measurement target 1: Measurement target Note: CTSUCHAC2\[0\] corresponds to TS16 and CTSUCHAC2\[7\] corresponds to TS23.
    #[inline(always)]
    pub fn ctsuchac2(&mut self) -> CTSUCHAC2_W<CTSUCHAC2_SPEC> {
        CTSUCHAC2_W::new(self, 0)
    }
}
/**CTSU Channel Enable Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchac2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHAC2_SPEC;
impl crate::RegisterSpec for CTSUCHAC2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac2::R`](R) reader structure
impl crate::Readable for CTSUCHAC2_SPEC {}
///`write(|w| ..)` method takes [`ctsuchac2::W`](W) writer structure
impl crate::Writable for CTSUCHAC2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC2 to value 0
impl crate::Resettable for CTSUCHAC2_SPEC {}
