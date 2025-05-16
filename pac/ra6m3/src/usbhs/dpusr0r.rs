///Register `DPUSR0R` reader
pub type R = crate::R<DPUSR0R_SPEC>;
///Field `DOVCAHM` reader - OVRCURA InputIndicates OVRCURA input signal on the HS side of USB port.
pub type DOVCAHM_R = crate::BitReader;
///Field `DOVCBHM` reader - OVRCURB InputIndicates OVRCURB input signal on the HS side of USB port.
pub type DOVCBHM_R = crate::BitReader;
///Field `DVBSTSHM` reader - VBUS InputIndicates VBUS input signal on the HS side of USB port.
pub type DVBSTSHM_R = crate::BitReader;
impl R {
    ///Bit 20 - OVRCURA InputIndicates OVRCURA input signal on the HS side of USB port.
    #[inline(always)]
    pub fn dovcahm(&self) -> DOVCAHM_R {
        DOVCAHM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - OVRCURB InputIndicates OVRCURB input signal on the HS side of USB port.
    #[inline(always)]
    pub fn dovcbhm(&self) -> DOVCBHM_R {
        DOVCBHM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - VBUS InputIndicates VBUS input signal on the HS side of USB port.
    #[inline(always)]
    pub fn dvbstshm(&self) -> DVBSTSHM_R {
        DVBSTSHM_R::new(((self.bits >> 23) & 1) != 0)
    }
}
/**Deep Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPUSR0R_SPEC;
impl crate::RegisterSpec for DPUSR0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpusr0r::R`](R) reader structure
impl crate::Readable for DPUSR0R_SPEC {}
///`reset()` method sets DPUSR0R to value 0
impl crate::Resettable for DPUSR0R_SPEC {}
