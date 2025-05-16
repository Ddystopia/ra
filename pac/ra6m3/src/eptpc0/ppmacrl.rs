///Register `PPMACRL` reader
pub type R = crate::R<PPMACRL_SPEC>;
///Register `PPMACRL` writer
pub type W = crate::W<PPMACRL_SPEC>;
///Field `PPMACRL` reader - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages.
pub type PPMACRL_R = crate::FieldReader<u32>;
///Field `PPMACRL` writer - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages.
pub type PPMACRL_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages.
    #[inline(always)]
    pub fn ppmacrl(&self) -> PPMACRL_R {
        PPMACRL_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the lower-order 24 bits of the destination MAC address for PTP-primary messages.
    #[inline(always)]
    pub fn ppmacrl(&mut self) -> PPMACRL_W<PPMACRL_SPEC> {
        PPMACRL_W::new(self, 0)
    }
}
/**PTP-primary Message Destination MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`ppmacrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppmacrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PPMACRL_SPEC;
impl crate::RegisterSpec for PPMACRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ppmacrl::R`](R) reader structure
impl crate::Readable for PPMACRL_SPEC {}
///`write(|w| ..)` method takes [`ppmacrl::W`](W) writer structure
impl crate::Writable for PPMACRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PPMACRL to value 0
impl crate::Resettable for PPMACRL_SPEC {}
