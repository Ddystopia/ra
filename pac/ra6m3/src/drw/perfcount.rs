///Register `PERFCOUNT%s` reader
pub type R = crate::R<PERFCOUNT_SPEC>;
///Register `PERFCOUNT%s` writer
pub type W = crate::W<PERFCOUNT_SPEC>;
///Field `PERFCOUNT` reader - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H.
pub type PERFCOUNT_R = crate::FieldReader<u32>;
///Field `PERFCOUNT` writer - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H.
pub type PERFCOUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H.
    #[inline(always)]
    pub fn perfcount(&self) -> PERFCOUNT_R {
        PERFCOUNT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Counter value.The counter is reset by writing PERFCOUNT = 0000 0000H.
    #[inline(always)]
    pub fn perfcount(&mut self) -> PERFCOUNT_W<PERFCOUNT_SPEC> {
        PERFCOUNT_W::new(self, 0)
    }
}
/**Performance Counter %s

You can [`read`](crate::Reg::read) this register and get [`perfcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERFCOUNT_SPEC;
impl crate::RegisterSpec for PERFCOUNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`perfcount::R`](R) reader structure
impl crate::Readable for PERFCOUNT_SPEC {}
///`write(|w| ..)` method takes [`perfcount::W`](W) writer structure
impl crate::Writable for PERFCOUNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PERFCOUNT%s to value 0
impl crate::Resettable for PERFCOUNT_SPEC {}
