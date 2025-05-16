///Register `LCCVRM` reader
pub type R = crate::R<LCCVRM_SPEC>;
///Field `LCCVRM` reader - These bits are for reading the lower-order 32 bits of the integer portion of the local timer counter's value.
pub type LCCVRM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the lower-order 32 bits of the integer portion of the local timer counter's value.
    #[inline(always)]
    pub fn lccvrm(&self) -> LCCVRM_R {
        LCCVRM_R::new(self.bits)
    }
}
/**Local Time Counters

You can [`read`](crate::Reg::read) this register and get [`lccvrm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCCVRM_SPEC;
impl crate::RegisterSpec for LCCVRM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lccvrm::R`](R) reader structure
impl crate::Readable for LCCVRM_SPEC {}
///`reset()` method sets LCCVRM to value 0
impl crate::Resettable for LCCVRM_SPEC {}
