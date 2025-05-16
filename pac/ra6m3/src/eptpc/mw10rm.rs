///Register `MW10RM` reader
pub type R = crate::R<MW10RM_SPEC>;
///Field `MW10RM` reader - These bits are for reading the middle-order 32 bits of the negative gradient value.
pub type MW10RM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the middle-order 32 bits of the negative gradient value.
    #[inline(always)]
    pub fn mw10rm(&self) -> MW10RM_R {
        MW10RM_R::new(self.bits)
    }
}
/**Negative Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`mw10rm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MW10RM_SPEC;
impl crate::RegisterSpec for MW10RM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mw10rm::R`](R) reader structure
impl crate::Readable for MW10RM_SPEC {}
///`reset()` method sets MW10RM to value 0
impl crate::Resettable for MW10RM_SPEC {}
