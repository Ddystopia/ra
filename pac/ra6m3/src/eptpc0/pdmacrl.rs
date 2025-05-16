///Register `PDMACRL` reader
pub type R = crate::R<PDMACRL_SPEC>;
///Register `PDMACRL` writer
pub type W = crate::W<PDMACRL_SPEC>;
///Field `PDMACRL` reader - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages.
pub type PDMACRL_R = crate::FieldReader<u32>;
///Field `PDMACRL` writer - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages.
pub type PDMACRL_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages.
    #[inline(always)]
    pub fn pdmacrl(&self) -> PDMACRL_R {
        PDMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-pdelay messages.
    #[inline(always)]
    pub fn pdmacrl(&mut self) -> PDMACRL_W<PDMACRL_SPEC> {
        PDMACRL_W::new(self, 0)
    }
}
/**PTP-pdelay Message MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`pdmacrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdmacrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PDMACRL_SPEC;
impl crate::RegisterSpec for PDMACRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pdmacrl::R`](R) reader structure
impl crate::Readable for PDMACRL_SPEC {}
///`write(|w| ..)` method takes [`pdmacrl::W`](W) writer structure
impl crate::Writable for PDMACRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDMACRL to value 0x0e
impl crate::Resettable for PDMACRL_SPEC {
    const RESET_VALUE: u32 = 0x0e;
}
