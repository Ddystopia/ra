///Register `ADDBLDRA` reader
pub type R = crate::R<ADDBLDRA_SPEC>;
///Field `ADDBLDRA` reader - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode.
pub type ADDBLDRA_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode.
    #[inline(always)]
    pub fn addbldra(&self) -> ADDBLDRA_R {
        ADDBLDRA_R::new(self.bits)
    }
}
/**A/D Data Duplication Register A

You can [`read`](crate::Reg::read) this register and get [`addbldra::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADDBLDRA_SPEC;
impl crate::RegisterSpec for ADDBLDRA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`addbldra::R`](R) reader structure
impl crate::Readable for ADDBLDRA_SPEC {}
///`reset()` method sets ADDBLDRA to value 0
impl crate::Resettable for ADDBLDRA_SPEC {}
