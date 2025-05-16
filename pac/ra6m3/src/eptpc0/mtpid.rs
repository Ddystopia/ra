///Register `MTPID` reader
pub type R = crate::R<MTPID_SPEC>;
///Register `MTPID` writer
pub type W = crate::W<MTPID_SPEC>;
///Field `PNUM` reader - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock.
pub type PNUM_R = crate::FieldReader<u16>;
///Field `PNUM` writer - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock.
pub type PNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock.
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Master Clock Port Number SettingThese bits hold the setting for the port number of the master clock.
    #[inline(always)]
    pub fn pnum(&mut self) -> PNUM_W<MTPID_SPEC> {
        PNUM_W::new(self, 0)
    }
}
/**Master clock port number register

You can [`read`](crate::Reg::read) this register and get [`mtpid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtpid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MTPID_SPEC;
impl crate::RegisterSpec for MTPID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mtpid::R`](R) reader structure
impl crate::Readable for MTPID_SPEC {}
///`write(|w| ..)` method takes [`mtpid::W`](W) writer structure
impl crate::Writable for MTPID_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTPID to value 0
impl crate::Resettable for MTPID_SPEC {}
