///Register `BUS%sERRADD` reader
pub type R = crate::R<BUSERRADD_SPEC>;
///Field `BERAD` reader - Bus Error AddressWhen a bus error occurs, It stores an error address.
pub type BERAD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bus Error AddressWhen a bus error occurs, It stores an error address.
    #[inline(always)]
    pub fn berad(&self) -> BERAD_R {
        BERAD_R::new(self.bits)
    }
}
/**Bus Error Address Register %s

You can [`read`](crate::Reg::read) this register and get [`buserradd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUSERRADD_SPEC;
impl crate::RegisterSpec for BUSERRADD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`buserradd::R`](R) reader structure
impl crate::Readable for BUSERRADD_SPEC {}
///`reset()` method sets BUS%sERRADD to value 0
impl crate::Resettable for BUSERRADD_SPEC {}
