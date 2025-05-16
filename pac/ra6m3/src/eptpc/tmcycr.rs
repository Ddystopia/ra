///Register `TMCYCR%s` reader
pub type R = crate::R<TMCYCR_SPEC>;
///Register `TMCYCR%s` writer
pub type W = crate::W<TMCYCR_SPEC>;
///Field `TMCYCR` reader - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock.
pub type TMCYCR_R = crate::FieldReader<u32>;
///Field `TMCYCR` writer - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock.
pub type TMCYCR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 0:29 - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock.
    #[inline(always)]
    pub fn tmcycr(&self) -> TMCYCR_R {
        TMCYCR_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 0:29 - These bits set the cycle of the pulse output timer in nanoseconds. Set a value that is equivalent to at least four cycles of the STCA clock.
    #[inline(always)]
    pub fn tmcycr(&mut self) -> TMCYCR_W<TMCYCR_SPEC> {
        TMCYCR_W::new(self, 0)
    }
}
/**Timer Cycle Setting Registers %s

You can [`read`](crate::Reg::read) this register and get [`tmcycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmcycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TMCYCR_SPEC;
impl crate::RegisterSpec for TMCYCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tmcycr::R`](R) reader structure
impl crate::Readable for TMCYCR_SPEC {}
///`write(|w| ..)` method takes [`tmcycr::W`](W) writer structure
impl crate::Writable for TMCYCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMCYCR%s to value 0
impl crate::Resettable for TMCYCR_SPEC {}
