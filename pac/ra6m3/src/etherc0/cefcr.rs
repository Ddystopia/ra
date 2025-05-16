///Register `CEFCR` reader
pub type R = crate::R<CEFCR_SPEC>;
///Register `CEFCR` writer
pub type W = crate::W<CEFCR_SPEC>;
///Field `CEFCR` reader - CRC Error Frame Receive Counter RegisterThe CEFCR register is a counter indicating the number of received frames where a CRC error has been detected.
pub type CEFCR_R = crate::FieldReader<u32>;
///Field `CEFCR` writer - CRC Error Frame Receive Counter RegisterThe CEFCR register is a counter indicating the number of received frames where a CRC error has been detected.
pub type CEFCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CRC Error Frame Receive Counter RegisterThe CEFCR register is a counter indicating the number of received frames where a CRC error has been detected.
    #[inline(always)]
    pub fn cefcr(&self) -> CEFCR_R {
        CEFCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CRC Error Frame Receive Counter RegisterThe CEFCR register is a counter indicating the number of received frames where a CRC error has been detected.
    #[inline(always)]
    pub fn cefcr(&mut self) -> CEFCR_W<CEFCR_SPEC> {
        CEFCR_W::new(self, 0)
    }
}
/**CRC Error Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`cefcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cefcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CEFCR_SPEC;
impl crate::RegisterSpec for CEFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cefcr::R`](R) reader structure
impl crate::Readable for CEFCR_SPEC {}
///`write(|w| ..)` method takes [`cefcr::W`](W) writer structure
impl crate::Writable for CEFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEFCR to value 0
impl crate::Resettable for CEFCR_SPEC {}
