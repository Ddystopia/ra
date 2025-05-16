///Register `HOCOUTCR` reader
pub type R = crate::R<HOCOUTCR_SPEC>;
///Register `HOCOUTCR` writer
pub type W = crate::W<HOCOUTCR_SPEC>;
///Field `HOCOUTRM` reader - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original HOCO trimming bits
pub type HOCOUTRM_R = crate::FieldReader;
///Field `HOCOUTRM` writer - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original HOCO trimming bits
pub type HOCOUTRM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original HOCO trimming bits
    #[inline(always)]
    pub fn hocoutrm(&self) -> HOCOUTRM_R {
        HOCOUTRM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - HOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original HOCO trimming bits
    #[inline(always)]
    pub fn hocoutrm(&mut self) -> HOCOUTRM_W<HOCOUTCR_SPEC> {
        HOCOUTRM_W::new(self, 0)
    }
}
/**HOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOCOUTCR_SPEC;
impl crate::RegisterSpec for HOCOUTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`hocoutcr::R`](R) reader structure
impl crate::Readable for HOCOUTCR_SPEC {}
///`write(|w| ..)` method takes [`hocoutcr::W`](W) writer structure
impl crate::Writable for HOCOUTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HOCOUTCR to value 0
impl crate::Resettable for HOCOUTCR_SPEC {}
