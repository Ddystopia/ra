///Register `MOMCR` reader
pub type R = crate::R<MOMCR_SPEC>;
///Register `MOMCR` writer
pub type W = crate::W<MOMCR_SPEC>;
/**Main Clock Oscillator Drive Capability 0 Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODRV0_A {
    ///0: 20MHz to 24MHz
    _00 = 0,
    ///1: 16MHz to 20MHz
    _01 = 1,
    ///2: 8MHz to 16MHz
    _10 = 2,
    ///3: 8MHz
    _11 = 3,
}
impl From<MODRV0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODRV0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODRV0_A {
    type Ux = u8;
}
impl crate::IsEnum for MODRV0_A {}
///Field `MODRV0` reader - Main Clock Oscillator Drive Capability 0 Switching
pub type MODRV0_R = crate::FieldReader<MODRV0_A>;
impl MODRV0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODRV0_A {
        match self.bits {
            0 => MODRV0_A::_00,
            1 => MODRV0_A::_01,
            2 => MODRV0_A::_10,
            3 => MODRV0_A::_11,
            _ => unreachable!(),
        }
    }
    ///20MHz to 24MHz
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODRV0_A::_00
    }
    ///16MHz to 20MHz
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODRV0_A::_01
    }
    ///8MHz to 16MHz
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODRV0_A::_10
    }
    ///8MHz
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MODRV0_A::_11
    }
}
///Field `MODRV0` writer - Main Clock Oscillator Drive Capability 0 Switching
pub type MODRV0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODRV0_A, crate::Safe>;
impl<'a, REG> MODRV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///20MHz to 24MHz
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV0_A::_00)
    }
    ///16MHz to 20MHz
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV0_A::_01)
    }
    ///8MHz to 16MHz
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV0_A::_10)
    }
    ///8MHz
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV0_A::_11)
    }
}
/**Main Clock Oscillator Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSEL_A {
    ///0: Resonator
    _0 = 0,
    ///1: External clock input
    _1 = 1,
}
impl From<MOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MOSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MOSEL` reader - Main Clock Oscillator Switching
pub type MOSEL_R = crate::BitReader<MOSEL_A>;
impl MOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOSEL_A {
        match self.bits {
            false => MOSEL_A::_0,
            true => MOSEL_A::_1,
        }
    }
    ///Resonator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSEL_A::_0
    }
    ///External clock input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSEL_A::_1
    }
}
///Field `MOSEL` writer - Main Clock Oscillator Switching
pub type MOSEL_W<'a, REG> = crate::BitWriter<'a, REG, MOSEL_A>;
impl<'a, REG> MOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resonator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOSEL_A::_0)
    }
    ///External clock input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOSEL_A::_1)
    }
}
/**Main Clock Oscillator Drive Capability Auto Switching Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTODRVEN_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<AUTODRVEN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTODRVEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTODRVEN` reader - Main Clock Oscillator Drive Capability Auto Switching Enable
pub type AUTODRVEN_R = crate::BitReader<AUTODRVEN_A>;
impl AUTODRVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTODRVEN_A {
        match self.bits {
            false => AUTODRVEN_A::_0,
            true => AUTODRVEN_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUTODRVEN_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUTODRVEN_A::_1
    }
}
///Field `AUTODRVEN` writer - Main Clock Oscillator Drive Capability Auto Switching Enable
pub type AUTODRVEN_W<'a, REG> = crate::BitWriter<'a, REG, AUTODRVEN_A>;
impl<'a, REG> AUTODRVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTODRVEN_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTODRVEN_A::_1)
    }
}
impl R {
    ///Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching
    #[inline(always)]
    pub fn modrv0(&self) -> MODRV0_R {
        MODRV0_R::new((self.bits >> 4) & 3)
    }
    ///Bit 6 - Main Clock Oscillator Switching
    #[inline(always)]
    pub fn mosel(&self) -> MOSEL_R {
        MOSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Main Clock Oscillator Drive Capability Auto Switching Enable
    #[inline(always)]
    pub fn autodrven(&self) -> AUTODRVEN_R {
        AUTODRVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 4:5 - Main Clock Oscillator Drive Capability 0 Switching
    #[inline(always)]
    pub fn modrv0(&mut self) -> MODRV0_W<MOMCR_SPEC> {
        MODRV0_W::new(self, 4)
    }
    ///Bit 6 - Main Clock Oscillator Switching
    #[inline(always)]
    pub fn mosel(&mut self) -> MOSEL_W<MOMCR_SPEC> {
        MOSEL_W::new(self, 6)
    }
    ///Bit 7 - Main Clock Oscillator Drive Capability Auto Switching Enable
    #[inline(always)]
    pub fn autodrven(&mut self) -> AUTODRVEN_W<MOMCR_SPEC> {
        AUTODRVEN_W::new(self, 7)
    }
}
/**Main Clock Oscillator Mode Oscillation Control Register

You can [`read`](crate::Reg::read) this register and get [`momcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MOMCR_SPEC;
impl crate::RegisterSpec for MOMCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`momcr::R`](R) reader structure
impl crate::Readable for MOMCR_SPEC {}
///`write(|w| ..)` method takes [`momcr::W`](W) writer structure
impl crate::Writable for MOMCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOMCR to value 0
impl crate::Resettable for MOMCR_SPEC {}
