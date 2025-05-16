///Register `D1FIFOLL` reader
pub type R = crate::R<D1FIFOLL_SPEC>;
///Register `D1FIFOLL` writer
pub type W = crate::W<D1FIFOLL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D1FIFO Port Register LL

You can [`read`](crate::Reg::read) this register and get [`d1fifoll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1FIFOLL_SPEC;
impl crate::RegisterSpec for D1FIFOLL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`d1fifoll::R`](R) reader structure
impl crate::Readable for D1FIFOLL_SPEC {}
///`write(|w| ..)` method takes [`d1fifoll::W`](W) writer structure
impl crate::Writable for D1FIFOLL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFOLL to value 0
impl crate::Resettable for D1FIFOLL_SPEC {}
