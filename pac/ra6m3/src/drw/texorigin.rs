///Register `TEXORIGIN` writer
pub type W = crate::W<TEXORIGIN_SPEC>;
///Field `TEXORIGIN` writer - Texture base address
pub type TEXORIGIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Texture base address
    #[inline(always)]
    pub fn texorigin(&mut self) -> TEXORIGIN_W<TEXORIGIN_SPEC> {
        TEXORIGIN_W::new(self, 0)
    }
}
/**Texture Base Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texorigin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXORIGIN_SPEC;
impl crate::RegisterSpec for TEXORIGIN_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`texorigin::W`](W) writer structure
impl crate::Writable for TEXORIGIN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEXORIGIN to value 0
impl crate::Resettable for TEXORIGIN_SPEC {}
