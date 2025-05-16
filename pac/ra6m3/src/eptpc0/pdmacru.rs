///Register `PDMACRU` reader
pub type R = crate::R<PDMACRU_SPEC>;
///Register `PDMACRU` writer
pub type W = crate::W<PDMACRU_SPEC>;
///Field `PDMACRU` reader - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages.
pub type PDMACRU_R = crate::FieldReader<u32>;
///Field `PDMACRU` writer - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages.
pub type PDMACRU_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages.
    #[inline(always)]
    pub fn pdmacru(&self) -> PDMACRU_R {
        PDMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-pdelay messages.
    #[inline(always)]
    pub fn pdmacru(&mut self) -> PDMACRU_W<PDMACRU_SPEC> {
        PDMACRU_W::new(self, 0)
    }
}
/**PTP-pdelay Message MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`pdmacru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmacru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PDMACRU_SPEC;
impl crate::RegisterSpec for PDMACRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pdmacru::R`](R) reader structure
impl crate::Readable for PDMACRU_SPEC {}
///`write(|w| ..)` method takes [`pdmacru::W`](W) writer structure
impl crate::Writable for PDMACRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDMACRU to value 0x0001_80c2
impl crate::Resettable for PDMACRU_SPEC {
    const RESET_VALUE: u32 = 0x0001_80c2;
}
