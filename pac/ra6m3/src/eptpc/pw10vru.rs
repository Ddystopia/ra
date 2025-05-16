///Register `PW10VRU` reader
pub type R = crate::R<PW10VRU_SPEC>;
///Field `PW10VRU` reader - These bits are for reading the higher-order 32 bits of the positive gradient value.
pub type PW10VRU_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the higher-order 32 bits of the positive gradient value.
    #[inline(always)]
    pub fn pw10vru(&self) -> PW10VRU_R {
        PW10VRU_R::new(self.bits)
    }
}
/**Positive Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`pw10vru::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PW10VRU_SPEC;
impl crate::RegisterSpec for PW10VRU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pw10vru::R`](R) reader structure
impl crate::Readable for PW10VRU_SPEC {}
///`reset()` method sets PW10VRU to value 0
impl crate::Resettable for PW10VRU_SPEC {}
