///Register `D0FIFOHH` reader
pub type R = crate::R<D0FIFOHH_SPEC>;
///Register `D0FIFOHH` writer
pub type W = crate::W<D0FIFOHH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D0FIFO Port Register HH

You can [`read`](crate::Reg::read) this register and get [`d0fifohh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifohh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D0FIFOHH_SPEC;
impl crate::RegisterSpec for D0FIFOHH_SPEC {
    type Ux = u8;
}
///`read()` method returns [`d0fifohh::R`](R) reader structure
impl crate::Readable for D0FIFOHH_SPEC {}
///`write(|w| ..)` method takes [`d0fifohh::W`](W) writer structure
impl crate::Writable for D0FIFOHH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D0FIFOHH to value 0
impl crate::Resettable for D0FIFOHH_SPEC {}
