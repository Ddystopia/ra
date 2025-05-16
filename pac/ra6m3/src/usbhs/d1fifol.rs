///Register `D1FIFOL` reader
pub type R = crate::R<D1FIFOL_SPEC>;
///Register `D1FIFOL` writer
pub type W = crate::W<D1FIFOL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**D1FIFO Port Register L

You can [`read`](crate::Reg::read) this register and get [`d1fifol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct D1FIFOL_SPEC;
impl crate::RegisterSpec for D1FIFOL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`d1fifol::R`](R) reader structure
impl crate::Readable for D1FIFOL_SPEC {}
///`write(|w| ..)` method takes [`d1fifol::W`](W) writer structure
impl crate::Writable for D1FIFOL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D1FIFOL to value 0
impl crate::Resettable for D1FIFOL_SPEC {}
