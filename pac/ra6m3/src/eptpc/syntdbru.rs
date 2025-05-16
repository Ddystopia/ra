///Register `SYNTDBRU` reader
pub type R = crate::R<SYNTDBRU_SPEC>;
///Register `SYNTDBRU` writer
pub type W = crate::W<SYNTDBRU_SPEC>;
///Field `SYNTDBRU` reader - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization.
pub type SYNTDBRU_R = crate::FieldReader<u32>;
///Field `SYNTDBRU` writer - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization.
pub type SYNTDBRU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization.
    #[inline(always)]
    pub fn syntdbru(&self) -> SYNTDBRU_R {
        SYNTDBRU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the threshold for detection of synchronization.
    #[inline(always)]
    pub fn syntdbru(&mut self) -> SYNTDBRU_W<SYNTDBRU_SPEC> {
        SYNTDBRU_W::new(self, 0)
    }
}
/**Synchronization Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdbru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdbru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNTDBRU_SPEC;
impl crate::RegisterSpec for SYNTDBRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syntdbru::R`](R) reader structure
impl crate::Readable for SYNTDBRU_SPEC {}
///`write(|w| ..)` method takes [`syntdbru::W`](W) writer structure
impl crate::Writable for SYNTDBRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTDBRU to value 0
impl crate::Resettable for SYNTDBRU_SPEC {}
