///Register `MTCIDL` reader
pub type R = crate::R<MTCIDL_SPEC>;
///Register `MTCIDL` writer
pub type W = crate::W<MTCIDL_SPEC>;
///Field `MTCIDL` reader - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock.
pub type MTCIDL_R = crate::FieldReader<u32>;
///Field `MTCIDL` writer - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock.
pub type MTCIDL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock.
    #[inline(always)]
    pub fn mtcidl(&self) -> MTCIDL_R {
        MTCIDL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of the master clock.
    #[inline(always)]
    pub fn mtcidl(&mut self) -> MTCIDL_W<MTCIDL_SPEC> {
        MTCIDL_W::new(self, 0)
    }
}
/**Master Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`mtcidl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcidl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MTCIDL_SPEC;
impl crate::RegisterSpec for MTCIDL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mtcidl::R`](R) reader structure
impl crate::Readable for MTCIDL_SPEC {}
///`write(|w| ..)` method takes [`mtcidl::W`](W) writer structure
impl crate::Writable for MTCIDL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTCIDL to value 0
impl crate::Resettable for MTCIDL_SPEC {}
