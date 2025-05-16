///Register `JCHSZU` reader
pub type R = crate::R<JCHSZU_SPEC>;
///Register `JCHSZU` writer
pub type W = crate::W<JCHSZU_SPEC>;
///Field `HSZU` reader - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type HSZU_R = crate::FieldReader;
///Field `HSZU` writer - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type HSZU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hszu(&self) -> HSZU_R {
        HSZU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Upper Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hszu(&mut self) -> HSZU_W<JCHSZU_SPEC> {
        HSZU_W::new(self, 0)
    }
}
/**JPEG Code Horizontal Size Upper Register

You can [`read`](crate::Reg::read) this register and get [`jchszu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchszu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCHSZU_SPEC;
impl crate::RegisterSpec for JCHSZU_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jchszu::R`](R) reader structure
impl crate::Readable for JCHSZU_SPEC {}
///`write(|w| ..)` method takes [`jchszu::W`](W) writer structure
impl crate::Writable for JCHSZU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCHSZU to value 0
impl crate::Resettable for JCHSZU_SPEC {}
