///Register `COLOR1` writer
pub type W = crate::W<COLOR1_SPEC>;
///Field `COLOR1B` writer - Blue channel of color 1
pub type COLOR1B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLOR1G` writer - Green channel of color 1
pub type COLOR1G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLOR1R` writer - Red channel of color 1
pub type COLOR1R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLOR1A` writer - Alpha channel of color 1(0x00: transparent. . . 0xFF: opaque)
pub type COLOR1A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Blue channel of color 1
    #[inline(always)]
    pub fn color1b(&mut self) -> COLOR1B_W<COLOR1_SPEC> {
        COLOR1B_W::new(self, 0)
    }
    ///Bits 8:15 - Green channel of color 1
    #[inline(always)]
    pub fn color1g(&mut self) -> COLOR1G_W<COLOR1_SPEC> {
        COLOR1G_W::new(self, 8)
    }
    ///Bits 16:23 - Red channel of color 1
    #[inline(always)]
    pub fn color1r(&mut self) -> COLOR1R_W<COLOR1_SPEC> {
        COLOR1R_W::new(self, 16)
    }
    ///Bits 24:31 - Alpha channel of color 1(0x00: transparent. . . 0xFF: opaque)
    #[inline(always)]
    pub fn color1a(&mut self) -> COLOR1A_W<COLOR1_SPEC> {
        COLOR1A_W::new(self, 24)
    }
}
/**Base Color Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COLOR1_SPEC;
impl crate::RegisterSpec for COLOR1_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`color1::W`](W) writer structure
impl crate::Writable for COLOR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COLOR1 to value 0
impl crate::Resettable for COLOR1_SPEC {}
