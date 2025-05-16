///Register `LUYADD` writer
pub type W = crate::W<LUYADD_SPEC>;
///Field `LUYADD` writer - U limiter y-axis increment
pub type LUYADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - U limiter y-axis increment
    #[inline(always)]
    pub fn luyadd(&mut self) -> LUYADD_W<LUYADD_SPEC> {
        LUYADD_W::new(self, 0)
    }
}
/**U Limiter Y-Axis Increment Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`luyadd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUYADD_SPEC;
impl crate::RegisterSpec for LUYADD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`luyadd::W`](W) writer structure
impl crate::Writable for LUYADD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LUYADD to value 0
impl crate::Resettable for LUYADD_SPEC {}
