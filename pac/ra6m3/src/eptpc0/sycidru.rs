///Register `SYCIDRU` reader
pub type R = crate::R<SYCIDRU_SPEC>;
///Register `SYCIDRU` writer
pub type W = crate::W<SYCIDRU_SPEC>;
///Field `SYCIDRU` reader - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port.
pub type SYCIDRU_R = crate::FieldReader<u32>;
///Field `SYCIDRU` writer - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port.
pub type SYCIDRU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port.
    #[inline(always)]
    pub fn sycidru(&self) -> SYCIDRU_R {
        SYCIDRU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the higher-order 32 bits of the clock-ID of your port.
    #[inline(always)]
    pub fn sycidru(&mut self) -> SYCIDRU_W<SYCIDRU_SPEC> {
        SYCIDRU_W::new(self, 0)
    }
}
/**SYNFP Local Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`sycidru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sycidru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYCIDRU_SPEC;
impl crate::RegisterSpec for SYCIDRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sycidru::R`](R) reader structure
impl crate::Readable for SYCIDRU_SPEC {}
///`write(|w| ..)` method takes [`sycidru::W`](W) writer structure
impl crate::Writable for SYCIDRU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYCIDRU to value 0
impl crate::Resettable for SYCIDRU_SPEC {}
