///Register `GMPR` reader
pub type R = crate::R<GMPR_SPEC>;
///Register `GMPR` writer
pub type W = crate::W<GMPR_SPEC>;
///Field `GMPR2` reader - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages.
pub type GMPR2_R = crate::FieldReader;
///Field `GMPR2` writer - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages.
pub type GMPR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GMPR1` reader - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages.
pub type GMPR1_R = crate::FieldReader;
///Field `GMPR1` writer - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages.
pub type GMPR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages.
    #[inline(always)]
    pub fn gmpr2(&self) -> GMPR2_R {
        GMPR2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages.
    #[inline(always)]
    pub fn gmpr1(&self) -> GMPR1_R {
        GMPR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - grandmasterPriority2 Field Value SettingThese bits are used to set the value of the grandmasterPriority2 fields of Announce messages.
    #[inline(always)]
    pub fn gmpr2(&mut self) -> GMPR2_W<GMPR_SPEC> {
        GMPR2_W::new(self, 0)
    }
    ///Bits 16:23 - grandmasterPriority1 Field Value SettingThese bits are used to set the value of the grandmasterPriority1 fields of Announce messages.
    #[inline(always)]
    pub fn gmpr1(&mut self) -> GMPR1_W<GMPR_SPEC> {
        GMPR1_W::new(self, 16)
    }
}
/**grandmasterPriority Field Setting Register

You can [`read`](crate::Reg::read) this register and get [`gmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GMPR_SPEC;
impl crate::RegisterSpec for GMPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gmpr::R`](R) reader structure
impl crate::Readable for GMPR_SPEC {}
///`write(|w| ..)` method takes [`gmpr::W`](W) writer structure
impl crate::Writable for GMPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GMPR to value 0
impl crate::Resettable for GMPR_SPEC {}
