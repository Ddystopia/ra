///Register `D0FIFOH` reader
pub type R = crate::R<D0FIFOH_SPEC>;
///Register `D0FIFOH` writer
pub type W = crate::W<D0FIFOH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D0FIFO Port Register H

You can [`read`](crate::Reg::read) this register and get [`d0fifoh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D0FIFOH_SPEC;
impl crate::RegisterSpec for D0FIFOH_SPEC {
    type Ux = u16;
}
///`read()` method returns [`d0fifoh::R`](R) reader structure
impl crate::Readable for D0FIFOH_SPEC {}
///`write(|w| ..)` method takes [`d0fifoh::W`](W) writer structure
impl crate::Writable for D0FIFOH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D0FIFOH to value 0
impl crate::Resettable for D0FIFOH_SPEC {}
