///Register `RFRH` reader
pub type R = crate::R<RFRH_SPEC>;
///Register `RFRH` writer
pub type W = crate::W<RFRH_SPEC>;
///Field `RFC16` reader - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle.
pub type RFC16_R = crate::BitReader;
///Field `RFC16` writer - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle.
pub type RFC16_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle.
    #[inline(always)]
    pub fn rfc16(&self) -> RFC16_R {
        RFC16_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Frequency Comparison Value (b16) To generate the operating clock from the LOCOclock, this bit sets the comparison value of the 128-Hz clock cycle.
    #[inline(always)]
    pub fn rfc16(&mut self) -> RFC16_W<RFRH_SPEC> {
        RFC16_W::new(self, 0)
    }
}
/**Frequency Register H

You can [`read`](crate::Reg::read) this register and get [`rfrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFRH_SPEC;
impl crate::RegisterSpec for RFRH_SPEC {
    type Ux = u16;
}
///`read()` method returns [`rfrh::R`](R) reader structure
impl crate::Readable for RFRH_SPEC {}
///`write(|w| ..)` method takes [`rfrh::W`](W) writer structure
impl crate::Writable for RFRH_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFRH to value 0
impl crate::Resettable for RFRH_SPEC {}
