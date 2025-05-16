///Register `HOCOWTCR` reader
pub type R = crate::R<HOCOWTCR_SPEC>;
///Register `HOCOWTCR` writer
pub type W = crate::W<HOCOWTCR_SPEC>;
///Field `HSTS` reader - HOCO wait time settingWaiting time (sec) = setting of the HSTS\[2:0\] bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)
pub type HSTS_R = crate::FieldReader;
///Field `HSTS` writer - HOCO wait time settingWaiting time (sec) = setting of the HSTS\[2:0\] bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)
pub type HSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - HOCO wait time settingWaiting time (sec) = setting of the HSTS\[2:0\] bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)
    #[inline(always)]
    pub fn hsts(&self) -> HSTS_R {
        HSTS_R::new(self.bits & 7)
    }
}
impl W {
    ///Bits 0:2 - HOCO wait time settingWaiting time (sec) = setting of the HSTS\[2:0\] bits/fLOCO(Trimmed) + 3/fLOC(Untrimmed)
    #[inline(always)]
    pub fn hsts(&mut self) -> HSTS_W<HOCOWTCR_SPEC> {
        HSTS_W::new(self, 0)
    }
}
/**High-speed on-chip oscillator wait control register

You can [`read`](crate::Reg::read) this register and get [`hocowtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocowtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOCOWTCR_SPEC;
impl crate::RegisterSpec for HOCOWTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`hocowtcr::R`](R) reader structure
impl crate::Readable for HOCOWTCR_SPEC {}
///`write(|w| ..)` method takes [`hocowtcr::W`](W) writer structure
impl crate::Writable for HOCOWTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HOCOWTCR to value 0x02
impl crate::Resettable for HOCOWTCR_SPEC {
    const RESET_VALUE: u8 = 0x02;
}
