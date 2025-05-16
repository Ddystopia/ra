///Register `SYMACRL` reader
pub type R = crate::R<SYMACRL_SPEC>;
///Register `SYMACRL` writer
pub type W = crate::W<SYMACRL_SPEC>;
///Field `SYMACRL` reader - These bits hold the setting for the lower-order 24 bits of the local MAC address.
pub type SYMACRL_R = crate::FieldReader<u32>;
///Field `SYMACRL` writer - These bits hold the setting for the lower-order 24 bits of the local MAC address.
pub type SYMACRL_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the local MAC address.
    #[inline(always)]
    pub fn symacrl(&self) -> SYMACRL_R {
        SYMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the local MAC address.
    #[inline(always)]
    pub fn symacrl(&mut self) -> SYMACRL_W<SYMACRL_SPEC> {
        SYMACRL_W::new(self, 0)
    }
}
/**SYNFP MAC Address Registers

You can [`read`](crate::Reg::read) this register and get [`symacrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`symacrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYMACRL_SPEC;
impl crate::RegisterSpec for SYMACRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`symacrl::R`](R) reader structure
impl crate::Readable for SYMACRL_SPEC {}
///`write(|w| ..)` method takes [`symacrl::W`](W) writer structure
impl crate::Writable for SYMACRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYMACRL to value 0
impl crate::Resettable for SYMACRL_SPEC {}
