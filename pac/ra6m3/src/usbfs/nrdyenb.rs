///Register `NRDYENB` reader
pub type R = crate::R<NRDYENB_SPEC>;
///Register `NRDYENB` writer
pub type W = crate::W<NRDYENB_SPEC>;
/**NRDY Interrupt Enable for PIPE0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE0NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE0NRDYE` reader - NRDY Interrupt Enable for PIPE0
pub type PIPE0NRDYE_R = crate::BitReader<PIPE0NRDYE_A>;
impl PIPE0NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0NRDYE_A {
        match self.bits {
            false => PIPE0NRDYE_A::_0,
            true => PIPE0NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0NRDYE_A::_1
    }
}
///Field `PIPE0NRDYE` writer - NRDY Interrupt Enable for PIPE0
pub type PIPE0NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE0NRDYE_A>;
impl<'a, REG> PIPE0NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE1NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE1NRDYE` reader - NRDY Interrupt Enable for PIPE1
pub type PIPE1NRDYE_R = crate::BitReader<PIPE1NRDYE_A>;
impl PIPE1NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1NRDYE_A {
        match self.bits {
            false => PIPE1NRDYE_A::_0,
            true => PIPE1NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1NRDYE_A::_1
    }
}
///Field `PIPE1NRDYE` writer - NRDY Interrupt Enable for PIPE1
pub type PIPE1NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE1NRDYE_A>;
impl<'a, REG> PIPE1NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE2NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE2NRDYE` reader - NRDY Interrupt Enable for PIPE2
pub type PIPE2NRDYE_R = crate::BitReader<PIPE2NRDYE_A>;
impl PIPE2NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2NRDYE_A {
        match self.bits {
            false => PIPE2NRDYE_A::_0,
            true => PIPE2NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2NRDYE_A::_1
    }
}
///Field `PIPE2NRDYE` writer - NRDY Interrupt Enable for PIPE2
pub type PIPE2NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE2NRDYE_A>;
impl<'a, REG> PIPE2NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE3NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE3NRDYE` reader - NRDY Interrupt Enable for PIPE3
pub type PIPE3NRDYE_R = crate::BitReader<PIPE3NRDYE_A>;
impl PIPE3NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3NRDYE_A {
        match self.bits {
            false => PIPE3NRDYE_A::_0,
            true => PIPE3NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3NRDYE_A::_1
    }
}
///Field `PIPE3NRDYE` writer - NRDY Interrupt Enable for PIPE3
pub type PIPE3NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE3NRDYE_A>;
impl<'a, REG> PIPE3NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE4NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE4NRDYE` reader - NRDY Interrupt Enable for PIPE4
pub type PIPE4NRDYE_R = crate::BitReader<PIPE4NRDYE_A>;
impl PIPE4NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4NRDYE_A {
        match self.bits {
            false => PIPE4NRDYE_A::_0,
            true => PIPE4NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4NRDYE_A::_1
    }
}
///Field `PIPE4NRDYE` writer - NRDY Interrupt Enable for PIPE4
pub type PIPE4NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE4NRDYE_A>;
impl<'a, REG> PIPE4NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE5NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE5NRDYE` reader - NRDY Interrupt Enable for PIPE5
pub type PIPE5NRDYE_R = crate::BitReader<PIPE5NRDYE_A>;
impl PIPE5NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5NRDYE_A {
        match self.bits {
            false => PIPE5NRDYE_A::_0,
            true => PIPE5NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5NRDYE_A::_1
    }
}
///Field `PIPE5NRDYE` writer - NRDY Interrupt Enable for PIPE5
pub type PIPE5NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE5NRDYE_A>;
impl<'a, REG> PIPE5NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE6NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE6NRDYE` reader - NRDY Interrupt Enable for PIPE6
pub type PIPE6NRDYE_R = crate::BitReader<PIPE6NRDYE_A>;
impl PIPE6NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6NRDYE_A {
        match self.bits {
            false => PIPE6NRDYE_A::_0,
            true => PIPE6NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6NRDYE_A::_1
    }
}
///Field `PIPE6NRDYE` writer - NRDY Interrupt Enable for PIPE6
pub type PIPE6NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE6NRDYE_A>;
impl<'a, REG> PIPE6NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE7NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE7NRDYE` reader - NRDY Interrupt Enable for PIPE7
pub type PIPE7NRDYE_R = crate::BitReader<PIPE7NRDYE_A>;
impl PIPE7NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7NRDYE_A {
        match self.bits {
            false => PIPE7NRDYE_A::_0,
            true => PIPE7NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7NRDYE_A::_1
    }
}
///Field `PIPE7NRDYE` writer - NRDY Interrupt Enable for PIPE7
pub type PIPE7NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE7NRDYE_A>;
impl<'a, REG> PIPE7NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE8NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE8NRDYE` reader - NRDY Interrupt Enable for PIPE8
pub type PIPE8NRDYE_R = crate::BitReader<PIPE8NRDYE_A>;
impl PIPE8NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8NRDYE_A {
        match self.bits {
            false => PIPE8NRDYE_A::_0,
            true => PIPE8NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8NRDYE_A::_1
    }
}
///Field `PIPE8NRDYE` writer - NRDY Interrupt Enable for PIPE8
pub type PIPE8NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE8NRDYE_A>;
impl<'a, REG> PIPE8NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8NRDYE_A::_1)
    }
}
/**NRDY Interrupt Enable for PIPE9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE9NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE9NRDYE` reader - NRDY Interrupt Enable for PIPE9
pub type PIPE9NRDYE_R = crate::BitReader<PIPE9NRDYE_A>;
impl PIPE9NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9NRDYE_A {
        match self.bits {
            false => PIPE9NRDYE_A::_0,
            true => PIPE9NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9NRDYE_A::_1
    }
}
///Field `PIPE9NRDYE` writer - NRDY Interrupt Enable for PIPE9
pub type PIPE9NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE9NRDYE_A>;
impl<'a, REG> PIPE9NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9NRDYE_A::_1)
    }
}
impl R {
    ///Bit 0 - NRDY Interrupt Enable for PIPE0
    #[inline(always)]
    pub fn pipe0nrdye(&self) -> PIPE0NRDYE_R {
        PIPE0NRDYE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NRDY Interrupt Enable for PIPE1
    #[inline(always)]
    pub fn pipe1nrdye(&self) -> PIPE1NRDYE_R {
        PIPE1NRDYE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NRDY Interrupt Enable for PIPE2
    #[inline(always)]
    pub fn pipe2nrdye(&self) -> PIPE2NRDYE_R {
        PIPE2NRDYE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NRDY Interrupt Enable for PIPE3
    #[inline(always)]
    pub fn pipe3nrdye(&self) -> PIPE3NRDYE_R {
        PIPE3NRDYE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NRDY Interrupt Enable for PIPE4
    #[inline(always)]
    pub fn pipe4nrdye(&self) -> PIPE4NRDYE_R {
        PIPE4NRDYE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NRDY Interrupt Enable for PIPE5
    #[inline(always)]
    pub fn pipe5nrdye(&self) -> PIPE5NRDYE_R {
        PIPE5NRDYE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NRDY Interrupt Enable for PIPE6
    #[inline(always)]
    pub fn pipe6nrdye(&self) -> PIPE6NRDYE_R {
        PIPE6NRDYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NRDY Interrupt Enable for PIPE7
    #[inline(always)]
    pub fn pipe7nrdye(&self) -> PIPE7NRDYE_R {
        PIPE7NRDYE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - NRDY Interrupt Enable for PIPE8
    #[inline(always)]
    pub fn pipe8nrdye(&self) -> PIPE8NRDYE_R {
        PIPE8NRDYE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NRDY Interrupt Enable for PIPE9
    #[inline(always)]
    pub fn pipe9nrdye(&self) -> PIPE9NRDYE_R {
        PIPE9NRDYE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - NRDY Interrupt Enable for PIPE0
    #[inline(always)]
    pub fn pipe0nrdye(&mut self) -> PIPE0NRDYE_W<NRDYENB_SPEC> {
        PIPE0NRDYE_W::new(self, 0)
    }
    ///Bit 1 - NRDY Interrupt Enable for PIPE1
    #[inline(always)]
    pub fn pipe1nrdye(&mut self) -> PIPE1NRDYE_W<NRDYENB_SPEC> {
        PIPE1NRDYE_W::new(self, 1)
    }
    ///Bit 2 - NRDY Interrupt Enable for PIPE2
    #[inline(always)]
    pub fn pipe2nrdye(&mut self) -> PIPE2NRDYE_W<NRDYENB_SPEC> {
        PIPE2NRDYE_W::new(self, 2)
    }
    ///Bit 3 - NRDY Interrupt Enable for PIPE3
    #[inline(always)]
    pub fn pipe3nrdye(&mut self) -> PIPE3NRDYE_W<NRDYENB_SPEC> {
        PIPE3NRDYE_W::new(self, 3)
    }
    ///Bit 4 - NRDY Interrupt Enable for PIPE4
    #[inline(always)]
    pub fn pipe4nrdye(&mut self) -> PIPE4NRDYE_W<NRDYENB_SPEC> {
        PIPE4NRDYE_W::new(self, 4)
    }
    ///Bit 5 - NRDY Interrupt Enable for PIPE5
    #[inline(always)]
    pub fn pipe5nrdye(&mut self) -> PIPE5NRDYE_W<NRDYENB_SPEC> {
        PIPE5NRDYE_W::new(self, 5)
    }
    ///Bit 6 - NRDY Interrupt Enable for PIPE6
    #[inline(always)]
    pub fn pipe6nrdye(&mut self) -> PIPE6NRDYE_W<NRDYENB_SPEC> {
        PIPE6NRDYE_W::new(self, 6)
    }
    ///Bit 7 - NRDY Interrupt Enable for PIPE7
    #[inline(always)]
    pub fn pipe7nrdye(&mut self) -> PIPE7NRDYE_W<NRDYENB_SPEC> {
        PIPE7NRDYE_W::new(self, 7)
    }
    ///Bit 8 - NRDY Interrupt Enable for PIPE8
    #[inline(always)]
    pub fn pipe8nrdye(&mut self) -> PIPE8NRDYE_W<NRDYENB_SPEC> {
        PIPE8NRDYE_W::new(self, 8)
    }
    ///Bit 9 - NRDY Interrupt Enable for PIPE9
    #[inline(always)]
    pub fn pipe9nrdye(&mut self) -> PIPE9NRDYE_W<NRDYENB_SPEC> {
        PIPE9NRDYE_W::new(self, 9)
    }
}
/**NRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NRDYENB_SPEC;
impl crate::RegisterSpec for NRDYENB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nrdyenb::R`](R) reader structure
impl crate::Readable for NRDYENB_SPEC {}
///`write(|w| ..)` method takes [`nrdyenb::W`](W) writer structure
impl crate::Writable for NRDYENB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NRDYENB to value 0
impl crate::Resettable for NRDYENB_SPEC {}
