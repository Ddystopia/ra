///Register `ADDR%s` reader
pub type R = crate::R<ADDR_SPEC>;
///Field `ADDR` reader - The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion.
pub type ADDR_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - The ADDR register is a 16-bit read-only registers for storing the result of A/D conversion.
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
/**A/D Data Register %s

You can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`addr::R`](R) reader structure
impl crate::Readable for ADDR_SPEC {}
///`reset()` method sets ADDR%s to value 0
impl crate::Resettable for ADDR_SPEC {}
