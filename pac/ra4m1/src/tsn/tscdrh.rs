///Register `TSCDRH` reader
pub type R = crate::R<TSCDRH_SPEC>;
///Field `TSCDRH` reader - The calibration data stores the higher 8 bits of the converted value.
pub type TSCDRH_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - The calibration data stores the higher 8 bits of the converted value.
    #[inline(always)]
    pub fn tscdrh(&self) -> TSCDRH_R {
        TSCDRH_R::new(self.bits)
    }
}
/**Temperature Sensor Calibration Data Register H

You can [`read`](crate::Reg::read) this register and get [`tscdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSCDRH_SPEC;
impl crate::RegisterSpec for TSCDRH_SPEC {
    type Ux = u8;
}
///`read()` method returns [`tscdrh::R`](R) reader structure
impl crate::Readable for TSCDRH_SPEC {}
///`reset()` method sets TSCDRH to value 0
impl crate::Resettable for TSCDRH_SPEC {}
