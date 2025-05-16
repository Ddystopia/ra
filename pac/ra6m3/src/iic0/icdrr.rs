///Register `ICDRR` reader
pub type R = crate::R<ICDRR_SPEC>;
///Field `ICDRR` reader - 8-bit register that stores the received data
pub type ICDRR_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - 8-bit register that stores the received data
    #[inline(always)]
    pub fn icdrr(&self) -> ICDRR_R {
        ICDRR_R::new(self.bits)
    }
}
/**I2C Bus Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`icdrr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICDRR_SPEC;
impl crate::RegisterSpec for ICDRR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`icdrr::R`](R) reader structure
impl crate::Readable for ICDRR_SPEC {}
///`reset()` method sets ICDRR to value 0
impl crate::Resettable for ICDRR_SPEC {}
