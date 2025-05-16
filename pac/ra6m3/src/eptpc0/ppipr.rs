///Register `PPIPR` reader
pub type R = crate::R<PPIPR_SPEC>;
///Register `PPIPR` writer
pub type W = crate::W<PPIPR_SPEC>;
///Field `PPIPR` reader - These bits hold the setting for the destination IP address for PTPprimary messages.
pub type PPIPR_R = crate::FieldReader<u32>;
///Field `PPIPR` writer - These bits hold the setting for the destination IP address for PTPprimary messages.
pub type PPIPR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the destination IP address for PTPprimary messages.
    #[inline(always)]
    pub fn ppipr(&self) -> PPIPR_R {
        PPIPR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the destination IP address for PTPprimary messages.
    #[inline(always)]
    pub fn ppipr(&mut self) -> PPIPR_W<PPIPR_SPEC> {
        PPIPR_W::new(self, 0)
    }
}
/**PTP-primary Message Destination IP Address Setting Register

You can [`read`](crate::Reg::read) this register and get [`ppipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PPIPR_SPEC;
impl crate::RegisterSpec for PPIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ppipr::R`](R) reader structure
impl crate::Readable for PPIPR_SPEC {}
///`write(|w| ..)` method takes [`ppipr::W`](W) writer structure
impl crate::Writable for PPIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PPIPR to value 0xe000_0181
impl crate::Resettable for PPIPR_SPEC {
    const RESET_VALUE: u32 = 0xe000_0181;
}
