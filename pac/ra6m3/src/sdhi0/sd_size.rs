///Register `SD_SIZE` reader
pub type R = crate::R<SD_SIZE_SPEC>;
///Register `SD_SIZE` writer
pub type W = crate::W<SD_SIZE_SPEC>;
///Field `LEN` reader - Transfer Data SizeThese bits specify a size between 1 and 512 bytes for the transfer of single blocks.In cases of multiple block transfer with automatic issuing of CMD12 (CMD18 and CMD25), the only specifiable transfer data size is 512 bytes. Furthermore, in cases of multiple block transfer without automatic issuing of CMD12, as well as 512 bytes, 32, 64, 128, and 256 bytes are specifiable. However, in the reading of 32, 64, 128, and 256 bytes for the transfer of multiple blocks, this is restricted to multiple block transfer by CMD53.Additionally, if a command accompanies data transfer, do not set these bits to 0.
pub type LEN_R = crate::FieldReader<u16>;
///Field `LEN` writer - Transfer Data SizeThese bits specify a size between 1 and 512 bytes for the transfer of single blocks.In cases of multiple block transfer with automatic issuing of CMD12 (CMD18 and CMD25), the only specifiable transfer data size is 512 bytes. Furthermore, in cases of multiple block transfer without automatic issuing of CMD12, as well as 512 bytes, 32, 64, 128, and 256 bytes are specifiable. However, in the reading of 32, 64, 128, and 256 bytes for the transfer of multiple blocks, this is restricted to multiple block transfer by CMD53.Additionally, if a command accompanies data transfer, do not set these bits to 0.
pub type LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transfer Data SizeThese bits specify a size between 1 and 512 bytes for the transfer of single blocks.In cases of multiple block transfer with automatic issuing of CMD12 (CMD18 and CMD25), the only specifiable transfer data size is 512 bytes. Furthermore, in cases of multiple block transfer without automatic issuing of CMD12, as well as 512 bytes, 32, 64, 128, and 256 bytes are specifiable. However, in the reading of 32, 64, 128, and 256 bytes for the transfer of multiple blocks, this is restricted to multiple block transfer by CMD53.Additionally, if a command accompanies data transfer, do not set these bits to 0.
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Transfer Data SizeThese bits specify a size between 1 and 512 bytes for the transfer of single blocks.In cases of multiple block transfer with automatic issuing of CMD12 (CMD18 and CMD25), the only specifiable transfer data size is 512 bytes. Furthermore, in cases of multiple block transfer without automatic issuing of CMD12, as well as 512 bytes, 32, 64, 128, and 256 bytes are specifiable. However, in the reading of 32, 64, 128, and 256 bytes for the transfer of multiple blocks, this is restricted to multiple block transfer by CMD53.Additionally, if a command accompanies data transfer, do not set these bits to 0.
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<SD_SIZE_SPEC> {
        LEN_W::new(self, 0)
    }
}
/**Transfer Data Length Register

You can [`read`](crate::Reg::read) this register and get [`sd_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_SIZE_SPEC;
impl crate::RegisterSpec for SD_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_size::R`](R) reader structure
impl crate::Readable for SD_SIZE_SPEC {}
///`write(|w| ..)` method takes [`sd_size::W`](W) writer structure
impl crate::Writable for SD_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_SIZE to value 0x0200
impl crate::Resettable for SD_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
