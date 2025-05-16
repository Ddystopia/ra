///Register `TSR` reader
pub type R = crate::R<TSR_SPEC>;
///Field `TSR` reader - Free-running counter value for the time stamp function
pub type TSR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Free-running counter value for the time stamp function
    #[inline(always)]
    pub fn tsr(&self) -> TSR_R {
        TSR_R::new(self.bits)
    }
}
/**Time Stamp Register

You can [`read`](crate::Reg::read) this register and get [`tsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`tsr::R`](R) reader structure
impl crate::Readable for TSR_SPEC {}
///`reset()` method sets TSR to value 0
impl crate::Resettable for TSR_SPEC {}
