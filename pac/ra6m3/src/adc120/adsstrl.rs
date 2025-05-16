///Register `ADSSTRL` reader
pub type R = crate::R<ADSSTRL_SPEC>;
///Register `ADSSTRL` writer
pub type W = crate::W<ADSSTRL_SPEC>;
///Field `SST` reader - Sampling Time Setting (AN016-AN020)
pub type SST_R = crate::FieldReader;
///Field `SST` writer - Sampling Time Setting (AN016-AN020)
pub type SST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sampling Time Setting (AN016-AN020)
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Sampling Time Setting (AN016-AN020)
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W<ADSSTRL_SPEC> {
        SST_W::new(self, 0)
    }
}
/**A/D Sampling State Register L

You can [`read`](crate::Reg::read) this register and get [`adsstrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSSTRL_SPEC;
impl crate::RegisterSpec for ADSSTRL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adsstrl::R`](R) reader structure
impl crate::Readable for ADSSTRL_SPEC {}
///`write(|w| ..)` method takes [`adsstrl::W`](W) writer structure
impl crate::Writable for ADSSTRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTRL to value 0x0b
impl crate::Resettable for ADSSTRL_SPEC {
    const RESET_VALUE: u8 = 0x0b;
}
