///Register `JCDRID` reader
pub type R = crate::R<JCDRID_SPEC>;
///Register `JCDRID` writer
pub type W = crate::W<JCDRID_SPEC>;
///Field `DRID` reader - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
pub type DRID_R = crate::FieldReader;
///Field `DRID` writer - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
pub type DRID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn drid(&self) -> DRID_R {
        DRID_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - Lower Bytes of MCUs Preceding RST MarkerWhen both upper and lower bytes are set to 00h, neither a DRI nor an RST marker is placed.NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn drid(&mut self) -> DRID_W<JCDRID_SPEC> {
        DRID_W::new(self, 0)
    }
}
/**JPEG Code DRI Lower Register

You can [`read`](crate::Reg::read) this register and get [`jcdrid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jcdrid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCDRID_SPEC;
impl crate::RegisterSpec for JCDRID_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcdrid::R`](R) reader structure
impl crate::Readable for JCDRID_SPEC {}
///`write(|w| ..)` method takes [`jcdrid::W`](W) writer structure
impl crate::Writable for JCDRID_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JCDRID to value 0
impl crate::Resettable for JCDRID_SPEC {}
