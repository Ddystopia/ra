///Register `HCR` reader
pub type R = crate::R<HCR_SPEC>;
///Register `HCR` writer
pub type W = crate::W<HCR_SPEC>;
///Field `HST` reader - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start.
pub type HST_R = crate::FieldReader<u16>;
///Field `HST` writer - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start.
pub type HST_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `HSZ` reader - Horizontal Capture Size Number of bytes to capture horizontally.
pub type HSZ_R = crate::FieldReader<u16>;
///Field `HSZ` writer - Horizontal Capture Size Number of bytes to capture horizontally.
pub type HSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start.
    #[inline(always)]
    pub fn hst(&self) -> HST_R {
        HST_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Horizontal Capture Size Number of bytes to capture horizontally.
    #[inline(always)]
    pub fn hsz(&self) -> HSZ_R {
        HSZ_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Horizontal Capture Start Byte Position Horizontal position in bytes where capture is to start.
    #[inline(always)]
    pub fn hst(&mut self) -> HST_W<HCR_SPEC> {
        HST_W::new(self, 0)
    }
    ///Bits 16:27 - Horizontal Capture Size Number of bytes to capture horizontally.
    #[inline(always)]
    pub fn hsz(&mut self) -> HSZ_W<HCR_SPEC> {
        HSZ_W::new(self, 16)
    }
}
/**Horizontal Capture Register

You can [`read`](crate::Reg::read) this register and get [`hcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HCR_SPEC;
impl crate::RegisterSpec for HCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hcr::R`](R) reader structure
impl crate::Readable for HCR_SPEC {}
///`write(|w| ..)` method takes [`hcr::W`](W) writer structure
impl crate::Writable for HCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HCR to value 0
impl crate::Resettable for HCR_SPEC {}
