///Register `DACS%s` reader
pub type R = crate::R<DACS_SPEC>;
///Register `DACS%s` writer
pub type W = crate::W<DACS_SPEC>;
///Field `DACS` reader - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\[7:0\] bits for the channel in use is prohibited.
pub type DACS_R = crate::FieldReader;
///Field `DACS` writer - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\[7:0\] bits for the channel in use is prohibited.
pub type DACS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\[7:0\] bits for the channel in use is prohibited.
    #[inline(always)]
    pub fn dacs(&self) -> DACS_R {
        DACS_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\[7:0\] bits for the channel in use is prohibited.
    #[inline(always)]
    pub fn dacs(&mut self) -> DACS_W<'_, DACS_SPEC> {
        DACS_W::new(self, 0)
    }
}
/**D/A Conversion Value Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`dacs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DACS_SPEC;
impl crate::RegisterSpec for DACS_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dacs::R`](R) reader structure
impl crate::Readable for DACS_SPEC {}
///`write(|w| ..)` method takes [`dacs::W`](W) writer structure
impl crate::Writable for DACS_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DACS%s to value 0
impl crate::Resettable for DACS_SPEC {}
