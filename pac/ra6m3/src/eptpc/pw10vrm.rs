///Register `PW10VRM` reader
pub type R = crate::R<PW10VRM_SPEC>;
///Field `PW10VRM` reader - These bits are for reading the middle-order 32 bits of the positive gradient value.
pub type PW10VRM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the middle-order 32 bits of the positive gradient value.
    #[inline(always)]
    pub fn pw10vrm(&self) -> PW10VRM_R {
        PW10VRM_R::new(self.bits)
    }
}
/**Positive Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`pw10vrm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PW10VRM_SPEC;
impl crate::RegisterSpec for PW10VRM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pw10vrm::R`](R) reader structure
impl crate::Readable for PW10VRM_SPEC {}
///`reset()` method sets PW10VRM to value 0
impl crate::Resettable for PW10VRM_SPEC {}
