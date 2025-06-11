///Register `RDAYCP%s` reader
pub type R = crate::R<RDAYCP_SPEC>;
///Field `DATE1` reader - 1-Day Capture Capture value for the ones place of minutes
pub type DATE1_R = crate::FieldReader;
///Field `DATE10` reader - 10-Day Capture Capture value for the tens place of minutes
pub type DATE10_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - 1-Day Capture Capture value for the ones place of minutes
    #[inline(always)]
    pub fn date1(&self) -> DATE1_R {
        DATE1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:5 - 10-Day Capture Capture value for the tens place of minutes
    #[inline(always)]
    pub fn date10(&self) -> DATE10_R {
        DATE10_R::new((self.bits >> 4) & 3)
    }
}
/**Date Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rdaycp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RDAYCP_SPEC;
impl crate::RegisterSpec for RDAYCP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rdaycp::R`](R) reader structure
impl crate::Readable for RDAYCP_SPEC {}
///`reset()` method sets RDAYCP%s to value 0
impl crate::Resettable for RDAYCP_SPEC {}
