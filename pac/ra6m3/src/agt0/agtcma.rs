///Register `AGTCMA` reader
pub type R = crate::R<AGTCMA_SPEC>;
///Register `AGTCMA` writer
pub type W = crate::W<AGTCMA_SPEC>;
///Field `AGTCMA` reader - AGT Compare Match A RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH
pub type AGTCMA_R = crate::FieldReader<u16>;
///Field `AGTCMA` writer - AGT Compare Match A RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH
pub type AGTCMA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - AGT Compare Match A RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH
    #[inline(always)]
    pub fn agtcma(&self) -> AGTCMA_R {
        AGTCMA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - AGT Compare Match A RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, set to FFFFH
    #[inline(always)]
    pub fn agtcma(&mut self) -> AGTCMA_W<AGTCMA_SPEC> {
        AGTCMA_W::new(self, 0)
    }
}
/**AGT Compare Match A Register

You can [`read`](crate::Reg::read) this register and get [`agtcma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTCMA_SPEC;
impl crate::RegisterSpec for AGTCMA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`agtcma::R`](R) reader structure
impl crate::Readable for AGTCMA_SPEC {}
///`write(|w| ..)` method takes [`agtcma::W`](W) writer structure
impl crate::Writable for AGTCMA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCMA to value 0xffff
impl crate::Resettable for AGTCMA_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
