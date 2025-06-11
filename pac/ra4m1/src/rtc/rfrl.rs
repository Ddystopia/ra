///Register `RFRL` reader
pub type R = crate::R<RFRL_SPEC>;
///Register `RFRL` writer
pub type W = crate::W<RFRL_SPEC>;
///Field `RFC` reader - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle.
pub type RFC_R = crate::FieldReader<u16>;
///Field `RFC` writer - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle.
pub type RFC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle.
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Frequency Comparison Value(b15-b0) To generate the operating clock from the main clock, this bit sets the comparison value of the 128-Hz clock cycle.
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W<RFRL_SPEC> {
        RFC_W::new(self, 0)
    }
}
/**Frequency Register L

You can [`read`](crate::Reg::read) this register and get [`rfrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RFRL_SPEC;
impl crate::RegisterSpec for RFRL_SPEC {
    type Ux = u16;
}
///`read()` method returns [`rfrl::R`](R) reader structure
impl crate::Readable for RFRL_SPEC {}
///`write(|w| ..)` method takes [`rfrl::W`](W) writer structure
impl crate::Writable for RFRL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RFRL to value 0
impl crate::Resettable for RFRL_SPEC {}
