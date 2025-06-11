///Register `IWDTRR` reader
pub type R = crate::R<IWDTRR_SPEC>;
///Register `IWDTRR` writer
pub type W = crate::W<IWDTRR_SPEC>;
///Field `IWDTRR` reader - The counter is refreshed by writing 0x00 and then writing 0xFF to this register.
pub type IWDTRR_R = crate::FieldReader;
///Field `IWDTRR` writer - The counter is refreshed by writing 0x00 and then writing 0xFF to this register.
pub type IWDTRR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - The counter is refreshed by writing 0x00 and then writing 0xFF to this register.
    #[inline(always)]
    pub fn iwdtrr(&self) -> IWDTRR_R {
        IWDTRR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - The counter is refreshed by writing 0x00 and then writing 0xFF to this register.
    #[inline(always)]
    pub fn iwdtrr(&mut self) -> IWDTRR_W<IWDTRR_SPEC> {
        IWDTRR_W::new(self, 0)
    }
}
/**IWDT Refresh Register

You can [`read`](crate::Reg::read) this register and get [`iwdtrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IWDTRR_SPEC;
impl crate::RegisterSpec for IWDTRR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`iwdtrr::R`](R) reader structure
impl crate::Readable for IWDTRR_SPEC {}
///`write(|w| ..)` method takes [`iwdtrr::W`](W) writer structure
impl crate::Writable for IWDTRR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IWDTRR to value 0xff
impl crate::Resettable for IWDTRR_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
