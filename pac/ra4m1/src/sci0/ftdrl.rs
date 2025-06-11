///Register `FTDRL` writer
pub type W = crate::W<FTDRL_SPEC>;
///Field `TDATL` writer - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
pub type TDATL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Serial transmit data(b7-b0) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)
    #[inline(always)]
    pub fn tdatl(&mut self) -> TDATL_W<FTDRL_SPEC> {
        TDATL_W::new(self, 0)
    }
}
/**Transmit FIFO Data Register L

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FTDRL_SPEC;
impl crate::RegisterSpec for FTDRL_SPEC {
    type Ux = u8;
}
///`write(|w| ..)` method takes [`ftdrl::W`](W) writer structure
impl crate::Writable for FTDRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTDRL to value 0xff
impl crate::Resettable for FTDRL_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
