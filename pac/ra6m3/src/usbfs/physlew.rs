///Register `PHYSLEW` reader
pub type R = crate::R<PHYSLEW_SPEC>;
///Register `PHYSLEW` writer
pub type W = crate::W<PHYSLEW_SPEC>;
/**Receiver Cross Point Adjustment 00

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWR00_A {
    ///0: Reserved
    _0 = 0,
    ///1: Host or device controller mode.
    _1 = 1,
}
impl From<SLEWR00_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWR00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEWR00` reader - Receiver Cross Point Adjustment 00
pub type SLEWR00_R = crate::BitReader<SLEWR00_A>;
impl SLEWR00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLEWR00_A {
        match self.bits {
            false => SLEWR00_A::_0,
            true => SLEWR00_A::_1,
        }
    }
    ///Reserved
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWR00_A::_0
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWR00_A::_1
    }
}
///Field `SLEWR00` writer - Receiver Cross Point Adjustment 00
pub type SLEWR00_W<'a, REG> = crate::BitWriter<'a, REG, SLEWR00_A>;
impl<'a, REG> SLEWR00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reserved
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWR00_A::_0)
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWR00_A::_1)
    }
}
/**Receiver Cross Point Adjustment 01

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWR01_A {
    ///0: Reserved
    _0 = 0,
    ///1: Host or device controller mode.
    _1 = 1,
}
impl From<SLEWR01_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWR01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEWR01` reader - Receiver Cross Point Adjustment 01
pub type SLEWR01_R = crate::BitReader<SLEWR01_A>;
impl SLEWR01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLEWR01_A {
        match self.bits {
            false => SLEWR01_A::_0,
            true => SLEWR01_A::_1,
        }
    }
    ///Reserved
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWR01_A::_0
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWR01_A::_1
    }
}
///Field `SLEWR01` writer - Receiver Cross Point Adjustment 01
pub type SLEWR01_W<'a, REG> = crate::BitWriter<'a, REG, SLEWR01_A>;
impl<'a, REG> SLEWR01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reserved
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWR01_A::_0)
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWR01_A::_1)
    }
}
/**Receiver Cross Point Adjustment 00

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWF00_A {
    ///0: Reserved
    _0 = 0,
    ///1: Host or device controller mode.
    _1 = 1,
}
impl From<SLEWF00_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWF00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEWF00` reader - Receiver Cross Point Adjustment 00
pub type SLEWF00_R = crate::BitReader<SLEWF00_A>;
impl SLEWF00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLEWF00_A {
        match self.bits {
            false => SLEWF00_A::_0,
            true => SLEWF00_A::_1,
        }
    }
    ///Reserved
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWF00_A::_0
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWF00_A::_1
    }
}
///Field `SLEWF00` writer - Receiver Cross Point Adjustment 00
pub type SLEWF00_W<'a, REG> = crate::BitWriter<'a, REG, SLEWF00_A>;
impl<'a, REG> SLEWF00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reserved
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWF00_A::_0)
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWF00_A::_1)
    }
}
/**Receiver Cross Point Adjustment 01

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEWF01_A {
    ///0: Reserved
    _0 = 0,
    ///1: Host or device controller mode.
    _1 = 1,
}
impl From<SLEWF01_A> for bool {
    #[inline(always)]
    fn from(variant: SLEWF01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEWF01` reader - Receiver Cross Point Adjustment 01
pub type SLEWF01_R = crate::BitReader<SLEWF01_A>;
impl SLEWF01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLEWF01_A {
        match self.bits {
            false => SLEWF01_A::_0,
            true => SLEWF01_A::_1,
        }
    }
    ///Reserved
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEWF01_A::_0
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEWF01_A::_1
    }
}
///Field `SLEWF01` writer - Receiver Cross Point Adjustment 01
pub type SLEWF01_W<'a, REG> = crate::BitWriter<'a, REG, SLEWF01_A>;
impl<'a, REG> SLEWF01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reserved
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWF01_A::_0)
    }
    ///Host or device controller mode.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SLEWF01_A::_1)
    }
}
impl R {
    ///Bit 0 - Receiver Cross Point Adjustment 00
    #[inline(always)]
    pub fn slewr00(&self) -> SLEWR00_R {
        SLEWR00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receiver Cross Point Adjustment 01
    #[inline(always)]
    pub fn slewr01(&self) -> SLEWR01_R {
        SLEWR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver Cross Point Adjustment 00
    #[inline(always)]
    pub fn slewf00(&self) -> SLEWF00_R {
        SLEWF00_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receiver Cross Point Adjustment 01
    #[inline(always)]
    pub fn slewf01(&self) -> SLEWF01_R {
        SLEWF01_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receiver Cross Point Adjustment 00
    #[inline(always)]
    pub fn slewr00(&mut self) -> SLEWR00_W<PHYSLEW_SPEC> {
        SLEWR00_W::new(self, 0)
    }
    ///Bit 1 - Receiver Cross Point Adjustment 01
    #[inline(always)]
    pub fn slewr01(&mut self) -> SLEWR01_W<PHYSLEW_SPEC> {
        SLEWR01_W::new(self, 1)
    }
    ///Bit 2 - Receiver Cross Point Adjustment 00
    #[inline(always)]
    pub fn slewf00(&mut self) -> SLEWF00_W<PHYSLEW_SPEC> {
        SLEWF00_W::new(self, 2)
    }
    ///Bit 3 - Receiver Cross Point Adjustment 01
    #[inline(always)]
    pub fn slewf01(&mut self) -> SLEWF01_W<PHYSLEW_SPEC> {
        SLEWF01_W::new(self, 3)
    }
}
/**PHY Cross Point Adjustment Register

You can [`read`](crate::Reg::read) this register and get [`physlew::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physlew::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHYSLEW_SPEC;
impl crate::RegisterSpec for PHYSLEW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`physlew::R`](R) reader structure
impl crate::Readable for PHYSLEW_SPEC {}
///`write(|w| ..)` method takes [`physlew::W`](W) writer structure
impl crate::Writable for PHYSLEW_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PHYSLEW to value 0x0e
impl crate::Resettable for PHYSLEW_SPEC {
    const RESET_VALUE: u32 = 0x0e;
}
