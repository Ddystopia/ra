///Register `SD_RSP32` reader
pub type R = crate::R<SD_RSP32_SPEC>;
///Field `SD_RSP32` reader - Store the response from the SD card/MMC
pub type SD_RSP32_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Store the response from the SD card/MMC
    #[inline(always)]
    pub fn sd_rsp32(&self) -> SD_RSP32_R {
        SD_RSP32_R::new(self.bits)
    }
}
/**SD Card Response Register 32

You can [`read`](crate::Reg::read) this register and get [`sd_rsp32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_RSP32_SPEC;
impl crate::RegisterSpec for SD_RSP32_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp32::R`](R) reader structure
impl crate::Readable for SD_RSP32_SPEC {}
///`reset()` method sets SD_RSP32 to value 0
impl crate::Resettable for SD_RSP32_SPEC {}
