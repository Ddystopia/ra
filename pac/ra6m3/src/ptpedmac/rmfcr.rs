///Register `RMFCR` reader
pub type R = crate::R<RMFCR_SPEC>;
///Register `RMFCR` writer
pub type W = crate::W<RMFCR_SPEC>;
///Field `MFC` reader - Missed-Frame CounterThese bits indicate the number of frames that are discarded and not transferred to the receive buffer during reception.
pub type MFC_R = crate::FieldReader<u16>;
///Field `MFC` writer - Missed-Frame CounterThese bits indicate the number of frames that are discarded and not transferred to the receive buffer during reception.
pub type MFC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Missed-Frame CounterThese bits indicate the number of frames that are discarded and not transferred to the receive buffer during reception.
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Missed-Frame CounterThese bits indicate the number of frames that are discarded and not transferred to the receive buffer during reception.
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W<RMFCR_SPEC> {
        MFC_W::new(self, 0)
    }
}
/**Missed-Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rmfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMFCR_SPEC;
impl crate::RegisterSpec for RMFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rmfcr::R`](R) reader structure
impl crate::Readable for RMFCR_SPEC {}
///`write(|w| ..)` method takes [`rmfcr::W`](W) writer structure
impl crate::Writable for RMFCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RMFCR to value 0
impl crate::Resettable for RMFCR_SPEC {}
