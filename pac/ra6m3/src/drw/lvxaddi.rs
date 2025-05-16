///Register `LVXADDI` writer
pub type W = crate::W<LVXADDI_SPEC>;
///Field `LVXADDI` writer - V limiter x-axis increment integer part
pub type LVXADDI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - V limiter x-axis increment integer part
    #[inline(always)]
    pub fn lvxaddi(&mut self) -> LVXADDI_W<LVXADDI_SPEC> {
        LVXADDI_W::new(self, 0)
    }
}
/**V Limiter X-Axis Increment Integer Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvxaddi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVXADDI_SPEC;
impl crate::RegisterSpec for LVXADDI_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lvxaddi::W`](W) writer structure
impl crate::Writable for LVXADDI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVXADDI to value 0
impl crate::Resettable for LVXADDI_SPEC {}
