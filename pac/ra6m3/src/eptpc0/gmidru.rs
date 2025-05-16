///Register `GMIDRU` reader
pub type R = crate::R<GMIDRU_SPEC>;
///Register `GMIDRU` writer
pub type W = crate::W<GMIDRU_SPEC>;
///Field `GMIDRU` reader - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
pub type GMIDRU_R = crate::FieldReader<u32>;
///Field `GMIDRU` writer - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
pub type GMIDRU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
    #[inline(always)]
    pub fn gmidru(&self) -> GMIDRU_R {
        GMIDRU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the value of the grandmasterIdentity fields of Announce messages.
    #[inline(always)]
    pub fn gmidru(&mut self) -> GMIDRU_W<GMIDRU_SPEC> {
        GMIDRU_W::new(self, 0)
    }
}
/**grandmasterIdentity Field Setting Registers

You can [`read`](crate::Reg::read) this register and get [`gmidru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmidru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GMIDRU_SPEC;
impl crate::RegisterSpec for GMIDRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gmidru::R`](R) reader structure
impl crate::Readable for GMIDRU_SPEC {}
///`write(|w| ..)` method takes [`gmidru::W`](W) writer structure
impl crate::Writable for GMIDRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GMIDRU to value 0
impl crate::Resettable for GMIDRU_SPEC {}
