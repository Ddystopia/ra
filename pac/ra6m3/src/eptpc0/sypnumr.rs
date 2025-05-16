///Register `SYPNUMR` reader
pub type R = crate::R<SYPNUMR_SPEC>;
///Register `SYPNUMR` writer
pub type W = crate::W<SYPNUMR_SPEC>;
///Field `PNUM` reader - Local Port Number SettingThese bits hold the setting for the port number of the local port.
pub type PNUM_R = crate::FieldReader<u16>;
///Field `PNUM` writer - Local Port Number SettingThese bits hold the setting for the port number of the local port.
pub type PNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Local Port Number SettingThese bits hold the setting for the port number of the local port.
    #[inline(always)]
    pub fn pnum(&self) -> PNUM_R {
        PNUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Local Port Number SettingThese bits hold the setting for the port number of the local port.
    #[inline(always)]
    pub fn pnum(&mut self) -> PNUM_W<SYPNUMR_SPEC> {
        PNUM_W::new(self, 0)
    }
}
/**SYNFP Local Port Number Register

You can [`read`](crate::Reg::read) this register and get [`sypnumr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sypnumr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYPNUMR_SPEC;
impl crate::RegisterSpec for SYPNUMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sypnumr::R`](R) reader structure
impl crate::Readable for SYPNUMR_SPEC {}
///`write(|w| ..)` method takes [`sypnumr::W`](W) writer structure
impl crate::Writable for SYPNUMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYPNUMR to value 0
impl crate::Resettable for SYPNUMR_SPEC {}
