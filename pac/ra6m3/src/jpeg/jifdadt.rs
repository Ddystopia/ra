///Register `JIFDADT` reader
pub type R = crate::R<JIFDADT_SPEC>;
///Register `JIFDADT` writer
pub type W = crate::W<JIFDADT_SPEC>;
///Field `ALPHA` reader - Setting of the alpha value for output in ARGB8888 format.
pub type ALPHA_R = crate::FieldReader;
///Field `ALPHA` writer - Setting of the alpha value for output in ARGB8888 format.
pub type ALPHA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Setting of the alpha value for output in ARGB8888 format.
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Setting of the alpha value for output in ARGB8888 format.
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W<JIFDADT_SPEC> {
        ALPHA_W::new(self, 0)
    }
}
/**JPEG Interface Decompression alpha Set Register

You can [`read`](crate::Reg::read) this register and get [`jifdadt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jifdadt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JIFDADT_SPEC;
impl crate::RegisterSpec for JIFDADT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`jifdadt::R`](R) reader structure
impl crate::Readable for JIFDADT_SPEC {}
///`write(|w| ..)` method takes [`jifdadt::W`](W) writer structure
impl crate::Writable for JIFDADT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JIFDADT to value 0
impl crate::Resettable for JIFDADT_SPEC {}
