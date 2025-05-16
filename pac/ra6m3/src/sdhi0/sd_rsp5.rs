///Register `SD_RSP5` reader
pub type R = crate::R<SD_RSP5_SPEC>;
///Field `SD_RSP5` reader - Store the response from the SD card/MMC
pub type SD_RSP5_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Store the response from the SD card/MMC
    #[inline(always)]
    pub fn sd_rsp5(&self) -> SD_RSP5_R {
        SD_RSP5_R::new((self.bits & 0xffff) as u16)
    }
}
/**SD Card Response Register 5

You can [`read`](crate::Reg::read) this register and get [`sd_rsp5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_RSP5_SPEC;
impl crate::RegisterSpec for SD_RSP5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp5::R`](R) reader structure
impl crate::Readable for SD_RSP5_SPEC {}
///`reset()` method sets SD_RSP5 to value 0
impl crate::Resettable for SD_RSP5_SPEC {}
