///Register `GTDVU` reader
pub type R = crate::R<GTDVU_SPEC>;
///Register `GTDVU` writer
pub type W = crate::W<GTDVU_SPEC>;
///Field `GTDVU` reader - Dead Time Value Register U
pub type GTDVU_R = crate::FieldReader<u32>;
///Field `GTDVU` writer - Dead Time Value Register U
pub type GTDVU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Dead Time Value Register U
    #[inline(always)]
    pub fn gtdvu(&self) -> GTDVU_R {
        GTDVU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Dead Time Value Register U
    #[inline(always)]
    pub fn gtdvu(&mut self) -> GTDVU_W<GTDVU_SPEC> {
        GTDVU_W::new(self, 0)
    }
}
/**General PWM Timer Dead Time Value Register U

You can [`read`](crate::Reg::read) this register and get [`gtdvu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDVU_SPEC;
impl crate::RegisterSpec for GTDVU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdvu::R`](R) reader structure
impl crate::Readable for GTDVU_SPEC {}
///`write(|w| ..)` method takes [`gtdvu::W`](W) writer structure
impl crate::Writable for GTDVU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDVU to value 0xffff_ffff
impl crate::Resettable for GTDVU_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
