///Register `TROCR` reader
pub type R = crate::R<TROCR_SPEC>;
///Register `TROCR` writer
pub type W = crate::W<TROCR_SPEC>;
///Field `TROCR` reader - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted.
pub type TROCR_R = crate::FieldReader<u32>;
///Field `TROCR` writer - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted.
pub type TROCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted.
    #[inline(always)]
    pub fn trocr(&self) -> TROCR_R {
        TROCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transmit Retry Over Counter RegisterThe TROCR register is a counter indicating the number of frames that fail to be retransmitted.
    #[inline(always)]
    pub fn trocr(&mut self) -> TROCR_W<TROCR_SPEC> {
        TROCR_W::new(self, 0)
    }
}
/**Transmit Retry Over Counter Register

You can [`read`](crate::Reg::read) this register and get [`trocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TROCR_SPEC;
impl crate::RegisterSpec for TROCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`trocr::R`](R) reader structure
impl crate::Readable for TROCR_SPEC {}
///`write(|w| ..)` method takes [`trocr::W`](W) writer structure
impl crate::Writable for TROCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TROCR to value 0
impl crate::Resettable for TROCR_SPEC {}
