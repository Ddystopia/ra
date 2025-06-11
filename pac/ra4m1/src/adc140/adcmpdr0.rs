///Register `ADCMPDR0` reader
pub type R = crate::R<ADCMPDR0_SPEC>;
///Register `ADCMPDR0` writer
pub type W = crate::W<ADCMPDR0_SPEC>;
///Field `ADCMPDR0` reader - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A.
pub type ADCMPDR0_R = crate::FieldReader<u16>;
///Field `ADCMPDR0` writer - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A.
pub type ADCMPDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A.
    #[inline(always)]
    pub fn adcmpdr0(&self) -> ADCMPDR0_R {
        ADCMPDR0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - The ADCMPDR0 register sets the reference data when the compare window A function is used. ADCMPDR0 sets the lower-side level of window A.
    #[inline(always)]
    pub fn adcmpdr0(&mut self) -> ADCMPDR0_W<ADCMPDR0_SPEC> {
        ADCMPDR0_W::new(self, 0)
    }
}
/**A/D Compare Function Window A Lower-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpdr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPDR0_SPEC;
impl crate::RegisterSpec for ADCMPDR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpdr0::R`](R) reader structure
impl crate::Readable for ADCMPDR0_SPEC {}
///`write(|w| ..)` method takes [`adcmpdr0::W`](W) writer structure
impl crate::Writable for ADCMPDR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPDR0 to value 0
impl crate::Resettable for ADCMPDR0_SPEC {}
