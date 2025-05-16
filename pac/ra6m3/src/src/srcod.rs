///Register `SRCOD` reader
pub type R = crate::R<SRCOD_SPEC>;
///Field `SRCOD` reader - SRCOD is a 32-bit read-only register used to output the data after sampling rate conversion. The data in the 16-stage output data FIFO is read through SRCOD. When the number of data in the output data FIFO is zero after the start of conversion, the value previously read is read again.
pub type SRCOD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SRCOD is a 32-bit read-only register used to output the data after sampling rate conversion. The data in the 16-stage output data FIFO is read through SRCOD. When the number of data in the output data FIFO is zero after the start of conversion, the value previously read is read again.
    #[inline(always)]
    pub fn srcod(&self) -> SRCOD_R {
        SRCOD_R::new(self.bits)
    }
}
/**Output Data Register

You can [`read`](crate::Reg::read) this register and get [`srcod::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRCOD_SPEC;
impl crate::RegisterSpec for SRCOD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`srcod::R`](R) reader structure
impl crate::Readable for SRCOD_SPEC {}
///`reset()` method sets SRCOD to value 0
impl crate::Resettable for SRCOD_SPEC {}
