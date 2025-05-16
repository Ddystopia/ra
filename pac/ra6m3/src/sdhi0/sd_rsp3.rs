///Register `SD_RSP3` reader
pub type R = crate::R<SD_RSP3_SPEC>;
///Field `SD_RSP3` reader - Store the response from the SD card/MMC
pub type SD_RSP3_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Store the response from the SD card/MMC
    #[inline(always)]
    pub fn sd_rsp3(&self) -> SD_RSP3_R {
        SD_RSP3_R::new((self.bits & 0xffff) as u16)
    }
}
/**SD Card Response Register 3

You can [`read`](crate::Reg::read) this register and get [`sd_rsp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_RSP3_SPEC;
impl crate::RegisterSpec for SD_RSP3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_rsp3::R`](R) reader structure
impl crate::Readable for SD_RSP3_SPEC {}
///`reset()` method sets SD_RSP3 to value 0
impl crate::Resettable for SD_RSP3_SPEC {}
