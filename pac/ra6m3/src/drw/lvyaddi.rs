///Register `LVYADDI` writer
pub type W = crate::W<LVYADDI_SPEC>;
///Field `LVYADDI` writer - V limiter y-axis increment integer part
pub type LVYADDI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - V limiter y-axis increment integer part
    #[inline(always)]
    pub fn lvyaddi(&mut self) -> LVYADDI_W<LVYADDI_SPEC> {
        LVYADDI_W::new(self, 0)
    }
}
/**V Limiter Y-Axis Increment Integer Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvyaddi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVYADDI_SPEC;
impl crate::RegisterSpec for LVYADDI_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lvyaddi::W`](W) writer structure
impl crate::Writable for LVYADDI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVYADDI to value 0
impl crate::Resettable for LVYADDI_SPEC {}
