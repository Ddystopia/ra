///Register `SD_BUF0` reader
pub type R = crate::R<SD_BUF0_SPEC>;
///Register `SD_BUF0` writer
pub type W = crate::W<SD_BUF0_SPEC>;
///Field `SD_BUF` reader - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data.
pub type SD_BUF_R = crate::FieldReader<u32>;
///Field `SD_BUF` writer - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data.
pub type SD_BUF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data.
    #[inline(always)]
    pub fn sd_buf(&self) -> SD_BUF_R {
        SD_BUF_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - SD Buffer RegisterWhen writing to the SD card, the write data is written to this register. When reading from the SD card, the read data is read from this register. This register is internally connected to two 512-byte buffers.If both buffers are not empty when executing multiple block read, SD/MMC clock is stopped to suspend receiving data. When one of buffers is empty, SD/MMC clock is supplied to resume receiving data.
    #[inline(always)]
    pub fn sd_buf(&mut self) -> SD_BUF_W<SD_BUF0_SPEC> {
        SD_BUF_W::new(self, 0)
    }
}
/**SD Buffer Register

You can [`read`](crate::Reg::read) this register and get [`sd_buf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_BUF0_SPEC;
impl crate::RegisterSpec for SD_BUF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_buf0::R`](R) reader structure
impl crate::Readable for SD_BUF0_SPEC {}
///`write(|w| ..)` method takes [`sd_buf0::W`](W) writer structure
impl crate::Writable for SD_BUF0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_BUF0 to value 0
impl crate::Resettable for SD_BUF0_SPEC {}
