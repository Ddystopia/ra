///Register `RDFAR` reader
pub type R = crate::R<RDFAR_SPEC>;
///Field `RDFAR` reader - Receive Descriptor Fetch Address RegisterThe RDFAR register indicates the start address of the last fetched receive descriptor when the EDMAC fetches descriptor information from the receive descriptor.Refer to the address indicated by the RDFAR register to recognize which receive descriptor information the EDMAC is using for the current processing. Note that the address of the receive descriptor that the EDMAC fetches may not match the read value of the RDFAR register during data reception.
pub type RDFAR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Receive Descriptor Fetch Address RegisterThe RDFAR register indicates the start address of the last fetched receive descriptor when the EDMAC fetches descriptor information from the receive descriptor.Refer to the address indicated by the RDFAR register to recognize which receive descriptor information the EDMAC is using for the current processing. Note that the address of the receive descriptor that the EDMAC fetches may not match the read value of the RDFAR register during data reception.
    #[inline(always)]
    pub fn rdfar(&self) -> RDFAR_R {
        RDFAR_R::new(self.bits)
    }
}
/**Receive Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`rdfar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDFAR_SPEC;
impl crate::RegisterSpec for RDFAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdfar::R`](R) reader structure
impl crate::Readable for RDFAR_SPEC {}
///`reset()` method sets RDFAR to value 0
impl crate::Resettable for RDFAR_SPEC {}
