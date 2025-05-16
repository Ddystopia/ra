///Register `ADCMPDR1` reader
pub type R = crate::R<ADCMPDR1_SPEC>;
///Register `ADCMPDR1` writer
pub type W = crate::W<ADCMPDR1_SPEC>;
///Field `ADCMPDR1` reader - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A..
pub type ADCMPDR1_R = crate::FieldReader<u16>;
///Field `ADCMPDR1` writer - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A..
pub type ADCMPDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A..
    #[inline(always)]
    pub fn adcmpdr1(&self) -> ADCMPDR1_R {
        ADCMPDR1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - The ADCMPDR1 register sets the reference data when the compare window A function is used. ADCMPDR1 sets the upper-side level of window A..
    #[inline(always)]
    pub fn adcmpdr1(&mut self) -> ADCMPDR1_W<ADCMPDR1_SPEC> {
        ADCMPDR1_W::new(self, 0)
    }
}
/**A/D Compare Function Window A Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpdr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPDR1_SPEC;
impl crate::RegisterSpec for ADCMPDR1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpdr1::R`](R) reader structure
impl crate::Readable for ADCMPDR1_SPEC {}
///`write(|w| ..)` method takes [`adcmpdr1::W`](W) writer structure
impl crate::Writable for ADCMPDR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPDR1 to value 0
impl crate::Resettable for ADCMPDR1_SPEC {}
