///Register `PCDR` reader
pub type R = crate::R<PCDR_SPEC>;
///Field `PCDR` reader - The PDC includes a 32-bit-wide, 22-stage FIFO for the storage of captured data. The PCDR register is a 4-byte space to which the FIFO is mapped, and four bytes of data are read from the PCDR register at a time.
pub type PCDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The PDC includes a 32-bit-wide, 22-stage FIFO for the storage of captured data. The PCDR register is a 4-byte space to which the FIFO is mapped, and four bytes of data are read from the PCDR register at a time.
    #[inline(always)]
    pub fn pcdr(&self) -> PCDR_R {
        PCDR_R::new(self.bits)
    }
}
/**PDC Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`pcdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCDR_SPEC;
impl crate::RegisterSpec for PCDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcdr::R`](R) reader structure
impl crate::Readable for PCDR_SPEC {}
///`reset()` method sets PCDR to value 0
impl crate::Resettable for PCDR_SPEC {}
