///Register `SSIFRDR` reader
pub type R = crate::R<SSIFRDR_SPEC>;
///Field `SSIFRDR` reader - SSIFRDR is a read-only FIFO register consisting of eight stages of 32-bit registers for storing serially received data.
pub type SSIFRDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SSIFRDR is a read-only FIFO register consisting of eight stages of 32-bit registers for storing serially received data.
    #[inline(always)]
    pub fn ssifrdr(&self) -> SSIFRDR_R {
        SSIFRDR_R::new(self.bits)
    }
}
/**Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`ssifrdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SSIFRDR_SPEC;
impl crate::RegisterSpec for SSIFRDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ssifrdr::R`](R) reader structure
impl crate::Readable for SSIFRDR_SPEC {}
///`reset()` method sets SSIFRDR to value 0
impl crate::Resettable for SSIFRDR_SPEC {}
