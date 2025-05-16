///Register `LCIVRM` reader
pub type R = crate::R<LCIVRM_SPEC>;
///Register `LCIVRM` writer
pub type W = crate::W<LCIVRM_SPEC>;
///Field `LCIVRM` reader - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter.
pub type LCIVRM_R = crate::FieldReader<u32>;
///Field `LCIVRM` writer - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter.
pub type LCIVRM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter.
    #[inline(always)]
    pub fn lcivrm(&self) -> LCIVRM_R {
        LCIVRM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the integer portion of the initial value for the local timer counter.
    #[inline(always)]
    pub fn lcivrm(&mut self) -> LCIVRM_W<LCIVRM_SPEC> {
        LCIVRM_W::new(self, 0)
    }
}
/**Local Time Counter Initial Value Registers

You can [`read`](crate::Reg::read) this register and get [`lcivrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcivrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCIVRM_SPEC;
impl crate::RegisterSpec for LCIVRM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lcivrm::R`](R) reader structure
impl crate::Readable for LCIVRM_SPEC {}
///`write(|w| ..)` method takes [`lcivrm::W`](W) writer structure
impl crate::Writable for LCIVRM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCIVRM to value 0
impl crate::Resettable for LCIVRM_SPEC {}
