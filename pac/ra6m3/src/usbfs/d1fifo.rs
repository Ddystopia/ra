///Register `D1FIFO` reader
pub type R = crate::R<D1FIFO_SPEC>;
///Register `D1FIFO` writer
pub type W = crate::W<D1FIFO_SPEC>;
///Field `FIFOPORT` reader - FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FIFOPORT_R = crate::FieldReader<u16>;
///Field `FIFOPORT` writer - FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
pub type FIFOPORT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&self) -> FIFOPORT_R {
        FIFOPORT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits.
    #[inline(always)]
    pub fn fifoport(&mut self) -> FIFOPORT_W<D1FIFO_SPEC> {
        FIFOPORT_W::new(self, 0)
    }
}
/**D1FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1FIFO_SPEC;
impl crate::RegisterSpec for D1FIFO_SPEC {
    type Ux = u16;
}
///`read()` method returns [`d1fifo::R`](R) reader structure
impl crate::Readable for D1FIFO_SPEC {}
///`write(|w| ..)` method takes [`d1fifo::W`](W) writer structure
impl crate::Writable for D1FIFO_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFO to value 0
impl crate::Resettable for D1FIFO_SPEC {}
