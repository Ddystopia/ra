///Register `ORIGIN` writer
pub type W = crate::W<ORIGIN_SPEC>;
///Field `ORIGIN` writer - Address of the first pixel in framebuffer
pub type ORIGIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Address of the first pixel in framebuffer
    #[inline(always)]
    pub fn origin(&mut self) -> ORIGIN_W<ORIGIN_SPEC> {
        ORIGIN_W::new(self, 0)
    }
}
/**Framebuffer Base Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`origin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ORIGIN_SPEC;
impl crate::RegisterSpec for ORIGIN_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`origin::W`](W) writer structure
impl crate::Writable for ORIGIN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ORIGIN to value 0
impl crate::Resettable for ORIGIN_SPEC {}
