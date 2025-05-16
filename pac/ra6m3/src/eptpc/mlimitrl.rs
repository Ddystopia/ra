///Register `MLIMITRL` reader
pub type R = crate::R<MLIMITRL_SPEC>;
///Register `MLIMITRL` writer
pub type W = crate::W<MLIMITRL_SPEC>;
///Field `MLIMITRL` reader - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient.
pub type MLIMITRL_R = crate::FieldReader<u32>;
///Field `MLIMITRL` writer - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient.
pub type MLIMITRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient.
    #[inline(always)]
    pub fn mlimitrl(&self) -> MLIMITRL_R {
        MLIMITRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the negative gradient.
    #[inline(always)]
    pub fn mlimitrl(&mut self) -> MLIMITRL_W<MLIMITRL_SPEC> {
        MLIMITRL_W::new(self, 0)
    }
}
/**Negative Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`mlimitrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlimitrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MLIMITRL_SPEC;
impl crate::RegisterSpec for MLIMITRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mlimitrl::R`](R) reader structure
impl crate::Readable for MLIMITRL_SPEC {}
///`write(|w| ..)` method takes [`mlimitrl::W`](W) writer structure
impl crate::Writable for MLIMITRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MLIMITRL to value 0
impl crate::Resettable for MLIMITRL_SPEC {}
