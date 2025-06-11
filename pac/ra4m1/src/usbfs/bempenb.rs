///Register `BEMPENB` reader
pub type R = crate::R<BEMPENB_SPEC>;
///Register `BEMPENB` writer
pub type W = crate::W<BEMPENB_SPEC>;
/**BEMP Interrupt Enable for PIPE0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE0BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE0BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE0BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE0BEMPE` reader - BEMP Interrupt Enable for PIPE0
pub type PIPE0BEMPE_R = crate::BitReader<PIPE0BEMPE_A>;
impl PIPE0BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE0BEMPE_A {
        match self.bits {
            false => PIPE0BEMPE_A::_0,
            true => PIPE0BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE0BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE0BEMPE_A::_1
    }
}
///Field `PIPE0BEMPE` writer - BEMP Interrupt Enable for PIPE0
pub type PIPE0BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE0BEMPE_A>;
impl<'a, REG> PIPE0BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE0BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE1BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE1BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE1BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE1BEMPE` reader - BEMP Interrupt Enable for PIPE1
pub type PIPE1BEMPE_R = crate::BitReader<PIPE1BEMPE_A>;
impl PIPE1BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE1BEMPE_A {
        match self.bits {
            false => PIPE1BEMPE_A::_0,
            true => PIPE1BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE1BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE1BEMPE_A::_1
    }
}
///Field `PIPE1BEMPE` writer - BEMP Interrupt Enable for PIPE1
pub type PIPE1BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE1BEMPE_A>;
impl<'a, REG> PIPE1BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE1BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE2BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE2BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE2BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE2BEMPE` reader - BEMP Interrupt Enable for PIPE2
pub type PIPE2BEMPE_R = crate::BitReader<PIPE2BEMPE_A>;
impl PIPE2BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE2BEMPE_A {
        match self.bits {
            false => PIPE2BEMPE_A::_0,
            true => PIPE2BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE2BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE2BEMPE_A::_1
    }
}
///Field `PIPE2BEMPE` writer - BEMP Interrupt Enable for PIPE2
pub type PIPE2BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE2BEMPE_A>;
impl<'a, REG> PIPE2BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE2BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE3BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE3BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE3BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE3BEMPE` reader - BEMP Interrupt Enable for PIPE3
pub type PIPE3BEMPE_R = crate::BitReader<PIPE3BEMPE_A>;
impl PIPE3BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE3BEMPE_A {
        match self.bits {
            false => PIPE3BEMPE_A::_0,
            true => PIPE3BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE3BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE3BEMPE_A::_1
    }
}
///Field `PIPE3BEMPE` writer - BEMP Interrupt Enable for PIPE3
pub type PIPE3BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE3BEMPE_A>;
impl<'a, REG> PIPE3BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE3BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE4

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE4BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE4BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE4BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE4BEMPE` reader - BEMP Interrupt Enable for PIPE4
pub type PIPE4BEMPE_R = crate::BitReader<PIPE4BEMPE_A>;
impl PIPE4BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE4BEMPE_A {
        match self.bits {
            false => PIPE4BEMPE_A::_0,
            true => PIPE4BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE4BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE4BEMPE_A::_1
    }
}
///Field `PIPE4BEMPE` writer - BEMP Interrupt Enable for PIPE4
pub type PIPE4BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE4BEMPE_A>;
impl<'a, REG> PIPE4BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE4BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE5

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE5BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE5BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE5BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE5BEMPE` reader - BEMP Interrupt Enable for PIPE5
pub type PIPE5BEMPE_R = crate::BitReader<PIPE5BEMPE_A>;
impl PIPE5BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE5BEMPE_A {
        match self.bits {
            false => PIPE5BEMPE_A::_0,
            true => PIPE5BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE5BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE5BEMPE_A::_1
    }
}
///Field `PIPE5BEMPE` writer - BEMP Interrupt Enable for PIPE5
pub type PIPE5BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE5BEMPE_A>;
impl<'a, REG> PIPE5BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE5BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE6BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE6BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE6BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE6BEMPE` reader - BEMP Interrupt Enable for PIPE6
pub type PIPE6BEMPE_R = crate::BitReader<PIPE6BEMPE_A>;
impl PIPE6BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE6BEMPE_A {
        match self.bits {
            false => PIPE6BEMPE_A::_0,
            true => PIPE6BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE6BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE6BEMPE_A::_1
    }
}
///Field `PIPE6BEMPE` writer - BEMP Interrupt Enable for PIPE6
pub type PIPE6BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE6BEMPE_A>;
impl<'a, REG> PIPE6BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE6BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE7BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE7BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE7BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE7BEMPE` reader - BEMP Interrupt Enable for PIPE7
pub type PIPE7BEMPE_R = crate::BitReader<PIPE7BEMPE_A>;
impl PIPE7BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE7BEMPE_A {
        match self.bits {
            false => PIPE7BEMPE_A::_0,
            true => PIPE7BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE7BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE7BEMPE_A::_1
    }
}
///Field `PIPE7BEMPE` writer - BEMP Interrupt Enable for PIPE7
pub type PIPE7BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE7BEMPE_A>;
impl<'a, REG> PIPE7BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE7BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE8

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE8BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE8BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE8BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE8BEMPE` reader - BEMP Interrupt Enable for PIPE8
pub type PIPE8BEMPE_R = crate::BitReader<PIPE8BEMPE_A>;
impl PIPE8BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE8BEMPE_A {
        match self.bits {
            false => PIPE8BEMPE_A::_0,
            true => PIPE8BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE8BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE8BEMPE_A::_1
    }
}
///Field `PIPE8BEMPE` writer - BEMP Interrupt Enable for PIPE8
pub type PIPE8BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE8BEMPE_A>;
impl<'a, REG> PIPE8BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE8BEMPE_A::_1)
    }
}
/**BEMP Interrupt Enable for PIPE9

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIPE9BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<PIPE9BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: PIPE9BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PIPE9BEMPE` reader - BEMP Interrupt Enable for PIPE9
pub type PIPE9BEMPE_R = crate::BitReader<PIPE9BEMPE_A>;
impl PIPE9BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PIPE9BEMPE_A {
        match self.bits {
            false => PIPE9BEMPE_A::_0,
            true => PIPE9BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIPE9BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIPE9BEMPE_A::_1
    }
}
///Field `PIPE9BEMPE` writer - BEMP Interrupt Enable for PIPE9
pub type PIPE9BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, PIPE9BEMPE_A>;
impl<'a, REG> PIPE9BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PIPE9BEMPE_A::_1)
    }
}
impl R {
    ///Bit 0 - BEMP Interrupt Enable for PIPE0
    #[inline(always)]
    pub fn pipe0bempe(&self) -> PIPE0BEMPE_R {
        PIPE0BEMPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BEMP Interrupt Enable for PIPE1
    #[inline(always)]
    pub fn pipe1bempe(&self) -> PIPE1BEMPE_R {
        PIPE1BEMPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BEMP Interrupt Enable for PIPE2
    #[inline(always)]
    pub fn pipe2bempe(&self) -> PIPE2BEMPE_R {
        PIPE2BEMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BEMP Interrupt Enable for PIPE3
    #[inline(always)]
    pub fn pipe3bempe(&self) -> PIPE3BEMPE_R {
        PIPE3BEMPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BEMP Interrupt Enable for PIPE4
    #[inline(always)]
    pub fn pipe4bempe(&self) -> PIPE4BEMPE_R {
        PIPE4BEMPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BEMP Interrupt Enable for PIPE5
    #[inline(always)]
    pub fn pipe5bempe(&self) -> PIPE5BEMPE_R {
        PIPE5BEMPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BEMP Interrupt Enable for PIPE6
    #[inline(always)]
    pub fn pipe6bempe(&self) -> PIPE6BEMPE_R {
        PIPE6BEMPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BEMP Interrupt Enable for PIPE7
    #[inline(always)]
    pub fn pipe7bempe(&self) -> PIPE7BEMPE_R {
        PIPE7BEMPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - BEMP Interrupt Enable for PIPE8
    #[inline(always)]
    pub fn pipe8bempe(&self) -> PIPE8BEMPE_R {
        PIPE8BEMPE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BEMP Interrupt Enable for PIPE9
    #[inline(always)]
    pub fn pipe9bempe(&self) -> PIPE9BEMPE_R {
        PIPE9BEMPE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BEMP Interrupt Enable for PIPE0
    #[inline(always)]
    pub fn pipe0bempe(&mut self) -> PIPE0BEMPE_W<BEMPENB_SPEC> {
        PIPE0BEMPE_W::new(self, 0)
    }
    ///Bit 1 - BEMP Interrupt Enable for PIPE1
    #[inline(always)]
    pub fn pipe1bempe(&mut self) -> PIPE1BEMPE_W<BEMPENB_SPEC> {
        PIPE1BEMPE_W::new(self, 1)
    }
    ///Bit 2 - BEMP Interrupt Enable for PIPE2
    #[inline(always)]
    pub fn pipe2bempe(&mut self) -> PIPE2BEMPE_W<BEMPENB_SPEC> {
        PIPE2BEMPE_W::new(self, 2)
    }
    ///Bit 3 - BEMP Interrupt Enable for PIPE3
    #[inline(always)]
    pub fn pipe3bempe(&mut self) -> PIPE3BEMPE_W<BEMPENB_SPEC> {
        PIPE3BEMPE_W::new(self, 3)
    }
    ///Bit 4 - BEMP Interrupt Enable for PIPE4
    #[inline(always)]
    pub fn pipe4bempe(&mut self) -> PIPE4BEMPE_W<BEMPENB_SPEC> {
        PIPE4BEMPE_W::new(self, 4)
    }
    ///Bit 5 - BEMP Interrupt Enable for PIPE5
    #[inline(always)]
    pub fn pipe5bempe(&mut self) -> PIPE5BEMPE_W<BEMPENB_SPEC> {
        PIPE5BEMPE_W::new(self, 5)
    }
    ///Bit 6 - BEMP Interrupt Enable for PIPE6
    #[inline(always)]
    pub fn pipe6bempe(&mut self) -> PIPE6BEMPE_W<BEMPENB_SPEC> {
        PIPE6BEMPE_W::new(self, 6)
    }
    ///Bit 7 - BEMP Interrupt Enable for PIPE7
    #[inline(always)]
    pub fn pipe7bempe(&mut self) -> PIPE7BEMPE_W<BEMPENB_SPEC> {
        PIPE7BEMPE_W::new(self, 7)
    }
    ///Bit 8 - BEMP Interrupt Enable for PIPE8
    #[inline(always)]
    pub fn pipe8bempe(&mut self) -> PIPE8BEMPE_W<BEMPENB_SPEC> {
        PIPE8BEMPE_W::new(self, 8)
    }
    ///Bit 9 - BEMP Interrupt Enable for PIPE9
    #[inline(always)]
    pub fn pipe9bempe(&mut self) -> PIPE9BEMPE_W<BEMPENB_SPEC> {
        PIPE9BEMPE_W::new(self, 9)
    }
}
/**BEMP Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`bempenb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BEMPENB_SPEC;
impl crate::RegisterSpec for BEMPENB_SPEC {
    type Ux = u16;
}
///`read()` method returns [`bempenb::R`](R) reader structure
impl crate::Readable for BEMPENB_SPEC {}
///`write(|w| ..)` method takes [`bempenb::W`](W) writer structure
impl crate::Writable for BEMPENB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BEMPENB to value 0
impl crate::Resettable for BEMPENB_SPEC {}
