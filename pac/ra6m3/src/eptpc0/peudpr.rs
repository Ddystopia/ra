///Register `PEUDPR` reader
pub type R = crate::R<PEUDPR_SPEC>;
///Register `PEUDPR` writer
pub type W = crate::W<PEUDPR_SPEC>;
///Field `EVUPT` reader - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages.
pub type EVUPT_R = crate::FieldReader<u16>;
///Field `EVUPT` writer - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages.
pub type EVUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages.
    #[inline(always)]
    pub fn evupt(&self) -> EVUPT_R {
        EVUPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - PTP Event Message Destination Port Number SettingThese bits hold the setting for the value of the destination port number field within the UDP headers of PTP event messages.
    #[inline(always)]
    pub fn evupt(&mut self) -> EVUPT_W<PEUDPR_SPEC> {
        EVUPT_W::new(self, 0)
    }
}
/**PTP Event Message UDP Destination Port Number Setting Register

You can [`read`](crate::Reg::read) this register and get [`peudpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peudpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PEUDPR_SPEC;
impl crate::RegisterSpec for PEUDPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peudpr::R`](R) reader structure
impl crate::Readable for PEUDPR_SPEC {}
///`write(|w| ..)` method takes [`peudpr::W`](W) writer structure
impl crate::Writable for PEUDPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PEUDPR to value 0x013f
impl crate::Resettable for PEUDPR_SPEC {
    const RESET_VALUE: u32 = 0x013f;
}
