///Register `GTDVD` reader
pub type R = crate::R<GTDVD_SPEC>;
///Register `GTDVD` writer
pub type W = crate::W<GTDVD_SPEC>;
///Field `GTDVD` reader - Dead Time Value Register D
pub type GTDVD_R = crate::FieldReader<u32>;
///Field `GTDVD` writer - Dead Time Value Register D
pub type GTDVD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Dead Time Value Register D
    #[inline(always)]
    pub fn gtdvd(&self) -> GTDVD_R {
        GTDVD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Dead Time Value Register D
    #[inline(always)]
    pub fn gtdvd(&mut self) -> GTDVD_W<GTDVD_SPEC> {
        GTDVD_W::new(self, 0)
    }
}
/**General PWM Timer Dead Time Value Register D

You can [`read`](crate::Reg::read) this register and get [`gtdvd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDVD_SPEC;
impl crate::RegisterSpec for GTDVD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdvd::R`](R) reader structure
impl crate::Readable for GTDVD_SPEC {}
///`write(|w| ..)` method takes [`gtdvd::W`](W) writer structure
impl crate::Writable for GTDVD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDVD to value 0xffff_ffff
impl crate::Resettable for GTDVD_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
