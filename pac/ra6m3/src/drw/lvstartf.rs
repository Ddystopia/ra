///Register `LVSTARTF` writer
pub type W = crate::W<LVSTARTF_SPEC>;
///Field `LVSTARTF` writer - V limiter start value fractional part
pub type LVSTARTF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    ///Bits 0:15 - V limiter start value fractional part
    #[inline(always)]
    pub fn lvstartf(&mut self) -> LVSTARTF_W<LVSTARTF_SPEC> {
        LVSTARTF_W::new(self, 0)
    }
}
/**V Limiter Start Value Fractional Part Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvstartf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVSTARTF_SPEC;
impl crate::RegisterSpec for LVSTARTF_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lvstartf::W`](W) writer structure
impl crate::Writable for LVSTARTF_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVSTARTF to value 0
impl crate::Resettable for LVSTARTF_SPEC {}
