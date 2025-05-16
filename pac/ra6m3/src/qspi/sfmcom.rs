///Register `SFMCOM` reader
pub type R = crate::R<SFMCOM_SPEC>;
///Register `SFMCOM` writer
pub type W = crate::W<SFMCOM_SPEC>;
///Field `SFMD` reader - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode.
pub type SFMD_R = crate::FieldReader;
///Field `SFMD` writer - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode.
pub type SFMD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode.
    #[inline(always)]
    pub fn sfmd(&self) -> SFMD_R {
        SFMD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Port for direct communication with the SPI bus.Input/output to and from this port is converted to an SPI bus cycle. This port is accessible in the direct communication mode (DCOM=1) only.Access to this port is ignored in the ROM access mode.
    #[inline(always)]
    pub fn sfmd(&mut self) -> SFMD_W<SFMCOM_SPEC> {
        SFMD_W::new(self, 0)
    }
}
/**Communication Port Register

You can [`read`](crate::Reg::read) this register and get [`sfmcom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMCOM_SPEC;
impl crate::RegisterSpec for SFMCOM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmcom::R`](R) reader structure
impl crate::Readable for SFMCOM_SPEC {}
///`write(|w| ..)` method takes [`sfmcom::W`](W) writer structure
impl crate::Writable for SFMCOM_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCOM to value 0
impl crate::Resettable for SFMCOM_SPEC {}
