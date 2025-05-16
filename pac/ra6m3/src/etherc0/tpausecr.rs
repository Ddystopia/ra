///Register `TPAUSECR` reader
pub type R = crate::R<TPAUSECR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**PAUSE Frame Retransmit Counter

You can [`read`](crate::Reg::read) this register and get [`tpausecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TPAUSECR_SPEC;
impl crate::RegisterSpec for TPAUSECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tpausecr::R`](R) reader structure
impl crate::Readable for TPAUSECR_SPEC {}
///`reset()` method sets TPAUSECR to value 0
impl crate::Resettable for TPAUSECR_SPEC {}
