///Register `BCNT3CP%s` reader
pub type R = crate::R<BCNT3CP_SPEC>;
///Field `BCNT3CP` reader - BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected.
pub type BCNT3CP_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - BCNT3CP is a read-only register that captures the BCNT3 value when a time capture event is detected.
    #[inline(always)]
    pub fn bcnt3cp(&self) -> BCNT3CP_R {
        BCNT3CP_R::new(self.bits)
    }
}
/**BCNT3 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt3cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT3CP_SPEC;
impl crate::RegisterSpec for BCNT3CP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt3cp::R`](R) reader structure
impl crate::Readable for BCNT3CP_SPEC {}
///`reset()` method sets BCNT3CP%s to value 0
impl crate::Resettable for BCNT3CP_SPEC {}
