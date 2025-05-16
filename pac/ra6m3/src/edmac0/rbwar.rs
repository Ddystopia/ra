///Register `RBWAR` reader
pub type R = crate::R<RBWAR_SPEC>;
///Field `RBWAR` reader - Receive Buffer Write Address RegisterThe RBWAR register indicates the last address that the EDMAC has written data to when writing to the receive buffer.Refer to the address indicated by the RBWAR register to recognize which address in the receive buffer the EDMAC is writing data to. Note that the address that the EDMAC is outputting to the receive buffer may not match the read value of the RBWAR register during data reception.
pub type RBWAR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Receive Buffer Write Address RegisterThe RBWAR register indicates the last address that the EDMAC has written data to when writing to the receive buffer.Refer to the address indicated by the RBWAR register to recognize which address in the receive buffer the EDMAC is writing data to. Note that the address that the EDMAC is outputting to the receive buffer may not match the read value of the RBWAR register during data reception.
    #[inline(always)]
    pub fn rbwar(&self) -> RBWAR_R {
        RBWAR_R::new(self.bits)
    }
}
/**Receive Buffer Write Address Register

You can [`read`](crate::Reg::read) this register and get [`rbwar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RBWAR_SPEC;
impl crate::RegisterSpec for RBWAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rbwar::R`](R) reader structure
impl crate::Readable for RBWAR_SPEC {}
///`reset()` method sets RBWAR to value 0
impl crate::Resettable for RBWAR_SPEC {}
