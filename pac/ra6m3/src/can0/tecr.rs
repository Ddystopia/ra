///Register `TECR` reader
pub type R = crate::R<TECR_SPEC>;
///Field `TECR` reader - Transmit error count functionTECR increments or decrements the counter value according to the error status of the CAN module during transmission.
pub type TECR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Transmit error count functionTECR increments or decrements the counter value according to the error status of the CAN module during transmission.
    #[inline(always)]
    pub fn tecr(&self) -> TECR_R {
        TECR_R::new(self.bits)
    }
}
/**Transmit Error Count Register

You can [`read`](crate::Reg::read) this register and get [`tecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TECR_SPEC;
impl crate::RegisterSpec for TECR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tecr::R`](R) reader structure
impl crate::Readable for TECR_SPEC {}
///`reset()` method sets TECR to value 0
impl crate::Resettable for TECR_SPEC {}
