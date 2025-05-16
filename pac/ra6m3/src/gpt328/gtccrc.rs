///Register `GTCCRC` reader
pub type R = crate::R<GTCCRC_SPEC>;
///Register `GTCCRC` writer
pub type W = crate::W<GTCCRC_SPEC>;
///Field `GTCCRC` reader - Compare Capture Register C
pub type GTCCRC_R = crate::FieldReader<u32>;
///Field `GTCCRC` writer - Compare Capture Register C
pub type GTCCRC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Compare Capture Register C
    #[inline(always)]
    pub fn gtccrc(&self) -> GTCCRC_R {
        GTCCRC_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Compare Capture Register C
    #[inline(always)]
    pub fn gtccrc(&mut self) -> GTCCRC_W<GTCCRC_SPEC> {
        GTCCRC_W::new(self, 0)
    }
}
/**General PWM Timer Compare Capture Register C

You can [`read`](crate::Reg::read) this register and get [`gtccrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCCRC_SPEC;
impl crate::RegisterSpec for GTCCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtccrc::R`](R) reader structure
impl crate::Readable for GTCCRC_SPEC {}
///`write(|w| ..)` method takes [`gtccrc::W`](W) writer structure
impl crate::Writable for GTCCRC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRC to value 0xffff_ffff
impl crate::Resettable for GTCCRC_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
