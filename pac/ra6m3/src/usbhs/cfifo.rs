///Register `CFIFO` reader
pub type R = crate::R<CFIFO_SPEC>;
///Register `CFIFO` writer
pub type W = crate::W<CFIFO_SPEC>;
///Field `FIFOPORT` reader - FIFO Port.Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FIFOPORT_R = crate::FieldReader<u32>;
///Field `FIFOPORT` writer - FIFO Port.Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FIFOPORT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - FIFO Port.Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&self) -> FIFOPORT_R {
        FIFOPORT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - FIFO Port.Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FIFOPORT_W<CFIFO_SPEC> {
        FIFOPORT_W::new(self, 0)
    }
}
/**CFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFIFO_SPEC;
impl crate::RegisterSpec for CFIFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cfifo::R`](R) reader structure
impl crate::Readable for CFIFO_SPEC {}
///`write(|w| ..)` method takes [`cfifo::W`](W) writer structure
impl crate::Writable for CFIFO_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFO to value 0
impl crate::Resettable for CFIFO_SPEC {}
