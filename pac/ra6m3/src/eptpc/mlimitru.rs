///Register `MLIMITRU` reader
pub type R = crate::R<MLIMITRU_SPEC>;
///Register `MLIMITRU` writer
pub type W = crate::W<MLIMITRU_SPEC>;
///Field `MLIMITRU` reader - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient.
pub type MLIMITRU_R = crate::FieldReader<u32>;
///Field `MLIMITRU` writer - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient.
pub type MLIMITRU_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient.
    #[inline(always)]
    pub fn mlimitru(&self) -> MLIMITRU_R {
        MLIMITRU_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    ///Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the negative gradient.
    #[inline(always)]
    pub fn mlimitru(&mut self) -> MLIMITRU_W<MLIMITRU_SPEC> {
        MLIMITRU_W::new(self, 0)
    }
}
/**Negative Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`mlimitru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlimitru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MLIMITRU_SPEC;
impl crate::RegisterSpec for MLIMITRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mlimitru::R`](R) reader structure
impl crate::Readable for MLIMITRU_SPEC {}
///`write(|w| ..)` method takes [`mlimitru::W`](W) writer structure
impl crate::Writable for MLIMITRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MLIMITRU to value 0
impl crate::Resettable for MLIMITRU_SPEC {}
