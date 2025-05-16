///Register `MPDRL` reader
pub type R = crate::R<MPDRL_SPEC>;
///Field `MPDRL` reader - These bits indicate the lower-order 32 bits of the calculated meanPathDelay value.
pub type MPDRL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits indicate the lower-order 32 bits of the calculated meanPathDelay value.
    #[inline(always)]
    pub fn mpdrl(&self) -> MPDRL_R {
        MPDRL_R::new(self.bits)
    }
}
/**meanPathDelay Value Registers

You can [`read`](crate::Reg::read) this register and get [`mpdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MPDRL_SPEC;
impl crate::RegisterSpec for MPDRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mpdrl::R`](R) reader structure
impl crate::Readable for MPDRL_SPEC {}
///`reset()` method sets MPDRL to value 0
impl crate::Resettable for MPDRL_SPEC {}
