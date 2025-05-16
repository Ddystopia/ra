///Register `PETYPER` reader
pub type R = crate::R<PETYPER_SPEC>;
///Register `PETYPER` writer
pub type W = crate::W<PETYPER_SPEC>;
///Field `TYPE` reader - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format.
pub type TYPE_R = crate::FieldReader<u16>;
///Field `TYPE` writer - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format.
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format.
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - PTP Message EtherType Value SettingThese bits hold the setting for the EtherType field value for frames in the Ethernet II format.
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<PETYPER_SPEC> {
        TYPE_W::new(self, 0)
    }
}
/**PTP Message EtherType Setting Register

You can [`read`](crate::Reg::read) this register and get [`petyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`petyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PETYPER_SPEC;
impl crate::RegisterSpec for PETYPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`petyper::R`](R) reader structure
impl crate::Readable for PETYPER_SPEC {}
///`write(|w| ..)` method takes [`petyper::W`](W) writer structure
impl crate::Writable for PETYPER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PETYPER to value 0x88f7
impl crate::Resettable for PETYPER_SPEC {
    const RESET_VALUE: u32 = 0x88f7;
}
