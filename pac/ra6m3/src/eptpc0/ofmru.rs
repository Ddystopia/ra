///Register `OFMRU` reader
pub type R = crate::R<OFMRU_SPEC>;
///Field `OFMRU` reader - These bits indicate the higher-order 32 bits of the calculated offsetFromMaster value.
pub type OFMRU_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits indicate the higher-order 32 bits of the calculated offsetFromMaster value.
    #[inline(always)]
    pub fn ofmru(&self) -> OFMRU_R {
        OFMRU_R::new(self.bits)
    }
}
/**offsetFromMaster Value Registers

You can [`read`](crate::Reg::read) this register and get [`ofmru::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OFMRU_SPEC;
impl crate::RegisterSpec for OFMRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ofmru::R`](R) reader structure
impl crate::Readable for OFMRU_SPEC {}
///`reset()` method sets OFMRU to value 0
impl crate::Resettable for OFMRU_SPEC {}
