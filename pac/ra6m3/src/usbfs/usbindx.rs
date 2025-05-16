///Register `USBINDX` reader
pub type R = crate::R<USBINDX_SPEC>;
///Register `USBINDX` writer
pub type W = crate::W<USBINDX_SPEC>;
///Field `WINDEX` reader - IndexThese bits store the USB request wIndex value.
pub type WINDEX_R = crate::FieldReader<u16>;
///Field `WINDEX` writer - IndexThese bits store the USB request wIndex value.
pub type WINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - IndexThese bits store the USB request wIndex value.
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - IndexThese bits store the USB request wIndex value.
    #[inline(always)]
    pub fn windex(&mut self) -> WINDEX_W<USBINDX_SPEC> {
        WINDEX_W::new(self, 0)
    }
}
/**USB Request Index Register

You can [`read`](crate::Reg::read) this register and get [`usbindx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbindx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBINDX_SPEC;
impl crate::RegisterSpec for USBINDX_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbindx::R`](R) reader structure
impl crate::Readable for USBINDX_SPEC {}
///`write(|w| ..)` method takes [`usbindx::W`](W) writer structure
impl crate::Writable for USBINDX_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBINDX to value 0
impl crate::Resettable for USBINDX_SPEC {}
