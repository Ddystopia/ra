///Register `SD_ARG1` reader
pub type R = crate::R<SD_ARG1_SPEC>;
///Register `SD_ARG1` writer
pub type W = crate::W<SD_ARG1_SPEC>;
///Field `SD_ARG1` reader - Argument Register 1Set command format\[39:24\] (argument)
pub type SD_ARG1_R = crate::FieldReader<u16>;
///Field `SD_ARG1` writer - Argument Register 1Set command format\[39:24\] (argument)
pub type SD_ARG1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Argument Register 1Set command format\[39:24\] (argument)
    #[inline(always)]
    pub fn sd_arg1(&self) -> SD_ARG1_R {
        SD_ARG1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Argument Register 1Set command format\[39:24\] (argument)
    #[inline(always)]
    pub fn sd_arg1(&mut self) -> SD_ARG1_W<SD_ARG1_SPEC> {
        SD_ARG1_W::new(self, 0)
    }
}
/**SD Command Argument Register 1

You can [`read`](crate::Reg::read) this register and get [`sd_arg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_ARG1_SPEC;
impl crate::RegisterSpec for SD_ARG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_arg1::R`](R) reader structure
impl crate::Readable for SD_ARG1_SPEC {}
///`write(|w| ..)` method takes [`sd_arg1::W`](W) writer structure
impl crate::Writable for SD_ARG1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_ARG1 to value 0
impl crate::Resettable for SD_ARG1_SPEC {}
