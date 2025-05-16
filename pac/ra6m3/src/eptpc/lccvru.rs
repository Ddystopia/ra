///Register `LCCVRU` reader
pub type R = crate::R<LCCVRU_SPEC>;
///Field `LCCVRU` reader - These bits are for reading the higher-order 16 bits of the integer portion of the local timer counter's value.
pub type LCCVRU_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - These bits are for reading the higher-order 16 bits of the integer portion of the local timer counter's value.
    #[inline(always)]
    pub fn lccvru(&self) -> LCCVRU_R {
        LCCVRU_R::new((self.bits & 0xffff) as u16)
    }
}
/**Local Time Counters

You can [`read`](crate::Reg::read) this register and get [`lccvru::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCCVRU_SPEC;
impl crate::RegisterSpec for LCCVRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lccvru::R`](R) reader structure
impl crate::Readable for LCCVRU_SPEC {}
///`reset()` method sets LCCVRU to value 0
impl crate::Resettable for LCCVRU_SPEC {}
