///Register `GTPDBR` reader
pub type R = crate::R<GTPDBR_SPEC>;
///Register `GTPDBR` writer
pub type W = crate::W<GTPDBR_SPEC>;
///Field `GTPDBR` reader - Cycle Setting Double-Buffer Register
pub type GTPDBR_R = crate::FieldReader<u32>;
///Field `GTPDBR` writer - Cycle Setting Double-Buffer Register
pub type GTPDBR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Cycle Setting Double-Buffer Register
    #[inline(always)]
    pub fn gtpdbr(&self) -> GTPDBR_R {
        GTPDBR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Cycle Setting Double-Buffer Register
    #[inline(always)]
    pub fn gtpdbr(&mut self) -> GTPDBR_W<GTPDBR_SPEC> {
        GTPDBR_W::new(self, 0)
    }
}
/**General PWM Timer Cycle Setting Double-Buffer Register

You can [`read`](crate::Reg::read) this register and get [`gtpdbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpdbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTPDBR_SPEC;
impl crate::RegisterSpec for GTPDBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtpdbr::R`](R) reader structure
impl crate::Readable for GTPDBR_SPEC {}
///`write(|w| ..)` method takes [`gtpdbr::W`](W) writer structure
impl crate::Writable for GTPDBR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPDBR to value 0xffff_ffff
impl crate::Resettable for GTPDBR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
