///Register `GTPBR` reader
pub type R = crate::R<GTPBR_SPEC>;
///Register `GTPBR` writer
pub type W = crate::W<GTPBR_SPEC>;
///Field `GTPBR` reader - Cycle Setting Buffer Register
pub type GTPBR_R = crate::FieldReader<u32>;
///Field `GTPBR` writer - Cycle Setting Buffer Register
pub type GTPBR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Cycle Setting Buffer Register
    #[inline(always)]
    pub fn gtpbr(&self) -> GTPBR_R {
        GTPBR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Cycle Setting Buffer Register
    #[inline(always)]
    pub fn gtpbr(&mut self) -> GTPBR_W<GTPBR_SPEC> {
        GTPBR_W::new(self, 0)
    }
}
/**General PWM Timer Cycle Setting Buffer Register

You can [`read`](crate::Reg::read) this register and get [`gtpbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTPBR_SPEC;
impl crate::RegisterSpec for GTPBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtpbr::R`](R) reader structure
impl crate::Readable for GTPBR_SPEC {}
///`write(|w| ..)` method takes [`gtpbr::W`](W) writer structure
impl crate::Writable for GTPBR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPBR to value 0xffff_ffff
impl crate::Resettable for GTPBR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
