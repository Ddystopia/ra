///Register `TSLATR` reader
pub type R = crate::R<TSLATR_SPEC>;
///Register `TSLATR` writer
pub type W = crate::W<TSLATR_SPEC>;
///Field `EGP` reader - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports.
pub type EGP_R = crate::FieldReader<u16>;
///Field `EGP` writer - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports.
pub type EGP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INGP` reader - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports.
pub type INGP_R = crate::FieldReader<u16>;
///Field `INGP` writer - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports.
pub type INGP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports.
    #[inline(always)]
    pub fn egp(&self) -> EGP_R {
        EGP_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports.
    #[inline(always)]
    pub fn ingp(&self) -> INGP_R {
        INGP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Input Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the input ports.
    #[inline(always)]
    pub fn egp(&mut self) -> EGP_W<TSLATR_SPEC> {
        EGP_W::new(self, 0)
    }
    ///Bits 16:31 - Output Port Timestamp Latency SettingThese bits hold the setting for the time stamp latency (ns) for the output ports.
    #[inline(always)]
    pub fn ingp(&mut self) -> INGP_W<TSLATR_SPEC> {
        INGP_W::new(self, 16)
    }
}
/**Timestamp Latency Setting Register

You can [`read`](crate::Reg::read) this register and get [`tslatr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tslatr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSLATR_SPEC;
impl crate::RegisterSpec for TSLATR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tslatr::R`](R) reader structure
impl crate::Readable for TSLATR_SPEC {}
///`write(|w| ..)` method takes [`tslatr::W`](W) writer structure
impl crate::Writable for TSLATR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSLATR to value 0
impl crate::Resettable for TSLATR_SPEC {}
