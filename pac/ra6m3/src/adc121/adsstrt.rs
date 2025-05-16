///Register `ADSSTRT` reader
pub type R = crate::R<ADSSTRT_SPEC>;
///Register `ADSSTRT` writer
pub type W = crate::W<ADSSTRT_SPEC>;
///Field `SST` reader - Sampling Time Setting (temperature sensor output)
pub type SST_R = crate::FieldReader;
///Field `SST` writer - Sampling Time Setting (temperature sensor output)
pub type SST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sampling Time Setting (temperature sensor output)
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Sampling Time Setting (temperature sensor output)
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W<ADSSTRT_SPEC> {
        SST_W::new(self, 0)
    }
}
/**A/D Sampling State Register T

You can [`read`](crate::Reg::read) this register and get [`adsstrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSSTRT_SPEC;
impl crate::RegisterSpec for ADSSTRT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adsstrt::R`](R) reader structure
impl crate::Readable for ADSSTRT_SPEC {}
///`write(|w| ..)` method takes [`adsstrt::W`](W) writer structure
impl crate::Writable for ADSSTRT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTRT to value 0x0b
impl crate::Resettable for ADSSTRT_SPEC {
    const RESET_VALUE: u8 = 0x0b;
}
