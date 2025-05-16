///Register `LOCOUTCR` reader
pub type R = crate::R<LOCOUTCR_SPEC>;
///Register `LOCOUTCR` writer
pub type W = crate::W<LOCOUTCR_SPEC>;
///Field `LOCOUTRM` reader - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original LOCO trimming bits
pub type LOCOUTRM_R = crate::FieldReader;
///Field `LOCOUTRM` writer - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original LOCO trimming bits
pub type LOCOUTRM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original LOCO trimming bits
    #[inline(always)]
    pub fn locoutrm(&self) -> LOCOUTRM_R {
        LOCOUTRM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - LOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original LOCO trimming bits
    #[inline(always)]
    pub fn locoutrm(&mut self) -> LOCOUTRM_W<LOCOUTCR_SPEC> {
        LOCOUTRM_W::new(self, 0)
    }
}
/**LOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`locoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOCOUTCR_SPEC;
impl crate::RegisterSpec for LOCOUTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`locoutcr::R`](R) reader structure
impl crate::Readable for LOCOUTCR_SPEC {}
///`write(|w| ..)` method takes [`locoutcr::W`](W) writer structure
impl crate::Writable for LOCOUTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCOUTCR to value 0
impl crate::Resettable for LOCOUTCR_SPEC {}
