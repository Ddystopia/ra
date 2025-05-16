///Register `SRR` reader
pub type R = crate::R<SRR_SPEC>;
///Register `SRR` writer
pub type W = crate::W<SRR_SPEC>;
///Field `SRMV` reader - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages.
pub type SRMV_R = crate::FieldReader<u16>;
///Field `SRMV` writer - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages.
pub type SRMV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages.
    #[inline(always)]
    pub fn srmv(&self) -> SRMV_R {
        SRMV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - stepsRemoved Field Value SettingThese bits set the value of the stepsRemoved fields of Announce messages.
    #[inline(always)]
    pub fn srmv(&mut self) -> SRMV_W<SRR_SPEC> {
        SRMV_W::new(self, 0)
    }
}
/**stepsRemoved Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`srr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRR_SPEC;
impl crate::RegisterSpec for SRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`srr::R`](R) reader structure
impl crate::Readable for SRR_SPEC {}
///`write(|w| ..)` method takes [`srr::W`](W) writer structure
impl crate::Writable for SRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRR to value 0
impl crate::Resettable for SRR_SPEC {}
