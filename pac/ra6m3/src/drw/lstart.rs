///Register `L%sSTART` writer
pub type W = crate::W<LSTART_SPEC>;
///Field `LSTART` writer - Start value of the n'th limiter(n=1-6)
pub type LSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Start value of the n'th limiter(n=1-6)
    #[inline(always)]
    pub fn lstart(&mut self) -> LSTART_W<LSTART_SPEC> {
        LSTART_W::new(self, 0)
    }
}
/**Limiter %s Start Value Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LSTART_SPEC;
impl crate::RegisterSpec for LSTART_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lstart::W`](W) writer structure
impl crate::Writable for LSTART_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L%sSTART to value 0
impl crate::Resettable for LSTART_SPEC {}
