///Register `CFIFOH` reader
pub type R = crate::R<CFIFOH_SPEC>;
///Register `CFIFOH` writer
pub type W = crate::W<CFIFOH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CFIFO Port Register H

You can [`read`](crate::Reg::read) this register and get [`cfifoh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFIFOH_SPEC;
impl crate::RegisterSpec for CFIFOH_SPEC {
    type Ux = u16;
}
///`read()` method returns [`cfifoh::R`](R) reader structure
impl crate::Readable for CFIFOH_SPEC {}
///`write(|w| ..)` method takes [`cfifoh::W`](W) writer structure
impl crate::Writable for CFIFOH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOH to value 0
impl crate::Resettable for CFIFOH_SPEC {}
