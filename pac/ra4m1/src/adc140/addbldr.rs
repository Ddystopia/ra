///Register `ADDBLDR` reader
pub type R = crate::R<ADDBLDR_SPEC>;
///Field `ADDBLDR` reader - This is a 16-bit read-only register for storing the result of A/D conversion in response to the second trigger in double trigger mode.
pub type ADDBLDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - This is a 16-bit read-only register for storing the result of A/D conversion in response to the second trigger in double trigger mode.
    #[inline(always)]
    pub fn addbldr(&self) -> ADDBLDR_R {
        ADDBLDR_R::new(self.bits)
    }
}
/**A/D Data Duplication Register

You can [`read`](crate::Reg::read) this register and get [`addbldr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADDBLDR_SPEC;
impl crate::RegisterSpec for ADDBLDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`addbldr::R`](R) reader structure
impl crate::Readable for ADDBLDR_SPEC {}
///`reset()` method sets ADDBLDR to value 0
impl crate::Resettable for ADDBLDR_SPEC {}
