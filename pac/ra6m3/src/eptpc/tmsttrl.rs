///Register `TMSTTRL%s` reader
pub type R = crate::R<TMSTTRL_SPEC>;
///Register `TMSTTRL%s` writer
pub type W = crate::W<TMSTTRL_SPEC>;
///Field `TMSTTRL` reader - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds.
pub type TMSTTRL_R = crate::FieldReader<u32>;
///Field `TMSTTRL` writer - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds.
pub type TMSTTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds.
    #[inline(always)]
    pub fn tmsttrl(&self) -> TMSTTRL_R {
        TMSTTRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the start time of the pulse output timer in nanoseconds.
    #[inline(always)]
    pub fn tmsttrl(&mut self) -> TMSTTRL_W<TMSTTRL_SPEC> {
        TMSTTRL_W::new(self, 0)
    }
}
/**Timer Start Time Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`tmsttrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsttrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TMSTTRL_SPEC;
impl crate::RegisterSpec for TMSTTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tmsttrl::R`](R) reader structure
impl crate::Readable for TMSTTRL_SPEC {}
///`write(|w| ..)` method takes [`tmsttrl::W`](W) writer structure
impl crate::Writable for TMSTTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMSTTRL%s to value 0
impl crate::Resettable for TMSTTRL_SPEC {}
