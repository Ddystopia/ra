///Register `TSCDRL` reader
pub type R = crate::R<TSCDRL_SPEC>;
///Field `TSCDRL` reader - The calibration data stores the lower 8 bits of the converted value.
pub type TSCDRL_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - The calibration data stores the lower 8 bits of the converted value.
    #[inline(always)]
    pub fn tscdrl(&self) -> TSCDRL_R {
        TSCDRL_R::new(self.bits)
    }
}
/**Temperature Sensor Calibration Data Register L

You can [`read`](crate::Reg::read) this register and get [`tscdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSCDRL_SPEC;
impl crate::RegisterSpec for TSCDRL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tscdrl::R`](R) reader structure
impl crate::Readable for TSCDRL_SPEC {}
///`reset()` method sets TSCDRL to value 0
impl crate::Resettable for TSCDRL_SPEC {}
