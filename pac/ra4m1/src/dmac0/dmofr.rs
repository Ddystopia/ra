///Register `DMOFR` reader
pub type R = crate::R<DMOFR_SPEC>;
///Register `DMOFR` writer
pub type W = crate::W<DMOFR_SPEC>;
///Field `DMOFR` reader - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination.
pub type DMOFR_R = crate::FieldReader<u32>;
///Field `DMOFR` writer - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination.
pub type DMOFR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination.
    #[inline(always)]
    pub fn dmofr(&self) -> DMOFR_R {
        DMOFR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Specifies the offset when offset addition is selected as the address update mode for transfer source or destination.
    #[inline(always)]
    pub fn dmofr(&mut self) -> DMOFR_W<DMOFR_SPEC> {
        DMOFR_W::new(self, 0)
    }
}
/**DMA Offset Register

You can [`read`](crate::Reg::read) this register and get [`dmofr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmofr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMOFR_SPEC;
impl crate::RegisterSpec for DMOFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dmofr::R`](R) reader structure
impl crate::Readable for DMOFR_SPEC {}
///`write(|w| ..)` method takes [`dmofr::W`](W) writer structure
impl crate::Writable for DMOFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMOFR to value 0
impl crate::Resettable for DMOFR_SPEC {}
