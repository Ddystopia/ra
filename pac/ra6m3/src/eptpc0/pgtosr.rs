///Register `PGTOSR` reader
pub type R = crate::R<PGTOSR_SPEC>;
///Register `PGTOSR` writer
pub type W = crate::W<PGTOSR_SPEC>;
///Field `GETO` reader - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages.
pub type GETO_R = crate::FieldReader;
///Field `GETO` writer - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages.
pub type GETO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages.
    #[inline(always)]
    pub fn geto(&self) -> GETO_R {
        GETO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - PTP general Message TOS Field Value SettingThese bits hold the setting for the value of the TOS field within the IPv4 headers of PTP general messages.
    #[inline(always)]
    pub fn geto(&mut self) -> GETO_W<PGTOSR_SPEC> {
        GETO_W::new(self, 0)
    }
}
/**PTP general Message TOS Setting Register

You can [`read`](crate::Reg::read) this register and get [`pgtosr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgtosr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PGTOSR_SPEC;
impl crate::RegisterSpec for PGTOSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pgtosr::R`](R) reader structure
impl crate::Readable for PGTOSR_SPEC {}
///`write(|w| ..)` method takes [`pgtosr::W`](W) writer structure
impl crate::Writable for PGTOSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PGTOSR to value 0
impl crate::Resettable for PGTOSR_SPEC {}
