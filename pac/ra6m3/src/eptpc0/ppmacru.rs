///Register `PPMACRU` reader
pub type R = crate::R<PPMACRU_SPEC>;
///Register `PPMACRU` writer
pub type W = crate::W<PPMACRU_SPEC>;
///Field `PPMACRU` reader - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages.
pub type PPMACRU_R = crate::FieldReader<u32>;
///Field `PPMACRU` writer - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages.
pub type PPMACRU_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages.
    #[inline(always)]
    pub fn ppmacru(&self) -> PPMACRU_R {
        PPMACRU_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - These bits hold the setting for the higher-order 24 bits of the destination MAC address for PTP-primary messages.
    #[inline(always)]
    pub fn ppmacru(&mut self) -> PPMACRU_W<PPMACRU_SPEC> {
        PPMACRU_W::new(self, 0)
    }
}
/**PTP-primary Message Destination MAC Address Setting Registers

You can [`read`](crate::Reg::read) this register and get [`ppmacru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppmacru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PPMACRU_SPEC;
impl crate::RegisterSpec for PPMACRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ppmacru::R`](R) reader structure
impl crate::Readable for PPMACRU_SPEC {}
///`write(|w| ..)` method takes [`ppmacru::W`](W) writer structure
impl crate::Writable for PPMACRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PPMACRU to value 0x0001_1b19
impl crate::Resettable for PPMACRU_SPEC {
    const RESET_VALUE: u32 = 0x0001_1b19;
}
