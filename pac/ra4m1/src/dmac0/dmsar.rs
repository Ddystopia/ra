///Register `DMSAR` reader
pub type R = crate::R<DMSAR_SPEC>;
///Register `DMSAR` writer
pub type W = crate::W<DMSAR_SPEC>;
///Field `DMSAR` reader - Specifies the transfer source start address.
pub type DMSAR_R = crate::FieldReader<u32>;
///Field `DMSAR` writer - Specifies the transfer source start address.
pub type DMSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Specifies the transfer source start address.
    #[inline(always)]
    pub fn dmsar(&self) -> DMSAR_R {
        DMSAR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Specifies the transfer source start address.
    #[inline(always)]
    pub fn dmsar(&mut self) -> DMSAR_W<DMSAR_SPEC> {
        DMSAR_W::new(self, 0)
    }
}
/**DMA Source Address Register

You can [`read`](crate::Reg::read) this register and get [`dmsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMSAR_SPEC;
impl crate::RegisterSpec for DMSAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dmsar::R`](R) reader structure
impl crate::Readable for DMSAR_SPEC {}
///`write(|w| ..)` method takes [`dmsar::W`](W) writer structure
impl crate::Writable for DMSAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMSAR to value 0
impl crate::Resettable for DMSAR_SPEC {}
