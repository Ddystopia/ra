///Register `BG_BGC` reader
pub type R = crate::R<BG_BGC_SPEC>;
///Register `BG_BGC` writer
pub type W = crate::W<BG_BGC_SPEC>;
///Field `B` reader - B value for background plane valid pixel areaUnsigned; 8-bit integer
pub type B_R = crate::FieldReader;
///Field `B` writer - B value for background plane valid pixel areaUnsigned; 8-bit integer
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `G` reader - G value for background plane valid pixel areaUnsigned; 8-bit integer
pub type G_R = crate::FieldReader;
///Field `G` writer - G value for background plane valid pixel areaUnsigned; 8-bit integer
pub type G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `R` reader - R value for background plane valid pixel area.Unsigned; 8-bit integer.
pub type R_R = crate::FieldReader;
///Field `R` writer - R value for background plane valid pixel area.Unsigned; 8-bit integer.
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - B value for background plane valid pixel areaUnsigned; 8-bit integer
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - G value for background plane valid pixel areaUnsigned; 8-bit integer
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - R value for background plane valid pixel area.Unsigned; 8-bit integer.
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - B value for background plane valid pixel areaUnsigned; 8-bit integer
    #[inline(always)]
    pub fn b(&mut self) -> B_W<BG_BGC_SPEC> {
        B_W::new(self, 0)
    }
    ///Bits 8:15 - G value for background plane valid pixel areaUnsigned; 8-bit integer
    #[inline(always)]
    pub fn g(&mut self) -> G_W<BG_BGC_SPEC> {
        G_W::new(self, 8)
    }
    ///Bits 16:23 - R value for background plane valid pixel area.Unsigned; 8-bit integer.
    #[inline(always)]
    pub fn r(&mut self) -> R_W<BG_BGC_SPEC> {
        R_W::new(self, 16)
    }
}
/**Background Plane Setting Background Color Register

You can [`read`](crate::Reg::read) this register and get [`bg_bgc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_bgc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BG_BGC_SPEC;
impl crate::RegisterSpec for BG_BGC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bg_bgc::R`](R) reader structure
impl crate::Readable for BG_BGC_SPEC {}
///`write(|w| ..)` method takes [`bg_bgc::W`](W) writer structure
impl crate::Writable for BG_BGC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BG_BGC to value 0
impl crate::Resettable for BG_BGC_SPEC {}
