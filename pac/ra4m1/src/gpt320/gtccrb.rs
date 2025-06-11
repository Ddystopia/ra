///Register `GTCCRB` reader
pub type R = crate::R<GTCCRB_SPEC>;
///Register `GTCCRB` writer
pub type W = crate::W<GTCCRB_SPEC>;
///Field `GTCCRB` reader - Compare Capture Register B
pub type GTCCRB_R = crate::FieldReader<u32>;
///Field `GTCCRB` writer - Compare Capture Register B
pub type GTCCRB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Compare Capture Register B
    #[inline(always)]
    pub fn gtccrb(&self) -> GTCCRB_R {
        GTCCRB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Compare Capture Register B
    #[inline(always)]
    pub fn gtccrb(&mut self) -> GTCCRB_W<GTCCRB_SPEC> {
        GTCCRB_W::new(self, 0)
    }
}
/**General PWM Timer Compare Capture Register B

You can [`read`](crate::Reg::read) this register and get [`gtccrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCCRB_SPEC;
impl crate::RegisterSpec for GTCCRB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtccrb::R`](R) reader structure
impl crate::Readable for GTCCRB_SPEC {}
///`write(|w| ..)` method takes [`gtccrb::W`](W) writer structure
impl crate::Writable for GTCCRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCCRB to value 0xffff_ffff
impl crate::Resettable for GTCCRB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
