///Register `SNZCR` reader
pub type R = crate::R<SNZCR_SPEC>;
///Register `SNZCR` writer
pub type W = crate::W<SNZCR_SPEC>;
/**RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDREQEN_A {
    ///0: Ignore RXD0 falling edge in Standby mode.
    _0 = 0,
    ///1: Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode.
    _1 = 1,
}
impl From<RXDREQEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDREQEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDREQEN` reader - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.
pub type RXDREQEN_R = crate::BitReader<RXDREQEN_A>;
impl RXDREQEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDREQEN_A {
        match self.bits {
            false => RXDREQEN_A::_0,
            true => RXDREQEN_A::_1,
        }
    }
    ///Ignore RXD0 falling edge in Standby mode.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXDREQEN_A::_0
    }
    ///Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXDREQEN_A::_1
    }
}
///Field `RXDREQEN` writer - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.
pub type RXDREQEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDREQEN_A>;
impl<'a, REG> RXDREQEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Ignore RXD0 falling edge in Standby mode.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDREQEN_A::_0)
    }
    ///Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDREQEN_A::_1)
    }
}
/**DTC Enable in Snooze Mode

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZDTCEN_A {
    ///0: Disable DTC operation
    _0 = 0,
    ///1: Enable DTC operation
    _1 = 1,
}
impl From<SNZDTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SNZDTCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZDTCEN` reader - DTC Enable in Snooze Mode
pub type SNZDTCEN_R = crate::BitReader<SNZDTCEN_A>;
impl SNZDTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZDTCEN_A {
        match self.bits {
            false => SNZDTCEN_A::_0,
            true => SNZDTCEN_A::_1,
        }
    }
    ///Disable DTC operation
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZDTCEN_A::_0
    }
    ///Enable DTC operation
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZDTCEN_A::_1
    }
}
///Field `SNZDTCEN` writer - DTC Enable in Snooze Mode
pub type SNZDTCEN_W<'a, REG> = crate::BitWriter<'a, REG, SNZDTCEN_A>;
impl<'a, REG> SNZDTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable DTC operation
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZDTCEN_A::_0)
    }
    ///Enable DTC operation
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZDTCEN_A::_1)
    }
}
/**Snooze Mode Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZE_A {
    ///0: Disable Snooze Mode
    _0 = 0,
    ///1: Enable Snooze Mode
    _1 = 1,
}
impl From<SNZE_A> for bool {
    #[inline(always)]
    fn from(variant: SNZE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZE` reader - Snooze Mode Enable
pub type SNZE_R = crate::BitReader<SNZE_A>;
impl SNZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZE_A {
        match self.bits {
            false => SNZE_A::_0,
            true => SNZE_A::_1,
        }
    }
    ///Disable Snooze Mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZE_A::_0
    }
    ///Enable Snooze Mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZE_A::_1
    }
}
///Field `SNZE` writer - Snooze Mode Enable
pub type SNZE_W<'a, REG> = crate::BitWriter<'a, REG, SNZE_A>;
impl<'a, REG> SNZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable Snooze Mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZE_A::_0)
    }
    ///Enable Snooze Mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZE_A::_1)
    }
}
impl R {
    ///Bit 0 - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.
    #[inline(always)]
    pub fn rxdreqen(&self) -> RXDREQEN_R {
        RXDREQEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTC Enable in Snooze Mode
    #[inline(always)]
    pub fn snzdtcen(&self) -> SNZDTCEN_R {
        SNZDTCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 7 - Snooze Mode Enable
    #[inline(always)]
    pub fn snze(&self) -> SNZE_R {
        SNZE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RXD0 Snooze Request Enable NOTE: Do not set to 1 other than in asynchronous mode.
    #[inline(always)]
    pub fn rxdreqen(&mut self) -> RXDREQEN_W<SNZCR_SPEC> {
        RXDREQEN_W::new(self, 0)
    }
    ///Bit 1 - DTC Enable in Snooze Mode
    #[inline(always)]
    pub fn snzdtcen(&mut self) -> SNZDTCEN_W<SNZCR_SPEC> {
        SNZDTCEN_W::new(self, 1)
    }
    ///Bit 7 - Snooze Mode Enable
    #[inline(always)]
    pub fn snze(&mut self) -> SNZE_W<SNZCR_SPEC> {
        SNZE_W::new(self, 7)
    }
}
/**Snooze Control Register

You can [`read`](crate::Reg::read) this register and get [`snzcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SNZCR_SPEC;
impl crate::RegisterSpec for SNZCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`snzcr::R`](R) reader structure
impl crate::Readable for SNZCR_SPEC {}
///`write(|w| ..)` method takes [`snzcr::W`](W) writer structure
impl crate::Writable for SNZCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZCR to value 0
impl crate::Resettable for SNZCR_SPEC {}
