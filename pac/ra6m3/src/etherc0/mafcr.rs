///Register `MAFCR` reader
pub type R = crate::R<MAFCR_SPEC>;
///Register `MAFCR` writer
pub type W = crate::W<MAFCR_SPEC>;
///Field `MAFCR` reader - Multicast Address Frame Receive Counter RegisterThe MAFCR register is a counter indicating the number of times a frame where the multicast address is set has been received.
pub type MAFCR_R = crate::FieldReader<u32>;
///Field `MAFCR` writer - Multicast Address Frame Receive Counter RegisterThe MAFCR register is a counter indicating the number of times a frame where the multicast address is set has been received.
pub type MAFCR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Multicast Address Frame Receive Counter RegisterThe MAFCR register is a counter indicating the number of times a frame where the multicast address is set has been received.
    #[inline(always)]
    pub fn mafcr(&self) -> MAFCR_R {
        MAFCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Multicast Address Frame Receive Counter RegisterThe MAFCR register is a counter indicating the number of times a frame where the multicast address is set has been received.
    #[inline(always)]
    pub fn mafcr(&mut self) -> MAFCR_W<MAFCR_SPEC> {
        MAFCR_W::new(self, 0)
    }
}
/**Multicast Address Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`mafcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mafcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MAFCR_SPEC;
impl crate::RegisterSpec for MAFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mafcr::R`](R) reader structure
impl crate::Readable for MAFCR_SPEC {}
///`write(|w| ..)` method takes [`mafcr::W`](W) writer structure
impl crate::Writable for MAFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MAFCR to value 0
impl crate::Resettable for MAFCR_SPEC {}
