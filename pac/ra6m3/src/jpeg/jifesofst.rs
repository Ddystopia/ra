///Register `JIFESOFST` reader
pub type R = crate::R<JIFESOFST_SPEC>;
///Register `JIFESOFST` writer
pub type W = crate::W<JIFESOFST_SPEC>;
///Field `ESMW` reader - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0.
pub type ESMW_R = crate::FieldReader<u16>;
///Field `ESMW` writer - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0.
pub type ESMW_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0.
    #[inline(always)]
    pub fn esmw(&self) -> ESMW_R {
        ESMW_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - Input Image Data Lines Offset(in 8-byte units)The lower three bits should be set to 0.
    #[inline(always)]
    pub fn esmw(&mut self) -> ESMW_W<JIFESOFST_SPEC> {
        ESMW_W::new(self, 0)
    }
}
/**JPEG Interface Compression Line Offset Register

You can [`read`](crate::Reg::read) this register and get [`jifesofst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifesofst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFESOFST_SPEC;
impl crate::RegisterSpec for JIFESOFST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifesofst::R`](R) reader structure
impl crate::Readable for JIFESOFST_SPEC {}
///`write(|w| ..)` method takes [`jifesofst::W`](W) writer structure
impl crate::Writable for JIFESOFST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFESOFST to value 0
impl crate::Resettable for JIFESOFST_SPEC {}
