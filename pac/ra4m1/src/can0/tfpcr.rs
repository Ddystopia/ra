///Register `TFPCR` writer
pub type W = crate::W<TFPCR_SPEC>;
///Field `TFPCR` writer - The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR.
pub type TFPCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR.
    #[inline(always)]
    pub fn tfpcr(&mut self) -> TFPCR_W<TFPCR_SPEC> {
        TFPCR_W::new(self, 0)
    }
}
/**Transmit FIFO Pointer Control Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfpcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TFPCR_SPEC;
impl crate::RegisterSpec for TFPCR_SPEC {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`tfpcr::W`](W) writer structure
impl crate::Writable for TFPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TFPCR to value 0
impl crate::Resettable for TFPCR_SPEC {}
