///Register `SYTLIR` reader
pub type R = crate::R<SYTLIR_SPEC>;
///Register `SYTLIR` writer
pub type W = crate::W<SYTLIR_SPEC>;
///Field `ANCE` reader - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages.
pub type ANCE_R = crate::FieldReader;
///Field `ANCE` writer - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages.
pub type ANCE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SYNC` reader - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages.
pub type SYNC_R = crate::FieldReader;
///Field `SYNC` writer - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages.
pub type SYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DREQ` reader - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages.
pub type DREQ_R = crate::FieldReader;
///Field `DREQ` writer - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages.
pub type DREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages.
    #[inline(always)]
    pub fn ance(&self) -> ANCE_R {
        ANCE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages.
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages.
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Announce Message Transmission Interval SettingThese bits set the interval for the transmission of Announce messages.
    #[inline(always)]
    pub fn ance(&mut self) -> ANCE_W<SYTLIR_SPEC> {
        ANCE_W::new(self, 0)
    }
    ///Bits 8:15 - Sync Message Transmission Interval SettingThese bits set the interval for the transmission of Sync messages. The setting is also placed in the logMessageInterval field of transmitted Sync messages.
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<SYTLIR_SPEC> {
        SYNC_W::new(self, 8)
    }
    ///Bits 16:23 - Delay_Req Transmission Interval Average Value/ Pdelay_Req Transmission Interval SettingThe bits set the average interval for the transmission of Delay_Req messages and the interval for the transmission of Pdelay_Req messages.The setting is also placed in the logMessageInterval field of Delay_Resp messages.
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W<SYTLIR_SPEC> {
        DREQ_W::new(self, 16)
    }
}
/**SYNFP Transmission Interval Setting Register

You can [`read`](crate::Reg::read) this register and get [`sytlir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sytlir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYTLIR_SPEC;
impl crate::RegisterSpec for SYTLIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sytlir::R`](R) reader structure
impl crate::Readable for SYTLIR_SPEC {}
///`write(|w| ..)` method takes [`sytlir::W`](W) writer structure
impl crate::Writable for SYTLIR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYTLIR to value 0x01
impl crate::Resettable for SYTLIR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
