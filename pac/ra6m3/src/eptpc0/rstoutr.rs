///Register `RSTOUTR` reader
pub type R = crate::R<RSTOUTR_SPEC>;
///Register `RSTOUTR` writer
pub type W = crate::W<RSTOUTR_SPEC>;
///Field `RSTOUTR` reader - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout.
pub type RSTOUTR_R = crate::FieldReader<u32>;
///Field `RSTOUTR` writer - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout.
pub type RSTOUTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout.
    #[inline(always)]
    pub fn rstoutr(&self) -> RSTOUTR_R {
        RSTOUTR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Response Message Reception Timeout Time SettingA response message not being received within n x 1024 (ns), where n is the setting, is judged to represent a timeout.
    #[inline(always)]
    pub fn rstoutr(&mut self) -> RSTOUTR_W<RSTOUTR_SPEC> {
        RSTOUTR_W::new(self, 0)
    }
}
/**Response Message Reception Timeout Register

You can [`read`](crate::Reg::read) this register and get [`rstoutr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstoutr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RSTOUTR_SPEC;
impl crate::RegisterSpec for RSTOUTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rstoutr::R`](R) reader structure
impl crate::Readable for RSTOUTR_SPEC {}
///`write(|w| ..)` method takes [`rstoutr::W`](W) writer structure
impl crate::Writable for RSTOUTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RSTOUTR to value 0
impl crate::Resettable for RSTOUTR_SPEC {}
