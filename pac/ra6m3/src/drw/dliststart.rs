///Register `DLISTSTART` writer
pub type W = crate::W<DLISTSTART_SPEC>;
///Field `DLISTSTART` writer - Display list start address
pub type DLISTSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Display list start address
    #[inline(always)]
    pub fn dliststart(&mut self) -> DLISTSTART_W<DLISTSTART_SPEC> {
        DLISTSTART_W::new(self, 0)
    }
}
/**Display List Start Address Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dliststart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DLISTSTART_SPEC;
impl crate::RegisterSpec for DLISTSTART_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dliststart::W`](W) writer structure
impl crate::Writable for DLISTSTART_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DLISTSTART to value 0
impl crate::Resettable for DLISTSTART_SPEC {}
