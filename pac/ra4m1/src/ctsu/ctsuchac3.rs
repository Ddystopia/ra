///Register `CTSUCHAC3` reader
pub type R = crate::R<CTSUCHAC3_SPEC>;
///Register `CTSUCHAC3` writer
pub type W = crate::W<CTSUCHAC3_SPEC>;
/**CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\[0\] corresponds to TS24 and CTSUCHAC3\[7\] corresponds to TS31.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC3_A {
    ///0: TS pin which correspond to the bit number of CTSUCHAC3 register set whether the measurement target.
    CTSUCHAC3 = 0,
}
impl From<CTSUCHAC3_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC3_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUCHAC3_A {}
///Field `CTSUCHAC3` reader - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\[0\] corresponds to TS24 and CTSUCHAC3\[7\] corresponds to TS31.
pub type CTSUCHAC3_R = crate::FieldReader<CTSUCHAC3_A>;
impl CTSUCHAC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCHAC3_A {
        match self.bits {
            _ => CTSUCHAC3_A::CTSUCHAC3,
        }
    }
    ///TS pin which correspond to the bit number of CTSUCHAC3 register set whether the measurement target.
    #[inline(always)]
    pub fn is_ctsuchac3(&self) -> bool {
        matches!(self.variant(), CTSUCHAC3_A::CTSUCHAC3)
    }
}
///Field `CTSUCHAC3` writer - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\[0\] corresponds to TS24 and CTSUCHAC3\[7\] corresponds to TS31.
pub type CTSUCHAC3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CTSUCHAC3_A, crate::Safe>;
impl<'a, REG> CTSUCHAC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TS pin which correspond to the bit number of CTSUCHAC3 register set whether the measurement target.
    #[inline(always)]
    pub fn ctsuchac3(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC3_A::CTSUCHAC3)
    }
}
impl R {
    ///Bits 0:7 - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\[0\] corresponds to TS24 and CTSUCHAC3\[7\] corresponds to TS31.
    #[inline(always)]
    pub fn ctsuchac3(&self) -> CTSUCHAC3_R {
        CTSUCHAC3_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - CTSU Channel Enable Control 3. 0: Not measurement target 1: Measurement target Note: CTSUCHAC3\[0\] corresponds to TS24 and CTSUCHAC3\[7\] corresponds to TS31.
    #[inline(always)]
    pub fn ctsuchac3(&mut self) -> CTSUCHAC3_W<CTSUCHAC3_SPEC> {
        CTSUCHAC3_W::new(self, 0)
    }
}
/**CTSU Channel Enable Control Register 3

You can [`read`](crate::Reg::read) this register and get [`ctsuchac3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHAC3_SPEC;
impl crate::RegisterSpec for CTSUCHAC3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchac3::R`](R) reader structure
impl crate::Readable for CTSUCHAC3_SPEC {}
///`write(|w| ..)` method takes [`ctsuchac3::W`](W) writer structure
impl crate::Writable for CTSUCHAC3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHAC3 to value 0
impl crate::Resettable for CTSUCHAC3_SPEC {}
