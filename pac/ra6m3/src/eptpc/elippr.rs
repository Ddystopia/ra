///Register `ELIPPR` reader
pub type R = crate::R<ELIPPR_SPEC>;
///Register `ELIPPR` writer
pub type W = crate::W<ELIPPR_SPEC>;
/**Pulse Output Timer 0 Rising Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP0_A {
    ///0: Rising edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Rising edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCP0_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCP0` reader - Pulse Output Timer 0 Rising Edge Detection Event Output Enable
pub type CYCP0_R = crate::BitReader<CYCP0_A>;
impl CYCP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCP0_A {
        match self.bits {
            false => CYCP0_A::_0,
            true => CYCP0_A::_1,
        }
    }
    ///Rising edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP0_A::_0
    }
    ///Rising edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP0_A::_1
    }
}
///Field `CYCP0` writer - Pulse Output Timer 0 Rising Edge Detection Event Output Enable
pub type CYCP0_W<'a, REG> = crate::BitWriter<'a, REG, CYCP0_A>;
impl<'a, REG> CYCP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP0_A::_0)
    }
    ///Rising edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP0_A::_1)
    }
}
/**Pulse Output Timer 1 Rising Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP1_A {
    ///0: Rising edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Rising edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCP1_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCP1` reader - Pulse Output Timer 1 Rising Edge Detection Event Output Enable
pub type CYCP1_R = crate::BitReader<CYCP1_A>;
impl CYCP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCP1_A {
        match self.bits {
            false => CYCP1_A::_0,
            true => CYCP1_A::_1,
        }
    }
    ///Rising edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP1_A::_0
    }
    ///Rising edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP1_A::_1
    }
}
///Field `CYCP1` writer - Pulse Output Timer 1 Rising Edge Detection Event Output Enable
pub type CYCP1_W<'a, REG> = crate::BitWriter<'a, REG, CYCP1_A>;
impl<'a, REG> CYCP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP1_A::_0)
    }
    ///Rising edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP1_A::_1)
    }
}
/**Pulse Output Timer 2 Rising Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP2_A {
    ///0: Rising edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Rising edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCP2_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCP2` reader - Pulse Output Timer 2 Rising Edge Detection Event Output Enable
pub type CYCP2_R = crate::BitReader<CYCP2_A>;
impl CYCP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCP2_A {
        match self.bits {
            false => CYCP2_A::_0,
            true => CYCP2_A::_1,
        }
    }
    ///Rising edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP2_A::_0
    }
    ///Rising edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP2_A::_1
    }
}
///Field `CYCP2` writer - Pulse Output Timer 2 Rising Edge Detection Event Output Enable
pub type CYCP2_W<'a, REG> = crate::BitWriter<'a, REG, CYCP2_A>;
impl<'a, REG> CYCP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP2_A::_0)
    }
    ///Rising edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP2_A::_1)
    }
}
/**Pulse Output Timer 3 Rising Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP3_A {
    ///0: Rising edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Rising edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCP3_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCP3` reader - Pulse Output Timer 3 Rising Edge Detection Event Output Enable
pub type CYCP3_R = crate::BitReader<CYCP3_A>;
impl CYCP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCP3_A {
        match self.bits {
            false => CYCP3_A::_0,
            true => CYCP3_A::_1,
        }
    }
    ///Rising edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP3_A::_0
    }
    ///Rising edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP3_A::_1
    }
}
///Field `CYCP3` writer - Pulse Output Timer 3 Rising Edge Detection Event Output Enable
pub type CYCP3_W<'a, REG> = crate::BitWriter<'a, REG, CYCP3_A>;
impl<'a, REG> CYCP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP3_A::_0)
    }
    ///Rising edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP3_A::_1)
    }
}
/**Pulse Output Timer 4 Rising Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP4_A {
    ///0: Rising edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Rising edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCP4_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCP4` reader - Pulse Output Timer 4 Rising Edge Detection Event Output Enable
pub type CYCP4_R = crate::BitReader<CYCP4_A>;
impl CYCP4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCP4_A {
        match self.bits {
            false => CYCP4_A::_0,
            true => CYCP4_A::_1,
        }
    }
    ///Rising edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP4_A::_0
    }
    ///Rising edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP4_A::_1
    }
}
///Field `CYCP4` writer - Pulse Output Timer 4 Rising Edge Detection Event Output Enable
pub type CYCP4_W<'a, REG> = crate::BitWriter<'a, REG, CYCP4_A>;
impl<'a, REG> CYCP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP4_A::_0)
    }
    ///Rising edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP4_A::_1)
    }
}
/**Pulse Output Timer 5 Rising Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCP5_A {
    ///0: Rising edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Rising edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCP5_A> for bool {
    #[inline(always)]
    fn from(variant: CYCP5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCP5` reader - Pulse Output Timer 5 Rising Edge Detection Event Output Enable
pub type CYCP5_R = crate::BitReader<CYCP5_A>;
impl CYCP5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCP5_A {
        match self.bits {
            false => CYCP5_A::_0,
            true => CYCP5_A::_1,
        }
    }
    ///Rising edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCP5_A::_0
    }
    ///Rising edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCP5_A::_1
    }
}
///Field `CYCP5` writer - Pulse Output Timer 5 Rising Edge Detection Event Output Enable
pub type CYCP5_W<'a, REG> = crate::BitWriter<'a, REG, CYCP5_A>;
impl<'a, REG> CYCP5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rising edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP5_A::_0)
    }
    ///Rising edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCP5_A::_1)
    }
}
/**Pulse Output Timer 0 Falling Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN0_A {
    ///0: Falling edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Falling edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCN0_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCN0` reader - Pulse Output Timer 0 Falling Edge Detection Event Output Enable
pub type CYCN0_R = crate::BitReader<CYCN0_A>;
impl CYCN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCN0_A {
        match self.bits {
            false => CYCN0_A::_0,
            true => CYCN0_A::_1,
        }
    }
    ///Falling edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN0_A::_0
    }
    ///Falling edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN0_A::_1
    }
}
///Field `CYCN0` writer - Pulse Output Timer 0 Falling Edge Detection Event Output Enable
pub type CYCN0_W<'a, REG> = crate::BitWriter<'a, REG, CYCN0_A>;
impl<'a, REG> CYCN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edges of the signal from pulse output timer 0 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN0_A::_0)
    }
    ///Falling edges of the signal from pulse output timer 0 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN0_A::_1)
    }
}
/**Pulse Output Timer 1 Falling Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN1_A {
    ///0: Falling edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Falling edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCN1_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCN1` reader - Pulse Output Timer 1 Falling Edge Detection Event Output Enable
pub type CYCN1_R = crate::BitReader<CYCN1_A>;
impl CYCN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCN1_A {
        match self.bits {
            false => CYCN1_A::_0,
            true => CYCN1_A::_1,
        }
    }
    ///Falling edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN1_A::_0
    }
    ///Falling edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN1_A::_1
    }
}
///Field `CYCN1` writer - Pulse Output Timer 1 Falling Edge Detection Event Output Enable
pub type CYCN1_W<'a, REG> = crate::BitWriter<'a, REG, CYCN1_A>;
impl<'a, REG> CYCN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edges of the signal from pulse output timer 1 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN1_A::_0)
    }
    ///Falling edges of the signal from pulse output timer 1 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN1_A::_1)
    }
}
/**Pulse Output Timer 2 Falling Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN2_A {
    ///0: Falling edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Falling edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCN2_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCN2` reader - Pulse Output Timer 2 Falling Edge Detection Event Output Enable
pub type CYCN2_R = crate::BitReader<CYCN2_A>;
impl CYCN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCN2_A {
        match self.bits {
            false => CYCN2_A::_0,
            true => CYCN2_A::_1,
        }
    }
    ///Falling edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN2_A::_0
    }
    ///Falling edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN2_A::_1
    }
}
///Field `CYCN2` writer - Pulse Output Timer 2 Falling Edge Detection Event Output Enable
pub type CYCN2_W<'a, REG> = crate::BitWriter<'a, REG, CYCN2_A>;
impl<'a, REG> CYCN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edges of the signal from pulse output timer 2 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN2_A::_0)
    }
    ///Falling edges of the signal from pulse output timer 2 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN2_A::_1)
    }
}
/**Pulse Output Timer 3 Falling Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN3_A {
    ///0: Falling edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Falling edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCN3_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCN3` reader - Pulse Output Timer 3 Falling Edge Detection Event Output Enable
pub type CYCN3_R = crate::BitReader<CYCN3_A>;
impl CYCN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCN3_A {
        match self.bits {
            false => CYCN3_A::_0,
            true => CYCN3_A::_1,
        }
    }
    ///Falling edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN3_A::_0
    }
    ///Falling edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN3_A::_1
    }
}
///Field `CYCN3` writer - Pulse Output Timer 3 Falling Edge Detection Event Output Enable
pub type CYCN3_W<'a, REG> = crate::BitWriter<'a, REG, CYCN3_A>;
impl<'a, REG> CYCN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edges of the signal from pulse output timer 3 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN3_A::_0)
    }
    ///Falling edges of the signal from pulse output timer 3 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN3_A::_1)
    }
}
/**Pulse Output Timer 4 Falling Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN4_A {
    ///0: Falling edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Falling edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCN4_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCN4` reader - Pulse Output Timer 4 Falling Edge Detection Event Output Enable
pub type CYCN4_R = crate::BitReader<CYCN4_A>;
impl CYCN4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCN4_A {
        match self.bits {
            false => CYCN4_A::_0,
            true => CYCN4_A::_1,
        }
    }
    ///Falling edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN4_A::_0
    }
    ///Falling edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN4_A::_1
    }
}
///Field `CYCN4` writer - Pulse Output Timer 4 Falling Edge Detection Event Output Enable
pub type CYCN4_W<'a, REG> = crate::BitWriter<'a, REG, CYCN4_A>;
impl<'a, REG> CYCN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edges of the signal from pulse output timer 4 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN4_A::_0)
    }
    ///Falling edges of the signal from pulse output timer 4 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN4_A::_1)
    }
}
/**Pulse Output Timer 5 Falling Edge Detection Event Output Enable

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCN5_A {
    ///0: Falling edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals.
    _0 = 0,
    ///1: Falling edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals.
    _1 = 1,
}
impl From<CYCN5_A> for bool {
    #[inline(always)]
    fn from(variant: CYCN5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CYCN5` reader - Pulse Output Timer 5 Falling Edge Detection Event Output Enable
pub type CYCN5_R = crate::BitReader<CYCN5_A>;
impl CYCN5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CYCN5_A {
        match self.bits {
            false => CYCN5_A::_0,
            true => CYCN5_A::_1,
        }
    }
    ///Falling edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CYCN5_A::_0
    }
    ///Falling edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CYCN5_A::_1
    }
}
///Field `CYCN5` writer - Pulse Output Timer 5 Falling Edge Detection Event Output Enable
pub type CYCN5_W<'a, REG> = crate::BitWriter<'a, REG, CYCN5_A>;
impl<'a, REG> CYCN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edges of the signal from pulse output timer 5 are not conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN5_A::_0)
    }
    ///Falling edges of the signal from pulse output timer 5 are conveyed to the ELC as event signals.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CYCN5_A::_1)
    }
}
/**Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSP_A {
    ///0: Prohibits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer.
    _0 = 0,
    ///1: Permits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer.
    _1 = 1,
}
impl From<PLSP_A> for bool {
    #[inline(always)]
    fn from(variant: PLSP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLSP` reader - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission
pub type PLSP_R = crate::BitReader<PLSP_A>;
impl PLSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLSP_A {
        match self.bits {
            false => PLSP_A::_0,
            true => PLSP_A::_1,
        }
    }
    ///Prohibits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLSP_A::_0
    }
    ///Permits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLSP_A::_1
    }
}
///Field `PLSP` writer - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission
pub type PLSP_W<'a, REG> = crate::BitWriter<'a, REG, PLSP_A>;
impl<'a, REG> PLSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PLSP_A::_0)
    }
    ///Permits IPLS interrupt requests due to rising edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PLSP_A::_1)
    }
}
/**Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLSN_A {
    ///0: Prohibits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer.
    _0 = 0,
    ///1: Permits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer.
    _1 = 1,
}
impl From<PLSN_A> for bool {
    #[inline(always)]
    fn from(variant: PLSN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLSN` reader - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission
pub type PLSN_R = crate::BitReader<PLSN_A>;
impl PLSN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLSN_A {
        match self.bits {
            false => PLSN_A::_0,
            true => PLSN_A::_1,
        }
    }
    ///Prohibits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLSN_A::_0
    }
    ///Permits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLSN_A::_1
    }
}
///Field `PLSN` writer - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission
pub type PLSN_W<'a, REG> = crate::BitWriter<'a, REG, PLSN_A>;
impl<'a, REG> PLSN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prohibits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PLSN_A::_0)
    }
    ///Permits IPLS interrupt requests due to falling edges of signals from the selected pulse output timer.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PLSN_A::_1)
    }
}
impl R {
    ///Bit 0 - Pulse Output Timer 0 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp0(&self) -> CYCP0_R {
        CYCP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp1(&self) -> CYCP1_R {
        CYCP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pulse Output Timer 2 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp2(&self) -> CYCP2_R {
        CYCP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pulse Output Timer 3 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp3(&self) -> CYCP3_R {
        CYCP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pulse Output Timer 4 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp4(&self) -> CYCP4_R {
        CYCP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pulse Output Timer 5 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp5(&self) -> CYCP5_R {
        CYCP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Pulse Output Timer 0 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn0(&self) -> CYCN0_R {
        CYCN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pulse Output Timer 1 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn1(&self) -> CYCN1_R {
        CYCN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pulse Output Timer 2 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn2(&self) -> CYCN2_R {
        CYCN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pulse Output Timer 3 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn3(&self) -> CYCN3_R {
        CYCN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pulse Output Timer 4 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn4(&self) -> CYCN4_R {
        CYCN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pulse Output Timer 5 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn5(&self) -> CYCN5_R {
        CYCN5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission
    #[inline(always)]
    pub fn plsp(&self) -> PLSP_R {
        PLSP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission
    #[inline(always)]
    pub fn plsn(&self) -> PLSN_R {
        PLSN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pulse Output Timer 0 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp0(&mut self) -> CYCP0_W<ELIPPR_SPEC> {
        CYCP0_W::new(self, 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp1(&mut self) -> CYCP1_W<ELIPPR_SPEC> {
        CYCP1_W::new(self, 1)
    }
    ///Bit 2 - Pulse Output Timer 2 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp2(&mut self) -> CYCP2_W<ELIPPR_SPEC> {
        CYCP2_W::new(self, 2)
    }
    ///Bit 3 - Pulse Output Timer 3 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp3(&mut self) -> CYCP3_W<ELIPPR_SPEC> {
        CYCP3_W::new(self, 3)
    }
    ///Bit 4 - Pulse Output Timer 4 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp4(&mut self) -> CYCP4_W<ELIPPR_SPEC> {
        CYCP4_W::new(self, 4)
    }
    ///Bit 5 - Pulse Output Timer 5 Rising Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycp5(&mut self) -> CYCP5_W<ELIPPR_SPEC> {
        CYCP5_W::new(self, 5)
    }
    ///Bit 8 - Pulse Output Timer 0 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn0(&mut self) -> CYCN0_W<ELIPPR_SPEC> {
        CYCN0_W::new(self, 8)
    }
    ///Bit 9 - Pulse Output Timer 1 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn1(&mut self) -> CYCN1_W<ELIPPR_SPEC> {
        CYCN1_W::new(self, 9)
    }
    ///Bit 10 - Pulse Output Timer 2 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn2(&mut self) -> CYCN2_W<ELIPPR_SPEC> {
        CYCN2_W::new(self, 10)
    }
    ///Bit 11 - Pulse Output Timer 3 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn3(&mut self) -> CYCN3_W<ELIPPR_SPEC> {
        CYCN3_W::new(self, 11)
    }
    ///Bit 12 - Pulse Output Timer 4 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn4(&mut self) -> CYCN4_W<ELIPPR_SPEC> {
        CYCN4_W::new(self, 12)
    }
    ///Bit 13 - Pulse Output Timer 5 Falling Edge Detection Event Output Enable
    #[inline(always)]
    pub fn cycn5(&mut self) -> CYCN5_W<ELIPPR_SPEC> {
        CYCN5_W::new(self, 13)
    }
    ///Bit 16 - Pulse Output Timer Rising Edge Detection IPLS Interrupt Request Permission
    #[inline(always)]
    pub fn plsp(&mut self) -> PLSP_W<ELIPPR_SPEC> {
        PLSP_W::new(self, 16)
    }
    ///Bit 24 - Pulse Output Timer Falling Edge Detection IPLS Interrupt Request Permission
    #[inline(always)]
    pub fn plsn(&mut self) -> PLSN_W<ELIPPR_SPEC> {
        PLSN_W::new(self, 24)
    }
}
/**ELC Output/IPLS Interrupt Request Permission Register

You can [`read`](crate::Reg::read) this register and get [`elippr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`elippr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ELIPPR_SPEC;
impl crate::RegisterSpec for ELIPPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`elippr::R`](R) reader structure
impl crate::Readable for ELIPPR_SPEC {}
///`write(|w| ..)` method takes [`elippr::W`](W) writer structure
impl crate::Writable for ELIPPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELIPPR to value 0x3f3f
impl crate::Resettable for ELIPPR_SPEC {
    const RESET_VALUE: u32 = 0x3f3f;
}
