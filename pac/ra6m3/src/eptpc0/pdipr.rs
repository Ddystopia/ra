///Register `PDIPR` reader
pub type R = crate::R<PDIPR_SPEC>;
///Register `PDIPR` writer
pub type W = crate::W<PDIPR_SPEC>;
///Field `PDIPR` reader - These bits hold the setting for the destination IP address for PTPpdelay messages.
pub type PDIPR_R = crate::FieldReader<u32>;
///Field `PDIPR` writer - These bits hold the setting for the destination IP address for PTPpdelay messages.
pub type PDIPR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - These bits hold the setting for the destination IP address for PTPpdelay messages.
    #[inline(always)]
    pub fn pdipr(&self) -> PDIPR_R {
        PDIPR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - These bits hold the setting for the destination IP address for PTPpdelay messages.
    #[inline(always)]
    pub fn pdipr(&mut self) -> PDIPR_W<PDIPR_SPEC> {
        PDIPR_W::new(self, 0)
    }
}
/**PTP-pdelay Message Destination IP Address Setting Register

You can [`read`](crate::Reg::read) this register and get [`pdipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PDIPR_SPEC;
impl crate::RegisterSpec for PDIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pdipr::R`](R) reader structure
impl crate::Readable for PDIPR_SPEC {}
///`write(|w| ..)` method takes [`pdipr::W`](W) writer structure
impl crate::Writable for PDIPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDIPR to value 0xe000_006b
impl crate::Resettable for PDIPR_SPEC {
    const RESET_VALUE: u32 = 0xe000_006b;
}
