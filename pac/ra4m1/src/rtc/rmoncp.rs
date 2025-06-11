///Register `RMONCP%s` reader
pub type R = crate::R<RMONCP_SPEC>;
///Field `MON1` reader - 1-Month Capture Capture value for the ones place of months
pub type MON1_R = crate::FieldReader;
///Field `MON10` reader - 10-Month Capture Capture value for the tens place of months
pub type MON10_R = crate::BitReader;
impl R {
    ///Bits 0:3 - 1-Month Capture Capture value for the ones place of months
    #[inline(always)]
    pub fn mon1(&self) -> MON1_R {
        MON1_R::new(self.bits & 0x0f)
    }
    ///Bit 4 - 10-Month Capture Capture value for the tens place of months
    #[inline(always)]
    pub fn mon10(&self) -> MON10_R {
        MON10_R::new(((self.bits >> 4) & 1) != 0)
    }
}
/**Month Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmoncp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMONCP_SPEC;
impl crate::RegisterSpec for RMONCP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rmoncp::R`](R) reader structure
impl crate::Readable for RMONCP_SPEC {}
///`reset()` method sets RMONCP%s to value 0
impl crate::Resettable for RMONCP_SPEC {}
