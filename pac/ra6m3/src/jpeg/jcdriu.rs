///Register `JCDRIU` reader
pub type R = crate::R<JCDRIU_SPEC>;
///Register `JCDRIU` writer
pub type W = crate::W<JCDRIU_SPEC>;
///Field `DRIU` reader - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
pub type DRIU_R = crate::FieldReader;
///Field `DRIU` writer - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
pub type DRIU_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn driu(&self) -> DRIU_R {
        DRIU_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Upper Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn driu(&mut self) -> DRIU_W<JCDRIU_SPEC> {
        DRIU_W::new(self, 0)
    }
}
/**JPEG Code DRI Upper Register

You can [`read`](crate::Reg::read) this register and get [`jcdriu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcdriu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCDRIU_SPEC;
impl crate::RegisterSpec for JCDRIU_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcdriu::R`](R) reader structure
impl crate::Readable for JCDRIU_SPEC {}
///`write(|w| ..)` method takes [`jcdriu::W`](W) writer structure
impl crate::Writable for JCDRIU_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCDRIU to value 0
impl crate::Resettable for JCDRIU_SPEC {}
