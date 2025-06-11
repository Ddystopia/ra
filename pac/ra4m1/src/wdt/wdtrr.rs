///Register `WDTRR` reader
pub type R = crate::R<WDTRR_SPEC>;
///Register `WDTRR` writer
pub type W = crate::W<WDTRR_SPEC>;
///Field `WDTRR` reader - WDTRR is an 8-bit register that refreshes the down-counter of the WDT.
pub type WDTRR_R = crate::FieldReader;
///Field `WDTRR` writer - WDTRR is an 8-bit register that refreshes the down-counter of the WDT.
pub type WDTRR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - WDTRR is an 8-bit register that refreshes the down-counter of the WDT.
    #[inline(always)]
    pub fn wdtrr(&self) -> WDTRR_R {
        WDTRR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - WDTRR is an 8-bit register that refreshes the down-counter of the WDT.
    #[inline(always)]
    pub fn wdtrr(&mut self) -> WDTRR_W<WDTRR_SPEC> {
        WDTRR_W::new(self, 0)
    }
}
/**WDT Refresh Register

You can [`read`](crate::Reg::read) this register and get [`wdtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDTRR_SPEC;
impl crate::RegisterSpec for WDTRR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`wdtrr::R`](R) reader structure
impl crate::Readable for WDTRR_SPEC {}
///`write(|w| ..)` method takes [`wdtrr::W`](W) writer structure
impl crate::Writable for WDTRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTRR to value 0xff
impl crate::Resettable for WDTRR_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
