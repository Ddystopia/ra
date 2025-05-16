///Register `OFMRL` reader
pub type R = crate::R<OFMRL_SPEC>;
///Field `OFMRL` reader - These bits indicate the lower-order 32 bits of the calculated offsetFromMaster value.
pub type OFMRL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits indicate the lower-order 32 bits of the calculated offsetFromMaster value.
    #[inline(always)]
    pub fn ofmrl(&self) -> OFMRL_R {
        OFMRL_R::new(self.bits)
    }
}
/**offsetFromMaster Value Registers

You can [`read`](crate::Reg::read) this register and get [`ofmrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OFMRL_SPEC;
impl crate::RegisterSpec for OFMRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ofmrl::R`](R) reader structure
impl crate::Readable for OFMRL_SPEC {}
///`reset()` method sets OFMRL to value 0
impl crate::Resettable for OFMRL_SPEC {}
