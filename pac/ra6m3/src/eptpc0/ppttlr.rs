///Register `PPTTLR` reader
pub type R = crate::R<PPTTLR_SPEC>;
///Register `PPTTLR` writer
pub type W = crate::W<PPTTLR_SPEC>;
///Field `PRTL` reader - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages.
pub type PRTL_R = crate::FieldReader;
///Field `PRTL` writer - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages.
pub type PRTL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages.
    #[inline(always)]
    pub fn prtl(&self) -> PRTL_R {
        PRTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - PTP-primary Message TTL Field Value SettingThese bits hold the setting for the value of the TTL field within the IPv4 headers of PTP-primary messages.
    #[inline(always)]
    pub fn prtl(&mut self) -> PRTL_W<PPTTLR_SPEC> {
        PRTL_W::new(self, 0)
    }
}
/**PTP-primary Message TTL Setting Register

You can [`read`](crate::Reg::read) this register and get [`ppttlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppttlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PPTTLR_SPEC;
impl crate::RegisterSpec for PPTTLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ppttlr::R`](R) reader structure
impl crate::Readable for PPTTLR_SPEC {}
///`write(|w| ..)` method takes [`ppttlr::W`](W) writer structure
impl crate::Writable for PPTTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PPTTLR to value 0x80
impl crate::Resettable for PPTTLR_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
