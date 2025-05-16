///Register `SD_RSP76` reader
pub type R = crate::R<SD_RSP76_SPEC>;
///Field `SD_RSP76` reader - Store the response from the SD card/MMC
pub type SD_RSP76_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Store the response from the SD card/MMC
    #[inline(always)]
    pub fn sd_rsp76(&self) -> SD_RSP76_R {
        SD_RSP76_R::new(self.bits & 0x00ff_ffff)
    }
}
/**SD Card Response Register 76

You can [`read`](crate::Reg::read) this register and get [`sd_rsp76::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_RSP76_SPEC;
impl crate::RegisterSpec for SD_RSP76_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp76::R`](R) reader structure
impl crate::Readable for SD_RSP76_SPEC {}
///`reset()` method sets SD_RSP76 to value 0
impl crate::Resettable for SD_RSP76_SPEC {}
