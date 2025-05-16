///Register `ADSSTR0%s` reader
pub type R = crate::R<ADSSTR0_SPEC>;
///Register `ADSSTR0%s` writer
pub type W = crate::W<ADSSTR0_SPEC>;
///Field `SST` reader - Sampling time setting
pub type SST_R = crate::FieldReader;
///Field `SST` writer - Sampling time setting
pub type SST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Sampling time setting
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Sampling time setting
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W<ADSSTR0_SPEC> {
        SST_W::new(self, 0)
    }
}
/**A/D Sampling State Register %s (Corresponding Channel is AN00%s )

You can [`read`](crate::Reg::read) this register and get [`adsstr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSSTR0_SPEC;
impl crate::RegisterSpec for ADSSTR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adsstr0::R`](R) reader structure
impl crate::Readable for ADSSTR0_SPEC {}
///`write(|w| ..)` method takes [`adsstr0::W`](W) writer structure
impl crate::Writable for ADSSTR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTR0%s to value 0x0b
impl crate::Resettable for ADSSTR0_SPEC {
    const RESET_VALUE: u8 = 0x0b;
}
