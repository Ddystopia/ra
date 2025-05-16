///Register `JCVSZD` reader
pub type R = crate::R<JCVSZD_SPEC>;
///Register `JCVSZD` writer
pub type W = crate::W<JCVSZD_SPEC>;
///Field `VSZD` reader - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type VSZD_R = crate::FieldReader;
///Field `VSZD` writer - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type VSZD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn vszd(&self) -> VSZD_R {
        VSZD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Lower Bytes of Vertical Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn vszd(&mut self) -> VSZD_W<JCVSZD_SPEC> {
        VSZD_W::new(self, 0)
    }
}
/**JPEG Code Vertical Size Lower Register

You can [`read`](crate::Reg::read) this register and get [`jcvszd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcvszd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCVSZD_SPEC;
impl crate::RegisterSpec for JCVSZD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcvszd::R`](R) reader structure
impl crate::Readable for JCVSZD_SPEC {}
///`write(|w| ..)` method takes [`jcvszd::W`](W) writer structure
impl crate::Writable for JCVSZD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCVSZD to value 0
impl crate::Resettable for JCVSZD_SPEC {}
