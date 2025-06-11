///Register `SSIFTDR` writer
pub type W = crate::W<SSIFTDR_SPEC>;
///Field `SSIFTDR` writer - Transmit FIFO Data
pub type SSIFTDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - Transmit FIFO Data
    #[inline(always)]
    pub fn ssiftdr(&mut self) -> SSIFTDR_W<SSIFTDR_SPEC> {
        SSIFTDR_W::new(self, 0)
    }
}
/**Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiftdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSIFTDR_SPEC;
impl crate::RegisterSpec for SSIFTDR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ssiftdr::W`](W) writer structure
impl crate::Writable for SSIFTDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SSIFTDR to value 0
impl crate::Resettable for SSIFTDR_SPEC {}
