///Register `AFSR` reader
pub type R = crate::R<AFSR_SPEC>;
///Register `AFSR` writer
pub type W = crate::W<AFSR_SPEC>;
///Field `AFSR` reader - After the standard ID of a received message is written, the value converted for data table search can be read.
pub type AFSR_R = crate::FieldReader<u16>;
///Field `AFSR` writer - After the standard ID of a received message is written, the value converted for data table search can be read.
pub type AFSR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - After the standard ID of a received message is written, the value converted for data table search can be read.
    #[inline(always)]
    pub fn afsr(&self) -> AFSR_R {
        AFSR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - After the standard ID of a received message is written, the value converted for data table search can be read.
    #[inline(always)]
    pub fn afsr(&mut self) -> AFSR_W<AFSR_SPEC> {
        AFSR_W::new(self, 0)
    }
}
/**Acceptance Filter Support Register

You can [`read`](crate::Reg::read) this register and get [`afsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AFSR_SPEC;
impl crate::RegisterSpec for AFSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`afsr::R`](R) reader structure
impl crate::Readable for AFSR_SPEC {}
///`write(|w| ..)` method takes [`afsr::W`](W) writer structure
impl crate::Writable for AFSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFSR to value 0
impl crate::Resettable for AFSR_SPEC {}
