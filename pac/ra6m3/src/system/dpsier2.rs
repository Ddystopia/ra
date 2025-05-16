///Register `DPSIER2` reader
pub type R = crate::R<DPSIER2_SPEC>;
///Register `DPSIER2` writer
pub type W = crate::W<DPSIER2_SPEC>;
/**LVD1 Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD1IE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DLVD1IE_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD1IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD1IE` reader - LVD1 Deep Standby Cancel Signal Enable
pub type DLVD1IE_R = crate::BitReader<DLVD1IE_A>;
impl DLVD1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLVD1IE_A {
        match self.bits {
            false => DLVD1IE_A::_0,
            true => DLVD1IE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD1IE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD1IE_A::_1
    }
}
///Field `DLVD1IE` writer - LVD1 Deep Standby Cancel Signal Enable
pub type DLVD1IE_W<'a, REG> = crate::BitWriter<'a, REG, DLVD1IE_A>;
impl<'a, REG> DLVD1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD1IE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD1IE_A::_1)
    }
}
/**LVD2 Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLVD2IE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DLVD2IE_A> for bool {
    #[inline(always)]
    fn from(variant: DLVD2IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLVD2IE` reader - LVD2 Deep Standby Cancel Signal Enable
pub type DLVD2IE_R = crate::BitReader<DLVD2IE_A>;
impl DLVD2IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLVD2IE_A {
        match self.bits {
            false => DLVD2IE_A::_0,
            true => DLVD2IE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLVD2IE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLVD2IE_A::_1
    }
}
///Field `DLVD2IE` writer - LVD2 Deep Standby Cancel Signal Enable
pub type DLVD2IE_W<'a, REG> = crate::BitWriter<'a, REG, DLVD2IE_A>;
impl<'a, REG> DLVD2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD2IE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLVD2IE_A::_1)
    }
}
/**RTC Interval interrupt Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRTCIIE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DTRTCIIE_A> for bool {
    #[inline(always)]
    fn from(variant: DTRTCIIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTRTCIIE` reader - RTC Interval interrupt Deep Standby Cancel Signal Enable
pub type DTRTCIIE_R = crate::BitReader<DTRTCIIE_A>;
impl DTRTCIIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTRTCIIE_A {
        match self.bits {
            false => DTRTCIIE_A::_0,
            true => DTRTCIIE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTRTCIIE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTRTCIIE_A::_1
    }
}
///Field `DTRTCIIE` writer - RTC Interval interrupt Deep Standby Cancel Signal Enable
pub type DTRTCIIE_W<'a, REG> = crate::BitWriter<'a, REG, DTRTCIIE_A>;
impl<'a, REG> DTRTCIIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTRTCIIE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTRTCIIE_A::_1)
    }
}
/**RTC Alarm interrupt Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRTCAIE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DRTCAIE_A> for bool {
    #[inline(always)]
    fn from(variant: DRTCAIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRTCAIE` reader - RTC Alarm interrupt Deep Standby Cancel Signal Enable
pub type DRTCAIE_R = crate::BitReader<DRTCAIE_A>;
impl DRTCAIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DRTCAIE_A {
        match self.bits {
            false => DRTCAIE_A::_0,
            true => DRTCAIE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRTCAIE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRTCAIE_A::_1
    }
}
///Field `DRTCAIE` writer - RTC Alarm interrupt Deep Standby Cancel Signal Enable
pub type DRTCAIE_W<'a, REG> = crate::BitWriter<'a, REG, DRTCAIE_A>;
impl<'a, REG> DRTCAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DRTCAIE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DRTCAIE_A::_1)
    }
}
/**NMI Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNMIE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DNMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DNMIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DNMIE` reader - NMI Pin Enable
pub type DNMIE_R = crate::BitReader<DNMIE_A>;
impl DNMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DNMIE_A {
        match self.bits {
            false => DNMIE_A::_0,
            true => DNMIE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DNMIE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DNMIE_A::_1
    }
}
///Field `DNMIE` writer - NMI Pin Enable
pub type DNMIE_W<'a, REG> = crate::BitWriter<'a, REG, DNMIE_A>;
impl<'a, REG> DNMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DNMIE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DNMIE_A::_1)
    }
}
impl R {
    ///Bit 0 - LVD1 Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd1ie(&self) -> DLVD1IE_R {
        DLVD1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LVD2 Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd2ie(&self) -> DLVD2IE_R {
        DLVD2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RTC Interval interrupt Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dtrtciie(&self) -> DTRTCIIE_R {
        DTRTCIIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RTC Alarm interrupt Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn drtcaie(&self) -> DRTCAIE_R {
        DRTCAIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NMI Pin Enable
    #[inline(always)]
    pub fn dnmie(&self) -> DNMIE_R {
        DNMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LVD1 Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd1ie(&mut self) -> DLVD1IE_W<DPSIER2_SPEC> {
        DLVD1IE_W::new(self, 0)
    }
    ///Bit 1 - LVD2 Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dlvd2ie(&mut self) -> DLVD2IE_W<DPSIER2_SPEC> {
        DLVD2IE_W::new(self, 1)
    }
    ///Bit 2 - RTC Interval interrupt Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dtrtciie(&mut self) -> DTRTCIIE_W<DPSIER2_SPEC> {
        DTRTCIIE_W::new(self, 2)
    }
    ///Bit 3 - RTC Alarm interrupt Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn drtcaie(&mut self) -> DRTCAIE_W<DPSIER2_SPEC> {
        DRTCAIE_W::new(self, 3)
    }
    ///Bit 4 - NMI Pin Enable
    #[inline(always)]
    pub fn dnmie(&mut self) -> DNMIE_W<DPSIER2_SPEC> {
        DNMIE_W::new(self, 4)
    }
}
/**Deep Standby Interrupt Enable Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIER2_SPEC;
impl crate::RegisterSpec for DPSIER2_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsier2::R`](R) reader structure
impl crate::Readable for DPSIER2_SPEC {}
///`write(|w| ..)` method takes [`dpsier2::W`](W) writer structure
impl crate::Writable for DPSIER2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER2 to value 0
impl crate::Resettable for DPSIER2_SPEC {}
