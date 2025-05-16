///Register `JCVSZU` reader
pub type R = crate::R<JCVSZU_SPEC>;
///Register `JCVSZU` writer
pub type W = crate::W<JCVSZU_SPEC>;
///Field `VSZU` reader - Upper Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type VSZU_R = crate::FieldReader;
///Field `VSZU` writer - Upper Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type VSZU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Upper Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn vszu(&self) -> VSZU_R {
        VSZU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Upper Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn vszu(&mut self) -> VSZU_W<JCVSZU_SPEC> {
        VSZU_W::new(self, 0)
    }
}
/**JPEG Code Vertical Size Upper Register

You can [`read`](crate::Reg::read) this register and get [`jcvszu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcvszu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCVSZU_SPEC;
impl crate::RegisterSpec for JCVSZU_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcvszu::R`](R) reader structure
impl crate::Readable for JCVSZU_SPEC {}
///`write(|w| ..)` method takes [`jcvszu::W`](W) writer structure
impl crate::Writable for JCVSZU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCVSZU to value 0
impl crate::Resettable for JCVSZU_SPEC {}
