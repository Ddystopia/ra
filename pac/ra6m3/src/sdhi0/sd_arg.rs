///Register `SD_ARG` reader
pub type R = crate::R<SD_ARG_SPEC>;
///Register `SD_ARG` writer
pub type W = crate::W<SD_ARG_SPEC>;
///Field `SD_ARG` reader - Argument RegisterSet command format\[39:8\] (argument)
pub type SD_ARG_R = crate::FieldReader<u32>;
///Field `SD_ARG` writer - Argument RegisterSet command format\[39:8\] (argument)
pub type SD_ARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Argument RegisterSet command format\[39:8\] (argument)
    #[inline(always)]
    pub fn sd_arg(&self) -> SD_ARG_R {
        SD_ARG_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Argument RegisterSet command format\[39:8\] (argument)
    #[inline(always)]
    pub fn sd_arg(&mut self) -> SD_ARG_W<SD_ARG_SPEC> {
        SD_ARG_W::new(self, 0)
    }
}
/**SD Command Argument Register

You can [`read`](crate::Reg::read) this register and get [`sd_arg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_ARG_SPEC;
impl crate::RegisterSpec for SD_ARG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_arg::R`](R) reader structure
impl crate::Readable for SD_ARG_SPEC {}
///`write(|w| ..)` method takes [`sd_arg::W`](W) writer structure
impl crate::Writable for SD_ARG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_ARG to value 0
impl crate::Resettable for SD_ARG_SPEC {}
