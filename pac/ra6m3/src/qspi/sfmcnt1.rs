///Register `SFMCNT1` reader
pub type R = crate::R<SFMCNT1_SPEC>;
///Register `SFMCNT1` writer
pub type W = crate::W<SFMCNT1_SPEC>;
///Field `QSPI_EXT` reader - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\[5:0\] to high-order 6bits of SHADDR\[31:0\]NOTE: Setting 6'h3F is prihibited.
pub type QSPI_EXT_R = crate::FieldReader;
///Field `QSPI_EXT` writer - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\[5:0\] to high-order 6bits of SHADDR\[31:0\]NOTE: Setting 6'h3F is prihibited.
pub type QSPI_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 26:31 - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\[5:0\] to high-order 6bits of SHADDR\[31:0\]NOTE: Setting 6'h3F is prihibited.
    #[inline(always)]
    pub fn qspi_ext(&self) -> QSPI_EXT_R {
        QSPI_EXT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 26:31 - BANK Switching AddressWhen accessing from 0x6000_0000 to 0x63FF_FFFF, Addres bus is Set QSPI_EXT\[5:0\] to high-order 6bits of SHADDR\[31:0\]NOTE: Setting 6'h3F is prihibited.
    #[inline(always)]
    pub fn qspi_ext(&mut self) -> QSPI_EXT_W<SFMCNT1_SPEC> {
        QSPI_EXT_W::new(self, 26)
    }
}
/**External QSPI Address Register 1

You can [`read`](crate::Reg::read) this register and get [`sfmcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SFMCNT1_SPEC;
impl crate::RegisterSpec for SFMCNT1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sfmcnt1::R`](R) reader structure
impl crate::Readable for SFMCNT1_SPEC {}
///`write(|w| ..)` method takes [`sfmcnt1::W`](W) writer structure
impl crate::Writable for SFMCNT1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SFMCNT1 to value 0
impl crate::Resettable for SFMCNT1_SPEC {}
