///Register `SYCONFR` reader
pub type R = crate::R<SYCONFR_SPEC>;
///Register `SYCONFR` writer
pub type W = crate::W<SYCONFR_SPEC>;
///Field `TCYC` reader - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles).
pub type TCYC_R = crate::FieldReader;
///Field `TCYC` writer - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles).
pub type TCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**Sync Message Transmission Bandwidth Securing Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBDIS_A {
    ///0: Securing of the bandwidth for the transmission of SYNC messages is enabled (transfer by the EDMAC is given lower priority).
    _0 = 0,
    ///1: Securing of the bandwidth for the transmission of SYNC messages is disabled (transfer by the EDMAC is given higher priority).
    _1 = 1,
}
impl From<SBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SBDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SBDIS` reader - Sync Message Transmission Bandwidth Securing Disable
pub type SBDIS_R = crate::BitReader<SBDIS_A>;
impl SBDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBDIS_A {
        match self.bits {
            false => SBDIS_A::_0,
            true => SBDIS_A::_1,
        }
    }
    ///Securing of the bandwidth for the transmission of SYNC messages is enabled (transfer by the EDMAC is given lower priority).
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBDIS_A::_0
    }
    ///Securing of the bandwidth for the transmission of SYNC messages is disabled (transfer by the EDMAC is given higher priority).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBDIS_A::_1
    }
}
///Field `SBDIS` writer - Sync Message Transmission Bandwidth Securing Disable
pub type SBDIS_W<'a, REG> = crate::BitWriter<'a, REG, SBDIS_A>;
impl<'a, REG> SBDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Securing of the bandwidth for the transmission of SYNC messages is enabled (transfer by the EDMAC is given lower priority).
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SBDIS_A::_0)
    }
    ///Securing of the bandwidth for the transmission of SYNC messages is disabled (transfer by the EDMAC is given higher priority).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SBDIS_A::_1)
    }
}
/**Receive Message domainNumber Filter Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILDIS_A {
    ///0: Filtering conditions for the reception of PTP messages include comparison with the domainNumber field.
    _0 = 0,
    ///1: Filtering conditions for the reception of PTP messages do not include comparison with the domainNumber field.
    _1 = 1,
}
impl From<FILDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FILDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FILDIS` reader - Receive Message domainNumber Filter Disable
pub type FILDIS_R = crate::BitReader<FILDIS_A>;
impl FILDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FILDIS_A {
        match self.bits {
            false => FILDIS_A::_0,
            true => FILDIS_A::_1,
        }
    }
    ///Filtering conditions for the reception of PTP messages include comparison with the domainNumber field.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FILDIS_A::_0
    }
    ///Filtering conditions for the reception of PTP messages do not include comparison with the domainNumber field.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FILDIS_A::_1
    }
}
///Field `FILDIS` writer - Receive Message domainNumber Filter Disable
pub type FILDIS_W<'a, REG> = crate::BitWriter<'a, REG, FILDIS_A>;
impl<'a, REG> FILDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Filtering conditions for the reception of PTP messages include comparison with the domainNumber field.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FILDIS_A::_0)
    }
    ///Filtering conditions for the reception of PTP messages do not include comparison with the domainNumber field.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FILDIS_A::_1)
    }
}
impl R {
    ///Bits 0:7 - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles).
    #[inline(always)]
    pub fn tcyc(&self) -> TCYC_R {
        TCYC_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 12 - Sync Message Transmission Bandwidth Securing Disable
    #[inline(always)]
    pub fn sbdis(&self) -> SBDIS_R {
        SBDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Receive Message domainNumber Filter Disable
    #[inline(always)]
    pub fn fildis(&self) -> FILDIS_R {
        FILDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - PTP Message Transmission Interval SettingThese bits are used to set the time from the completion of one transmission to the start of the next in cycles of the transmission clock. A value n in these bits means that a transmission interval of n cycles will be secured.No interval is secured if the setting is 00h.We recommend the setting 28h (40 cycles).
    #[inline(always)]
    pub fn tcyc(&mut self) -> TCYC_W<SYCONFR_SPEC> {
        TCYC_W::new(self, 0)
    }
    ///Bit 12 - Sync Message Transmission Bandwidth Securing Disable
    #[inline(always)]
    pub fn sbdis(&mut self) -> SBDIS_W<SYCONFR_SPEC> {
        SBDIS_W::new(self, 12)
    }
    ///Bit 16 - Receive Message domainNumber Filter Disable
    #[inline(always)]
    pub fn fildis(&mut self) -> FILDIS_W<SYCONFR_SPEC> {
        FILDIS_W::new(self, 16)
    }
}
/**SYNFP Operation Setting Register

You can [`read`](crate::Reg::read) this register and get [`syconfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syconfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYCONFR_SPEC;
impl crate::RegisterSpec for SYCONFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`syconfr::R`](R) reader structure
impl crate::Readable for SYCONFR_SPEC {}
///`write(|w| ..)` method takes [`syconfr::W`](W) writer structure
impl crate::Writable for SYCONFR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYCONFR to value 0
impl crate::Resettable for SYCONFR_SPEC {}
