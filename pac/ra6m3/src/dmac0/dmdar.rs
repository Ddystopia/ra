///Register `DMDAR` reader
pub type R = crate::R<DMDAR_SPEC>;
///Register `DMDAR` writer
pub type W = crate::W<DMDAR_SPEC>;
///Field `DMDAR` reader - Specifies the transfer destination start address.
pub type DMDAR_R = crate::FieldReader<u32>;
///Field `DMDAR` writer - Specifies the transfer destination start address.
pub type DMDAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Specifies the transfer destination start address.
    #[inline(always)]
    pub fn dmdar(&self) -> DMDAR_R {
        DMDAR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Specifies the transfer destination start address.
    #[inline(always)]
    pub fn dmdar(&mut self) -> DMDAR_W<DMDAR_SPEC> {
        DMDAR_W::new(self, 0)
    }
}
/**DMA Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`dmdar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMDAR_SPEC;
impl crate::RegisterSpec for DMDAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dmdar::R`](R) reader structure
impl crate::Readable for DMDAR_SPEC {}
///`write(|w| ..)` method takes [`dmdar::W`](W) writer structure
impl crate::Writable for DMDAR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMDAR to value 0
impl crate::Resettable for DMDAR_SPEC {}
