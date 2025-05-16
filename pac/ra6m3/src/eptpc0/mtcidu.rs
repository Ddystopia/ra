///Register `MTCIDU` reader
pub type R = crate::R<MTCIDU_SPEC>;
///Register `MTCIDU` writer
pub type W = crate::W<MTCIDU_SPEC>;
///Field `MTCIDU` reader - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock.
pub type MTCIDU_R = crate::FieldReader<u32>;
///Field `MTCIDU` writer - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock.
pub type MTCIDU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock.
    #[inline(always)]
    pub fn mtcidu(&self) -> MTCIDU_R {
        MTCIDU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of the master clock.
    #[inline(always)]
    pub fn mtcidu(&mut self) -> MTCIDU_W<MTCIDU_SPEC> {
        MTCIDU_W::new(self, 0)
    }
}
/**Master Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`mtcidu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcidu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MTCIDU_SPEC;
impl crate::RegisterSpec for MTCIDU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mtcidu::R`](R) reader structure
impl crate::Readable for MTCIDU_SPEC {}
///`write(|w| ..)` method takes [`mtcidu::W`](W) writer structure
impl crate::Writable for MTCIDU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTCIDU to value 0
impl crate::Resettable for MTCIDU_SPEC {}
