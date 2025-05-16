///Register `PETOSR` reader
pub type R = crate::R<PETOSR_SPEC>;
///Register `PETOSR` writer
pub type W = crate::W<PETOSR_SPEC>;
///Field `EVTO` reader - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages.
pub type EVTO_R = crate::FieldReader;
///Field `EVTO` writer - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages.
pub type EVTO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages.
    #[inline(always)]
    pub fn evto(&self) -> EVTO_R {
        EVTO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - PTP Event Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP event messages.
    #[inline(always)]
    pub fn evto(&mut self) -> EVTO_W<PETOSR_SPEC> {
        EVTO_W::new(self, 0)
    }
}
/**PTP Event Message TOS Setting Register

You can [`read`](crate::Reg::read) this register and get [`petosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`petosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PETOSR_SPEC;
impl crate::RegisterSpec for PETOSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`petosr::R`](R) reader structure
impl crate::Readable for PETOSR_SPEC {}
///`write(|w| ..)` method takes [`petosr::W`](W) writer structure
impl crate::Writable for PETOSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PETOSR to value 0
impl crate::Resettable for PETOSR_SPEC {}
