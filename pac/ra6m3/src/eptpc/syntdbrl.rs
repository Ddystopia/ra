///Register `SYNTDBRL` reader
pub type R = crate::R<SYNTDBRL_SPEC>;
///Register `SYNTDBRL` writer
pub type W = crate::W<SYNTDBRL_SPEC>;
///Field `SYNTDBRL` reader - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization.
pub type SYNTDBRL_R = crate::FieldReader<u32>;
///Field `SYNTDBRL` writer - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization.
pub type SYNTDBRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization.
    #[inline(always)]
    pub fn syntdbrl(&self) -> SYNTDBRL_R {
        SYNTDBRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the threshold for detection of synchronization.
    #[inline(always)]
    pub fn syntdbrl(&mut self) -> SYNTDBRL_W<SYNTDBRL_SPEC> {
        SYNTDBRL_W::new(self, 0)
    }
}
/**Synchronization Detection Threshold Registers

You can [`read`](crate::Reg::read) this register and get [`syntdbrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syntdbrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNTDBRL_SPEC;
impl crate::RegisterSpec for SYNTDBRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syntdbrl::R`](R) reader structure
impl crate::Readable for SYNTDBRL_SPEC {}
///`write(|w| ..)` method takes [`syntdbrl::W`](W) writer structure
impl crate::Writable for SYNTDBRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNTDBRL to value 0
impl crate::Resettable for SYNTDBRL_SPEC {}
