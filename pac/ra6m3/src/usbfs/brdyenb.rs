///Register `BRDYENB` reader
pub type R = crate::R<BRDYENB_SPEC>;
///Register `BRDYENB` writer
pub type W = crate::W<BRDYENB_SPEC>;
/**BRDY Interrupt Enable for PIPE0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE0BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE0BRDYE` reader - BRDY Interrupt Enable for PIPE0
pub type PIPE0BRDYE_R = crate::BitReader<PIPE0BRDYE_A>;
impl PIPE0BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0BRDYE_A {
        match self.bits {
            false => PIPE0BRDYE_A::_0,
            true => PIPE0BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BRDYE_A::_1
    }
}
///Field `PIPE0BRDYE` writer - BRDY Interrupt Enable for PIPE0
pub type PIPE0BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE0BRDYE_A>;
impl<'a, REG> PIPE0BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE1BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE1BRDYE` reader - BRDY Interrupt Enable for PIPE1
pub type PIPE1BRDYE_R = crate::BitReader<PIPE1BRDYE_A>;
impl PIPE1BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1BRDYE_A {
        match self.bits {
            false => PIPE1BRDYE_A::_0,
            true => PIPE1BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BRDYE_A::_1
    }
}
///Field `PIPE1BRDYE` writer - BRDY Interrupt Enable for PIPE1
pub type PIPE1BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE1BRDYE_A>;
impl<'a, REG> PIPE1BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE2BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE2BRDYE` reader - BRDY Interrupt Enable for PIPE2
pub type PIPE2BRDYE_R = crate::BitReader<PIPE2BRDYE_A>;
impl PIPE2BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2BRDYE_A {
        match self.bits {
            false => PIPE2BRDYE_A::_0,
            true => PIPE2BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BRDYE_A::_1
    }
}
///Field `PIPE2BRDYE` writer - BRDY Interrupt Enable for PIPE2
pub type PIPE2BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE2BRDYE_A>;
impl<'a, REG> PIPE2BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE3BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE3BRDYE` reader - BRDY Interrupt Enable for PIPE3
pub type PIPE3BRDYE_R = crate::BitReader<PIPE3BRDYE_A>;
impl PIPE3BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3BRDYE_A {
        match self.bits {
            false => PIPE3BRDYE_A::_0,
            true => PIPE3BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BRDYE_A::_1
    }
}
///Field `PIPE3BRDYE` writer - BRDY Interrupt Enable for PIPE3
pub type PIPE3BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE3BRDYE_A>;
impl<'a, REG> PIPE3BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE4BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE4BRDYE` reader - BRDY Interrupt Enable for PIPE4
pub type PIPE4BRDYE_R = crate::BitReader<PIPE4BRDYE_A>;
impl PIPE4BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4BRDYE_A {
        match self.bits {
            false => PIPE4BRDYE_A::_0,
            true => PIPE4BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BRDYE_A::_1
    }
}
///Field `PIPE4BRDYE` writer - BRDY Interrupt Enable for PIPE4
pub type PIPE4BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE4BRDYE_A>;
impl<'a, REG> PIPE4BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE5BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE5BRDYE` reader - BRDY Interrupt Enable for PIPE5
pub type PIPE5BRDYE_R = crate::BitReader<PIPE5BRDYE_A>;
impl PIPE5BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5BRDYE_A {
        match self.bits {
            false => PIPE5BRDYE_A::_0,
            true => PIPE5BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BRDYE_A::_1
    }
}
///Field `PIPE5BRDYE` writer - BRDY Interrupt Enable for PIPE5
pub type PIPE5BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE5BRDYE_A>;
impl<'a, REG> PIPE5BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE6BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE6BRDYE` reader - BRDY Interrupt Enable for PIPE6
pub type PIPE6BRDYE_R = crate::BitReader<PIPE6BRDYE_A>;
impl PIPE6BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6BRDYE_A {
        match self.bits {
            false => PIPE6BRDYE_A::_0,
            true => PIPE6BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BRDYE_A::_1
    }
}
///Field `PIPE6BRDYE` writer - BRDY Interrupt Enable for PIPE6
pub type PIPE6BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE6BRDYE_A>;
impl<'a, REG> PIPE6BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE7BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE7BRDYE` reader - BRDY Interrupt Enable for PIPE7
pub type PIPE7BRDYE_R = crate::BitReader<PIPE7BRDYE_A>;
impl PIPE7BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7BRDYE_A {
        match self.bits {
            false => PIPE7BRDYE_A::_0,
            true => PIPE7BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BRDYE_A::_1
    }
}
///Field `PIPE7BRDYE` writer - BRDY Interrupt Enable for PIPE7
pub type PIPE7BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE7BRDYE_A>;
impl<'a, REG> PIPE7BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE8BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE8BRDYE` reader - BRDY Interrupt Enable for PIPE8
pub type PIPE8BRDYE_R = crate::BitReader<PIPE8BRDYE_A>;
impl PIPE8BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8BRDYE_A {
        match self.bits {
            false => PIPE8BRDYE_A::_0,
            true => PIPE8BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BRDYE_A::_1
    }
}
///Field `PIPE8BRDYE` writer - BRDY Interrupt Enable for PIPE8
pub type PIPE8BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE8BRDYE_A>;
impl<'a, REG> PIPE8BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BRDYE_A::_1)
    }
}
/**BRDY Interrupt Enable for PIPE9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE9BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE9BRDYE` reader - BRDY Interrupt Enable for PIPE9
pub type PIPE9BRDYE_R = crate::BitReader<PIPE9BRDYE_A>;
impl PIPE9BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9BRDYE_A {
        match self.bits {
            false => PIPE9BRDYE_A::_0,
            true => PIPE9BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BRDYE_A::_1
    }
}
///Field `PIPE9BRDYE` writer - BRDY Interrupt Enable for PIPE9
pub type PIPE9BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE9BRDYE_A>;
impl<'a, REG> PIPE9BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BRDYE_A::_1)
    }
}
impl R {
    ///Bit 0 - BRDY Interrupt Enable for PIPE0
    #[inline(always)]
    pub fn pipe0brdye(&self) -> PIPE0BRDYE_R {
        PIPE0BRDYE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRDY Interrupt Enable for PIPE1
    #[inline(always)]
    pub fn pipe1brdye(&self) -> PIPE1BRDYE_R {
        PIPE1BRDYE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRDY Interrupt Enable for PIPE2
    #[inline(always)]
    pub fn pipe2brdye(&self) -> PIPE2BRDYE_R {
        PIPE2BRDYE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRDY Interrupt Enable for PIPE3
    #[inline(always)]
    pub fn pipe3brdye(&self) -> PIPE3BRDYE_R {
        PIPE3BRDYE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BRDY Interrupt Enable for PIPE4
    #[inline(always)]
    pub fn pipe4brdye(&self) -> PIPE4BRDYE_R {
        PIPE4BRDYE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BRDY Interrupt Enable for PIPE5
    #[inline(always)]
    pub fn pipe5brdye(&self) -> PIPE5BRDYE_R {
        PIPE5BRDYE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BRDY Interrupt Enable for PIPE6
    #[inline(always)]
    pub fn pipe6brdye(&self) -> PIPE6BRDYE_R {
        PIPE6BRDYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BRDY Interrupt Enable for PIPE7
    #[inline(always)]
    pub fn pipe7brdye(&self) -> PIPE7BRDYE_R {
        PIPE7BRDYE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BRDY Interrupt Enable for PIPE8
    #[inline(always)]
    pub fn pipe8brdye(&self) -> PIPE8BRDYE_R {
        PIPE8BRDYE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRDY Interrupt Enable for PIPE9
    #[inline(always)]
    pub fn pipe9brdye(&self) -> PIPE9BRDYE_R {
        PIPE9BRDYE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BRDY Interrupt Enable for PIPE0
    #[inline(always)]
    pub fn pipe0brdye(&mut self) -> PIPE0BRDYE_W<BRDYENB_SPEC> {
        PIPE0BRDYE_W::new(self, 0)
    }
    ///Bit 1 - BRDY Interrupt Enable for PIPE1
    #[inline(always)]
    pub fn pipe1brdye(&mut self) -> PIPE1BRDYE_W<BRDYENB_SPEC> {
        PIPE1BRDYE_W::new(self, 1)
    }
    ///Bit 2 - BRDY Interrupt Enable for PIPE2
    #[inline(always)]
    pub fn pipe2brdye(&mut self) -> PIPE2BRDYE_W<BRDYENB_SPEC> {
        PIPE2BRDYE_W::new(self, 2)
    }
    ///Bit 3 - BRDY Interrupt Enable for PIPE3
    #[inline(always)]
    pub fn pipe3brdye(&mut self) -> PIPE3BRDYE_W<BRDYENB_SPEC> {
        PIPE3BRDYE_W::new(self, 3)
    }
    ///Bit 4 - BRDY Interrupt Enable for PIPE4
    #[inline(always)]
    pub fn pipe4brdye(&mut self) -> PIPE4BRDYE_W<BRDYENB_SPEC> {
        PIPE4BRDYE_W::new(self, 4)
    }
    ///Bit 5 - BRDY Interrupt Enable for PIPE5
    #[inline(always)]
    pub fn pipe5brdye(&mut self) -> PIPE5BRDYE_W<BRDYENB_SPEC> {
        PIPE5BRDYE_W::new(self, 5)
    }
    ///Bit 6 - BRDY Interrupt Enable for PIPE6
    #[inline(always)]
    pub fn pipe6brdye(&mut self) -> PIPE6BRDYE_W<BRDYENB_SPEC> {
        PIPE6BRDYE_W::new(self, 6)
    }
    ///Bit 7 - BRDY Interrupt Enable for PIPE7
    #[inline(always)]
    pub fn pipe7brdye(&mut self) -> PIPE7BRDYE_W<BRDYENB_SPEC> {
        PIPE7BRDYE_W::new(self, 7)
    }
    ///Bit 8 - BRDY Interrupt Enable for PIPE8
    #[inline(always)]
    pub fn pipe8brdye(&mut self) -> PIPE8BRDYE_W<BRDYENB_SPEC> {
        PIPE8BRDYE_W::new(self, 8)
    }
    ///Bit 9 - BRDY Interrupt Enable for PIPE9
    #[inline(always)]
    pub fn pipe9brdye(&mut self) -> PIPE9BRDYE_W<BRDYENB_SPEC> {
        PIPE9BRDYE_W::new(self, 9)
    }
}
/**BRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`brdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRDYENB_SPEC;
impl crate::RegisterSpec for BRDYENB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`brdyenb::R`](R) reader structure
impl crate::Readable for BRDYENB_SPEC {}
///`write(|w| ..)` method takes [`brdyenb::W`](W) writer structure
impl crate::Writable for BRDYENB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRDYENB to value 0
impl crate::Resettable for BRDYENB_SPEC {}
