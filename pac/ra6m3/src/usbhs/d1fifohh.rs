///Register `D1FIFOHH` reader
pub type R = crate::R<D1FIFOHH_SPEC>;
///Register `D1FIFOHH` writer
pub type W = crate::W<D1FIFOHH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D1FIFO Port Register HH

You can [`read`](crate::Reg::read) this register and get [`d1fifohh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifohh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1FIFOHH_SPEC;
impl crate::RegisterSpec for D1FIFOHH_SPEC {
    type Ux = u8;
}
///`read()` method returns [`d1fifohh::R`](R) reader structure
impl crate::Readable for D1FIFOHH_SPEC {}
///`write(|w| ..)` method takes [`d1fifohh::W`](W) writer structure
impl crate::Writable for D1FIFOHH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFOHH to value 0
impl crate::Resettable for D1FIFOHH_SPEC {}
