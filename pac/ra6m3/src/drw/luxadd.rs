///Register `LUXADD` writer
pub type W = crate::W<LUXADD_SPEC>;
///Field `LUXADD` writer - U limiter x-axis increment
pub type LUXADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - U limiter x-axis increment
    #[inline(always)]
    pub fn luxadd(&mut self) -> LUXADD_W<LUXADD_SPEC> {
        LUXADD_W::new(self, 0)
    }
}
/**U Limiter X-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`luxadd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUXADD_SPEC;
impl crate::RegisterSpec for LUXADD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`luxadd::W`](W) writer structure
impl crate::Writable for LUXADD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LUXADD to value 0
impl crate::Resettable for LUXADD_SPEC {}
