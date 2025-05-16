///Register `GTCCRE` reader
pub type R = crate::R<GTCCRE_SPEC>;
///Register `GTCCRE` writer
pub type W = crate::W<GTCCRE_SPEC>;
///Field `GTCCRE` reader - Compare Capture Register E
pub type GTCCRE_R = crate::FieldReader<u32>;
///Field `GTCCRE` writer - Compare Capture Register E
pub type GTCCRE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Compare Capture Register E
    #[inline(always)]
    pub fn gtccre(&self) -> GTCCRE_R {
        GTCCRE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Compare Capture Register E
    #[inline(always)]
    pub fn gtccre(&mut self) -> GTCCRE_W<GTCCRE_SPEC> {
        GTCCRE_W::new(self, 0)
    }
}
/**General PWM Timer Compare Capture Register E

You can [`read`](crate::Reg::read) this register and get [`gtccre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCCRE_SPEC;
impl crate::RegisterSpec for GTCCRE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtccre::R`](R) reader structure
impl crate::Readable for GTCCRE_SPEC {}
///`write(|w| ..)` method takes [`gtccre::W`](W) writer structure
impl crate::Writable for GTCCRE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRE to value 0xffff_ffff
impl crate::Resettable for GTCCRE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
