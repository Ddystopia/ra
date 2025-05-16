///Register `TEXCLDATA` writer
pub type W = crate::W<TEXCLDATA_SPEC>;
///Field `CLDATA` writer - Texture CLUT data for Indexed texture format
pub type CLDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Texture CLUT data for Indexed texture format
    #[inline(always)]
    pub fn cldata(&mut self) -> CLDATA_W<TEXCLDATA_SPEC> {
        CLDATA_W::new(self, 0)
    }
}
/**CLUT Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texcldata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXCLDATA_SPEC;
impl crate::RegisterSpec for TEXCLDATA_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`texcldata::W`](W) writer structure
impl crate::Writable for TEXCLDATA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEXCLDATA to value 0
impl crate::Resettable for TEXCLDATA_SPEC {}
