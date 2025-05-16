///Register `AGTCMB` reader
pub type R = crate::R<AGTCMB_SPEC>;
///Register `AGTCMB` writer
pub type W = crate::W<AGTCMB_SPEC>;
///Field `AGTCMB` reader - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH
pub type AGTCMB_R = crate::FieldReader<u16>;
///Field `AGTCMB` writer - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH
pub type AGTCMB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH
    #[inline(always)]
    pub fn agtcmb(&self) -> AGTCMB_R {
        AGTCMB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - AGT Compare Match B RegisterNOTE : When 1 is written to the TSTOP bit in the AGTCR register, set to FFFFH
    #[inline(always)]
    pub fn agtcmb(&mut self) -> AGTCMB_W<AGTCMB_SPEC> {
        AGTCMB_W::new(self, 0)
    }
}
/**AGT Compare Match B Register

You can [`read`](crate::Reg::read) this register and get [`agtcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGTCMB_SPEC;
impl crate::RegisterSpec for AGTCMB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`agtcmb::R`](R) reader structure
impl crate::Readable for AGTCMB_SPEC {}
///`write(|w| ..)` method takes [`agtcmb::W`](W) writer structure
impl crate::Writable for AGTCMB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGTCMB to value 0xffff
impl crate::Resettable for AGTCMB_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
