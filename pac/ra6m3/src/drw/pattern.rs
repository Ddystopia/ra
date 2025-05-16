///Register `PATTERN` writer
pub type W = crate::W<PATTERN_SPEC>;
///Field `PATTERN` writer - Bitmap of the pattern
pub type PATTERN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    ///Bits 0:7 - Bitmap of the pattern
    #[inline(always)]
    pub fn pattern(&mut self) -> PATTERN_W<PATTERN_SPEC> {
        PATTERN_W::new(self, 0)
    }
}
/**Pattern Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattern::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PATTERN_SPEC;
impl crate::RegisterSpec for PATTERN_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pattern::W`](W) writer structure
impl crate::Writable for PATTERN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PATTERN to value 0
impl crate::Resettable for PATTERN_SPEC {}
