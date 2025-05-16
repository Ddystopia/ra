///Register `GR%s_AB7` reader
pub type R = crate::R<GR_AB7_SPEC>;
///Register `GR%s_AB7` writer
pub type W = crate::W<GR_AB7_SPEC>;
/**RGB-index chroma-key processing control.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKON_A {
    ///1: Enables chroma-key processing
    _1 = 1,
    ///0: Disables chroma-key processing
    _0 = 0,
}
impl From<CKON_A> for bool {
    #[inline(always)]
    fn from(variant: CKON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKON` reader - RGB-index chroma-key processing control.
pub type CKON_R = crate::BitReader<CKON_A>;
impl CKON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKON_A {
        match self.bits {
            true => CKON_A::_1,
            false => CKON_A::_0,
        }
    }
    ///Enables chroma-key processing
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CKON_A::_1
    }
    ///Disables chroma-key processing
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CKON_A::_0
    }
}
///Field `CKON` writer - RGB-index chroma-key processing control.
pub type CKON_W<'a, REG> = crate::BitWriter<'a, REG, CKON_A>;
impl<'a, REG> CKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enables chroma-key processing
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CKON_A::_1)
    }
    ///Disables chroma-key processing
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CKON_A::_0)
    }
}
///Field `ARCDEF` reader - Initial alpha value for alpha blending in rectangular area.
pub type ARCDEF_R = crate::FieldReader;
///Field `ARCDEF` writer - Initial alpha value for alpha blending in rectangular area.
pub type ARCDEF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - RGB-index chroma-key processing control.
    #[inline(always)]
    pub fn ckon(&self) -> CKON_R {
        CKON_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:23 - Initial alpha value for alpha blending in rectangular area.
    #[inline(always)]
    pub fn arcdef(&self) -> ARCDEF_R {
        ARCDEF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - RGB-index chroma-key processing control.
    #[inline(always)]
    pub fn ckon(&mut self) -> CKON_W<GR_AB7_SPEC> {
        CKON_W::new(self, 0)
    }
    ///Bits 16:23 - Initial alpha value for alpha blending in rectangular area.
    #[inline(always)]
    pub fn arcdef(&mut self) -> ARCDEF_W<GR_AB7_SPEC> {
        ARCDEF_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 7

You can [`read`](crate::Reg::read) this register and get [`gr_ab7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB7_SPEC;
impl crate::RegisterSpec for GR_AB7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab7::R`](R) reader structure
impl crate::Readable for GR_AB7_SPEC {}
///`write(|w| ..)` method takes [`gr_ab7::W`](W) writer structure
impl crate::Writable for GR_AB7_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB7 to value 0
impl crate::Resettable for GR_AB7_SPEC {}
