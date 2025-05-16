///Register `MPR` writer
pub type W = crate::W<MPR_SPEC>;
///Field `MP` writer - Manual PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is manually transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed. The read value is undefined.
pub type MP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    ///Bits 0:15 - Manual PAUSE Time SettingThese bits set the value of the pause_time parameter for a PAUSE frame that is manually transmitted. Transmission is not performed until the set value multiplied by 512 bit time has elapsed. The read value is undefined.
    #[inline(always)]
    pub fn mp(&mut self) -> MP_W<MPR_SPEC> {
        MP_W::new(self, 0)
    }
}
/**Manual PAUSE Frame Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MPR_SPEC;
impl crate::RegisterSpec for MPR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mpr::W`](W) writer structure
impl crate::Writable for MPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPR to value 0
impl crate::Resettable for MPR_SPEC {}
