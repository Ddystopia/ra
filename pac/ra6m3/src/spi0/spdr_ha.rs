///Register `SPDR_HA` reader
pub type R = crate::R<SPDR_HA_SPEC>;
///Register `SPDR_HA` writer
pub type W = crate::W<SPDR_HA_SPEC>;
///Field `SPDR_HA` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA.
pub type SPDR_HA_R = crate::FieldReader<u16>;
///Field `SPDR_HA` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA.
pub type SPDR_HA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA.
    #[inline(always)]
    pub fn spdr_ha(&self) -> SPDR_HA_R {
        SPDR_HA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in halfword (SPDCR.SPLW=0), access SPDR_HA.
    #[inline(always)]
    pub fn spdr_ha(&mut self) -> SPDR_HA_W<SPDR_HA_SPEC> {
        SPDR_HA_W::new(self, 0)
    }
}
/**SPI Data Register ( halfword access )

You can [`read`](crate::Reg::read) this register and get [`spdr_ha::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_ha::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPDR_HA_SPEC;
impl crate::RegisterSpec for SPDR_HA_SPEC {
    type Ux = u16;
}
///`read()` method returns [`spdr_ha::R`](R) reader structure
impl crate::Readable for SPDR_HA_SPEC {}
///`write(|w| ..)` method takes [`spdr_ha::W`](W) writer structure
impl crate::Writable for SPDR_HA_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDR_HA to value 0
impl crate::Resettable for SPDR_HA_SPEC {}
