///Register `ADSSTR%s` reader
pub type R = crate::R<ADSSTR_SPEC>;
///Register `ADSSTR%s` writer
pub type W = crate::W<ADSSTR_SPEC>;
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
    pub fn sst(&mut self) -> SST_W<'_, ADSSTR_SPEC> {
        SST_W::new(self, 0)
    }
}
/**A/D Sampling State Register %s

You can [`read`](crate::Reg::read) this register and get [`adsstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSSTR_SPEC;
impl crate::RegisterSpec for ADSSTR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`adsstr::R`](R) reader structure
impl crate::Readable for ADSSTR_SPEC {}
///`write(|w| ..)` method takes [`adsstr::W`](W) writer structure
impl crate::Writable for ADSSTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSSTR%s to value 0x0d
impl crate::Resettable for ADSSTR_SPEC {
    const RESET_VALUE: u8 = 0x0d;
}
