///Register `LVSTARTI` writer
pub type W = crate::W<LVSTARTI_SPEC>;
///Field `LVSTARTI` writer - V limiter start value integer part
pub type LVSTARTI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - V limiter start value integer part
    #[inline(always)]
    pub fn lvstarti(&mut self) -> LVSTARTI_W<LVSTARTI_SPEC> {
        LVSTARTI_W::new(self, 0)
    }
}
/**V Limiter Start Value Integer Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvstarti::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVSTARTI_SPEC;
impl crate::RegisterSpec for LVSTARTI_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lvstarti::W`](W) writer structure
impl crate::Writable for LVSTARTI_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVSTARTI to value 0
impl crate::Resettable for LVSTARTI_SPEC {}
