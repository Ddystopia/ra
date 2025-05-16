///Register `AGT` reader
pub type R = crate::R<AGT_SPEC>;
///Register `AGT` writer
pub type W = crate::W<AGT_SPEC>;
///Field `AGT` reader - 16bit counter and reload registerNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH.
pub type AGT_R = crate::FieldReader<u16>;
///Field `AGT` writer - 16bit counter and reload registerNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH.
pub type AGT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 16bit counter and reload registerNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH.
    #[inline(always)]
    pub fn agt(&self) -> AGT_R {
        AGT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - 16bit counter and reload registerNOTE : When 1 is written to the TSTOP bit in the AGTCRn register, the 16-bit counter is forcibly stopped and set to FFFFH.
    #[inline(always)]
    pub fn agt(&mut self) -> AGT_W<AGT_SPEC> {
        AGT_W::new(self, 0)
    }
}
/**AGT Counter Register

You can [`read`](crate::Reg::read) this register and get [`agt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AGT_SPEC;
impl crate::RegisterSpec for AGT_SPEC {
    type Ux = u16;
}
///`read()` method returns [`agt::R`](R) reader structure
impl crate::Readable for AGT_SPEC {}
///`write(|w| ..)` method takes [`agt::W`](W) writer structure
impl crate::Writable for AGT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AGT to value 0xffff
impl crate::Resettable for AGT_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
