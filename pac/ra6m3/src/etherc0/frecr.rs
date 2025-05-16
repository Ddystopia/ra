///Register `FRECR` reader
pub type R = crate::R<FRECR_SPEC>;
///Register `FRECR` writer
pub type W = crate::W<FRECR_SPEC>;
///Field `FRECR` reader - Frame Receive Error Counter RegisterThe FRECR register is a counter indicating the number of times a frame receive error has occurred.
pub type FRECR_R = crate::FieldReader<u32>;
///Field `FRECR` writer - Frame Receive Error Counter RegisterThe FRECR register is a counter indicating the number of times a frame receive error has occurred.
pub type FRECR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Frame Receive Error Counter RegisterThe FRECR register is a counter indicating the number of times a frame receive error has occurred.
    #[inline(always)]
    pub fn frecr(&self) -> FRECR_R {
        FRECR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Frame Receive Error Counter RegisterThe FRECR register is a counter indicating the number of times a frame receive error has occurred.
    #[inline(always)]
    pub fn frecr(&mut self) -> FRECR_W<FRECR_SPEC> {
        FRECR_W::new(self, 0)
    }
}
/**Frame Receive Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`frecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FRECR_SPEC;
impl crate::RegisterSpec for FRECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`frecr::R`](R) reader structure
impl crate::Readable for FRECR_SPEC {}
///`write(|w| ..)` method takes [`frecr::W`](W) writer structure
impl crate::Writable for FRECR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FRECR to value 0
impl crate::Resettable for FRECR_SPEC {}
