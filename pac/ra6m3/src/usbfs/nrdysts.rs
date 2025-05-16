///Register `NRDYSTS` reader
pub type R = crate::R<NRDYSTS_SPEC>;
///Register `NRDYSTS` writer
pub type W = crate::W<NRDYSTS_SPEC>;
/**NRDY Interrupt Status for PIPE0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE0NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE0NRDY` reader - NRDY Interrupt Status for PIPE0

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE0NRDY_R = crate::BitReader<PIPE0NRDY_A>;
impl PIPE0NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0NRDY_A {
        match self.bits {
            false => PIPE0NRDY_A::_0,
            true => PIPE0NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0NRDY_A::_1
    }
}
///Field `PIPE0NRDY` writer - NRDY Interrupt Status for PIPE0
pub type PIPE0NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE0NRDY_A>;
impl<'a, REG> PIPE0NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE1NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE1NRDY` reader - NRDY Interrupt Status for PIPE1

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE1NRDY_R = crate::BitReader<PIPE1NRDY_A>;
impl PIPE1NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1NRDY_A {
        match self.bits {
            false => PIPE1NRDY_A::_0,
            true => PIPE1NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1NRDY_A::_1
    }
}
///Field `PIPE1NRDY` writer - NRDY Interrupt Status for PIPE1
pub type PIPE1NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE1NRDY_A>;
impl<'a, REG> PIPE1NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE2NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE2NRDY` reader - NRDY Interrupt Status for PIPE2

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE2NRDY_R = crate::BitReader<PIPE2NRDY_A>;
impl PIPE2NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2NRDY_A {
        match self.bits {
            false => PIPE2NRDY_A::_0,
            true => PIPE2NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2NRDY_A::_1
    }
}
///Field `PIPE2NRDY` writer - NRDY Interrupt Status for PIPE2
pub type PIPE2NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE2NRDY_A>;
impl<'a, REG> PIPE2NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE3NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE3NRDY` reader - NRDY Interrupt Status for PIPE3

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE3NRDY_R = crate::BitReader<PIPE3NRDY_A>;
impl PIPE3NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3NRDY_A {
        match self.bits {
            false => PIPE3NRDY_A::_0,
            true => PIPE3NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3NRDY_A::_1
    }
}
///Field `PIPE3NRDY` writer - NRDY Interrupt Status for PIPE3
pub type PIPE3NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE3NRDY_A>;
impl<'a, REG> PIPE3NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE4NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE4NRDY` reader - NRDY Interrupt Status for PIPE4

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE4NRDY_R = crate::BitReader<PIPE4NRDY_A>;
impl PIPE4NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4NRDY_A {
        match self.bits {
            false => PIPE4NRDY_A::_0,
            true => PIPE4NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4NRDY_A::_1
    }
}
///Field `PIPE4NRDY` writer - NRDY Interrupt Status for PIPE4
pub type PIPE4NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE4NRDY_A>;
impl<'a, REG> PIPE4NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE5NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE5NRDY` reader - NRDY Interrupt Status for PIPE5

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE5NRDY_R = crate::BitReader<PIPE5NRDY_A>;
impl PIPE5NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5NRDY_A {
        match self.bits {
            false => PIPE5NRDY_A::_0,
            true => PIPE5NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5NRDY_A::_1
    }
}
///Field `PIPE5NRDY` writer - NRDY Interrupt Status for PIPE5
pub type PIPE5NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE5NRDY_A>;
impl<'a, REG> PIPE5NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE6NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE6NRDY` reader - NRDY Interrupt Status for PIPE6

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE6NRDY_R = crate::BitReader<PIPE6NRDY_A>;
impl PIPE6NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6NRDY_A {
        match self.bits {
            false => PIPE6NRDY_A::_0,
            true => PIPE6NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6NRDY_A::_1
    }
}
///Field `PIPE6NRDY` writer - NRDY Interrupt Status for PIPE6
pub type PIPE6NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE6NRDY_A>;
impl<'a, REG> PIPE6NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE7NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE7NRDY` reader - NRDY Interrupt Status for PIPE7

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE7NRDY_R = crate::BitReader<PIPE7NRDY_A>;
impl PIPE7NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7NRDY_A {
        match self.bits {
            false => PIPE7NRDY_A::_0,
            true => PIPE7NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7NRDY_A::_1
    }
}
///Field `PIPE7NRDY` writer - NRDY Interrupt Status for PIPE7
pub type PIPE7NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE7NRDY_A>;
impl<'a, REG> PIPE7NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE8NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE8NRDY` reader - NRDY Interrupt Status for PIPE8

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE8NRDY_R = crate::BitReader<PIPE8NRDY_A>;
impl PIPE8NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8NRDY_A {
        match self.bits {
            false => PIPE8NRDY_A::_0,
            true => PIPE8NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8NRDY_A::_1
    }
}
///Field `PIPE8NRDY` writer - NRDY Interrupt Status for PIPE8
pub type PIPE8NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE8NRDY_A>;
impl<'a, REG> PIPE8NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8NRDY_A::_1)
    }
}
/**NRDY Interrupt Status for PIPE9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9NRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE9NRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9NRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE9NRDY` reader - NRDY Interrupt Status for PIPE9

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE9NRDY_R = crate::BitReader<PIPE9NRDY_A>;
impl PIPE9NRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9NRDY_A {
        match self.bits {
            false => PIPE9NRDY_A::_0,
            true => PIPE9NRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9NRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9NRDY_A::_1
    }
}
///Field `PIPE9NRDY` writer - NRDY Interrupt Status for PIPE9
pub type PIPE9NRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE9NRDY_A>;
impl<'a, REG> PIPE9NRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9NRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9NRDY_A::_1)
    }
}
impl R {
    ///Bit 0 - NRDY Interrupt Status for PIPE0
    #[inline(always)]
    pub fn pipe0nrdy(&self) -> PIPE0NRDY_R {
        PIPE0NRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NRDY Interrupt Status for PIPE1
    #[inline(always)]
    pub fn pipe1nrdy(&self) -> PIPE1NRDY_R {
        PIPE1NRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NRDY Interrupt Status for PIPE2
    #[inline(always)]
    pub fn pipe2nrdy(&self) -> PIPE2NRDY_R {
        PIPE2NRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NRDY Interrupt Status for PIPE3
    #[inline(always)]
    pub fn pipe3nrdy(&self) -> PIPE3NRDY_R {
        PIPE3NRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NRDY Interrupt Status for PIPE4
    #[inline(always)]
    pub fn pipe4nrdy(&self) -> PIPE4NRDY_R {
        PIPE4NRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NRDY Interrupt Status for PIPE5
    #[inline(always)]
    pub fn pipe5nrdy(&self) -> PIPE5NRDY_R {
        PIPE5NRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NRDY Interrupt Status for PIPE6
    #[inline(always)]
    pub fn pipe6nrdy(&self) -> PIPE6NRDY_R {
        PIPE6NRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - NRDY Interrupt Status for PIPE7
    #[inline(always)]
    pub fn pipe7nrdy(&self) -> PIPE7NRDY_R {
        PIPE7NRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - NRDY Interrupt Status for PIPE8
    #[inline(always)]
    pub fn pipe8nrdy(&self) -> PIPE8NRDY_R {
        PIPE8NRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NRDY Interrupt Status for PIPE9
    #[inline(always)]
    pub fn pipe9nrdy(&self) -> PIPE9NRDY_R {
        PIPE9NRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - NRDY Interrupt Status for PIPE0
    #[inline(always)]
    pub fn pipe0nrdy(&mut self) -> PIPE0NRDY_W<NRDYSTS_SPEC> {
        PIPE0NRDY_W::new(self, 0)
    }
    ///Bit 1 - NRDY Interrupt Status for PIPE1
    #[inline(always)]
    pub fn pipe1nrdy(&mut self) -> PIPE1NRDY_W<NRDYSTS_SPEC> {
        PIPE1NRDY_W::new(self, 1)
    }
    ///Bit 2 - NRDY Interrupt Status for PIPE2
    #[inline(always)]
    pub fn pipe2nrdy(&mut self) -> PIPE2NRDY_W<NRDYSTS_SPEC> {
        PIPE2NRDY_W::new(self, 2)
    }
    ///Bit 3 - NRDY Interrupt Status for PIPE3
    #[inline(always)]
    pub fn pipe3nrdy(&mut self) -> PIPE3NRDY_W<NRDYSTS_SPEC> {
        PIPE3NRDY_W::new(self, 3)
    }
    ///Bit 4 - NRDY Interrupt Status for PIPE4
    #[inline(always)]
    pub fn pipe4nrdy(&mut self) -> PIPE4NRDY_W<NRDYSTS_SPEC> {
        PIPE4NRDY_W::new(self, 4)
    }
    ///Bit 5 - NRDY Interrupt Status for PIPE5
    #[inline(always)]
    pub fn pipe5nrdy(&mut self) -> PIPE5NRDY_W<NRDYSTS_SPEC> {
        PIPE5NRDY_W::new(self, 5)
    }
    ///Bit 6 - NRDY Interrupt Status for PIPE6
    #[inline(always)]
    pub fn pipe6nrdy(&mut self) -> PIPE6NRDY_W<NRDYSTS_SPEC> {
        PIPE6NRDY_W::new(self, 6)
    }
    ///Bit 7 - NRDY Interrupt Status for PIPE7
    #[inline(always)]
    pub fn pipe7nrdy(&mut self) -> PIPE7NRDY_W<NRDYSTS_SPEC> {
        PIPE7NRDY_W::new(self, 7)
    }
    ///Bit 8 - NRDY Interrupt Status for PIPE8
    #[inline(always)]
    pub fn pipe8nrdy(&mut self) -> PIPE8NRDY_W<NRDYSTS_SPEC> {
        PIPE8NRDY_W::new(self, 8)
    }
    ///Bit 9 - NRDY Interrupt Status for PIPE9
    #[inline(always)]
    pub fn pipe9nrdy(&mut self) -> PIPE9NRDY_W<NRDYSTS_SPEC> {
        PIPE9NRDY_W::new(self, 9)
    }
}
/**NRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nrdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct NRDYSTS_SPEC;
impl crate::RegisterSpec for NRDYSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`nrdysts::R`](R) reader structure
impl crate::Readable for NRDYSTS_SPEC {}
///`write(|w| ..)` method takes [`nrdysts::W`](W) writer structure
impl crate::Writable for NRDYSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x03ff;
}
///`reset()` method sets NRDYSTS to value 0
impl crate::Resettable for NRDYSTS_SPEC {}
