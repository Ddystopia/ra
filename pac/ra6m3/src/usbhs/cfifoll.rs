///Register `CFIFOLL` reader
pub type R = crate::R<CFIFOLL_SPEC>;
///Register `CFIFOLL` writer
pub type W = crate::W<CFIFOLL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CFIFO Port Register LL

You can [`read`](crate::Reg::read) this register and get [`cfifoll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFIFOLL_SPEC;
impl crate::RegisterSpec for CFIFOLL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cfifoll::R`](R) reader structure
impl crate::Readable for CFIFOLL_SPEC {}
///`write(|w| ..)` method takes [`cfifoll::W`](W) writer structure
impl crate::Writable for CFIFOLL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOLL to value 0
impl crate::Resettable for CFIFOLL_SPEC {}
