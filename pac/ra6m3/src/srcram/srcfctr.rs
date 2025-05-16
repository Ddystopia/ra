///Register `SRCFCTR[%s]` reader
pub type R = crate::R<SRCFCTR_SPEC>;
///Register `SRCFCTR[%s]` writer
pub type W = crate::W<SRCFCTR_SPEC>;
///Field `SRCFCOE` reader - Stores a filter coefficient value.
pub type SRCFCOE_R = crate::FieldReader<u32>;
///Field `SRCFCOE` writer - Stores a filter coefficient value.
pub type SRCFCOE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - Stores a filter coefficient value.
    #[inline(always)]
    pub fn srcfcoe(&self) -> SRCFCOE_R {
        SRCFCOE_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    ///Bits 0:21 - Stores a filter coefficient value.
    #[inline(always)]
    pub fn srcfcoe(&mut self) -> SRCFCOE_W<SRCFCTR_SPEC> {
        SRCFCOE_W::new(self, 0)
    }
}
/**Filter Coefficient Table \[%s\]

You can [`read`](crate::Reg::read) this register and get [`srcfctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcfctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCFCTR_SPEC;
impl crate::RegisterSpec for SRCFCTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`srcfctr::R`](R) reader structure
impl crate::Readable for SRCFCTR_SPEC {}
///`write(|w| ..)` method takes [`srcfctr::W`](W) writer structure
impl crate::Writable for SRCFCTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRCFCTR[%s] to value 0
impl crate::Resettable for SRCFCTR_SPEC {}
