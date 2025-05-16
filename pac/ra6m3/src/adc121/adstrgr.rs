///Register `ADSTRGR` reader
pub type R = crate::R<ADSTRGR_SPEC>;
///Register `ADSTRGR` writer
pub type W = crate::W<ADSTRGR_SPEC>;
///Field `TRSB` reader - A/D Conversion Start Trigger Select for Group BSelect the A/D conversion start trigger for group B in group scan mode.
pub type TRSB_R = crate::FieldReader;
///Field `TRSB` writer - A/D Conversion Start Trigger Select for Group BSelect the A/D conversion start trigger for group B in group scan mode.
pub type TRSB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TRSA` reader - A/D Conversion Start Trigger SelectSelect the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected.
pub type TRSA_R = crate::FieldReader;
///Field `TRSA` writer - A/D Conversion Start Trigger SelectSelect the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected.
pub type TRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - A/D Conversion Start Trigger Select for Group BSelect the A/D conversion start trigger for group B in group scan mode.
    #[inline(always)]
    pub fn trsb(&self) -> TRSB_R {
        TRSB_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - A/D Conversion Start Trigger SelectSelect the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected.
    #[inline(always)]
    pub fn trsa(&self) -> TRSA_R {
        TRSA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - A/D Conversion Start Trigger Select for Group BSelect the A/D conversion start trigger for group B in group scan mode.
    #[inline(always)]
    pub fn trsb(&mut self) -> TRSB_W<ADSTRGR_SPEC> {
        TRSB_W::new(self, 0)
    }
    ///Bits 8:13 - A/D Conversion Start Trigger SelectSelect the A/D conversion start trigger in single scan mode and continuous mode. In group scan mode, the A/D conversion start trigger for group A is selected.
    #[inline(always)]
    pub fn trsa(&mut self) -> TRSA_W<ADSTRGR_SPEC> {
        TRSA_W::new(self, 8)
    }
}
/**A/D Conversion Start Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`adstrgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adstrgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADSTRGR_SPEC;
impl crate::RegisterSpec for ADSTRGR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adstrgr::R`](R) reader structure
impl crate::Readable for ADSTRGR_SPEC {}
///`write(|w| ..)` method takes [`adstrgr::W`](W) writer structure
impl crate::Writable for ADSTRGR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADSTRGR to value 0
impl crate::Resettable for ADSTRGR_SPEC {}
