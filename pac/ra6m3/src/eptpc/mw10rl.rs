///Register `MW10RL` reader
pub type R = crate::R<MW10RL_SPEC>;
///Field `MW10RL` reader - These bits are for reading the lower-order 32 bits of the negative gradient value.
pub type MW10RL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the lower-order 32 bits of the negative gradient value.
    #[inline(always)]
    pub fn mw10rl(&self) -> MW10RL_R {
        MW10RL_R::new(self.bits)
    }
}
/**Negative Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`mw10rl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MW10RL_SPEC;
impl crate::RegisterSpec for MW10RL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mw10rl::R`](R) reader structure
impl crate::Readable for MW10RL_SPEC {}
///`reset()` method sets MW10RL to value 0
impl crate::Resettable for MW10RL_SPEC {}
