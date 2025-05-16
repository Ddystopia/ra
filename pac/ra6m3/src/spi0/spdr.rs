///Register `SPDR` reader
pub type R = crate::R<SPDR_SPEC>;
///Register `SPDR` writer
pub type W = crate::W<SPDR_SPEC>;
///Field `SPDR` reader - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in word (SPDCR.SPLW=1), access SPDR.
pub type SPDR_R = crate::FieldReader<u32>;
///Field `SPDR` writer - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in word (SPDCR.SPLW=1), access SPDR.
pub type SPDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in word (SPDCR.SPLW=1), access SPDR.
    #[inline(always)]
    pub fn spdr(&self) -> SPDR_R {
        SPDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - SPDR is the interface with the buffers that hold data for transmission and reception by the RSPI.When accessing in word (SPDCR.SPLW=1), access SPDR.
    #[inline(always)]
    pub fn spdr(&mut self) -> SPDR_W<SPDR_SPEC> {
        SPDR_W::new(self, 0)
    }
}
/**SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPDR_SPEC;
impl crate::RegisterSpec for SPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spdr::R`](R) reader structure
impl crate::Readable for SPDR_SPEC {}
///`write(|w| ..)` method takes [`spdr::W`](W) writer structure
impl crate::Writable for SPDR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPDR to value 0
impl crate::Resettable for SPDR_SPEC {}
