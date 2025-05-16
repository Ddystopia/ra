///Register `TFUCR` reader
pub type R = crate::R<TFUCR_SPEC>;
///Register `TFUCR` writer
pub type W = crate::W<TFUCR_SPEC>;
///Field `UNDER` reader - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh.
pub type UNDER_R = crate::FieldReader<u16>;
///Field `UNDER` writer - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh.
pub type UNDER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh.
    #[inline(always)]
    pub fn under(&self) -> UNDER_R {
        UNDER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Transmit FIFO Underflow CountThese bits indicate how many times the transmit FIFO has underflowed. The counter stops when the counter value reaches FFFFh.
    #[inline(always)]
    pub fn under(&mut self) -> UNDER_W<TFUCR_SPEC> {
        UNDER_W::new(self, 0)
    }
}
/**Transmit FIFO Underflow Counter

You can [`read`](crate::Reg::read) this register and get [`tfucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TFUCR_SPEC;
impl crate::RegisterSpec for TFUCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tfucr::R`](R) reader structure
impl crate::Readable for TFUCR_SPEC {}
///`write(|w| ..)` method takes [`tfucr::W`](W) writer structure
impl crate::Writable for TFUCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TFUCR to value 0
impl crate::Resettable for TFUCR_SPEC {}
