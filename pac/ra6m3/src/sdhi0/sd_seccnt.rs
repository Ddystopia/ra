///Register `SD_SECCNT` reader
pub type R = crate::R<SD_SECCNT_SPEC>;
///Register `SD_SECCNT` writer
pub type W = crate::W<SD_SECCNT_SPEC>;
///Field `SD_SECCNT` reader - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
pub type SD_SECCNT_R = crate::FieldReader<u32>;
///Field `SD_SECCNT` writer - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
pub type SD_SECCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
    #[inline(always)]
    pub fn sd_seccnt(&self) -> SD_SECCNT_R {
        SD_SECCNT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Number of Transfer BlocksNOTE: Do not change the value of this bit when the CBSY bit in SD_INFO2 is set to 1.
    #[inline(always)]
    pub fn sd_seccnt(&mut self) -> SD_SECCNT_W<SD_SECCNT_SPEC> {
        SD_SECCNT_W::new(self, 0)
    }
}
/**Block Count Register

You can [`read`](crate::Reg::read) this register and get [`sd_seccnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_seccnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SD_SECCNT_SPEC;
impl crate::RegisterSpec for SD_SECCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sd_seccnt::R`](R) reader structure
impl crate::Readable for SD_SECCNT_SPEC {}
///`write(|w| ..)` method takes [`sd_seccnt::W`](W) writer structure
impl crate::Writable for SD_SECCNT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SD_SECCNT to value 0
impl crate::Resettable for SD_SECCNT_SPEC {}
