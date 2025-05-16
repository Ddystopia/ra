///Register `ADTSDR` reader
pub type R = crate::R<ADTSDR_SPEC>;
///Field `ADTSDR` reader - This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output.
pub type ADTSDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - This is a 16-bit read-only register for storing the A/D conversion result of temperature sensor output.
    #[inline(always)]
    pub fn adtsdr(&self) -> ADTSDR_R {
        ADTSDR_R::new(self.bits)
    }
}
/**A/D Temperature Sensor Data Register

You can [`read`](crate::Reg::read) this register and get [`adtsdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADTSDR_SPEC;
impl crate::RegisterSpec for ADTSDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adtsdr::R`](R) reader structure
impl crate::Readable for ADTSDR_SPEC {}
///`reset()` method sets ADTSDR to value 0
impl crate::Resettable for ADTSDR_SPEC {}
