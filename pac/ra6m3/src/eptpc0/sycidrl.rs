///Register `SYCIDRL` reader
pub type R = crate::R<SYCIDRL_SPEC>;
///Register `SYCIDRL` writer
pub type W = crate::W<SYCIDRL_SPEC>;
///Field `SYCIDRL` reader - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port.
pub type SYCIDRL_R = crate::FieldReader<u32>;
///Field `SYCIDRL` writer - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port.
pub type SYCIDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port.
    #[inline(always)]
    pub fn sycidrl(&self) -> SYCIDRL_R {
        SYCIDRL_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the lower-order 32 bits of the clock-ID of your port.
    #[inline(always)]
    pub fn sycidrl(&mut self) -> SYCIDRL_W<SYCIDRL_SPEC> {
        SYCIDRL_W::new(self, 0)
    }
}
/**SYNFP Local Clock ID Registers

You can [`read`](crate::Reg::read) this register and get [`sycidrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sycidrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYCIDRL_SPEC;
impl crate::RegisterSpec for SYCIDRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sycidrl::R`](R) reader structure
impl crate::Readable for SYCIDRL_SPEC {}
///`write(|w| ..)` method takes [`sycidrl::W`](W) writer structure
impl crate::Writable for SYCIDRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYCIDRL to value 0
impl crate::Resettable for SYCIDRL_SPEC {}
