///Register `DODSR` reader
pub type R = crate::R<DODSR_SPEC>;
///Register `DODSR` writer
pub type W = crate::W<DODSR_SPEC>;
///Field `DODSR` reader - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes.
pub type DODSR_R = crate::FieldReader<u16>;
///Field `DODSR` writer - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes.
pub type DODSR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes.
    #[inline(always)]
    pub fn dodsr(&self) -> DODSR_R {
        DODSR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - This register stores 16-bit data for use as a reference in data comparison mode. This register also stores the results of operations in data addition and data subtraction modes.
    #[inline(always)]
    pub fn dodsr(&mut self) -> DODSR_W<DODSR_SPEC> {
        DODSR_W::new(self, 0)
    }
}
/**DOC Data Setting Register

You can [`read`](crate::Reg::read) this register and get [`dodsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dodsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DODSR_SPEC;
impl crate::RegisterSpec for DODSR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dodsr::R`](R) reader structure
impl crate::Readable for DODSR_SPEC {}
///`write(|w| ..)` method takes [`dodsr::W`](W) writer structure
impl crate::Writable for DODSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DODSR to value 0
impl crate::Resettable for DODSR_SPEC {}
