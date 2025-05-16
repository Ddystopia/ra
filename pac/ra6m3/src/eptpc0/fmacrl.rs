///Register `FMAC%sRL` reader
pub type R = crate::R<FMACRL_SPEC>;
///Register `FMAC%sRL` writer
pub type W = crate::W<FMACRL_SPEC>;
///Field `FMACRL` reader - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames.
pub type FMACRL_R = crate::FieldReader<u32>;
///Field `FMACRL` writer - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames.
pub type FMACRL_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames.
    #[inline(always)]
    pub fn fmacrl(&self) -> FMACRL_R {
        FMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for received multicast frames.
    #[inline(always)]
    pub fn fmacrl(&mut self) -> FMACRL_W<FMACRL_SPEC> {
        FMACRL_W::new(self, 0)
    }
}
/**Frame Reception Filter MAC Address %s Setting Registers

You can [`read`](crate::Reg::read) this register and get [`fmacrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmacrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FMACRL_SPEC;
impl crate::RegisterSpec for FMACRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fmacrl::R`](R) reader structure
impl crate::Readable for FMACRL_SPEC {}
///`write(|w| ..)` method takes [`fmacrl::W`](W) writer structure
impl crate::Writable for FMACRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMAC%sRL to value 0
impl crate::Resettable for FMACRL_SPEC {}
