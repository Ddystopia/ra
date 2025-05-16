///Register `PLIMITRM` reader
pub type R = crate::R<PLIMITRM_SPEC>;
///Register `PLIMITRM` writer
pub type W = crate::W<PLIMITRM_SPEC>;
///Field `PLIMITRM` reader - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient.
pub type PLIMITRM_R = crate::FieldReader<u32>;
///Field `PLIMITRM` writer - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient.
pub type PLIMITRM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient.
    #[inline(always)]
    pub fn plimitrm(&self) -> PLIMITRM_R {
        PLIMITRM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the middle-order 32 bits of the limit for the positive gradient.
    #[inline(always)]
    pub fn plimitrm(&mut self) -> PLIMITRM_W<PLIMITRM_SPEC> {
        PLIMITRM_W::new(self, 0)
    }
}
/**Positive Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`plimitrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plimitrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLIMITRM_SPEC;
impl crate::RegisterSpec for PLIMITRM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`plimitrm::R`](R) reader structure
impl crate::Readable for PLIMITRM_SPEC {}
///`write(|w| ..)` method takes [`plimitrm::W`](W) writer structure
impl crate::Writable for PLIMITRM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLIMITRM to value 0
impl crate::Resettable for PLIMITRM_SPEC {}
