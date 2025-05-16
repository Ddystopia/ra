///Register `GR2_CLUT0[%s]` reader
pub type R = crate::R<GR2_CLUT0_SPEC>;
///Register `GR2_CLUT0[%s]` writer
pub type W = crate::W<GR2_CLUT0_SPEC>;
///Field `B` reader - B Value of Color Palette n Plane for Graphics m Plane
pub type B_R = crate::FieldReader;
///Field `B` writer - B Value of Color Palette n Plane for Graphics m Plane
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `G` reader - G Value of Color Palette n Plane for Graphics m Plane
pub type G_R = crate::FieldReader;
///Field `G` writer - G Value of Color Palette n Plane for Graphics m Plane
pub type G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `R` reader - R Value of Color Palette n Plane for Graphics m Plane
pub type R_R = crate::FieldReader;
///Field `R` writer - R Value of Color Palette n Plane for Graphics m Plane
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `A` reader - Alpha Blending Value of Color Palette n Plane for Graphics m Plane
pub type A_R = crate::FieldReader;
///Field `A` writer - Alpha Blending Value of Color Palette n Plane for Graphics m Plane
pub type A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - B Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - G Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - R Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Alpha Blending Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - B Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn b(&mut self) -> B_W<GR2_CLUT0_SPEC> {
        B_W::new(self, 0)
    }
    ///Bits 8:15 - G Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn g(&mut self) -> G_W<GR2_CLUT0_SPEC> {
        G_W::new(self, 8)
    }
    ///Bits 16:23 - R Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn r(&mut self) -> R_W<GR2_CLUT0_SPEC> {
        R_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha Blending Value of Color Palette n Plane for Graphics m Plane
    #[inline(always)]
    pub fn a(&mut self) -> A_W<GR2_CLUT0_SPEC> {
        A_W::new(self, 24)
    }
}
/**Color Palette 0 Plane for Graphics 2 Plane

You can [`read`](crate::Reg::read) this register and get [`gr2_clut0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr2_clut0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR2_CLUT0_SPEC;
impl crate::RegisterSpec for GR2_CLUT0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr2_clut0::R`](R) reader structure
impl crate::Readable for GR2_CLUT0_SPEC {}
///`write(|w| ..)` method takes [`gr2_clut0::W`](W) writer structure
impl crate::Writable for GR2_CLUT0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR2_CLUT0[%s] to value 0
impl crate::Resettable for GR2_CLUT0_SPEC {}
