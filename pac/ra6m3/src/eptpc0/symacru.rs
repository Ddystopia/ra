///Register `SYMACRU` reader
pub type R = crate::R<SYMACRU_SPEC>;
///Register `SYMACRU` writer
pub type W = crate::W<SYMACRU_SPEC>;
///Field `SYMACRU` reader - These bits hold the setting for the higher-order 24 bits of the local MAC address.
pub type SYMACRU_R = crate::FieldReader<u32>;
///Field `SYMACRU` writer - These bits hold the setting for the higher-order 24 bits of the local MAC address.
pub type SYMACRU_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the local MAC address.
    #[inline(always)]
    pub fn symacru(&self) -> SYMACRU_R {
        SYMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the local MAC address.
    #[inline(always)]
    pub fn symacru(&mut self) -> SYMACRU_W<SYMACRU_SPEC> {
        SYMACRU_W::new(self, 0)
    }
}
/**SYNFP MAC Address Registers

You can [`read`](crate::Reg::read) this register and get [`symacru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`symacru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYMACRU_SPEC;
impl crate::RegisterSpec for SYMACRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`symacru::R`](R) reader structure
impl crate::Readable for SYMACRU_SPEC {}
///`write(|w| ..)` method takes [`symacru::W`](W) writer structure
impl crate::Writable for SYMACRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYMACRU to value 0
impl crate::Resettable for SYMACRU_SPEC {}
