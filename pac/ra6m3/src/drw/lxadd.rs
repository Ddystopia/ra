///Register `L%sXADD` writer
pub type W = crate::W<LXADD_SPEC>;
///Field `LXADD` writer - X-axis increment
pub type LXADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - X-axis increment
    #[inline(always)]
    pub fn lxadd(&mut self) -> LXADD_W<LXADD_SPEC> {
        LXADD_W::new(self, 0)
    }
}
/**Limiter %s X-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lxadd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LXADD_SPEC;
impl crate::RegisterSpec for LXADD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lxadd::W`](W) writer structure
impl crate::Writable for LXADD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L%sXADD to value 0
impl crate::Resettable for LXADD_SPEC {}
