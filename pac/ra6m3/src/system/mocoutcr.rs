///Register `MOCOUTCR` reader
pub type R = crate::R<MOCOUTCR_SPEC>;
///Register `MOCOUTCR` writer
pub type W = crate::W<MOCOUTCR_SPEC>;
///Field `MOCOUTRM` reader - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original MOCO trimming bits
pub type MOCOUTRM_R = crate::FieldReader;
///Field `MOCOUTRM` writer - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original MOCO trimming bits
pub type MOCOUTRM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original MOCO trimming bits
    #[inline(always)]
    pub fn mocoutrm(&self) -> MOCOUTRM_R {
        MOCOUTRM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:7 - MOCO User Trimming 1000_0000 : -128 1000_0001 : -127 1000_0010 : -126 . . . 1111_1111 : -1 0000_0000 : Center Code 0000_0001 : +1 . . . 0111_1101 : +125 0111_1110 : +126 0111_1111 : +127These bits are added to original MOCO trimming bits
    #[inline(always)]
    pub fn mocoutrm(&mut self) -> MOCOUTRM_W<MOCOUTCR_SPEC> {
        MOCOUTRM_W::new(self, 0)
    }
}
/**MOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MOCOUTCR_SPEC;
impl crate::RegisterSpec for MOCOUTCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mocoutcr::R`](R) reader structure
impl crate::Readable for MOCOUTCR_SPEC {}
///`write(|w| ..)` method takes [`mocoutcr::W`](W) writer structure
impl crate::Writable for MOCOUTCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOCOUTCR to value 0
impl crate::Resettable for MOCOUTCR_SPEC {}
