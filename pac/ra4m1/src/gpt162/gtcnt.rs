///Register `GTCNT` reader
pub type R = crate::R<GTCNT_SPEC>;
///Register `GTCNT` writer
pub type W = crate::W<GTCNT_SPEC>;
///Field `GTCNT` reader - Counter
pub type GTCNT_R = crate::FieldReader<u32>;
///Field `GTCNT` writer - Counter
pub type GTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Counter
    #[inline(always)]
    pub fn gtcnt(&self) -> GTCNT_R {
        GTCNT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Counter
    #[inline(always)]
    pub fn gtcnt(&mut self) -> GTCNT_W<GTCNT_SPEC> {
        GTCNT_W::new(self, 0)
    }
}
/**General PWM Timer Counter

You can [`read`](crate::Reg::read) this register and get [`gtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCNT_SPEC;
impl crate::RegisterSpec for GTCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtcnt::R`](R) reader structure
impl crate::Readable for GTCNT_SPEC {}
///`write(|w| ..)` method takes [`gtcnt::W`](W) writer structure
impl crate::Writable for GTCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCNT to value 0
impl crate::Resettable for GTCNT_SPEC {}
