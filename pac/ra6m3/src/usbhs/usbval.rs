///Register `USBVAL` reader
pub type R = crate::R<USBVAL_SPEC>;
///Register `USBVAL` writer
pub type W = crate::W<USBVAL_SPEC>;
///Field `WVALUE` reader - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write
pub type WVALUE_R = crate::FieldReader<u16>;
///Field `WVALUE` writer - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write
pub type WVALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Value of USB request wValue Finction controller selected : read-only Host controller selected : read-write
    #[inline(always)]
    pub fn wvalue(&mut self) -> WVALUE_W<USBVAL_SPEC> {
        WVALUE_W::new(self, 0)
    }
}
/**USB Request Value Register

You can [`read`](crate::Reg::read) this register and get [`usbval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBVAL_SPEC;
impl crate::RegisterSpec for USBVAL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbval::R`](R) reader structure
impl crate::Readable for USBVAL_SPEC {}
///`write(|w| ..)` method takes [`usbval::W`](W) writer structure
impl crate::Writable for USBVAL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBVAL to value 0
impl crate::Resettable for USBVAL_SPEC {}
