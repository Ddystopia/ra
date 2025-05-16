///Register `JCDTCM` reader
pub type R = crate::R<JCDTCM_SPEC>;
///Field `DCM` reader - Middle bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts. NOTE: Read-only in Decompression.
pub type DCM_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Middle bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn dcm(&self) -> DCM_R {
        DCM_R::new(self.bits)
    }
}
/**JPEG Code Data Count Middle Register

You can [`read`](crate::Reg::read) this register and get [`jcdtcm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCDTCM_SPEC;
impl crate::RegisterSpec for JCDTCM_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcdtcm::R`](R) reader structure
impl crate::Readable for JCDTCM_SPEC {}
///`reset()` method sets JCDTCM to value 0
impl crate::Resettable for JCDTCM_SPEC {}
