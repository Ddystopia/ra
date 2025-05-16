///Register `CACNTBR` reader
pub type R = crate::R<CACNTBR_SPEC>;
///Field `CACNTBR` reader - CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input
pub type CACNTBR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - CACNTBR is a 16-bit read-only register that retains the counter value at the time a valid reference signal edge is input
    #[inline(always)]
    pub fn cacntbr(&self) -> CACNTBR_R {
        CACNTBR_R::new(self.bits)
    }
}
/**CAC Counter Buffer Register

You can [`read`](crate::Reg::read) this register and get [`cacntbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACNTBR_SPEC;
impl crate::RegisterSpec for CACNTBR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`cacntbr::R`](R) reader structure
impl crate::Readable for CACNTBR_SPEC {}
///`reset()` method sets CACNTBR to value 0
impl crate::Resettable for CACNTBR_SPEC {}
