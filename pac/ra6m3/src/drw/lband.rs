///Register `L%sBAND` writer
pub type W = crate::W<LBAND_SPEC>;
///Field `LBAND` writer - Limiter m band width parameter
pub type LBAND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Limiter m band width parameter
    #[inline(always)]
    pub fn lband(&mut self) -> LBAND_W<LBAND_SPEC> {
        LBAND_W::new(self, 0)
    }
}
/**Limiter %s Band Width Parameter Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lband::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LBAND_SPEC;
impl crate::RegisterSpec for LBAND_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lband::W`](W) writer structure
impl crate::Writable for LBAND_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L%sBAND to value 0
impl crate::Resettable for LBAND_SPEC {}
