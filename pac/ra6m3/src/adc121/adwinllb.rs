///Register `ADWINLLB` reader
pub type R = crate::R<ADWINLLB_SPEC>;
///Register `ADWINLLB` writer
pub type W = crate::W<ADWINLLB_SPEC>;
///Field `ADWINLLB` reader - This register is used to compare A window function is used to set the lower level of the window B.
pub type ADWINLLB_R = crate::FieldReader<u16>;
///Field `ADWINLLB` writer - This register is used to compare A window function is used to set the lower level of the window B.
pub type ADWINLLB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B.
    #[inline(always)]
    pub fn adwinllb(&self) -> ADWINLLB_R {
        ADWINLLB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - This register is used to compare A window function is used to set the lower level of the window B.
    #[inline(always)]
    pub fn adwinllb(&mut self) -> ADWINLLB_W<ADWINLLB_SPEC> {
        ADWINLLB_W::new(self, 0)
    }
}
/**A/D Compare Function Window B Lower-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinllb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinllb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADWINLLB_SPEC;
impl crate::RegisterSpec for ADWINLLB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adwinllb::R`](R) reader structure
impl crate::Readable for ADWINLLB_SPEC {}
///`write(|w| ..)` method takes [`adwinllb::W`](W) writer structure
impl crate::Writable for ADWINLLB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADWINLLB to value 0
impl crate::Resettable for ADWINLLB_SPEC {}
