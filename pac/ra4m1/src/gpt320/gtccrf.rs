///Register `GTCCRF` reader
pub type R = crate::R<GTCCRF_SPEC>;
///Register `GTCCRF` writer
pub type W = crate::W<GTCCRF_SPEC>;
///Field `GTCCRF` reader - Compare Capture Register F
pub type GTCCRF_R = crate::FieldReader<u32>;
///Field `GTCCRF` writer - Compare Capture Register F
pub type GTCCRF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Compare Capture Register F
    #[inline(always)]
    pub fn gtccrf(&self) -> GTCCRF_R {
        GTCCRF_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Compare Capture Register F
    #[inline(always)]
    pub fn gtccrf(&mut self) -> GTCCRF_W<GTCCRF_SPEC> {
        GTCCRF_W::new(self, 0)
    }
}
/**General PWM Timer Compare Capture Register F

You can [`read`](crate::Reg::read) this register and get [`gtccrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCCRF_SPEC;
impl crate::RegisterSpec for GTCCRF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtccrf::R`](R) reader structure
impl crate::Readable for GTCCRF_SPEC {}
///`write(|w| ..)` method takes [`gtccrf::W`](W) writer structure
impl crate::Writable for GTCCRF_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRF to value 0xffff_ffff
impl crate::Resettable for GTCCRF_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
