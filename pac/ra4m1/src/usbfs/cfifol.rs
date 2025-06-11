///Register `CFIFOL` reader
pub type R = crate::R<CFIFOL_SPEC>;
///Register `CFIFOL` writer
pub type W = crate::W<CFIFOL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CFIFO Port Register L

You can [`read`](crate::Reg::read) this register and get [`cfifol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFIFOL_SPEC;
impl crate::RegisterSpec for CFIFOL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`cfifol::R`](R) reader structure
impl crate::Readable for CFIFOL_SPEC {}
///`write(|w| ..)` method takes [`cfifol::W`](W) writer structure
impl crate::Writable for CFIFOL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFIFOL to value 0
impl crate::Resettable for CFIFOL_SPEC {}
