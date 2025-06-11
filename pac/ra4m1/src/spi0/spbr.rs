///Register `SPBR` reader
pub type R = crate::R<SPBR_SPEC>;
///Register `SPBR` writer
pub type W = crate::W<SPBR_SPEC>;
///Field `SPR` reader - SPBR sets the bit rate in master mode.
pub type SPR_R = crate::FieldReader;
///Field `SPR` writer - SPBR sets the bit rate in master mode.
pub type SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - SPBR sets the bit rate in master mode.
    #[inline(always)]
    pub fn spr(&self) -> SPR_R {
        SPR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - SPBR sets the bit rate in master mode.
    #[inline(always)]
    pub fn spr(&mut self) -> SPR_W<SPBR_SPEC> {
        SPR_W::new(self, 0)
    }
}
/**SPI Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`spbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPBR_SPEC;
impl crate::RegisterSpec for SPBR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`spbr::R`](R) reader structure
impl crate::Readable for SPBR_SPEC {}
///`write(|w| ..)` method takes [`spbr::W`](W) writer structure
impl crate::Writable for SPBR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPBR to value 0xff
impl crate::Resettable for SPBR_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
