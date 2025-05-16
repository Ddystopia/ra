///Register `ADOCDR` reader
pub type R = crate::R<ADOCDR_SPEC>;
///Field `ADOCDR` reader - This is a 16-bit read-only register for storing the A/D result of internal reference voltage.
pub type ADOCDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - This is a 16-bit read-only register for storing the A/D result of internal reference voltage.
    #[inline(always)]
    pub fn adocdr(&self) -> ADOCDR_R {
        ADOCDR_R::new(self.bits)
    }
}
/**A/D Internal Reference Voltage Data Register

You can [`read`](crate::Reg::read) this register and get [`adocdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADOCDR_SPEC;
impl crate::RegisterSpec for ADOCDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adocdr::R`](R) reader structure
impl crate::Readable for ADOCDR_SPEC {}
///`reset()` method sets ADOCDR to value 0
impl crate::Resettable for ADOCDR_SPEC {}
