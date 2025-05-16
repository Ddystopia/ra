///Register `PSR` reader
pub type R = crate::R<PSR_SPEC>;
///Field `LMON` reader - ETn_LINKSTA Pin Status FlagThe link status can be read by connecting the link signal output from the PHY-LSI to the ETn_LINKSTA pin. For details on the polarity, refer to the specifications of the connected PHY-LSI.
pub type LMON_R = crate::BitReader;
impl R {
    ///Bit 0 - ETn_LINKSTA Pin Status FlagThe link status can be read by connecting the link signal output from the PHY-LSI to the ETn_LINKSTA pin. For details on the polarity, refer to the specifications of the connected PHY-LSI.
    #[inline(always)]
    pub fn lmon(&self) -> LMON_R {
        LMON_R::new((self.bits & 1) != 0)
    }
}
/**PHY Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`psr::R`](R) reader structure
impl crate::Readable for PSR_SPEC {}
///`reset()` method sets PSR to value 0
impl crate::Resettable for PSR_SPEC {}
