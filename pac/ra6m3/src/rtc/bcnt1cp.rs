///Register `BCNT1CP%s` reader
pub type R = crate::R<BCNT1CP_SPEC>;
///Field `BCNT1CP` reader - BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected.
pub type BCNT1CP_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - BCNT1CP is a read-only register that captures the BCNT1 value when a time capture event is detected.
    #[inline(always)]
    pub fn bcnt1cp(&self) -> BCNT1CP_R {
        BCNT1CP_R::new(self.bits)
    }
}
/**BCNT1 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt1cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT1CP_SPEC;
impl crate::RegisterSpec for BCNT1CP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt1cp::R`](R) reader structure
impl crate::Readable for BCNT1CP_SPEC {}
///`reset()` method sets BCNT1CP%s to value 0
impl crate::Resettable for BCNT1CP_SPEC {}
