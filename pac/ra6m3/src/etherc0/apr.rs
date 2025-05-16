///Register `APR` reader
pub type R = crate::R<APR_SPEC>;
///Register `APR` writer
pub type W = crate::W<APR_SPEC>;
///Field `AP` reader - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed.
pub type AP_R = crate::FieldReader<u16>;
///Field `AP` writer - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed.
pub type AP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed.
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Automatic PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is automatically transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed.
    #[inline(always)]
    pub fn ap(&mut self) -> AP_W<APR_SPEC> {
        AP_W::new(self, 0)
    }
}
/**Automatic PAUSE Frame Register

You can [`read`](crate::Reg::read) this register and get [`apr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APR_SPEC;
impl crate::RegisterSpec for APR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apr::R`](R) reader structure
impl crate::Readable for APR_SPEC {}
///`write(|w| ..)` method takes [`apr::W`](W) writer structure
impl crate::Writable for APR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APR to value 0
impl crate::Resettable for APR_SPEC {}
