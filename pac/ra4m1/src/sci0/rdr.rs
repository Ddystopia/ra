///Register `RDR` reader
pub type R = crate::R<RDR_SPEC>;
///Field `RDR` reader - RDR is an 8-bit register that stores receive data.
pub type RDR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - RDR is an 8-bit register that stores receive data.
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new(self.bits)
    }
}
/**Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rdr::R`](R) reader structure
impl crate::Readable for RDR_SPEC {}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDR_SPEC {}
