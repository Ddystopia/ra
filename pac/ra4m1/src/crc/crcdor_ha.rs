///Register `CRCDOR_HA` reader
pub type R = crate::R<CRCDOR_HA_SPEC>;
///Register `CRCDOR_HA` writer
pub type W = crate::W<CRCDOR_HA_SPEC>;
///Field `CRCDOR_HA` reader - Calculation output Data (Case of CRC-16 or CRC-CCITT )
pub type CRCDOR_HA_R = crate::FieldReader<u16>;
///Field `CRCDOR_HA` writer - Calculation output Data (Case of CRC-16 or CRC-CCITT )
pub type CRCDOR_HA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )
    #[inline(always)]
    pub fn crcdor_ha(&self) -> CRCDOR_HA_R {
        CRCDOR_HA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Calculation output Data (Case of CRC-16 or CRC-CCITT )
    #[inline(always)]
    pub fn crcdor_ha(&mut self) -> CRCDOR_HA_W<CRCDOR_HA_SPEC> {
        CRCDOR_HA_W::new(self, 0)
    }
}
/**CRC Data Output Register (halfword access)

You can [`read`](crate::Reg::read) this register and get [`crcdor_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCDOR_HA_SPEC;
impl crate::RegisterSpec for CRCDOR_HA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`crcdor_ha::R`](R) reader structure
impl crate::Readable for CRCDOR_HA_SPEC {}
///`write(|w| ..)` method takes [`crcdor_ha::W`](W) writer structure
impl crate::Writable for CRCDOR_HA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDOR_HA to value 0
impl crate::Resettable for CRCDOR_HA_SPEC {}
