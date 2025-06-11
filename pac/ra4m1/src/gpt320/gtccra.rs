///Register `GTCCRA` reader
pub type R = crate::R<GTCCRA_SPEC>;
///Register `GTCCRA` writer
pub type W = crate::W<GTCCRA_SPEC>;
///Field `GTCCRA` reader - Compare Capture Register A
pub type GTCCRA_R = crate::FieldReader<u32>;
///Field `GTCCRA` writer - Compare Capture Register A
pub type GTCCRA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Compare Capture Register A
    #[inline(always)]
    pub fn gtccra(&self) -> GTCCRA_R {
        GTCCRA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Compare Capture Register A
    #[inline(always)]
    pub fn gtccra(&mut self) -> GTCCRA_W<GTCCRA_SPEC> {
        GTCCRA_W::new(self, 0)
    }
}
/**General PWM Timer Compare Capture Register A

You can [`read`](crate::Reg::read) this register and get [`gtccra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCCRA_SPEC;
impl crate::RegisterSpec for GTCCRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtccra::R`](R) reader structure
impl crate::Readable for GTCCRA_SPEC {}
///`write(|w| ..)` method takes [`gtccra::W`](W) writer structure
impl crate::Writable for GTCCRA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRA to value 0xffff_ffff
impl crate::Resettable for GTCCRA_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
