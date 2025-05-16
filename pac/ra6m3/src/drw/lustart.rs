///Register `LUSTART` writer
pub type W = crate::W<LUSTART_SPEC>;
///Field `LUSTART` writer - U limiter start value
pub type LUSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - U limiter start value
    #[inline(always)]
    pub fn lustart(&mut self) -> LUSTART_W<LUSTART_SPEC> {
        LUSTART_W::new(self, 0)
    }
}
/**U Limiter Start Value Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lustart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUSTART_SPEC;
impl crate::RegisterSpec for LUSTART_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lustart::W`](W) writer structure
impl crate::Writable for LUSTART_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LUSTART to value 0
impl crate::Resettable for LUSTART_SPEC {}
