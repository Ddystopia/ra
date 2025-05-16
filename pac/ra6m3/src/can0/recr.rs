///Register `RECR` reader
pub type R = crate::R<RECR_SPEC>;
///Field `RECR` reader - Receive error count functionRECR increments or decrements the counter value according to the error status of the CAN module during reception.
pub type RECR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Receive error count functionRECR increments or decrements the counter value according to the error status of the CAN module during reception.
    #[inline(always)]
    pub fn recr(&self) -> RECR_R {
        RECR_R::new(self.bits)
    }
}
/**Receive Error Count Register

You can [`read`](crate::Reg::read) this register and get [`recr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RECR_SPEC;
impl crate::RegisterSpec for RECR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`recr::R`](R) reader structure
impl crate::Readable for RECR_SPEC {}
///`reset()` method sets RECR to value 0
impl crate::Resettable for RECR_SPEC {}
