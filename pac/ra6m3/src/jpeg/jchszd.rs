///Register `JCHSZD` reader
pub type R = crate::R<JCHSZD_SPEC>;
///Register `JCHSZD` writer
pub type W = crate::W<JCHSZD_SPEC>;
///Field `HSZD` reader - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type HSZD_R = crate::FieldReader;
///Field `HSZD` writer - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
pub type HSZD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hszd(&self) -> HSZD_R {
        HSZD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Lower Bytes of Horizontal Image SizeIn decompression process, a downloaded value from the JPEG coded data is set. NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn hszd(&mut self) -> HSZD_W<JCHSZD_SPEC> {
        HSZD_W::new(self, 0)
    }
}
/**JPEG Coded Horizontal Size Lower Register

You can [`read`](crate::Reg::read) this register and get [`jchszd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jchszd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCHSZD_SPEC;
impl crate::RegisterSpec for JCHSZD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jchszd::R`](R) reader structure
impl crate::Readable for JCHSZD_SPEC {}
///`write(|w| ..)` method takes [`jchszd::W`](W) writer structure
impl crate::Writable for JCHSZD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCHSZD to value 0
impl crate::Resettable for JCHSZD_SPEC {}
