///Register `PLIMITRU` reader
pub type R = crate::R<PLIMITRU_SPEC>;
///Register `PLIMITRU` writer
pub type W = crate::W<PLIMITRU_SPEC>;
///Field `PLIMITRU` reader - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient.
pub type PLIMITRU_R = crate::FieldReader<u32>;
///Field `PLIMITRU` writer - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient.
pub type PLIMITRU_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    ///Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient.
    #[inline(always)]
    pub fn plimitru(&self) -> PLIMITRU_R {
        PLIMITRU_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    ///Bits 0:30 - These bits hold the setting for the higher-order 31 bits of the limit for the positive gradient.
    #[inline(always)]
    pub fn plimitru(&mut self) -> PLIMITRU_W<PLIMITRU_SPEC> {
        PLIMITRU_W::new(self, 0)
    }
}
/**Positive Gradient Limit Registers

You can [`read`](crate::Reg::read) this register and get [`plimitru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plimitru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLIMITRU_SPEC;
impl crate::RegisterSpec for PLIMITRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`plimitru::R`](R) reader structure
impl crate::Readable for PLIMITRU_SPEC {}
///`write(|w| ..)` method takes [`plimitru::W`](W) writer structure
impl crate::Writable for PLIMITRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLIMITRU to value 0
impl crate::Resettable for PLIMITRU_SPEC {}
