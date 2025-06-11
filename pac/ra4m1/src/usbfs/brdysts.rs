///Register `BRDYSTS` reader
pub type R = crate::R<BRDYSTS_SPEC>;
///Register `BRDYSTS` writer
pub type W = crate::W<BRDYSTS_SPEC>;
/**BRDY Interrupt Status for PIPE0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE0BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE0BRDY` reader - BRDY Interrupt Status for PIPE0

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE0BRDY_R = crate::BitReader<PIPE0BRDY_A>;
impl PIPE0BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0BRDY_A {
        match self.bits {
            false => PIPE0BRDY_A::_0,
            true => PIPE0BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BRDY_A::_1
    }
}
///Field `PIPE0BRDY` writer - BRDY Interrupt Status for PIPE0
pub type PIPE0BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE0BRDY_A>;
impl<'a, REG> PIPE0BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE1BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE1BRDY` reader - BRDY Interrupt Status for PIPE1

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE1BRDY_R = crate::BitReader<PIPE1BRDY_A>;
impl PIPE1BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1BRDY_A {
        match self.bits {
            false => PIPE1BRDY_A::_0,
            true => PIPE1BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BRDY_A::_1
    }
}
///Field `PIPE1BRDY` writer - BRDY Interrupt Status for PIPE1
pub type PIPE1BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE1BRDY_A>;
impl<'a, REG> PIPE1BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE2BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE2BRDY` reader - BRDY Interrupt Status for PIPE2

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE2BRDY_R = crate::BitReader<PIPE2BRDY_A>;
impl PIPE2BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2BRDY_A {
        match self.bits {
            false => PIPE2BRDY_A::_0,
            true => PIPE2BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BRDY_A::_1
    }
}
///Field `PIPE2BRDY` writer - BRDY Interrupt Status for PIPE2
pub type PIPE2BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE2BRDY_A>;
impl<'a, REG> PIPE2BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE3BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE3BRDY` reader - BRDY Interrupt Status for PIPE3

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE3BRDY_R = crate::BitReader<PIPE3BRDY_A>;
impl PIPE3BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3BRDY_A {
        match self.bits {
            false => PIPE3BRDY_A::_0,
            true => PIPE3BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BRDY_A::_1
    }
}
///Field `PIPE3BRDY` writer - BRDY Interrupt Status for PIPE3
pub type PIPE3BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE3BRDY_A>;
impl<'a, REG> PIPE3BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE4BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE4BRDY` reader - BRDY Interrupt Status for PIPE4

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE4BRDY_R = crate::BitReader<PIPE4BRDY_A>;
impl PIPE4BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4BRDY_A {
        match self.bits {
            false => PIPE4BRDY_A::_0,
            true => PIPE4BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BRDY_A::_1
    }
}
///Field `PIPE4BRDY` writer - BRDY Interrupt Status for PIPE4
pub type PIPE4BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE4BRDY_A>;
impl<'a, REG> PIPE4BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE5BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE5BRDY` reader - BRDY Interrupt Status for PIPE5

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE5BRDY_R = crate::BitReader<PIPE5BRDY_A>;
impl PIPE5BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5BRDY_A {
        match self.bits {
            false => PIPE5BRDY_A::_0,
            true => PIPE5BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BRDY_A::_1
    }
}
///Field `PIPE5BRDY` writer - BRDY Interrupt Status for PIPE5
pub type PIPE5BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE5BRDY_A>;
impl<'a, REG> PIPE5BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE6BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE6BRDY` reader - BRDY Interrupt Status for PIPE6

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE6BRDY_R = crate::BitReader<PIPE6BRDY_A>;
impl PIPE6BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6BRDY_A {
        match self.bits {
            false => PIPE6BRDY_A::_0,
            true => PIPE6BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BRDY_A::_1
    }
}
///Field `PIPE6BRDY` writer - BRDY Interrupt Status for PIPE6
pub type PIPE6BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE6BRDY_A>;
impl<'a, REG> PIPE6BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE7BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE7BRDY` reader - BRDY Interrupt Status for PIPE7

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE7BRDY_R = crate::BitReader<PIPE7BRDY_A>;
impl PIPE7BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7BRDY_A {
        match self.bits {
            false => PIPE7BRDY_A::_0,
            true => PIPE7BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BRDY_A::_1
    }
}
///Field `PIPE7BRDY` writer - BRDY Interrupt Status for PIPE7
pub type PIPE7BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE7BRDY_A>;
impl<'a, REG> PIPE7BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE8BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE8BRDY` reader - BRDY Interrupt Status for PIPE8

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE8BRDY_R = crate::BitReader<PIPE8BRDY_A>;
impl PIPE8BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8BRDY_A {
        match self.bits {
            false => PIPE8BRDY_A::_0,
            true => PIPE8BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BRDY_A::_1
    }
}
///Field `PIPE8BRDY` writer - BRDY Interrupt Status for PIPE8
pub type PIPE8BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE8BRDY_A>;
impl<'a, REG> PIPE8BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BRDY_A::_1)
    }
}
/**BRDY Interrupt Status for PIPE9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BRDY_A {
    ///0: Interrupts are not generated.
    _0 = 0,
    ///1: Interrupts are generated.
    _1 = 1,
}
impl From<PIPE9BRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BRDY_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `PIPE9BRDY` reader - BRDY Interrupt Status for PIPE9

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type PIPE9BRDY_R = crate::BitReader<PIPE9BRDY_A>;
impl PIPE9BRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9BRDY_A {
        match self.bits {
            false => PIPE9BRDY_A::_0,
            true => PIPE9BRDY_A::_1,
        }
    }
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BRDY_A::_0
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BRDY_A::_1
    }
}
///Field `PIPE9BRDY` writer - BRDY Interrupt Status for PIPE9
pub type PIPE9BRDY_W<'a, REG> = crate::BitWriter0C<'a, REG, PIPE9BRDY_A>;
impl<'a, REG> PIPE9BRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupts are not generated.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BRDY_A::_0)
    }
    ///Interrupts are generated.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BRDY_A::_1)
    }
}
impl R {
    ///Bit 0 - BRDY Interrupt Status for PIPE0
    #[inline(always)]
    pub fn pipe0brdy(&self) -> PIPE0BRDY_R {
        PIPE0BRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRDY Interrupt Status for PIPE1
    #[inline(always)]
    pub fn pipe1brdy(&self) -> PIPE1BRDY_R {
        PIPE1BRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRDY Interrupt Status for PIPE2
    #[inline(always)]
    pub fn pipe2brdy(&self) -> PIPE2BRDY_R {
        PIPE2BRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRDY Interrupt Status for PIPE3
    #[inline(always)]
    pub fn pipe3brdy(&self) -> PIPE3BRDY_R {
        PIPE3BRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BRDY Interrupt Status for PIPE4
    #[inline(always)]
    pub fn pipe4brdy(&self) -> PIPE4BRDY_R {
        PIPE4BRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BRDY Interrupt Status for PIPE5
    #[inline(always)]
    pub fn pipe5brdy(&self) -> PIPE5BRDY_R {
        PIPE5BRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BRDY Interrupt Status for PIPE6
    #[inline(always)]
    pub fn pipe6brdy(&self) -> PIPE6BRDY_R {
        PIPE6BRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BRDY Interrupt Status for PIPE7
    #[inline(always)]
    pub fn pipe7brdy(&self) -> PIPE7BRDY_R {
        PIPE7BRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BRDY Interrupt Status for PIPE8
    #[inline(always)]
    pub fn pipe8brdy(&self) -> PIPE8BRDY_R {
        PIPE8BRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRDY Interrupt Status for PIPE9
    #[inline(always)]
    pub fn pipe9brdy(&self) -> PIPE9BRDY_R {
        PIPE9BRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BRDY Interrupt Status for PIPE0
    #[inline(always)]
    pub fn pipe0brdy(&mut self) -> PIPE0BRDY_W<BRDYSTS_SPEC> {
        PIPE0BRDY_W::new(self, 0)
    }
    ///Bit 1 - BRDY Interrupt Status for PIPE1
    #[inline(always)]
    pub fn pipe1brdy(&mut self) -> PIPE1BRDY_W<BRDYSTS_SPEC> {
        PIPE1BRDY_W::new(self, 1)
    }
    ///Bit 2 - BRDY Interrupt Status for PIPE2
    #[inline(always)]
    pub fn pipe2brdy(&mut self) -> PIPE2BRDY_W<BRDYSTS_SPEC> {
        PIPE2BRDY_W::new(self, 2)
    }
    ///Bit 3 - BRDY Interrupt Status for PIPE3
    #[inline(always)]
    pub fn pipe3brdy(&mut self) -> PIPE3BRDY_W<BRDYSTS_SPEC> {
        PIPE3BRDY_W::new(self, 3)
    }
    ///Bit 4 - BRDY Interrupt Status for PIPE4
    #[inline(always)]
    pub fn pipe4brdy(&mut self) -> PIPE4BRDY_W<BRDYSTS_SPEC> {
        PIPE4BRDY_W::new(self, 4)
    }
    ///Bit 5 - BRDY Interrupt Status for PIPE5
    #[inline(always)]
    pub fn pipe5brdy(&mut self) -> PIPE5BRDY_W<BRDYSTS_SPEC> {
        PIPE5BRDY_W::new(self, 5)
    }
    ///Bit 6 - BRDY Interrupt Status for PIPE6
    #[inline(always)]
    pub fn pipe6brdy(&mut self) -> PIPE6BRDY_W<BRDYSTS_SPEC> {
        PIPE6BRDY_W::new(self, 6)
    }
    ///Bit 7 - BRDY Interrupt Status for PIPE7
    #[inline(always)]
    pub fn pipe7brdy(&mut self) -> PIPE7BRDY_W<BRDYSTS_SPEC> {
        PIPE7BRDY_W::new(self, 7)
    }
    ///Bit 8 - BRDY Interrupt Status for PIPE8
    #[inline(always)]
    pub fn pipe8brdy(&mut self) -> PIPE8BRDY_W<BRDYSTS_SPEC> {
        PIPE8BRDY_W::new(self, 8)
    }
    ///Bit 9 - BRDY Interrupt Status for PIPE9
    #[inline(always)]
    pub fn pipe9brdy(&mut self) -> PIPE9BRDY_W<BRDYSTS_SPEC> {
        PIPE9BRDY_W::new(self, 9)
    }
}
/**BRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`brdysts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BRDYSTS_SPEC;
impl crate::RegisterSpec for BRDYSTS_SPEC {
    type Ux = u16;
}
///`read()` method returns [`brdysts::R`](R) reader structure
impl crate::Readable for BRDYSTS_SPEC {}
///`write(|w| ..)` method takes [`brdysts::W`](W) writer structure
impl crate::Writable for BRDYSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x03ff;
}
///`reset()` method sets BRDYSTS to value 0
impl crate::Resettable for BRDYSTS_SPEC {}
