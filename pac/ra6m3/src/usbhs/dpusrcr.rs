///Register `DPUSRCR` reader
pub type R = crate::R<DPUSRCR_SPEC>;
///Register `DPUSRCR` writer
pub type W = crate::W<DPUSRCR_SPEC>;
/**USB Transceiver Control Fix

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXPHY_A {
    ///0: Normal mode
    _0 = 0,
    ///1: Go to/Return from deep software standby mode
    _1 = 1,
}
impl From<FIXPHY_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPHY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXPHY` reader - USB Transceiver Control Fix
pub type FIXPHY_R = crate::BitReader<FIXPHY_A>;
impl FIXPHY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FIXPHY_A {
        match self.bits {
            false => FIXPHY_A::_0,
            true => FIXPHY_A::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXPHY_A::_0
    }
    ///Go to/Return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXPHY_A::_1
    }
}
///Field `FIXPHY` writer - USB Transceiver Control Fix
pub type FIXPHY_W<'a, REG> = crate::BitWriter<'a, REG, FIXPHY_A>;
impl<'a, REG> FIXPHY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FIXPHY_A::_0)
    }
    ///Go to/Return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FIXPHY_A::_1)
    }
}
/**USB Transceiver Control Fix for PLL

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXPHYPD_A {
    ///0: Normal mode
    _0 = 0,
    ///1: Go to/Return from deep software standby mode
    _1 = 1,
}
impl From<FIXPHYPD_A> for bool {
    #[inline(always)]
    fn from(variant: FIXPHYPD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FIXPHYPD` reader - USB Transceiver Control Fix for PLL
pub type FIXPHYPD_R = crate::BitReader<FIXPHYPD_A>;
impl FIXPHYPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FIXPHYPD_A {
        match self.bits {
            false => FIXPHYPD_A::_0,
            true => FIXPHYPD_A::_1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXPHYPD_A::_0
    }
    ///Go to/Return from deep software standby mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXPHYPD_A::_1
    }
}
///Field `FIXPHYPD` writer - USB Transceiver Control Fix for PLL
pub type FIXPHYPD_W<'a, REG> = crate::BitWriter<'a, REG, FIXPHYPD_A>;
impl<'a, REG> FIXPHYPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FIXPHYPD_A::_0)
    }
    ///Go to/Return from deep software standby mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FIXPHYPD_A::_1)
    }
}
impl R {
    ///Bit 0 - USB Transceiver Control Fix
    #[inline(always)]
    pub fn fixphy(&self) -> FIXPHY_R {
        FIXPHY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USB Transceiver Control Fix for PLL
    #[inline(always)]
    pub fn fixphypd(&self) -> FIXPHYPD_R {
        FIXPHYPD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USB Transceiver Control Fix
    #[inline(always)]
    pub fn fixphy(&mut self) -> FIXPHY_W<DPUSRCR_SPEC> {
        FIXPHY_W::new(self, 0)
    }
    ///Bit 1 - USB Transceiver Control Fix for PLL
    #[inline(always)]
    pub fn fixphypd(&mut self) -> FIXPHYPD_W<DPUSRCR_SPEC> {
        FIXPHYPD_W::new(self, 1)
    }
}
/**Deep Standby USB Suspend/Resume Command Register

You can [`read`](crate::Reg::read) this register and get [`dpusrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPUSRCR_SPEC;
impl crate::RegisterSpec for DPUSRCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [`dpusrcr::R`](R) reader structure
impl crate::Readable for DPUSRCR_SPEC {}
///`write(|w| ..)` method takes [`dpusrcr::W`](W) writer structure
impl crate::Writable for DPUSRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPUSRCR to value 0
impl crate::Resettable for DPUSRCR_SPEC {}
