///Register `LCCVRL` reader
pub type R = crate::R<LCCVRL_SPEC>;
///Field `LCCVRL` reader - These bits are for reading the fractional portion of the local timer counter's value (in nanoseconds).
pub type LCCVRL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the fractional portion of the local timer counter's value (in nanoseconds).
    #[inline(always)]
    pub fn lccvrl(&self) -> LCCVRL_R {
        LCCVRL_R::new(self.bits)
    }
}
/**Local Time Counters

You can [`read`](crate::Reg::read) this register and get [`lccvrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCCVRL_SPEC;
impl crate::RegisterSpec for LCCVRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lccvrl::R`](R) reader structure
impl crate::Readable for LCCVRL_SPEC {}
///`reset()` method sets LCCVRL to value 0
impl crate::Resettable for LCCVRL_SPEC {}
