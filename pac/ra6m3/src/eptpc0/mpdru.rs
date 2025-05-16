///Register `MPDRU` reader
pub type R = crate::R<MPDRU_SPEC>;
///Field `MPDRU` reader - These bits indicate the higher-order 32 bits of the calculated meanPathDelay value.
pub type MPDRU_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits indicate the higher-order 32 bits of the calculated meanPathDelay value.
    #[inline(always)]
    pub fn mpdru(&self) -> MPDRU_R {
        MPDRU_R::new(self.bits)
    }
}
/**meanPathDelay Value Registers

You can [`read`](crate::Reg::read) this register and get [`mpdru::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MPDRU_SPEC;
impl crate::RegisterSpec for MPDRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mpdru::R`](R) reader structure
impl crate::Readable for MPDRU_SPEC {}
///`reset()` method sets MPDRU to value 0
impl crate::Resettable for MPDRU_SPEC {}
