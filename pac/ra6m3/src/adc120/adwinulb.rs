///Register `ADWINULB` reader
pub type R = crate::R<ADWINULB_SPEC>;
///Register `ADWINULB` writer
pub type W = crate::W<ADWINULB_SPEC>;
///Field `ADWINULB` reader - This register is used to compare A window function is used to set the higher level of the window B.
pub type ADWINULB_R = crate::FieldReader<u16>;
///Field `ADWINULB` writer - This register is used to compare A window function is used to set the higher level of the window B.
pub type ADWINULB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - This register is used to compare A window function is used to set the higher level of the window B.
    #[inline(always)]
    pub fn adwinulb(&self) -> ADWINULB_R {
        ADWINULB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - This register is used to compare A window function is used to set the higher level of the window B.
    #[inline(always)]
    pub fn adwinulb(&mut self) -> ADWINULB_W<ADWINULB_SPEC> {
        ADWINULB_W::new(self, 0)
    }
}
/**A/D Compare Function Window B Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinulb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinulb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADWINULB_SPEC;
impl crate::RegisterSpec for ADWINULB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adwinulb::R`](R) reader structure
impl crate::Readable for ADWINULB_SPEC {}
///`write(|w| ..)` method takes [`adwinulb::W`](W) writer structure
impl crate::Writable for ADWINULB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADWINULB to value 0
impl crate::Resettable for ADWINULB_SPEC {}
