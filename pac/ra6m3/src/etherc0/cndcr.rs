///Register `CNDCR` reader
pub type R = crate::R<CNDCR_SPEC>;
///Register `CNDCR` writer
pub type W = crate::W<CNDCR_SPEC>;
///Field `CNDCR` reader - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission.
pub type CNDCR_R = crate::FieldReader<u32>;
///Field `CNDCR` writer - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission.
pub type CNDCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission.
    #[inline(always)]
    pub fn cndcr(&self) -> CNDCR_R {
        CNDCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Carrier Not Detect Counter RegisterThe CNDCR register is a counter indicating the number of times a carrier is not detected during preamble transmission.
    #[inline(always)]
    pub fn cndcr(&mut self) -> CNDCR_W<CNDCR_SPEC> {
        CNDCR_W::new(self, 0)
    }
}
/**Carrier Not Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cndcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CNDCR_SPEC;
impl crate::RegisterSpec for CNDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cndcr::R`](R) reader structure
impl crate::Readable for CNDCR_SPEC {}
///`write(|w| ..)` method takes [`cndcr::W`](W) writer structure
impl crate::Writable for CNDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNDCR to value 0
impl crate::Resettable for CNDCR_SPEC {}
