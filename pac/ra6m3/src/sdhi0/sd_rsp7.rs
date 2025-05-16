///Register `SD_RSP7` reader
pub type R = crate::R<SD_RSP7_SPEC>;
///Field `SD_RSP7` reader - Store the response from the SD card/MMC
pub type SD_RSP7_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Store the response from the SD card/MMC
    #[inline(always)]
    pub fn sd_rsp7(&self) -> SD_RSP7_R {
        SD_RSP7_R::new((self.bits & 0xff) as u8)
    }
}
/**SD Card Response Register 7

You can [`read`](crate::Reg::read) this register and get [`sd_rsp7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_RSP7_SPEC;
impl crate::RegisterSpec for SD_RSP7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp7::R`](R) reader structure
impl crate::Readable for SD_RSP7_SPEC {}
///`reset()` method sets SD_RSP7 to value 0
impl crate::Resettable for SD_RSP7_SPEC {}
