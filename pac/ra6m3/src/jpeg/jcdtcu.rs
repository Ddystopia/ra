///Register `JCDTCU` reader
pub type R = crate::R<JCDTCU_SPEC>;
///Field `DCU` reader - Upper bytes of the counted amount of data to be compressed The values of this register are reset before compression starts.NOTE: Read-only in Decompression.
pub type DCU_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Upper bytes of the counted amount of data to be compressed The values of this register are reset before compression starts.NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn dcu(&self) -> DCU_R {
        DCU_R::new(self.bits)
    }
}
/**JPEG Code Data Count Upper Register

You can [`read`](crate::Reg::read) this register and get [`jcdtcu::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCDTCU_SPEC;
impl crate::RegisterSpec for JCDTCU_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcdtcu::R`](R) reader structure
impl crate::Readable for JCDTCU_SPEC {}
///`reset()` method sets JCDTCU to value 0
impl crate::Resettable for JCDTCU_SPEC {}
