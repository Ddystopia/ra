///Register `CTSUSC` reader
pub type R = crate::R<CTSUSC_SPEC>;
///Field `CTSUSC` reader - CTSU Sensor CounterThese bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs.
pub type CTSUSC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - CTSU Sensor CounterThese bits indicate the measurement result of the CTSU. These bits indicate FFFFh when an overflow occurs.
    #[inline(always)]
    pub fn ctsusc(&self) -> CTSUSC_R {
        CTSUSC_R::new(self.bits)
    }
}
/**CTSU Sensor Counter

You can [`read`](crate::Reg::read) this register and get [`ctsusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUSC_SPEC;
impl crate::RegisterSpec for CTSUSC_SPEC {
    type Ux = u16;
}
///`read()` method returns [`ctsusc::R`](R) reader structure
impl crate::Readable for CTSUSC_SPEC {}
///`reset()` method sets CTSUSC to value 0
impl crate::Resettable for CTSUSC_SPEC {}
