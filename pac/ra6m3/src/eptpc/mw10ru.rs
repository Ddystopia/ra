///Register `MW10RU` reader
pub type R = crate::R<MW10RU_SPEC>;
///Field `MW10RU` reader - These bits are for reading the higher-order 32 bits of the negative gradient value.
pub type MW10RU_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - These bits are for reading the higher-order 32 bits of the negative gradient value.
    #[inline(always)]
    pub fn mw10ru(&self) -> MW10RU_R {
        MW10RU_R::new(self.bits)
    }
}
/**Negative Gradient Worst 10 Value Registers

You can [`read`](crate::Reg::read) this register and get [`mw10ru::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MW10RU_SPEC;
impl crate::RegisterSpec for MW10RU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mw10ru::R`](R) reader structure
impl crate::Readable for MW10RU_SPEC {}
///`reset()` method sets MW10RU to value 0
impl crate::Resettable for MW10RU_SPEC {}
