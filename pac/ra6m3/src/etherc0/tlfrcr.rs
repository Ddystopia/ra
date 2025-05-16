///Register `TLFRCR` reader
pub type R = crate::R<TLFRCR_SPEC>;
///Register `TLFRCR` writer
pub type W = crate::W<TLFRCR_SPEC>;
///Field `TLFRCR` reader - Too-Long Frame Receive Counter RegisterThe TLFRCR register is a counter indicating the number of times a long frame that is longer than the RFLR register value has been received.
pub type TLFRCR_R = crate::FieldReader<u32>;
///Field `TLFRCR` writer - Too-Long Frame Receive Counter RegisterThe TLFRCR register is a counter indicating the number of times a long frame that is longer than the RFLR register value has been received.
pub type TLFRCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Too-Long Frame Receive Counter RegisterThe TLFRCR register is a counter indicating the number of times a long frame that is longer than the RFLR register value has been received.
    #[inline(always)]
    pub fn tlfrcr(&self) -> TLFRCR_R {
        TLFRCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Too-Long Frame Receive Counter RegisterThe TLFRCR register is a counter indicating the number of times a long frame that is longer than the RFLR register value has been received.
    #[inline(always)]
    pub fn tlfrcr(&mut self) -> TLFRCR_W<TLFRCR_SPEC> {
        TLFRCR_W::new(self, 0)
    }
}
/**Too-Long Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tlfrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlfrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TLFRCR_SPEC;
impl crate::RegisterSpec for TLFRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tlfrcr::R`](R) reader structure
impl crate::Readable for TLFRCR_SPEC {}
///`write(|w| ..)` method takes [`tlfrcr::W`](W) writer structure
impl crate::Writable for TLFRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TLFRCR to value 0
impl crate::Resettable for TLFRCR_SPEC {}
