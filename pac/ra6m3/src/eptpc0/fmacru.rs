///Register `FMAC%sRU` reader
pub type R = crate::R<FMACRU_SPEC>;
///Register `FMAC%sRU` writer
pub type W = crate::W<FMACRU_SPEC>;
///Field `FMACRU` reader - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames.
pub type FMACRU_R = crate::FieldReader<u32>;
///Field `FMACRU` writer - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames.
pub type FMACRU_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames.
    #[inline(always)]
    pub fn fmacru(&self) -> FMACRU_R {
        FMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for received multicast frames.
    #[inline(always)]
    pub fn fmacru(&mut self) -> FMACRU_W<FMACRU_SPEC> {
        FMACRU_W::new(self, 0)
    }
}
/**Frame Reception Filter MAC Address %s Setting Registers

You can [`read`](crate::Reg::read) this register and get [`fmacru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmacru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FMACRU_SPEC;
impl crate::RegisterSpec for FMACRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fmacru::R`](R) reader structure
impl crate::Readable for FMACRU_SPEC {}
///`write(|w| ..)` method takes [`fmacru::W`](W) writer structure
impl crate::Writable for FMACRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMAC%sRU to value 0
impl crate::Resettable for FMACRU_SPEC {}
