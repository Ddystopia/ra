///Register `CDCR` reader
pub type R = crate::R<CDCR_SPEC>;
///Register `CDCR` writer
pub type W = crate::W<CDCR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Late Collision Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CDCR_SPEC;
impl crate::RegisterSpec for CDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cdcr::R`](R) reader structure
impl crate::Readable for CDCR_SPEC {}
///`write(|w| ..)` method takes [`cdcr::W`](W) writer structure
impl crate::Writable for CDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CDCR to value 0
impl crate::Resettable for CDCR_SPEC {}
