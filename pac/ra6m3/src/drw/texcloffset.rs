///Register `TEXCLOFFSET` writer
pub type W = crate::W<TEXCLOFFSET_SPEC>;
///Field `CLOFFSET` writer - Texture CLUT offset for Indexed texture format. CLOFFSET\[7:0\] is or'ed with the original index
pub type CLOFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Texture CLUT offset for Indexed texture format. CLOFFSET\[7:0\] is or'ed with the original index
    #[inline(always)]
    pub fn cloffset(&mut self) -> CLOFFSET_W<TEXCLOFFSET_SPEC> {
        CLOFFSET_W::new(self, 0)
    }
}
/**CLUT Offset Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`texcloffset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEXCLOFFSET_SPEC;
impl crate::RegisterSpec for TEXCLOFFSET_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`texcloffset::W`](W) writer structure
impl crate::Writable for TEXCLOFFSET_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TEXCLOFFSET to value 0
impl crate::Resettable for TEXCLOFFSET_SPEC {}
