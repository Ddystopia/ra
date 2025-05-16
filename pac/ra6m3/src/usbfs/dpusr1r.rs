///Register `DPUSR1R` reader
pub type R = crate::R<DPUSR1R_SPEC>;
///Register `DPUSR1R` writer
pub type W = crate::W<DPUSR1R_SPEC>;
/**USB DP Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINTE0_A {
    ///0: Recovery from deep software standby mode is disabled.
    _0 = 0,
    ///1: Recovery from deep software standby mode is enabled.
    _1 = 1,
}
impl From<DPINTE0_A> for bool {
    #[inline(always)]
    fn from(variant: DPINTE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINTE0` reader - USB DP Interrupt Enable/Clear
pub type DPINTE0_R = crate::BitReader<DPINTE0_A>;
impl DPINTE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPINTE0_A {
        match self.bits {
            false => DPINTE0_A::_0,
            true => DPINTE0_A::_1,
        }
    }
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINTE0_A::_0
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINTE0_A::_1
    }
}
///Field `DPINTE0` writer - USB DP Interrupt Enable/Clear
pub type DPINTE0_W<'a, REG> = crate::BitWriter<'a, REG, DPINTE0_A>;
impl<'a, REG> DPINTE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPINTE0_A::_0)
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPINTE0_A::_1)
    }
}
/**USB DM Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINTE0_A {
    ///0: Recovery from deep software standby mode is disabled.
    _0 = 0,
    ///1: Recovery from deep software standby mode is enabled.
    _1 = 1,
}
impl From<DMINTE0_A> for bool {
    #[inline(always)]
    fn from(variant: DMINTE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINTE0` reader - USB DM Interrupt Enable/Clear
pub type DMINTE0_R = crate::BitReader<DMINTE0_A>;
impl DMINTE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMINTE0_A {
        match self.bits {
            false => DMINTE0_A::_0,
            true => DMINTE0_A::_1,
        }
    }
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINTE0_A::_0
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINTE0_A::_1
    }
}
///Field `DMINTE0` writer - USB DM Interrupt Enable/Clear
pub type DMINTE0_W<'a, REG> = crate::BitWriter<'a, REG, DMINTE0_A>;
impl<'a, REG> DMINTE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DMINTE0_A::_0)
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DMINTE0_A::_1)
    }
}
/**USB OVRCURA Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRAE0_A {
    ///0: Recovery from deep software standby mode is disabled.
    _0 = 0,
    ///1: Recovery from deep software standby mode is enabled.
    _1 = 1,
}
impl From<DOVRCRAE0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRAE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRAE0` reader - USB OVRCURA Interrupt Enable/Clear
pub type DOVRCRAE0_R = crate::BitReader<DOVRCRAE0_A>;
impl DOVRCRAE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVRCRAE0_A {
        match self.bits {
            false => DOVRCRAE0_A::_0,
            true => DOVRCRAE0_A::_1,
        }
    }
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRAE0_A::_0
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRAE0_A::_1
    }
}
///Field `DOVRCRAE0` writer - USB OVRCURA Interrupt Enable/Clear
pub type DOVRCRAE0_W<'a, REG> = crate::BitWriter<'a, REG, DOVRCRAE0_A>;
impl<'a, REG> DOVRCRAE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOVRCRAE0_A::_0)
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOVRCRAE0_A::_1)
    }
}
/**USB OVRCURB Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRBE0_A {
    ///0: Recovery from deep software standby mode is disabled.
    _0 = 0,
    ///1: Recovery from deep software standby mode is enabled.
    _1 = 1,
}
impl From<DOVRCRBE0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRBE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRBE0` reader - USB OVRCURB Interrupt Enable/Clear
pub type DOVRCRBE0_R = crate::BitReader<DOVRCRBE0_A>;
impl DOVRCRBE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVRCRBE0_A {
        match self.bits {
            false => DOVRCRBE0_A::_0,
            true => DOVRCRBE0_A::_1,
        }
    }
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRBE0_A::_0
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRBE0_A::_1
    }
}
///Field `DOVRCRBE0` writer - USB OVRCURB Interrupt Enable/Clear
pub type DOVRCRBE0_W<'a, REG> = crate::BitWriter<'a, REG, DOVRCRBE0_A>;
impl<'a, REG> DOVRCRBE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DOVRCRBE0_A::_0)
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DOVRCRBE0_A::_1)
    }
}
/**USB VBUS Interrupt Enable/Clear

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBSE0_A {
    ///0: Recovery from deep software standby mode is disabled.
    _0 = 0,
    ///1: Recovery from deep software standby mode is enabled.
    _1 = 1,
}
impl From<DVBSE0_A> for bool {
    #[inline(always)]
    fn from(variant: DVBSE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBSE0` reader - USB VBUS Interrupt Enable/Clear
pub type DVBSE0_R = crate::BitReader<DVBSE0_A>;
impl DVBSE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVBSE0_A {
        match self.bits {
            false => DVBSE0_A::_0,
            true => DVBSE0_A::_1,
        }
    }
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBSE0_A::_0
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBSE0_A::_1
    }
}
///Field `DVBSE0` writer - USB VBUS Interrupt Enable/Clear
pub type DVBSE0_W<'a, REG> = crate::BitWriter<'a, REG, DVBSE0_A>;
impl<'a, REG> DVBSE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery from deep software standby mode is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVBSE0_A::_0)
    }
    ///Recovery from deep software standby mode is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVBSE0_A::_1)
    }
}
/**USB DP Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPINT0_A {
    ///0: The system has not returned from deep software standby mode.
    _0 = 0,
    ///1: The system has returned from deep software standby mode.
    _1 = 1,
}
impl From<DPINT0_A> for bool {
    #[inline(always)]
    fn from(variant: DPINT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPINT0` reader - USB DP Interrupt Source Recovery
pub type DPINT0_R = crate::BitReader<DPINT0_A>;
impl DPINT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPINT0_A {
        match self.bits {
            false => DPINT0_A::_0,
            true => DPINT0_A::_1,
        }
    }
    ///The system has not returned from deep software standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPINT0_A::_0
    }
    ///The system has returned from deep software standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPINT0_A::_1
    }
}
/**USB DM Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMINT0_A {
    ///0: The system has not returned from deep software standby mode.
    _0 = 0,
    ///1: The system has returned from deep software standby mode.
    _1 = 1,
}
impl From<DMINT0_A> for bool {
    #[inline(always)]
    fn from(variant: DMINT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMINT0` reader - USB DM Interrupt Source Recovery
pub type DMINT0_R = crate::BitReader<DMINT0_A>;
impl DMINT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMINT0_A {
        match self.bits {
            false => DMINT0_A::_0,
            true => DMINT0_A::_1,
        }
    }
    ///The system has not returned from deep software standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMINT0_A::_0
    }
    ///The system has returned from deep software standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMINT0_A::_1
    }
}
/**USB OVRCURA Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRA0_A {
    ///0: The system has not returned from deep software standby mode.
    _0 = 0,
    ///1: The system has returned from deep software standby mode.
    _1 = 1,
}
impl From<DOVRCRA0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRA0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRA0` reader - USB OVRCURA Interrupt Source Recovery
pub type DOVRCRA0_R = crate::BitReader<DOVRCRA0_A>;
impl DOVRCRA0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVRCRA0_A {
        match self.bits {
            false => DOVRCRA0_A::_0,
            true => DOVRCRA0_A::_1,
        }
    }
    ///The system has not returned from deep software standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRA0_A::_0
    }
    ///The system has returned from deep software standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRA0_A::_1
    }
}
/**USB OVRCURB Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOVRCRB0_A {
    ///0: The system has not returned from deep software standby mode.
    _0 = 0,
    ///1: The system has returned from deep software standby mode.
    _1 = 1,
}
impl From<DOVRCRB0_A> for bool {
    #[inline(always)]
    fn from(variant: DOVRCRB0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DOVRCRB0` reader - USB OVRCURB Interrupt Source Recovery
pub type DOVRCRB0_R = crate::BitReader<DOVRCRB0_A>;
impl DOVRCRB0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DOVRCRB0_A {
        match self.bits {
            false => DOVRCRB0_A::_0,
            true => DOVRCRB0_A::_1,
        }
    }
    ///The system has not returned from deep software standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOVRCRB0_A::_0
    }
    ///The system has returned from deep software standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOVRCRB0_A::_1
    }
}
/**USB VBUS Interrupt Source Recovery

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVBINT0_A {
    ///0: The system has not returned from deep software standby mode.
    _0 = 0,
    ///1: The system has returned from deep software standby mode.
    _1 = 1,
}
impl From<DVBINT0_A> for bool {
    #[inline(always)]
    fn from(variant: DVBINT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DVBINT0` reader - USB VBUS Interrupt Source Recovery
pub type DVBINT0_R = crate::BitReader<DVBINT0_A>;
impl DVBINT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVBINT0_A {
        match self.bits {
            false => DVBINT0_A::_0,
            true => DVBINT0_A::_1,
        }
    }
    ///The system has not returned from deep software standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVBINT0_A::_0
    }
    ///The system has returned from deep software standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVBINT0_A::_1
    }
}
impl R {
    ///Bit 0 - USB DP Interrupt Enable/Clear
    #[inline(always)]
    pub fn dpinte0(&self) -> DPINTE0_R {
        DPINTE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USB DM Interrupt Enable/Clear
    #[inline(always)]
    pub fn dminte0(&self) -> DMINTE0_R {
        DMINTE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USB OVRCURA Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrae0(&self) -> DOVRCRAE0_R {
        DOVRCRAE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USB OVRCURB Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrbe0(&self) -> DOVRCRBE0_R {
        DOVRCRBE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - USB VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbse0(&self) -> DVBSE0_R {
        DVBSE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - USB DP Interrupt Source Recovery
    #[inline(always)]
    pub fn dpint0(&self) -> DPINT0_R {
        DPINT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USB DM Interrupt Source Recovery
    #[inline(always)]
    pub fn dmint0(&self) -> DMINT0_R {
        DMINT0_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - USB OVRCURA Interrupt Source Recovery
    #[inline(always)]
    pub fn dovrcra0(&self) -> DOVRCRA0_R {
        DOVRCRA0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - USB OVRCURB Interrupt Source Recovery
    #[inline(always)]
    pub fn dovrcrb0(&self) -> DOVRCRB0_R {
        DOVRCRB0_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - USB VBUS Interrupt Source Recovery
    #[inline(always)]
    pub fn dvbint0(&self) -> DVBINT0_R {
        DVBINT0_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USB DP Interrupt Enable/Clear
    #[inline(always)]
    pub fn dpinte0(&mut self) -> DPINTE0_W<DPUSR1R_SPEC> {
        DPINTE0_W::new(self, 0)
    }
    ///Bit 1 - USB DM Interrupt Enable/Clear
    #[inline(always)]
    pub fn dminte0(&mut self) -> DMINTE0_W<DPUSR1R_SPEC> {
        DMINTE0_W::new(self, 1)
    }
    ///Bit 4 - USB OVRCURA Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrae0(&mut self) -> DOVRCRAE0_W<DPUSR1R_SPEC> {
        DOVRCRAE0_W::new(self, 4)
    }
    ///Bit 5 - USB OVRCURB Interrupt Enable/Clear
    #[inline(always)]
    pub fn dovrcrbe0(&mut self) -> DOVRCRBE0_W<DPUSR1R_SPEC> {
        DOVRCRBE0_W::new(self, 5)
    }
    ///Bit 7 - USB VBUS Interrupt Enable/Clear
    #[inline(always)]
    pub fn dvbse0(&mut self) -> DVBSE0_W<DPUSR1R_SPEC> {
        DVBSE0_W::new(self, 7)
    }
}
/**Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPUSR1R_SPEC;
impl crate::RegisterSpec for DPUSR1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpusr1r::R`](R) reader structure
impl crate::Readable for DPUSR1R_SPEC {}
///`write(|w| ..)` method takes [`dpusr1r::W`](W) writer structure
impl crate::Writable for DPUSR1R_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSR1R to value 0
impl crate::Resettable for DPUSR1R_SPEC {}
