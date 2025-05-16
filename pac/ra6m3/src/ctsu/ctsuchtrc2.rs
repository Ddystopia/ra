///Register `CTSUCHTRC2` reader
pub type R = crate::R<CTSUCHTRC2_SPEC>;
///Register `CTSUCHTRC2` writer
pub type W = crate::W<CTSUCHTRC2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**CTSU Channel Transmit/Receive Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUCHTRC2_SPEC;
impl crate::RegisterSpec for CTSUCHTRC2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsuchtrc2::R`](R) reader structure
impl crate::Readable for CTSUCHTRC2_SPEC {}
///`write(|w| ..)` method takes [`ctsuchtrc2::W`](W) writer structure
impl crate::Writable for CTSUCHTRC2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUCHTRC2 to value 0
impl crate::Resettable for CTSUCHTRC2_SPEC {}
