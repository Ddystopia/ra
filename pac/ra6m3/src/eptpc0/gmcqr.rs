///Register `GMCQR` reader
pub type R = crate::R<GMCQR_SPEC>;
///Register `GMCQR` writer
pub type W = crate::W<GMCQR_SPEC>;
///Field `GMCQR` reader - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance
pub type GMCQR_R = crate::FieldReader<u32>;
///Field `GMCQR` writer - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance
pub type GMCQR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance
    #[inline(always)]
    pub fn gmcqr(&self) -> GMCQR_R {
        GMCQR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits are used to set the value of the grandmasterClockQuality fields of Announce messages. The correspondence between bits and the grandmasterClockQuality fields is as listed below.b31 to b24: clockClassb23 to b16: clockAccuracyb15 to b0: offsetScaledLogVariance
    #[inline(always)]
    pub fn gmcqr(&mut self) -> GMCQR_W<GMCQR_SPEC> {
        GMCQR_W::new(self, 0)
    }
}
/**grandmasterClockQuality Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`gmcqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmcqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GMCQR_SPEC;
impl crate::RegisterSpec for GMCQR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gmcqr::R`](R) reader structure
impl crate::Readable for GMCQR_SPEC {}
///`write(|w| ..)` method takes [`gmcqr::W`](W) writer structure
impl crate::Writable for GMCQR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GMCQR to value 0
impl crate::Resettable for GMCQR_SPEC {}
