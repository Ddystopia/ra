///Register `SYRLIR` reader
pub type R = crate::R<SYRLIR_SPEC>;
///Field `ANCE` reader - Announce Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Announce message.
pub type ANCE_R = crate::FieldReader;
///Field `SYNC` reader - Sync Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Sync message.
pub type SYNC_R = crate::FieldReader;
///Field `DRESP` reader - Delay_Resp Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Delay_Resp message.
pub type DRESP_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Announce Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Announce message.
    #[inline(always)]
    pub fn ance(&self) -> ANCE_R {
        ANCE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Sync Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Sync message.
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Delay_Resp Message logMessageInterval Field IndicationThese bits indicate the logMessageInterval field value of a received Delay_Resp message.
    #[inline(always)]
    pub fn dresp(&self) -> DRESP_R {
        DRESP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
/**SYNFP Received logMessageInterval Value Indication Register

You can [`read`](crate::Reg::read) this register and get [`syrlir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYRLIR_SPEC;
impl crate::RegisterSpec for SYRLIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syrlir::R`](R) reader structure
impl crate::Readable for SYRLIR_SPEC {}
///`reset()` method sets SYRLIR to value 0
impl crate::Resettable for SYRLIR_SPEC {}
