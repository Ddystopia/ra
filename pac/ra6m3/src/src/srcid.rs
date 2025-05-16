///Register `SRCID` writer
pub type W = crate::W<SRCID_SPEC>;
///Field `SRCID` writer - SRCID is a 32-bit writ-only register that is used to input the data before sampling rate conversion. All the bits are read as 0.
pub type SRCID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    ///Bits 0:31 - SRCID is a 32-bit writ-only register that is used to input the data before sampling rate conversion. All the bits are read as 0.
    #[inline(always)]
    pub fn srcid(&mut self) -> SRCID_W<SRCID_SPEC> {
        SRCID_W::new(self, 0)
    }
}
/**Input Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcid::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCID_SPEC;
impl crate::RegisterSpec for SRCID_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`srcid::W`](W) writer structure
impl crate::Writable for SRCID_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCID to value 0
impl crate::Resettable for SRCID_SPEC {}
