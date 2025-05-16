///Register `JIFDDOFST` reader
pub type R = crate::R<JIFDDOFST_SPEC>;
///Register `JIFDDOFST` writer
pub type W = crate::W<JIFDDOFST_SPEC>;
///Field `DDMW` reader - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
pub type DDMW_R = crate::FieldReader<u16>;
///Field `DDMW` writer - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
pub type DDMW_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn ddmw(&self) -> DDMW_R {
        DDMW_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - Output Image Data Lines Offset (in 8-byte units) The lower three bits should be set to 0.
    #[inline(always)]
    pub fn ddmw(&mut self) -> DDMW_W<JIFDDOFST_SPEC> {
        DDMW_W::new(self, 0)
    }
}
/**JPEG Interface Decompression Line Offset Register

You can [`read`](crate::Reg::read) this register and get [`jifddofst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifddofst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDDOFST_SPEC;
impl crate::RegisterSpec for JIFDDOFST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifddofst::R`](R) reader structure
impl crate::Readable for JIFDDOFST_SPEC {}
///`write(|w| ..)` method takes [`jifddofst::W`](W) writer structure
impl crate::Writable for JIFDDOFST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDDOFST to value 0
impl crate::Resettable for JIFDDOFST_SPEC {}
