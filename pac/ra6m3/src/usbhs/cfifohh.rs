///Register `CFIFOHH` reader
pub type R = crate::R<CFIFOHH_SPEC>;
///Register `CFIFOHH` writer
pub type W = crate::W<CFIFOHH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CFIFO Port Register HH

You can [`read`](crate::Reg::read) this register and get [`cfifohh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifohh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFIFOHH_SPEC;
impl crate::RegisterSpec for CFIFOHH_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cfifohh::R`](R) reader structure
impl crate::Readable for CFIFOHH_SPEC {}
///`write(|w| ..)` method takes [`cfifohh::W`](W) writer structure
impl crate::Writable for CFIFOHH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOHH to value 0
impl crate::Resettable for CFIFOHH_SPEC {}
