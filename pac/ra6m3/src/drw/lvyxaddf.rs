///Register `LVYXADDF` writer
pub type W = crate::W<LVYXADDF_SPEC>;
///Field `LVXADDF` writer - V xlimiter increment fractional part
pub type LVXADDF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `LVYADDF` writer - V y limiter increment fractional part
pub type LVYADDF_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    ///Bits 0:15 - V xlimiter increment fractional part
    #[inline(always)]
    pub fn lvxaddf(&mut self) -> LVXADDF_W<LVYXADDF_SPEC> {
        LVXADDF_W::new(self, 0)
    }
    ///Bits 16:31 - V y limiter increment fractional part
    #[inline(always)]
    pub fn lvyaddf(&mut self) -> LVYADDF_W<LVYXADDF_SPEC> {
        LVYADDF_W::new(self, 16)
    }
}
/**V Limiter Increment Fractional Parts Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvyxaddf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LVYXADDF_SPEC;
impl crate::RegisterSpec for LVYXADDF_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lvyxaddf::W`](W) writer structure
impl crate::Writable for LVYXADDF_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LVYXADDF to value 0
impl crate::Resettable for LVYXADDF_SPEC {}
