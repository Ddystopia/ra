///Register `SIZE` writer
pub type W = crate::W<SIZE_SPEC>;
///Field `SIZEX` writer - Width of the bounding box in pixelsvalid range: 0 to 1024
pub type SIZEX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SIZEY` writer - Height of the bounding box in pixelsvalid range: 0 to 1024
pub type SIZEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    ///Bits 0:15 - Width of the bounding box in pixelsvalid range: 0 to 1024
    #[inline(always)]
    pub fn sizex(&mut self) -> SIZEX_W<SIZE_SPEC> {
        SIZEX_W::new(self, 0)
    }
    ///Bits 16:31 - Height of the bounding box in pixelsvalid range: 0 to 1024
    #[inline(always)]
    pub fn sizey(&mut self) -> SIZEY_W<SIZE_SPEC> {
        SIZEY_W::new(self, 16)
    }
}
/**Bounding Box Dimension Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`size::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SIZE_SPEC;
impl crate::RegisterSpec for SIZE_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`size::W`](W) writer structure
impl crate::Writable for SIZE_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SIZE to value 0
impl crate::Resettable for SIZE_SPEC {}
