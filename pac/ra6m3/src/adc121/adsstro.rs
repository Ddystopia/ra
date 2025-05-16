///Register `ADSSTRO` reader
pub type R = crate::R<ADSSTRO_SPEC>;
///Register `ADSSTRO` writer
pub type W = crate::W<ADSSTRO_SPEC>;
///Field `SST` reader - Sampling Time Setting (Internal reference voltage)
pub type SST_R = crate::FieldReader;
///Field `SST` writer - Sampling Time Setting (Internal reference voltage)
pub type SST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sampling Time Setting (Internal reference voltage)
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Sampling Time Setting (Internal reference voltage)
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W<ADSSTRO_SPEC> {
        SST_W::new(self, 0)
    }
}
/**A/D Sampling State Register O

You can [`read`](crate::Reg::read) this register and get [`adsstro::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstro::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSSTRO_SPEC;
impl crate::RegisterSpec for ADSSTRO_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adsstro::R`](R) reader structure
impl crate::Readable for ADSSTRO_SPEC {}
///`write(|w| ..)` method takes [`adsstro::W`](W) writer structure
impl crate::Writable for ADSSTRO_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTRO to value 0x0b
impl crate::Resettable for ADSSTRO_SPEC {
    const RESET_VALUE: u8 = 0x0b;
}
