///Register `CRCDIR_BY` reader
pub type R = crate::R<CRCDIR_BY_SPEC>;
///Register `CRCDIR_BY` writer
pub type W = crate::W<CRCDIR_BY_SPEC>;
///Field `CRCDIR_BY` reader - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )
pub type CRCDIR_BY_R = crate::FieldReader;
///Field `CRCDIR_BY` writer - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )
pub type CRCDIR_BY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )
    #[inline(always)]
    pub fn crcdir_by(&self) -> CRCDIR_BY_R {
        CRCDIR_BY_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Calculation input Data ( Case of CRC-8, CRC-16 or CRC-CCITT )
    #[inline(always)]
    pub fn crcdir_by(&mut self) -> CRCDIR_BY_W<CRCDIR_BY_SPEC> {
        CRCDIR_BY_W::new(self, 0)
    }
}
/**CRC Data Input Register (byte access)

You can [`read`](crate::Reg::read) this register and get [`crcdir_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCDIR_BY_SPEC;
impl crate::RegisterSpec for CRCDIR_BY_SPEC {
    type Ux = u8;
}
///`read()` method returns [`crcdir_by::R`](R) reader structure
impl crate::Readable for CRCDIR_BY_SPEC {}
///`write(|w| ..)` method takes [`crcdir_by::W`](W) writer structure
impl crate::Writable for CRCDIR_BY_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDIR_BY to value 0
impl crate::Resettable for CRCDIR_BY_SPEC {}
