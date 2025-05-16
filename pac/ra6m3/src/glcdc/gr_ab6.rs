///Register `GR%s_AB6` reader
pub type R = crate::R<GR_AB6_SPEC>;
///Register `GR%s_AB6` writer
pub type W = crate::W<GR_AB6_SPEC>;
/**Frame rate for alpha blending in rectangular area.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARCRATE_A {
    ///0: ARCRATE+1 frames
    ARCRATE = 0,
}
impl From<ARCRATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCRATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCRATE_A {
    type Ux = u8;
}
impl crate::IsEnum for ARCRATE_A {}
///Field `ARCRATE` reader - Frame rate for alpha blending in rectangular area.
pub type ARCRATE_R = crate::FieldReader<ARCRATE_A>;
impl ARCRATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARCRATE_A {
        match self.bits {
            _ => ARCRATE_A::ARCRATE,
        }
    }
    ///ARCRATE+1 frames
    #[inline(always)]
    pub fn is_arcrate(&self) -> bool {
        matches!(self.variant(), ARCRATE_A::ARCRATE)
    }
}
///Field `ARCRATE` writer - Frame rate for alpha blending in rectangular area.
pub type ARCRATE_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ARCRATE_A, crate::Safe>;
impl<'a, REG> ARCRATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ARCRATE+1 frames
    #[inline(always)]
    pub fn arcrate(self) -> &'a mut crate::W<REG> {
        self.variant(ARCRATE_A::ARCRATE)
    }
}
///Field `ARCCOEF` reader - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\[8\]: Sign (0: addition, 1: subtraction)\[7:0\]: Variation (absolute value)
pub type ARCCOEF_R = crate::FieldReader<u16>;
///Field `ARCCOEF` writer - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\[8\]: Sign (0: addition, 1: subtraction)\[7:0\]: Variation (absolute value)
pub type ARCCOEF_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:7 - Frame rate for alpha blending in rectangular area.
    #[inline(always)]
    pub fn arcrate(&self) -> ARCRATE_R {
        ARCRATE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:24 - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\[8\]: Sign (0: addition, 1: subtraction)\[7:0\]: Variation (absolute value)
    #[inline(always)]
    pub fn arccoef(&self) -> ARCCOEF_R {
        ARCCOEF_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:7 - Frame rate for alpha blending in rectangular area.
    #[inline(always)]
    pub fn arcrate(&mut self) -> ARCRATE_W<GR_AB6_SPEC> {
        ARCRATE_W::new(self, 0)
    }
    ///Bits 16:24 - Alpha coefficient for alpha blending in rectangular area (-255 to 255).\[8\]: Sign (0: addition, 1: subtraction)\[7:0\]: Variation (absolute value)
    #[inline(always)]
    pub fn arccoef(&mut self) -> ARCCOEF_W<GR_AB6_SPEC> {
        ARCCOEF_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 6

You can [`read`](crate::Reg::read) this register and get [`gr_ab6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB6_SPEC;
impl crate::RegisterSpec for GR_AB6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab6::R`](R) reader structure
impl crate::Readable for GR_AB6_SPEC {}
///`write(|w| ..)` method takes [`gr_ab6::W`](W) writer structure
impl crate::Writable for GR_AB6_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB6 to value 0
impl crate::Resettable for GR_AB6_SPEC {}
