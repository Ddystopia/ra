///Register `RMINCP%s` reader
pub type R = crate::R<RMINCP_SPEC>;
///Field `MIN1` reader - 1-Minute Capture Capture value for the ones place of minutes
pub type MIN1_R = crate::FieldReader;
///Field `MIN10` reader - 10-Minute Capture Capture value for the tens place of minutes
pub type MIN10_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - 1-Minute Capture Capture value for the ones place of minutes
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    ///Bits 4:6 - 10-Minute Capture Capture value for the tens place of minutes
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
}
/**Minute Capture Register %s

You can [`read`](crate::Reg::read) this register and get [`rmincp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RMINCP_SPEC;
impl crate::RegisterSpec for RMINCP_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rmincp::R`](R) reader structure
impl crate::Readable for RMINCP_SPEC {}
///`reset()` method sets RMINCP%s to value 0
impl crate::Resettable for RMINCP_SPEC {}
