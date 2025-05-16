///Register `RFCF` reader
pub type R = crate::R<RFCF_SPEC>;
///Field `RPAUSE` reader - Received PAUSE Frame CountNumber of received PAUSE frames
pub type RPAUSE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Received PAUSE Frame CountNumber of received PAUSE frames
    #[inline(always)]
    pub fn rpause(&self) -> RPAUSE_R {
        RPAUSE_R::new((self.bits & 0xff) as u8)
    }
}
/**Received PAUSE Frame Counter

You can [`read`](crate::Reg::read) this register and get [`rfcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFCF_SPEC;
impl crate::RegisterSpec for RFCF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rfcf::R`](R) reader structure
impl crate::Readable for RFCF_SPEC {}
///`reset()` method sets RFCF to value 0
impl crate::Resettable for RFCF_SPEC {}
