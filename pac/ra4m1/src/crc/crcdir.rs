///Register `CRCDIR` reader
pub type R = crate::R<CRCDIR_SPEC>;
///Register `CRCDIR` writer
pub type W = crate::W<CRCDIR_SPEC>;
///Field `CRCDIR` reader - Calculation input Data (Case of CRC-32, CRC-32C )
pub type CRCDIR_R = crate::FieldReader<u32>;
///Field `CRCDIR` writer - Calculation input Data (Case of CRC-32, CRC-32C )
pub type CRCDIR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )
    #[inline(always)]
    pub fn crcdir(&self) -> CRCDIR_R {
        CRCDIR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Calculation input Data (Case of CRC-32, CRC-32C )
    #[inline(always)]
    pub fn crcdir(&mut self) -> CRCDIR_W<CRCDIR_SPEC> {
        CRCDIR_W::new(self, 0)
    }
}
/**CRC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`crcdir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRCDIR_SPEC;
impl crate::RegisterSpec for CRCDIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`crcdir::R`](R) reader structure
impl crate::Readable for CRCDIR_SPEC {}
///`write(|w| ..)` method takes [`crcdir::W`](W) writer structure
impl crate::Writable for CRCDIR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRCDIR to value 0
impl crate::Resettable for CRCDIR_SPEC {}
