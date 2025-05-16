///Register `DTCVBR` reader
pub type R = crate::R<DTCVBR_SPEC>;
///Register `DTCVBR` writer
pub type W = crate::W<DTCVBR_SPEC>;
///Field `DTCVBR` reader - DTC Vector Base Address.Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0.
pub type DTCVBR_R = crate::FieldReader<u32>;
///Field `DTCVBR` writer - DTC Vector Base Address.Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0.
pub type DTCVBR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DTC Vector Base Address.Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0.
    #[inline(always)]
    pub fn dtcvbr(&self) -> DTCVBR_R {
        DTCVBR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DTC Vector Base Address.Note: A value cannot be set in the lower-order 10 bits. These bits are fixed to 0.
    #[inline(always)]
    pub fn dtcvbr(&mut self) -> DTCVBR_W<DTCVBR_SPEC> {
        DTCVBR_W::new(self, 0)
    }
}
/**DTC Vector Base Register

You can [`read`](crate::Reg::read) this register and get [`dtcvbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DTCVBR_SPEC;
impl crate::RegisterSpec for DTCVBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dtcvbr::R`](R) reader structure
impl crate::Readable for DTCVBR_SPEC {}
///`write(|w| ..)` method takes [`dtcvbr::W`](W) writer structure
impl crate::Writable for DTCVBR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTCVBR to value 0
impl crate::Resettable for DTCVBR_SPEC {}
