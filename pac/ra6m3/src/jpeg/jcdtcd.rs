///Register `JCDTCD` reader
pub type R = crate::R<JCDTCD_SPEC>;
///Field `DCD` reader - Lower bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts.NOTE: Read-only in Decompression.
pub type DCD_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Lower bytes of the counted amount of data to be compressedThe values of this register are reset before compression starts.NOTE: Read-only in Decompression.
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(self.bits)
    }
}
/**JPEG Code Data Count Lower Register

You can [`read`](crate::Reg::read) this register and get [`jcdtcd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct JCDTCD_SPEC;
impl crate::RegisterSpec for JCDTCD_SPEC {
    type Ux = u8;
}
///`read()` method returns [`jcdtcd::R`](R) reader structure
impl crate::Readable for JCDTCD_SPEC {}
///`reset()` method sets JCDTCD to value 0
impl crate::Resettable for JCDTCD_SPEC {}
