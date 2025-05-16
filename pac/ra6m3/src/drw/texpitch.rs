///Register `TEXPITCH` writer
pub type W = crate::W<TEXPITCH_SPEC>;
///Field `TEXPITCH` writer - Texels per texture linevalid range: 0 to 2048
pub type TEXPITCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Texels per texture linevalid range: 0 to 2048
    #[inline(always)]
    pub fn texpitch(&mut self) -> TEXPITCH_W<TEXPITCH_SPEC> {
        TEXPITCH_W::new(self, 0)
    }
}
/**Texels Per Texture Line Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texpitch::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXPITCH_SPEC;
impl crate::RegisterSpec for TEXPITCH_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`texpitch::W`](W) writer structure
impl crate::Writable for TEXPITCH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEXPITCH to value 0
impl crate::Resettable for TEXPITCH_SPEC {}
