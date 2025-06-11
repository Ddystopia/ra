///Register `RFPCR` writer
pub type W = crate::W<RFPCR_SPEC>;
///Field `RFPCR` writer - The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR.
pub type RFPCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR.
    #[inline(always)]
    pub fn rfpcr(&mut self) -> RFPCR_W<RFPCR_SPEC> {
        RFPCR_W::new(self, 0)
    }
}
/**Receive FIFO Pointer Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFPCR_SPEC;
impl crate::RegisterSpec for RFPCR_SPEC {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`rfpcr::W`](W) writer structure
impl crate::Writable for RFPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFPCR to value 0
impl crate::Resettable for RFPCR_SPEC {}
