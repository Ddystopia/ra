///Register `RFOCR` reader
pub type R = crate::R<RFOCR_SPEC>;
///Register `RFOCR` writer
pub type W = crate::W<RFOCR_SPEC>;
///Field `OVER` reader - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh.
pub type OVER_R = crate::FieldReader<u16>;
///Field `OVER` writer - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh.
pub type OVER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh.
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Receive FIFO Overflow CountThese bits indicate how many times the receive FIFO has overflowed. The counter stops when the counter value reaches FFFFh.
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W<RFOCR_SPEC> {
        OVER_W::new(self, 0)
    }
}
/**Receive FIFO Overflow Counter

You can [`read`](crate::Reg::read) this register and get [`rfocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFOCR_SPEC;
impl crate::RegisterSpec for RFOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rfocr::R`](R) reader structure
impl crate::Readable for RFOCR_SPEC {}
///`write(|w| ..)` method takes [`rfocr::W`](W) writer structure
impl crate::Writable for RFOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFOCR to value 0
impl crate::Resettable for RFOCR_SPEC {}
