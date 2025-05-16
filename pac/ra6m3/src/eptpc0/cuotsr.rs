///Register `CUOTSR` reader
pub type R = crate::R<CUOTSR_SPEC>;
///Register `CUOTSR` writer
pub type W = crate::W<CUOTSR_SPEC>;
///Field `TSRC` reader - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages.
pub type TSRC_R = crate::FieldReader;
///Field `TSRC` writer - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages.
pub type TSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CUTO` reader - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages.
pub type CUTO_R = crate::FieldReader<u16>;
///Field `CUTO` writer - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages.
pub type CUTO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:7 - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages.
    #[inline(always)]
    pub fn tsrc(&self) -> TSRC_R {
        TSRC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:31 - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages.
    #[inline(always)]
    pub fn cuto(&self) -> CUTO_R {
        CUTO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:7 - timeSource Field SettingThese bits set the value of the timeSource fields of Announce messages.
    #[inline(always)]
    pub fn tsrc(&mut self) -> TSRC_W<CUOTSR_SPEC> {
        TSRC_W::new(self, 0)
    }
    ///Bits 16:31 - currentUtcOffset Field SettingThese bits set the value of the currentUtcOffset fields of Announce messages.
    #[inline(always)]
    pub fn cuto(&mut self) -> CUTO_W<CUOTSR_SPEC> {
        CUTO_W::new(self, 16)
    }
}
/**currentUtcOffset/timeSource Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`cuotsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cuotsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CUOTSR_SPEC;
impl crate::RegisterSpec for CUOTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cuotsr::R`](R) reader structure
impl crate::Readable for CUOTSR_SPEC {}
///`write(|w| ..)` method takes [`cuotsr::W`](W) writer structure
impl crate::Writable for CUOTSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CUOTSR to value 0
impl crate::Resettable for CUOTSR_SPEC {}
