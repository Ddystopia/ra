///Register `TDFAR` reader
pub type R = crate::R<TDFAR_SPEC>;
///Field `TDFAR` reader - Transmit Descriptor Fetch Address RegisterThe TDFAR register indicates the start address of the last fetched transmit descriptor when the EDMAC fetches descriptor information from the transmit descriptor.Refer to the address indicated by the TDFAR register to recognize which transmit descriptor information the EDMAC is using for the current processing. Note that the address of the transmit descriptor that the EDMAC fetches may not match the read value of the TDFAR register.
pub type TDFAR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Transmit Descriptor Fetch Address RegisterThe TDFAR register indicates the start address of the last fetched transmit descriptor when the EDMAC fetches descriptor information from the transmit descriptor.Refer to the address indicated by the TDFAR register to recognize which transmit descriptor information the EDMAC is using for the current processing. Note that the address of the transmit descriptor that the EDMAC fetches may not match the read value of the TDFAR register.
    #[inline(always)]
    pub fn tdfar(&self) -> TDFAR_R {
        TDFAR_R::new(self.bits)
    }
}
/**Transmit Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`tdfar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TDFAR_SPEC;
impl crate::RegisterSpec for TDFAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tdfar::R`](R) reader structure
impl crate::Readable for TDFAR_SPEC {}
///`reset()` method sets TDFAR to value 0
impl crate::Resettable for TDFAR_SPEC {}
