///Register `LCCR` reader
pub type R = crate::R<LCCR_SPEC>;
///Register `LCCR` writer
pub type W = crate::W<LCCR_SPEC>;
///Field `LCCR` reader - Lost Carrier Counter RegisterThe LCCR register is a counter indicating the number of times a loss of carrier is detected during frame transmission.
pub type LCCR_R = crate::FieldReader<u32>;
///Field `LCCR` writer - Lost Carrier Counter RegisterThe LCCR register is a counter indicating the number of times a loss of carrier is detected during frame transmission.
pub type LCCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Lost Carrier Counter RegisterThe LCCR register is a counter indicating the number of times a loss of carrier is detected during frame transmission.
    #[inline(always)]
    pub fn lccr(&self) -> LCCR_R {
        LCCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Lost Carrier Counter RegisterThe LCCR register is a counter indicating the number of times a loss of carrier is detected during frame transmission.
    #[inline(always)]
    pub fn lccr(&mut self) -> LCCR_W<LCCR_SPEC> {
        LCCR_W::new(self, 0)
    }
}
/**Lost Carrier Counter Register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCCR_SPEC;
impl crate::RegisterSpec for LCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lccr::R`](R) reader structure
impl crate::Readable for LCCR_SPEC {}
///`write(|w| ..)` method takes [`lccr::W`](W) writer structure
impl crate::Writable for LCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCCR to value 0
impl crate::Resettable for LCCR_SPEC {}
