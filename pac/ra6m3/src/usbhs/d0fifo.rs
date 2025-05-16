///Register `D0FIFO` reader
pub type R = crate::R<D0FIFO_SPEC>;
///Register `D0FIFO` writer
pub type W = crate::W<D0FIFO_SPEC>;
///Field `FIFOPORT` reader - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FIFOPORT_R = crate::FieldReader<u32>;
///Field `FIFOPORT` writer - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FIFOPORT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&self) -> FIFOPORT_R {
        FIFOPORT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - FIFO Port Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FIFOPORT_W<D0FIFO_SPEC> {
        FIFOPORT_W::new(self, 0)
    }
}
/**D0FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D0FIFO_SPEC;
impl crate::RegisterSpec for D0FIFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`d0fifo::R`](R) reader structure
impl crate::Readable for D0FIFO_SPEC {}
///`write(|w| ..)` method takes [`d0fifo::W`](W) writer structure
impl crate::Writable for D0FIFO_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D0FIFO to value 0
impl crate::Resettable for D0FIFO_SPEC {}
