///Register `RFCR` reader
pub type R = crate::R<RFCR_SPEC>;
///Register `RFCR` writer
pub type W = crate::W<RFCR_SPEC>;
///Field `RFCR` reader - Received Alignment Error Frame Counter RegisterThe RFCR register is a counter indicating the number of times a frame has been received with the alignment error (frame is not an integral number of octets).
pub type RFCR_R = crate::FieldReader<u32>;
///Field `RFCR` writer - Received Alignment Error Frame Counter RegisterThe RFCR register is a counter indicating the number of times a frame has been received with the alignment error (frame is not an integral number of octets).
pub type RFCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Received Alignment Error Frame Counter RegisterThe RFCR register is a counter indicating the number of times a frame has been received with the alignment error (frame is not an integral number of octets).
    #[inline(always)]
    pub fn rfcr(&self) -> RFCR_R {
        RFCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Received Alignment Error Frame Counter RegisterThe RFCR register is a counter indicating the number of times a frame has been received with the alignment error (frame is not an integral number of octets).
    #[inline(always)]
    pub fn rfcr(&mut self) -> RFCR_W<RFCR_SPEC> {
        RFCR_W::new(self, 0)
    }
}
/**Received Alignment Error Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFCR_SPEC;
impl crate::RegisterSpec for RFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rfcr::R`](R) reader structure
impl crate::Readable for RFCR_SPEC {}
///`write(|w| ..)` method takes [`rfcr::W`](W) writer structure
impl crate::Writable for RFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFCR to value 0
impl crate::Resettable for RFCR_SPEC {}
