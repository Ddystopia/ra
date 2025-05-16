///Register `TBRAR` reader
pub type R = crate::R<TBRAR_SPEC>;
///Field `TBRAR` reader - Transmit Buffer Read Address RegisterThe TBRAR register indicates the last address that the EDMAC has read data from when reading data from the transmit buffer.Refer to the address indicated by the TBRAR register to recognize which address in the transmit buffer the EDMAC is reading from. Note that the address that the EDMAC is outputting to the transmit buffer may not match the read value of the TBRAR register.
pub type TBRAR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Transmit Buffer Read Address RegisterThe TBRAR register indicates the last address that the EDMAC has read data from when reading data from the transmit buffer.Refer to the address indicated by the TBRAR register to recognize which address in the transmit buffer the EDMAC is reading from. Note that the address that the EDMAC is outputting to the transmit buffer may not match the read value of the TBRAR register.
    #[inline(always)]
    pub fn tbrar(&self) -> TBRAR_R {
        TBRAR_R::new(self.bits)
    }
}
/**Transmit Buffer Read Address Register

You can [`read`](crate::Reg::read) this register and get [`tbrar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TBRAR_SPEC;
impl crate::RegisterSpec for TBRAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tbrar::R`](R) reader structure
impl crate::Readable for TBRAR_SPEC {}
///`reset()` method sets TBRAR to value 0
impl crate::Resettable for TBRAR_SPEC {}
