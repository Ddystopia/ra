///Register `PW10VRL` reader
pub type R = crate::R<PW10VRL_SPEC>;
///Field `PW10VRL` reader - These bits are for reading the lower-order 32 bits of the positive gradient value.
pub type PW10VRL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the lower-order 32 bits of the positive gradient value.
    #[inline(always)]
    pub fn pw10vrl(&self) -> PW10VRL_R {
        PW10VRL_R::new(self.bits)
    }
}
/**Positive Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`pw10vrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PW10VRL_SPEC;
impl crate::RegisterSpec for PW10VRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pw10vrl::R`](R) reader structure
impl crate::Readable for PW10VRL_SPEC {}
///`reset()` method sets PW10VRL to value 0
impl crate::Resettable for PW10VRL_SPEC {}
