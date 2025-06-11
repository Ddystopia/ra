///Register `CTSUCHAC4` reader
pub type R = crate::R<CTSUCHAC4_SPEC>;
///Register `CTSUCHAC4` writer
pub type W = crate::W<CTSUCHAC4_SPEC>;
/**CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\[0\] corresponds to TS32 and CTSUCHAC4\[3\] corresponds to TS35. but the write value of CTSUCHAC0\[4\],CTSUCHAC4\[5\],CTSUCHAC4\[6\],CTSUCHAC4\[7\] should be 0.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC4_A {
    ///0: TS pin which correspond to the bit number of CTSUCHAC4 register set whether the measurement target.
    CTSUCHAC4 = 0,
}
impl From<CTSUCHAC4_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC4_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHAC4_A {}
///Field `CTSUCHAC4` reader - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\[0\] corresponds to TS32 and CTSUCHAC4\[3\] corresponds to TS35. but the write value of CTSUCHAC0\[4\],CTSUCHAC4\[5\],CTSUCHAC4\[6\],CTSUCHAC4\[7\] should be 0.
pub type CTSUCHAC4_R = crate::FieldReader<CTSUCHAC4_A>;
impl CTSUCHAC4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCHAC4_A {
        match self.bits {
            _ => CTSUCHAC4_A::CTSUCHAC4,
        }
    }
    ///TS pin which correspond to the bit number of CTSUCHAC4 register set whether the measurement target.
    #[inline(always)]
    pub fn is_ctsuchac4(&self) -> bool {
        matches!(self.variant(), CTSUCHAC4_A::CTSUCHAC4)
    }
}
///Field `CTSUCHAC4` writer - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\[0\] corresponds to TS32 and CTSUCHAC4\[3\] corresponds to TS35. but the write value of CTSUCHAC0\[4\],CTSUCHAC4\[5\],CTSUCHAC4\[6\],CTSUCHAC4\[7\] should be 0.
pub type CTSUCHAC4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CTSUCHAC4_A, crate::Safe>;
impl<'a, REG> CTSUCHAC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TS pin which correspond to the bit number of CTSUCHAC4 register set whether the measurement target.
    #[inline(always)]
    pub fn ctsuchac4(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC4_A::CTSUCHAC4)
    }
}
impl R {
    ///Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\[0\] corresponds to TS32 and CTSUCHAC4\[3\] corresponds to TS35. but the write value of CTSUCHAC0\[4\],CTSUCHAC4\[5\],CTSUCHAC4\[6\],CTSUCHAC4\[7\] should be 0.
    #[inline(always)]
    pub fn ctsuchac4(&self) -> CTSUCHAC4_R {
        CTSUCHAC4_R::new(self.bits & 0x0f)
    }
}
impl W {
    ///Bits 0:3 - CTSU Channel Enable Control 4. 0: Not measurement target 1: Measurement target Note: CTSUCHAC4\[0\] corresponds to TS32 and CTSUCHAC4\[3\] corresponds to TS35. but the write value of CTSUCHAC0\[4\],CTSUCHAC4\[5\],CTSUCHAC4\[6\],CTSUCHAC4\[7\] should be 0.
    #[inline(always)]
    pub fn ctsuchac4(&mut self) -> CTSUCHAC4_W<CTSUCHAC4_SPEC> {
        CTSUCHAC4_W::new(self, 0)
    }
}
/**CTSU Channel Enable Control Register 4

You can [`read`](crate::Reg::read) this register and get [`ctsuchac4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHAC4_SPEC;
impl crate::RegisterSpec for CTSUCHAC4_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac4::R`](R) reader structure
impl crate::Readable for CTSUCHAC4_SPEC {}
///`write(|w| ..)` method takes [`ctsuchac4::W`](W) writer structure
impl crate::Writable for CTSUCHAC4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC4 to value 0
impl crate::Resettable for CTSUCHAC4_SPEC {}
