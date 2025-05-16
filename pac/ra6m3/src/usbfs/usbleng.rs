///Register `USBLENG` reader
pub type R = crate::R<USBLENG_SPEC>;
///Register `USBLENG` writer
pub type W = crate::W<USBLENG_SPEC>;
///Field `WLENGTH` reader - LengthThese bits store the USB request wLength value.
pub type WLENGTH_R = crate::FieldReader<u16>;
///Field `WLENGTH` writer - LengthThese bits store the USB request wLength value.
pub type WLENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - LengthThese bits store the USB request wLength value.
    #[inline(always)]
    pub fn wlength(&self) -> WLENGTH_R {
        WLENGTH_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - LengthThese bits store the USB request wLength value.
    #[inline(always)]
    pub fn wlength(&mut self) -> WLENGTH_W<USBLENG_SPEC> {
        WLENGTH_W::new(self, 0)
    }
}
/**USB Request Length Register

You can [`read`](crate::Reg::read) this register and get [`usbleng::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbleng::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBLENG_SPEC;
impl crate::RegisterSpec for USBLENG_SPEC {
    type Ux = u16;
}
///`read()` method returns [`usbleng::R`](R) reader structure
impl crate::Readable for USBLENG_SPEC {}
///`write(|w| ..)` method takes [`usbleng::W`](W) writer structure
impl crate::Writable for USBLENG_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBLENG to value 0
impl crate::Resettable for USBLENG_SPEC {}
