///Register `PLIMITRL` reader
pub type R = crate::R<PLIMITRL_SPEC>;
///Register `PLIMITRL` writer
pub type W = crate::W<PLIMITRL_SPEC>;
///Field `PLIMITRL` reader - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient.
pub type PLIMITRL_R = crate::FieldReader<u32>;
///Field `PLIMITRL` writer - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient.
pub type PLIMITRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient.
    #[inline(always)]
    pub fn plimitrl(&self) -> PLIMITRL_R {
        PLIMITRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the limit for the positive gradient.
    #[inline(always)]
    pub fn plimitrl(&mut self) -> PLIMITRL_W<PLIMITRL_SPEC> {
        PLIMITRL_W::new(self, 0)
    }
}
/**Positive Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`plimitrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plimitrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLIMITRL_SPEC;
impl crate::RegisterSpec for PLIMITRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`plimitrl::R`](R) reader structure
impl crate::Readable for PLIMITRL_SPEC {}
///`write(|w| ..)` method takes [`plimitrl::W`](W) writer structure
impl crate::Writable for PLIMITRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLIMITRL to value 0
impl crate::Resettable for PLIMITRL_SPEC {}
