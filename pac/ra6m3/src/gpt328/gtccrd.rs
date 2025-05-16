///Register `GTCCRD` reader
pub type R = crate::R<GTCCRD_SPEC>;
///Register `GTCCRD` writer
pub type W = crate::W<GTCCRD_SPEC>;
///Field `GTCCRD` reader - Compare Capture Register D
pub type GTCCRD_R = crate::FieldReader<u32>;
///Field `GTCCRD` writer - Compare Capture Register D
pub type GTCCRD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Compare Capture Register D
    #[inline(always)]
    pub fn gtccrd(&self) -> GTCCRD_R {
        GTCCRD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Compare Capture Register D
    #[inline(always)]
    pub fn gtccrd(&mut self) -> GTCCRD_W<GTCCRD_SPEC> {
        GTCCRD_W::new(self, 0)
    }
}
/**General PWM Timer Compare Capture Register D

You can [`read`](crate::Reg::read) this register and get [`gtccrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCCRD_SPEC;
impl crate::RegisterSpec for GTCCRD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtccrd::R`](R) reader structure
impl crate::Readable for GTCCRD_SPEC {}
///`write(|w| ..)` method takes [`gtccrd::W`](W) writer structure
impl crate::Writable for GTCCRD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRD to value 0xffff_ffff
impl crate::Resettable for GTCCRD_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
