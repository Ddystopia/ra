///Register `GTDBU` reader
pub type R = crate::R<GTDBU_SPEC>;
///Register `GTDBU` writer
pub type W = crate::W<GTDBU_SPEC>;
///Field `GTDVU` reader - Dead Time Buffer Register U
pub type GTDVU_R = crate::FieldReader<u32>;
///Field `GTDVU` writer - Dead Time Buffer Register U
pub type GTDVU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Dead Time Buffer Register U
    #[inline(always)]
    pub fn gtdvu(&self) -> GTDVU_R {
        GTDVU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Dead Time Buffer Register U
    #[inline(always)]
    pub fn gtdvu(&mut self) -> GTDVU_W<GTDBU_SPEC> {
        GTDVU_W::new(self, 0)
    }
}
/**General PWM Timer Dead Time Buffer Register U

You can [`read`](crate::Reg::read) this register and get [`gtdbu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdbu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDBU_SPEC;
impl crate::RegisterSpec for GTDBU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdbu::R`](R) reader structure
impl crate::Readable for GTDBU_SPEC {}
///`write(|w| ..)` method takes [`gtdbu::W`](W) writer structure
impl crate::Writable for GTDBU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDBU to value 0xffff_ffff
impl crate::Resettable for GTDBU_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
