///Register `ADDBLDRB` reader
pub type R = crate::R<ADDBLDRB_SPEC>;
///Field `ADDBLDRB` reader - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode.
pub type ADDBLDRB_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - This register is a 16-bit read-only registers for storing the result of A/D conversion in response to the respective triggers during extended operation in double trigger mode.
    #[inline(always)]
    pub fn addbldrb(&self) -> ADDBLDRB_R {
        ADDBLDRB_R::new(self.bits)
    }
}
/**A/D Data Duplication Register B

You can [`read`](crate::Reg::read) this register and get [`addbldrb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADDBLDRB_SPEC;
impl crate::RegisterSpec for ADDBLDRB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`addbldrb::R`](R) reader structure
impl crate::Readable for ADDBLDRB_SPEC {}
///`reset()` method sets ADDBLDRB to value 0
impl crate::Resettable for ADDBLDRB_SPEC {}
