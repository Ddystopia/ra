///Register `CTSURC` reader
pub type R = crate::R<CTSURC_SPEC>;
///Field `CTSURC` reader - CTSU Reference Counter
pub type CTSURC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - CTSU Reference Counter
    #[inline(always)]
    pub fn ctsurc(&self) -> CTSURC_R {
        CTSURC_R::new(self.bits)
    }
}
/**CTSU Reference Counter

You can [`read`](crate::Reg::read) this register and get [`ctsurc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSURC_SPEC;
impl crate::RegisterSpec for CTSURC_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctsurc::R`](R) reader structure
impl crate::Readable for CTSURC_SPEC {}
///`reset()` method sets CTSURC to value 0
impl crate::Resettable for CTSURC_SPEC {}
