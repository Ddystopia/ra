///Register `COLOR2` writer
pub type W = crate::W<COLOR2_SPEC>;
///Field `COLOR2B` writer - Blue channel of color 2
pub type COLOR2B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLOR2G` writer - Green channel of color 2
pub type COLOR2G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLOR2R` writer - Red channel of color 2
pub type COLOR2R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLOR2A` writer - Alpha channel of color 2(0x00: transparent. . . 0xFF: opaque)
pub type COLOR2A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Blue channel of color 2
    #[inline(always)]
    pub fn color2b(&mut self) -> COLOR2B_W<COLOR2_SPEC> {
        COLOR2B_W::new(self, 0)
    }
    ///Bits 8:15 - Green channel of color 2
    #[inline(always)]
    pub fn color2g(&mut self) -> COLOR2G_W<COLOR2_SPEC> {
        COLOR2G_W::new(self, 8)
    }
    ///Bits 16:23 - Red channel of color 2
    #[inline(always)]
    pub fn color2r(&mut self) -> COLOR2R_W<COLOR2_SPEC> {
        COLOR2R_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha channel of color 2(0x00: transparent. . . 0xFF: opaque)
    #[inline(always)]
    pub fn color2a(&mut self) -> COLOR2A_W<COLOR2_SPEC> {
        COLOR2A_W::new(self, 24)
    }
}
/**Secondary Color Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COLOR2_SPEC;
impl crate::RegisterSpec for COLOR2_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`color2::W`](W) writer structure
impl crate::Writable for COLOR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COLOR2 to value 0
impl crate::Resettable for COLOR2_SPEC {}
