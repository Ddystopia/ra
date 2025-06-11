///Register `BCNT0CP%s` reader
pub type R = crate::R<BCNT0CP_SPEC>;
///Field `BCNT0CP` reader - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected.
pub type BCNT0CP_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - BCNT0CP is a read-only register that captures the BCNT0 value when a time capture event is detected.
    #[inline(always)]
    pub fn bcnt0cp(&self) -> BCNT0CP_R {
        BCNT0CP_R::new(self.bits)
    }
}
/**BCNT0 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt0cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT0CP_SPEC;
impl crate::RegisterSpec for BCNT0CP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt0cp::R`](R) reader structure
impl crate::Readable for BCNT0CP_SPEC {}
///`reset()` method sets BCNT0CP%s to value 0
impl crate::Resettable for BCNT0CP_SPEC {}
