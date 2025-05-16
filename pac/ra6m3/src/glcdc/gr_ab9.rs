///Register `GR%s_AB9` reader
pub type R = crate::R<GR_AB9_SPEC>;
///Register `GR%s_AB9` writer
pub type W = crate::W<GR_AB9_SPEC>;
///Field `CKR` reader - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
pub type CKR_R = crate::FieldReader;
///Field `CKR` writer - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
pub type CKR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKB` reader - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
pub type CKB_R = crate::FieldReader;
///Field `CKB` writer - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
pub type CKB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKG` reader - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
pub type CKG_R = crate::FieldReader;
///Field `CKG` writer - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
pub type CKG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CKA` reader - A value after RGB-index chroma-key processing replacement.
pub type CKA_R = crate::FieldReader;
///Field `CKA` writer - A value after RGB-index chroma-key processing replacement.
pub type CKA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckr(&self) -> CKR_R {
        CKR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckb(&self) -> CKB_R {
        CKB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - A value after RGB-index chroma-key processing replacement.
    #[inline(always)]
    pub fn cka(&self) -> CKA_R {
        CKA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - R value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckr(&mut self) -> CKR_W<GR_AB9_SPEC> {
        CKR_W::new(self, 0)
    }
    ///Bits 8:15 - B value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckb(&mut self) -> CKB_W<GR_AB9_SPEC> {
        CKB_W::new(self, 8)
    }
    ///Bits 16:23 - G value after RGB-index chroma-key processing replacementUnsigned; 8 bits.
    #[inline(always)]
    pub fn ckg(&mut self) -> CKG_W<GR_AB9_SPEC> {
        CKG_W::new(self, 16)
    }
    ///Bits 24:31 - A value after RGB-index chroma-key processing replacement.
    #[inline(always)]
    pub fn cka(&mut self) -> CKA_W<GR_AB9_SPEC> {
        CKA_W::new(self, 24)
    }
}
/**Graphics %s Alpha Blending Control Register 9

You can [`read`](crate::Reg::read) this register and get [`gr_ab9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gr_ab9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GR_AB9_SPEC;
impl crate::RegisterSpec for GR_AB9_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gr_ab9::R`](R) reader structure
impl crate::Readable for GR_AB9_SPEC {}
///`write(|w| ..)` method takes [`gr_ab9::W`](W) writer structure
impl crate::Writable for GR_AB9_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GR%s_AB9 to value 0
impl crate::Resettable for GR_AB9_SPEC {}
