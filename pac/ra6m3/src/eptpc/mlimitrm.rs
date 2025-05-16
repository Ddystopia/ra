///Register `MLIMITRM` reader
pub type R = crate::R<MLIMITRM_SPEC>;
///Register `MLIMITRM` writer
pub type W = crate::W<MLIMITRM_SPEC>;
///Field `MLIMITRM` reader - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient.
pub type MLIMITRM_R = crate::FieldReader<u32>;
///Field `MLIMITRM` writer - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient.
pub type MLIMITRM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient.
    #[inline(always)]
    pub fn mlimitrm(&self) -> MLIMITRM_R {
        MLIMITRM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the negative gradient.
    #[inline(always)]
    pub fn mlimitrm(&mut self) -> MLIMITRM_W<MLIMITRM_SPEC> {
        MLIMITRM_W::new(self, 0)
    }
}
/**Negative Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`mlimitrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlimitrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MLIMITRM_SPEC;
impl crate::RegisterSpec for MLIMITRM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mlimitrm::R`](R) reader structure
impl crate::Readable for MLIMITRM_SPEC {}
///`write(|w| ..)` method takes [`mlimitrm::W`](W) writer structure
impl crate::Writable for MLIMITRM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MLIMITRM to value 0
impl crate::Resettable for MLIMITRM_SPEC {}
