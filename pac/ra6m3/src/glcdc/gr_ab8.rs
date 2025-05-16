///Register `GR%s_AB8` reader
pub type R = crate::R<GR_AB8_SPEC>;
///Register `GR%s_AB8` writer
pub type W = crate::W<GR_AB8_SPEC>;
///Field `CKKR` reader - R signal for RGB-index chroma-key processingUnsigned; 8 bits.
pub type CKKR_R = crate::FieldReader;
///Field `CKKR` writer - R signal for RGB-index chroma-key processingUnsigned; 8 bits.
pub type CKKR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKKB` reader - B signal for RGB-index chroma-key processingUnsigned; 8 bits.
pub type CKKB_R = crate::FieldReader;
///Field `CKKB` writer - B signal for RGB-index chroma-key processingUnsigned; 8 bits.
pub type CKKB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKKG` reader - G signal for RGB-index chroma-key processingUnsigned; 8 bits.
pub type CKKG_R = crate::FieldReader;
///Field `CKKG` writer - G signal for RGB-index chroma-key processingUnsigned; 8 bits.
pub type CKKG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - R signal for RGB-index chroma-key processingUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckkr(&self) -> CKKR_R {
        CKKR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - B signal for RGB-index chroma-key processingUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckkb(&self) -> CKKB_R {
        CKKB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - G signal for RGB-index chroma-key processingUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckkg(&self) -> CKKG_R {
        CKKG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - R signal for RGB-index chroma-key processingUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckkr(&mut self) -> CKKR_W<GR_AB8_SPEC> {
        CKKR_W::new(self, 0)
    }
    ///Bits 8:15 - B signal for RGB-index chroma-key processingUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckkb(&mut self) -> CKKB_W<GR_AB8_SPEC> {
        CKKB_W::new(self, 8)
    }
    ///Bits 16:23 - G signal for RGB-index chroma-key processingUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckkg(&mut self) -> CKKG_W<GR_AB8_SPEC> {
        CKKG_W::new(self, 16)
    }
}
/**Graphics %s Alpha Blending Control Register 8

You can [`read`](crate::Reg::read) this register and get [`gr_ab8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB8_SPEC;
impl crate::RegisterSpec for GR_AB8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab8::R`](R) reader structure
impl crate::Readable for GR_AB8_SPEC {}
///`write(|w| ..)` method takes [`gr_ab8::W`](W) writer structure
impl crate::Writable for GR_AB8_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB8 to value 0
impl crate::Resettable for GR_AB8_SPEC {}
