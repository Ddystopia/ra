///Register `GTPR` reader
pub type R = crate::R<GTPR_SPEC>;
///Register `GTPR` writer
pub type W = crate::W<GTPR_SPEC>;
///Field `GTPR` reader - Cycle Setting Register
pub type GTPR_R = crate::FieldReader<u32>;
///Field `GTPR` writer - Cycle Setting Register
pub type GTPR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Cycle Setting Register
    #[inline(always)]
    pub fn gtpr(&self) -> GTPR_R {
        GTPR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Cycle Setting Register
    #[inline(always)]
    pub fn gtpr(&mut self) -> GTPR_W<GTPR_SPEC> {
        GTPR_W::new(self, 0)
    }
}
/**General PWM Timer Cycle Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtpr::R`](R) reader structure
impl crate::Readable for GTPR_SPEC {}
///`write(|w| ..)` method takes [`gtpr::W`](W) writer structure
impl crate::Writable for GTPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPR to value 0xffff_ffff
impl crate::Resettable for GTPR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
