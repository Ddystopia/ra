///Register `BCNT2CP%s` reader
pub type R = crate::R<BCNT2CP_SPEC>;
///Field `BCNT2CP` reader - BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected.
pub type BCNT2CP_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - BCNT2CP is a read-only register that captures the BCNT2 value when a time capture event is detected.
    #[inline(always)]
    pub fn bcnt2cp(&self) -> BCNT2CP_R {
        BCNT2CP_R::new(self.bits)
    }
}
/**BCNT2 Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`bcnt2cp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCNT2CP_SPEC;
impl crate::RegisterSpec for BCNT2CP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bcnt2cp::R`](R) reader structure
impl crate::Readable for BCNT2CP_SPEC {}
///`reset()` method sets BCNT2CP%s to value 0
impl crate::Resettable for BCNT2CP_SPEC {}
