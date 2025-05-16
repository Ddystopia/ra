///Register `DPSBYCR` reader
pub type R = crate::R<DPSBYCR_SPEC>;
///Register `DPSBYCR` writer
pub type W = crate::W<DPSBYCR_SPEC>;
/**Power-Supply Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEEPCUT_A {
    ///0: Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode.
    _00 = 0,
    ///1: Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is not supplied in deep software standby mode.
    _01 = 1,
    ///2: Setting prohibited.
    _10 = 2,
    ///3: Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled.
    _11 = 3,
}
impl From<DEEPCUT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEEPCUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEEPCUT_A {
    type Ux = u8;
}
impl crate::IsEnum for DEEPCUT_A {}
///Field `DEEPCUT` reader - Power-Supply Control
pub type DEEPCUT_R = crate::FieldReader<DEEPCUT_A>;
impl DEEPCUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEEPCUT_A {
        match self.bits {
            0 => DEEPCUT_A::_00,
            1 => DEEPCUT_A::_01,
            2 => DEEPCUT_A::_10,
            3 => DEEPCUT_A::_11,
            _ => unreachable!(),
        }
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode.
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DEEPCUT_A::_00
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is not supplied in deep software standby mode.
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DEEPCUT_A::_01
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DEEPCUT_A::_10
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled.
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DEEPCUT_A::_11
    }
}
///Field `DEEPCUT` writer - Power-Supply Control
pub type DEEPCUT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DEEPCUT_A, crate::Safe>;
impl<'a, REG> DEEPCUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode.
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPCUT_A::_00)
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is not supplied in deep software standby mode.
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPCUT_A::_01)
    }
    ///Setting prohibited.
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPCUT_A::_10)
    }
    ///Power to the standby RAM, Low-speed on-chip oscillator, AGTn, and USBFS/HS resume detecting unit is supplied in deep software standby mode. In addition, LVD is disabled and the low power function in a power-on reset circuit is enabled.
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(DEEPCUT_A::_11)
    }
}
/**I/O Port Retention

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOKEEP_A {
    ///0: When the Deep Software Standby mode is canceled, the I/O ports are in the reset state.
    _0 = 0,
    ///1: When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode.
    _1 = 1,
}
impl From<IOKEEP_A> for bool {
    #[inline(always)]
    fn from(variant: IOKEEP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IOKEEP` reader - I/O Port Retention
pub type IOKEEP_R = crate::BitReader<IOKEEP_A>;
impl IOKEEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IOKEEP_A {
        match self.bits {
            false => IOKEEP_A::_0,
            true => IOKEEP_A::_1,
        }
    }
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the reset state.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IOKEEP_A::_0
    }
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IOKEEP_A::_1
    }
}
///Field `IOKEEP` writer - I/O Port Retention
pub type IOKEEP_W<'a, REG> = crate::BitWriter<'a, REG, IOKEEP_A>;
impl<'a, REG> IOKEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the reset state.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IOKEEP_A::_0)
    }
    ///When the Deep Software Standby mode is canceled, the I/O ports are in the same state as in the Deep Software Standby mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IOKEEP_A::_1)
    }
}
/**Deep Software Standby

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSBY_A {
    ///0: Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)
    _0 = 0,
    ///1: Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)
    _1 = 1,
}
impl From<DPSBY_A> for bool {
    #[inline(always)]
    fn from(variant: DPSBY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPSBY` reader - Deep Software Standby
pub type DPSBY_R = crate::BitReader<DPSBY_A>;
impl DPSBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPSBY_A {
        match self.bits {
            false => DPSBY_A::_0,
            true => DPSBY_A::_1,
        }
    }
    ///Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSBY_A::_0
    }
    ///Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSBY_A::_1
    }
}
///Field `DPSBY` writer - Deep Software Standby
pub type DPSBY_W<'a, REG> = crate::BitWriter<'a, REG, DPSBY_A>;
impl<'a, REG> DPSBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sleep mode (SBYCR.SSBY=0) / Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPSBY_A::_0)
    }
    ///Sleep mode (SBYCR.SSBY=0) / Deep Software Standby mode (SBYCR.SSBY=1)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPSBY_A::_1)
    }
}
impl R {
    ///Bits 0:1 - Power-Supply Control
    #[inline(always)]
    pub fn deepcut(&self) -> DEEPCUT_R {
        DEEPCUT_R::new(self.bits & 3)
    }
    ///Bit 6 - I/O Port Retention
    #[inline(always)]
    pub fn iokeep(&self) -> IOKEEP_R {
        IOKEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Deep Software Standby
    #[inline(always)]
    pub fn dpsby(&self) -> DPSBY_R {
        DPSBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Power-Supply Control
    #[inline(always)]
    pub fn deepcut(&mut self) -> DEEPCUT_W<DPSBYCR_SPEC> {
        DEEPCUT_W::new(self, 0)
    }
    ///Bit 6 - I/O Port Retention
    #[inline(always)]
    pub fn iokeep(&mut self) -> IOKEEP_W<DPSBYCR_SPEC> {
        IOKEEP_W::new(self, 6)
    }
    ///Bit 7 - Deep Software Standby
    #[inline(always)]
    pub fn dpsby(&mut self) -> DPSBY_W<DPSBYCR_SPEC> {
        DPSBY_W::new(self, 7)
    }
}
/**Deep Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`dpsbycr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsbycr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSBYCR_SPEC;
impl crate::RegisterSpec for DPSBYCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsbycr::R`](R) reader structure
impl crate::Readable for DPSBYCR_SPEC {}
///`write(|w| ..)` method takes [`dpsbycr::W`](W) writer structure
impl crate::Writable for DPSBYCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSBYCR to value 0x01
impl crate::Resettable for DPSBYCR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
