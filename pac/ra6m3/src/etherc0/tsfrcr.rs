///Register `TSFRCR` reader
pub type R = crate::R<TSFRCR_SPEC>;
///Register `TSFRCR` writer
pub type W = crate::W<TSFRCR_SPEC>;
///Field `TSFRCR` reader - Too-Short Frame Receive Counter RegisterThe TSFRCR register is a counter indicating the number of times a short frame that is shorter than 64 bytes has been received.
pub type TSFRCR_R = crate::FieldReader<u32>;
///Field `TSFRCR` writer - Too-Short Frame Receive Counter RegisterThe TSFRCR register is a counter indicating the number of times a short frame that is shorter than 64 bytes has been received.
pub type TSFRCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Too-Short Frame Receive Counter RegisterThe TSFRCR register is a counter indicating the number of times a short frame that is shorter than 64 bytes has been received.
    #[inline(always)]
    pub fn tsfrcr(&self) -> TSFRCR_R {
        TSFRCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Too-Short Frame Receive Counter RegisterThe TSFRCR register is a counter indicating the number of times a short frame that is shorter than 64 bytes has been received.
    #[inline(always)]
    pub fn tsfrcr(&mut self) -> TSFRCR_W<TSFRCR_SPEC> {
        TSFRCR_W::new(self, 0)
    }
}
/**Too-Short Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tsfrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsfrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSFRCR_SPEC;
impl crate::RegisterSpec for TSFRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tsfrcr::R`](R) reader structure
impl crate::Readable for TSFRCR_SPEC {}
///`write(|w| ..)` method takes [`tsfrcr::W`](W) writer structure
impl crate::Writable for TSFRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSFRCR to value 0
impl crate::Resettable for TSFRCR_SPEC {}
