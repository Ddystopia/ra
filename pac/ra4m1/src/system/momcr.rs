///Register `MOMCR` reader
pub type R = crate::R<MOMCR_SPEC>;
///Register `MOMCR` writer
pub type W = crate::W<MOMCR_SPEC>;
/**Main Clock Oscillator Drive Capability 1 Switching

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODRV1_A {
    ///0: 10 MHz to 20 MHz
    _0 = 0,
    ///1: 1 MHz to 10 MHz.
    _1 = 1,
}
impl From<MODRV1_A> for bool {
    #[inline(always)]
    fn from(variant: MODRV1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MODRV1` reader - Main Clock Oscillator Drive Capability 1 Switching
pub type MODRV1_R = crate::BitReader<MODRV1_A>;
impl MODRV1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODRV1_A {
        match self.bits {
            false => MODRV1_A::_0,
            true => MODRV1_A::_1,
        }
    }
    ///10 MHz to 20 MHz
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODRV1_A::_0
    }
    ///1 MHz to 10 MHz.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODRV1_A::_1
    }
}
///Field `MODRV1` writer - Main Clock Oscillator Drive Capability 1 Switching
pub type MODRV1_W<'a, REG> = crate::BitWriter<'a, REG, MODRV1_A>;
impl<'a, REG> MODRV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///10 MHz to 20 MHz
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV1_A::_0)
    }
    ///1 MHz to 10 MHz.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MODRV1_A::_1)
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
impl R {
    ///Bit 3 - Main Clock Oscillator Drive Capability 1 Switching
    #[inline(always)]
    pub fn modrv1(&self) -> MODRV1_R {
        MODRV1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Main Clock Oscillator Switching
    #[inline(always)]
    pub fn mosel(&self) -> MOSEL_R {
        MOSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Main Clock Oscillator Drive Capability 1 Switching
    #[inline(always)]
    pub fn modrv1(&mut self) -> MODRV1_W<MOMCR_SPEC> {
        MODRV1_W::new(self, 3)
    }
    ///Bit 6 - Main Clock Oscillator Switching
    #[inline(always)]
    pub fn mosel(&mut self) -> MOSEL_W<MOMCR_SPEC> {
        MOSEL_W::new(self, 6)
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
