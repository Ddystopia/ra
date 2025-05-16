///Register `TMSTTRU%s` reader
pub type R = crate::R<TMSTTRU_SPEC>;
///Register `TMSTTRU%s` writer
pub type W = crate::W<TMSTTRU_SPEC>;
///Field `TMSTTRU` reader - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds.
pub type TMSTTRU_R = crate::FieldReader<u32>;
///Field `TMSTTRU` writer - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds.
pub type TMSTTRU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds.
    #[inline(always)]
    pub fn tmsttru(&self) -> TMSTTRU_R {
        TMSTTRU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the start time of the pulse output timer in nanoseconds.
    #[inline(always)]
    pub fn tmsttru(&mut self) -> TMSTTRU_W<TMSTTRU_SPEC> {
        TMSTTRU_W::new(self, 0)
    }
}
/**Timer Start Time Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`tmsttru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsttru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TMSTTRU_SPEC;
impl crate::RegisterSpec for TMSTTRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tmsttru::R`](R) reader structure
impl crate::Readable for TMSTTRU_SPEC {}
///`write(|w| ..)` method takes [`tmsttru::W`](W) writer structure
impl crate::Writable for TMSTTRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMSTTRU%s to value 0
impl crate::Resettable for TMSTTRU_SPEC {}
