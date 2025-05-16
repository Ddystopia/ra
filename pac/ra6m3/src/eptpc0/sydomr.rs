///Register `SYDOMR` reader
pub type R = crate::R<SYDOMR_SPEC>;
///Register `SYDOMR` writer
pub type W = crate::W<SYDOMR_SPEC>;
///Field `DNUM` reader - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission.
pub type DNUM_R = crate::FieldReader;
///Field `DNUM` writer - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission.
pub type DNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission.
    #[inline(always)]
    pub fn dnum(&self) -> DNUM_R {
        DNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - domainNumber Field Value SettingThese bits are used to set the domainNumber field value of the PTP v2 header.When a message is received, this value is compared with the domainNumber field of the received frame as a condition for PTP reception processing.In generating messages, the value is used for the domainNumber field of the frame for transmission.
    #[inline(always)]
    pub fn dnum(&mut self) -> DNUM_W<SYDOMR_SPEC> {
        DNUM_W::new(self, 0)
    }
}
/**SYNFP Domain Number Setting Register

You can [`read`](crate::Reg::read) this register and get [`sydomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sydomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYDOMR_SPEC;
impl crate::RegisterSpec for SYDOMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sydomr::R`](R) reader structure
impl crate::Readable for SYDOMR_SPEC {}
///`write(|w| ..)` method takes [`sydomr::W`](W) writer structure
impl crate::Writable for SYDOMR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYDOMR to value 0
impl crate::Resettable for SYDOMR_SPEC {}
