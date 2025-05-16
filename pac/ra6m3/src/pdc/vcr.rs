///Register `VCR` reader
pub type R = crate::R<VCR_SPEC>;
///Register `VCR` writer
pub type W = crate::W<VCR_SPEC>;
///Field `VST` reader - Vertical Capture Start Line PositionNumber of the line where capture is to start.
pub type VST_R = crate::FieldReader<u16>;
///Field `VST` writer - Vertical Capture Start Line PositionNumber of the line where capture is to start.
pub type VST_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `VSZ` reader - Vertical Capture Size Number of lines to be captured.
pub type VSZ_R = crate::FieldReader<u16>;
///Field `VSZ` writer - Vertical Capture Size Number of lines to be captured.
pub type VSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Vertical Capture Start Line PositionNumber of the line where capture is to start.
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Vertical Capture Size Number of lines to be captured.
    #[inline(always)]
    pub fn vsz(&self) -> VSZ_R {
        VSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Vertical Capture Start Line PositionNumber of the line where capture is to start.
    #[inline(always)]
    pub fn vst(&mut self) -> VST_W<VCR_SPEC> {
        VST_W::new(self, 0)
    }
    ///Bits 16:27 - Vertical Capture Size Number of lines to be captured.
    #[inline(always)]
    pub fn vsz(&mut self) -> VSZ_W<VCR_SPEC> {
        VSZ_W::new(self, 16)
    }
}
/**Vertical Capture Register

You can [`read`](crate::Reg::read) this register and get [`vcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VCR_SPEC;
impl crate::RegisterSpec for VCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vcr::R`](R) reader structure
impl crate::Readable for VCR_SPEC {}
///`write(|w| ..)` method takes [`vcr::W`](W) writer structure
impl crate::Writable for VCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VCR to value 0
impl crate::Resettable for VCR_SPEC {}
