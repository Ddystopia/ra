///Register `TMPLSR%s` reader
pub type R = crate::R<TMPLSR_SPEC>;
///Register `TMPLSR%s` writer
pub type W = crate::W<TMPLSR_SPEC>;
///Field `TMPLSR` reader - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock.
pub type TMPLSR_R = crate::FieldReader<u32>;
///Field `TMPLSR` writer - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock.
pub type TMPLSR_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock.
    #[inline(always)]
    pub fn tmplsr(&self) -> TMPLSR_R {
        TMPLSR_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    ///Bits 0:28 - These bits set the width at high level of the pulse signal from the timer in nanoseconds. Set a value that is equivalent to at least two cycles of the STCA clock.
    #[inline(always)]
    pub fn tmplsr(&mut self) -> TMPLSR_W<TMPLSR_SPEC> {
        TMPLSR_W::new(self, 0)
    }
}
/**Timer Pulse Width Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`tmplsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmplsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TMPLSR_SPEC;
impl crate::RegisterSpec for TMPLSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tmplsr::R`](R) reader structure
impl crate::Readable for TMPLSR_SPEC {}
///`write(|w| ..)` method takes [`tmplsr::W`](W) writer structure
impl crate::Writable for TMPLSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMPLSR%s to value 0
impl crate::Resettable for TMPLSR_SPEC {}
