///Register `SNZREQCR` reader
pub type R = crate::R<SNZREQCR_SPEC>;
///Register `SNZREQCR` writer
pub type W = crate::W<SNZREQCR_SPEC>;
/**Snooze Request Enable 0Enable IRQ0 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN0_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN0` reader - Snooze Request Enable 0Enable IRQ0 pin snooze request
pub type SNZREQEN0_R = crate::BitReader<SNZREQEN0_A>;
impl SNZREQEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN0_A {
        match self.bits {
            false => SNZREQEN0_A::_0,
            true => SNZREQEN0_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN0_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN0_A::_1
    }
}
///Field `SNZREQEN0` writer - Snooze Request Enable 0Enable IRQ0 pin snooze request
pub type SNZREQEN0_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN0_A>;
impl<'a, REG> SNZREQEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN0_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN0_A::_1)
    }
}
/**Snooze Request Enable 1Enable IRQ1 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN1_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN1` reader - Snooze Request Enable 1Enable IRQ1 pin snooze request
pub type SNZREQEN1_R = crate::BitReader<SNZREQEN1_A>;
impl SNZREQEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN1_A {
        match self.bits {
            false => SNZREQEN1_A::_0,
            true => SNZREQEN1_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN1_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN1_A::_1
    }
}
///Field `SNZREQEN1` writer - Snooze Request Enable 1Enable IRQ1 pin snooze request
pub type SNZREQEN1_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN1_A>;
impl<'a, REG> SNZREQEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN1_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN1_A::_1)
    }
}
/**Snooze Request Enable 2Enable IRQ2 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN2_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN2` reader - Snooze Request Enable 2Enable IRQ2 pin snooze request
pub type SNZREQEN2_R = crate::BitReader<SNZREQEN2_A>;
impl SNZREQEN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN2_A {
        match self.bits {
            false => SNZREQEN2_A::_0,
            true => SNZREQEN2_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN2_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN2_A::_1
    }
}
///Field `SNZREQEN2` writer - Snooze Request Enable 2Enable IRQ2 pin snooze request
pub type SNZREQEN2_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN2_A>;
impl<'a, REG> SNZREQEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN2_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN2_A::_1)
    }
}
/**Snooze Request Enable 3Enable IRQ3 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN3_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN3` reader - Snooze Request Enable 3Enable IRQ3 pin snooze request
pub type SNZREQEN3_R = crate::BitReader<SNZREQEN3_A>;
impl SNZREQEN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN3_A {
        match self.bits {
            false => SNZREQEN3_A::_0,
            true => SNZREQEN3_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN3_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN3_A::_1
    }
}
///Field `SNZREQEN3` writer - Snooze Request Enable 3Enable IRQ3 pin snooze request
pub type SNZREQEN3_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN3_A>;
impl<'a, REG> SNZREQEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN3_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN3_A::_1)
    }
}
/**Snooze Request Enable 4Enable IRQ4 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN4_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN4_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN4` reader - Snooze Request Enable 4Enable IRQ4 pin snooze request
pub type SNZREQEN4_R = crate::BitReader<SNZREQEN4_A>;
impl SNZREQEN4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN4_A {
        match self.bits {
            false => SNZREQEN4_A::_0,
            true => SNZREQEN4_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN4_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN4_A::_1
    }
}
///Field `SNZREQEN4` writer - Snooze Request Enable 4Enable IRQ4 pin snooze request
pub type SNZREQEN4_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN4_A>;
impl<'a, REG> SNZREQEN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN4_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN4_A::_1)
    }
}
/**Snooze Request Enable 5Enable IRQ5 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN5_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN5` reader - Snooze Request Enable 5Enable IRQ5 pin snooze request
pub type SNZREQEN5_R = crate::BitReader<SNZREQEN5_A>;
impl SNZREQEN5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN5_A {
        match self.bits {
            false => SNZREQEN5_A::_0,
            true => SNZREQEN5_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN5_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN5_A::_1
    }
}
///Field `SNZREQEN5` writer - Snooze Request Enable 5Enable IRQ5 pin snooze request
pub type SNZREQEN5_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN5_A>;
impl<'a, REG> SNZREQEN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN5_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN5_A::_1)
    }
}
/**Snooze Request Enable 6Enable IRQ6 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN6_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN6` reader - Snooze Request Enable 6Enable IRQ6 pin snooze request
pub type SNZREQEN6_R = crate::BitReader<SNZREQEN6_A>;
impl SNZREQEN6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN6_A {
        match self.bits {
            false => SNZREQEN6_A::_0,
            true => SNZREQEN6_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN6_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN6_A::_1
    }
}
///Field `SNZREQEN6` writer - Snooze Request Enable 6Enable IRQ6 pin snooze request
pub type SNZREQEN6_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN6_A>;
impl<'a, REG> SNZREQEN6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN6_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN6_A::_1)
    }
}
/**Snooze Request Enable 7Enable IRQ7 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN7_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN7_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN7` reader - Snooze Request Enable 7Enable IRQ7 pin snooze request
pub type SNZREQEN7_R = crate::BitReader<SNZREQEN7_A>;
impl SNZREQEN7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN7_A {
        match self.bits {
            false => SNZREQEN7_A::_0,
            true => SNZREQEN7_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN7_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN7_A::_1
    }
}
///Field `SNZREQEN7` writer - Snooze Request Enable 7Enable IRQ7 pin snooze request
pub type SNZREQEN7_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN7_A>;
impl<'a, REG> SNZREQEN7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN7_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN7_A::_1)
    }
}
/**Snooze Request Enable 8Enable IRQ8 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN8_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN8_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN8` reader - Snooze Request Enable 8Enable IRQ8 pin snooze request
pub type SNZREQEN8_R = crate::BitReader<SNZREQEN8_A>;
impl SNZREQEN8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN8_A {
        match self.bits {
            false => SNZREQEN8_A::_0,
            true => SNZREQEN8_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN8_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN8_A::_1
    }
}
///Field `SNZREQEN8` writer - Snooze Request Enable 8Enable IRQ8 pin snooze request
pub type SNZREQEN8_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN8_A>;
impl<'a, REG> SNZREQEN8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN8_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN8_A::_1)
    }
}
/**Snooze Request Enable 9Enable IRQ9 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN9_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN9_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN9` reader - Snooze Request Enable 9Enable IRQ9 pin snooze request
pub type SNZREQEN9_R = crate::BitReader<SNZREQEN9_A>;
impl SNZREQEN9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN9_A {
        match self.bits {
            false => SNZREQEN9_A::_0,
            true => SNZREQEN9_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN9_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN9_A::_1
    }
}
///Field `SNZREQEN9` writer - Snooze Request Enable 9Enable IRQ9 pin snooze request
pub type SNZREQEN9_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN9_A>;
impl<'a, REG> SNZREQEN9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN9_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN9_A::_1)
    }
}
/**Snooze Request Enable 10Enable IRQ10 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN10_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN10_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN10` reader - Snooze Request Enable 10Enable IRQ10 pin snooze request
pub type SNZREQEN10_R = crate::BitReader<SNZREQEN10_A>;
impl SNZREQEN10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN10_A {
        match self.bits {
            false => SNZREQEN10_A::_0,
            true => SNZREQEN10_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN10_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN10_A::_1
    }
}
///Field `SNZREQEN10` writer - Snooze Request Enable 10Enable IRQ10 pin snooze request
pub type SNZREQEN10_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN10_A>;
impl<'a, REG> SNZREQEN10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN10_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN10_A::_1)
    }
}
/**Snooze Request Enable 11Enable IRQ11 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN11_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN11_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN11` reader - Snooze Request Enable 11Enable IRQ11 pin snooze request
pub type SNZREQEN11_R = crate::BitReader<SNZREQEN11_A>;
impl SNZREQEN11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN11_A {
        match self.bits {
            false => SNZREQEN11_A::_0,
            true => SNZREQEN11_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN11_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN11_A::_1
    }
}
///Field `SNZREQEN11` writer - Snooze Request Enable 11Enable IRQ11 pin snooze request
pub type SNZREQEN11_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN11_A>;
impl<'a, REG> SNZREQEN11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN11_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN11_A::_1)
    }
}
/**Snooze Request Enable 12Enable IRQ12 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN12_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN12_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN12` reader - Snooze Request Enable 12Enable IRQ12 pin snooze request
pub type SNZREQEN12_R = crate::BitReader<SNZREQEN12_A>;
impl SNZREQEN12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN12_A {
        match self.bits {
            false => SNZREQEN12_A::_0,
            true => SNZREQEN12_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN12_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN12_A::_1
    }
}
///Field `SNZREQEN12` writer - Snooze Request Enable 12Enable IRQ12 pin snooze request
pub type SNZREQEN12_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN12_A>;
impl<'a, REG> SNZREQEN12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN12_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN12_A::_1)
    }
}
/**Snooze Request Enable 13Enable IRQ13 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN13_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN13_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN13` reader - Snooze Request Enable 13Enable IRQ13 pin snooze request
pub type SNZREQEN13_R = crate::BitReader<SNZREQEN13_A>;
impl SNZREQEN13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN13_A {
        match self.bits {
            false => SNZREQEN13_A::_0,
            true => SNZREQEN13_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN13_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN13_A::_1
    }
}
///Field `SNZREQEN13` writer - Snooze Request Enable 13Enable IRQ13 pin snooze request
pub type SNZREQEN13_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN13_A>;
impl<'a, REG> SNZREQEN13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN13_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN13_A::_1)
    }
}
/**Snooze Request Enable 14Enable IRQ14 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN14_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN14_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN14` reader - Snooze Request Enable 14Enable IRQ14 pin snooze request
pub type SNZREQEN14_R = crate::BitReader<SNZREQEN14_A>;
impl SNZREQEN14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN14_A {
        match self.bits {
            false => SNZREQEN14_A::_0,
            true => SNZREQEN14_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN14_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN14_A::_1
    }
}
///Field `SNZREQEN14` writer - Snooze Request Enable 14Enable IRQ14 pin snooze request
pub type SNZREQEN14_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN14_A>;
impl<'a, REG> SNZREQEN14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN14_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN14_A::_1)
    }
}
/**Snooze Request Enable 15Enable IRQ15 pin snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN15_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN15_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN15_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN15` reader - Snooze Request Enable 15Enable IRQ15 pin snooze request
pub type SNZREQEN15_R = crate::BitReader<SNZREQEN15_A>;
impl SNZREQEN15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN15_A {
        match self.bits {
            false => SNZREQEN15_A::_0,
            true => SNZREQEN15_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN15_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN15_A::_1
    }
}
///Field `SNZREQEN15` writer - Snooze Request Enable 15Enable IRQ15 pin snooze request
pub type SNZREQEN15_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN15_A>;
impl<'a, REG> SNZREQEN15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN15_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN15_A::_1)
    }
}
/**Snooze Request Enable 17Enable KR snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN17_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN17_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN17` reader - Snooze Request Enable 17Enable KR snooze request
pub type SNZREQEN17_R = crate::BitReader<SNZREQEN17_A>;
impl SNZREQEN17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN17_A {
        match self.bits {
            false => SNZREQEN17_A::_0,
            true => SNZREQEN17_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN17_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN17_A::_1
    }
}
///Field `SNZREQEN17` writer - Snooze Request Enable 17Enable KR snooze request
pub type SNZREQEN17_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN17_A>;
impl<'a, REG> SNZREQEN17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN17_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN17_A::_1)
    }
}
/**Snooze Request Enable 22Enable Comparator-OC0 snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN22_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN22_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN22` reader - Snooze Request Enable 22Enable Comparator-OC0 snooze request
pub type SNZREQEN22_R = crate::BitReader<SNZREQEN22_A>;
impl SNZREQEN22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN22_A {
        match self.bits {
            false => SNZREQEN22_A::_0,
            true => SNZREQEN22_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN22_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN22_A::_1
    }
}
///Field `SNZREQEN22` writer - Snooze Request Enable 22Enable Comparator-OC0 snooze request
pub type SNZREQEN22_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN22_A>;
impl<'a, REG> SNZREQEN22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN22_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN22_A::_1)
    }
}
/**Snooze Request Enable 24Enable RTC alarm snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN24_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN24_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN24` reader - Snooze Request Enable 24Enable RTC alarm snooze request
pub type SNZREQEN24_R = crate::BitReader<SNZREQEN24_A>;
impl SNZREQEN24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN24_A {
        match self.bits {
            false => SNZREQEN24_A::_0,
            true => SNZREQEN24_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN24_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN24_A::_1
    }
}
///Field `SNZREQEN24` writer - Snooze Request Enable 24Enable RTC alarm snooze request
pub type SNZREQEN24_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN24_A>;
impl<'a, REG> SNZREQEN24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN24_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN24_A::_1)
    }
}
/**Snooze Request Enable 25Enable RTC period snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN25_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN25_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN25_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN25` reader - Snooze Request Enable 25Enable RTC period snooze request
pub type SNZREQEN25_R = crate::BitReader<SNZREQEN25_A>;
impl SNZREQEN25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN25_A {
        match self.bits {
            false => SNZREQEN25_A::_0,
            true => SNZREQEN25_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN25_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN25_A::_1
    }
}
///Field `SNZREQEN25` writer - Snooze Request Enable 25Enable RTC period snooze request
pub type SNZREQEN25_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN25_A>;
impl<'a, REG> SNZREQEN25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN25_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN25_A::_1)
    }
}
/**Snooze Request Enable 28Enable AGT1 underflow snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN28_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN28_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN28_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN28` reader - Snooze Request Enable 28Enable AGT1 underflow snooze request
pub type SNZREQEN28_R = crate::BitReader<SNZREQEN28_A>;
impl SNZREQEN28_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN28_A {
        match self.bits {
            false => SNZREQEN28_A::_0,
            true => SNZREQEN28_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN28_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN28_A::_1
    }
}
///Field `SNZREQEN28` writer - Snooze Request Enable 28Enable AGT1 underflow snooze request
pub type SNZREQEN28_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN28_A>;
impl<'a, REG> SNZREQEN28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN28_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN28_A::_1)
    }
}
/**Snooze Request Enable 29Enable AGT1 compare match A snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN29_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN29_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN29_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN29` reader - Snooze Request Enable 29Enable AGT1 compare match A snooze request
pub type SNZREQEN29_R = crate::BitReader<SNZREQEN29_A>;
impl SNZREQEN29_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN29_A {
        match self.bits {
            false => SNZREQEN29_A::_0,
            true => SNZREQEN29_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN29_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN29_A::_1
    }
}
///Field `SNZREQEN29` writer - Snooze Request Enable 29Enable AGT1 compare match A snooze request
pub type SNZREQEN29_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN29_A>;
impl<'a, REG> SNZREQEN29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN29_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN29_A::_1)
    }
}
/**Snooze Request Enable 30Enable AGT1 compare match B snooze request

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNZREQEN30_A {
    ///0: Disable snooze request
    _0 = 0,
    ///1: Enable snooze request
    _1 = 1,
}
impl From<SNZREQEN30_A> for bool {
    #[inline(always)]
    fn from(variant: SNZREQEN30_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SNZREQEN30` reader - Snooze Request Enable 30Enable AGT1 compare match B snooze request
pub type SNZREQEN30_R = crate::BitReader<SNZREQEN30_A>;
impl SNZREQEN30_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SNZREQEN30_A {
        match self.bits {
            false => SNZREQEN30_A::_0,
            true => SNZREQEN30_A::_1,
        }
    }
    ///Disable snooze request
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SNZREQEN30_A::_0
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SNZREQEN30_A::_1
    }
}
///Field `SNZREQEN30` writer - Snooze Request Enable 30Enable AGT1 compare match B snooze request
pub type SNZREQEN30_W<'a, REG> = crate::BitWriter<'a, REG, SNZREQEN30_A>;
impl<'a, REG> SNZREQEN30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable snooze request
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN30_A::_0)
    }
    ///Enable snooze request
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SNZREQEN30_A::_1)
    }
}
impl R {
    ///Bit 0 - Snooze Request Enable 0Enable IRQ0 pin snooze request
    #[inline(always)]
    pub fn snzreqen0(&self) -> SNZREQEN0_R {
        SNZREQEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Snooze Request Enable 1Enable IRQ1 pin snooze request
    #[inline(always)]
    pub fn snzreqen1(&self) -> SNZREQEN1_R {
        SNZREQEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Snooze Request Enable 2Enable IRQ2 pin snooze request
    #[inline(always)]
    pub fn snzreqen2(&self) -> SNZREQEN2_R {
        SNZREQEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Snooze Request Enable 3Enable IRQ3 pin snooze request
    #[inline(always)]
    pub fn snzreqen3(&self) -> SNZREQEN3_R {
        SNZREQEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Snooze Request Enable 4Enable IRQ4 pin snooze request
    #[inline(always)]
    pub fn snzreqen4(&self) -> SNZREQEN4_R {
        SNZREQEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Snooze Request Enable 5Enable IRQ5 pin snooze request
    #[inline(always)]
    pub fn snzreqen5(&self) -> SNZREQEN5_R {
        SNZREQEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Snooze Request Enable 6Enable IRQ6 pin snooze request
    #[inline(always)]
    pub fn snzreqen6(&self) -> SNZREQEN6_R {
        SNZREQEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Snooze Request Enable 7Enable IRQ7 pin snooze request
    #[inline(always)]
    pub fn snzreqen7(&self) -> SNZREQEN7_R {
        SNZREQEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Snooze Request Enable 8Enable IRQ8 pin snooze request
    #[inline(always)]
    pub fn snzreqen8(&self) -> SNZREQEN8_R {
        SNZREQEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Snooze Request Enable 9Enable IRQ9 pin snooze request
    #[inline(always)]
    pub fn snzreqen9(&self) -> SNZREQEN9_R {
        SNZREQEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Snooze Request Enable 10Enable IRQ10 pin snooze request
    #[inline(always)]
    pub fn snzreqen10(&self) -> SNZREQEN10_R {
        SNZREQEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Snooze Request Enable 11Enable IRQ11 pin snooze request
    #[inline(always)]
    pub fn snzreqen11(&self) -> SNZREQEN11_R {
        SNZREQEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Snooze Request Enable 12Enable IRQ12 pin snooze request
    #[inline(always)]
    pub fn snzreqen12(&self) -> SNZREQEN12_R {
        SNZREQEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Snooze Request Enable 13Enable IRQ13 pin snooze request
    #[inline(always)]
    pub fn snzreqen13(&self) -> SNZREQEN13_R {
        SNZREQEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Snooze Request Enable 14Enable IRQ14 pin snooze request
    #[inline(always)]
    pub fn snzreqen14(&self) -> SNZREQEN14_R {
        SNZREQEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Snooze Request Enable 15Enable IRQ15 pin snooze request
    #[inline(always)]
    pub fn snzreqen15(&self) -> SNZREQEN15_R {
        SNZREQEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Snooze Request Enable 17Enable KR snooze request
    #[inline(always)]
    pub fn snzreqen17(&self) -> SNZREQEN17_R {
        SNZREQEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 22 - Snooze Request Enable 22Enable Comparator-OC0 snooze request
    #[inline(always)]
    pub fn snzreqen22(&self) -> SNZREQEN22_R {
        SNZREQEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Snooze Request Enable 24Enable RTC alarm snooze request
    #[inline(always)]
    pub fn snzreqen24(&self) -> SNZREQEN24_R {
        SNZREQEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Snooze Request Enable 25Enable RTC period snooze request
    #[inline(always)]
    pub fn snzreqen25(&self) -> SNZREQEN25_R {
        SNZREQEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Snooze Request Enable 28Enable AGT1 underflow snooze request
    #[inline(always)]
    pub fn snzreqen28(&self) -> SNZREQEN28_R {
        SNZREQEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Snooze Request Enable 29Enable AGT1 compare match A snooze request
    #[inline(always)]
    pub fn snzreqen29(&self) -> SNZREQEN29_R {
        SNZREQEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Snooze Request Enable 30Enable AGT1 compare match B snooze request
    #[inline(always)]
    pub fn snzreqen30(&self) -> SNZREQEN30_R {
        SNZREQEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Snooze Request Enable 0Enable IRQ0 pin snooze request
    #[inline(always)]
    pub fn snzreqen0(&mut self) -> SNZREQEN0_W<SNZREQCR_SPEC> {
        SNZREQEN0_W::new(self, 0)
    }
    ///Bit 1 - Snooze Request Enable 1Enable IRQ1 pin snooze request
    #[inline(always)]
    pub fn snzreqen1(&mut self) -> SNZREQEN1_W<SNZREQCR_SPEC> {
        SNZREQEN1_W::new(self, 1)
    }
    ///Bit 2 - Snooze Request Enable 2Enable IRQ2 pin snooze request
    #[inline(always)]
    pub fn snzreqen2(&mut self) -> SNZREQEN2_W<SNZREQCR_SPEC> {
        SNZREQEN2_W::new(self, 2)
    }
    ///Bit 3 - Snooze Request Enable 3Enable IRQ3 pin snooze request
    #[inline(always)]
    pub fn snzreqen3(&mut self) -> SNZREQEN3_W<SNZREQCR_SPEC> {
        SNZREQEN3_W::new(self, 3)
    }
    ///Bit 4 - Snooze Request Enable 4Enable IRQ4 pin snooze request
    #[inline(always)]
    pub fn snzreqen4(&mut self) -> SNZREQEN4_W<SNZREQCR_SPEC> {
        SNZREQEN4_W::new(self, 4)
    }
    ///Bit 5 - Snooze Request Enable 5Enable IRQ5 pin snooze request
    #[inline(always)]
    pub fn snzreqen5(&mut self) -> SNZREQEN5_W<SNZREQCR_SPEC> {
        SNZREQEN5_W::new(self, 5)
    }
    ///Bit 6 - Snooze Request Enable 6Enable IRQ6 pin snooze request
    #[inline(always)]
    pub fn snzreqen6(&mut self) -> SNZREQEN6_W<SNZREQCR_SPEC> {
        SNZREQEN6_W::new(self, 6)
    }
    ///Bit 7 - Snooze Request Enable 7Enable IRQ7 pin snooze request
    #[inline(always)]
    pub fn snzreqen7(&mut self) -> SNZREQEN7_W<SNZREQCR_SPEC> {
        SNZREQEN7_W::new(self, 7)
    }
    ///Bit 8 - Snooze Request Enable 8Enable IRQ8 pin snooze request
    #[inline(always)]
    pub fn snzreqen8(&mut self) -> SNZREQEN8_W<SNZREQCR_SPEC> {
        SNZREQEN8_W::new(self, 8)
    }
    ///Bit 9 - Snooze Request Enable 9Enable IRQ9 pin snooze request
    #[inline(always)]
    pub fn snzreqen9(&mut self) -> SNZREQEN9_W<SNZREQCR_SPEC> {
        SNZREQEN9_W::new(self, 9)
    }
    ///Bit 10 - Snooze Request Enable 10Enable IRQ10 pin snooze request
    #[inline(always)]
    pub fn snzreqen10(&mut self) -> SNZREQEN10_W<SNZREQCR_SPEC> {
        SNZREQEN10_W::new(self, 10)
    }
    ///Bit 11 - Snooze Request Enable 11Enable IRQ11 pin snooze request
    #[inline(always)]
    pub fn snzreqen11(&mut self) -> SNZREQEN11_W<SNZREQCR_SPEC> {
        SNZREQEN11_W::new(self, 11)
    }
    ///Bit 12 - Snooze Request Enable 12Enable IRQ12 pin snooze request
    #[inline(always)]
    pub fn snzreqen12(&mut self) -> SNZREQEN12_W<SNZREQCR_SPEC> {
        SNZREQEN12_W::new(self, 12)
    }
    ///Bit 13 - Snooze Request Enable 13Enable IRQ13 pin snooze request
    #[inline(always)]
    pub fn snzreqen13(&mut self) -> SNZREQEN13_W<SNZREQCR_SPEC> {
        SNZREQEN13_W::new(self, 13)
    }
    ///Bit 14 - Snooze Request Enable 14Enable IRQ14 pin snooze request
    #[inline(always)]
    pub fn snzreqen14(&mut self) -> SNZREQEN14_W<SNZREQCR_SPEC> {
        SNZREQEN14_W::new(self, 14)
    }
    ///Bit 15 - Snooze Request Enable 15Enable IRQ15 pin snooze request
    #[inline(always)]
    pub fn snzreqen15(&mut self) -> SNZREQEN15_W<SNZREQCR_SPEC> {
        SNZREQEN15_W::new(self, 15)
    }
    ///Bit 17 - Snooze Request Enable 17Enable KR snooze request
    #[inline(always)]
    pub fn snzreqen17(&mut self) -> SNZREQEN17_W<SNZREQCR_SPEC> {
        SNZREQEN17_W::new(self, 17)
    }
    ///Bit 22 - Snooze Request Enable 22Enable Comparator-OC0 snooze request
    #[inline(always)]
    pub fn snzreqen22(&mut self) -> SNZREQEN22_W<SNZREQCR_SPEC> {
        SNZREQEN22_W::new(self, 22)
    }
    ///Bit 24 - Snooze Request Enable 24Enable RTC alarm snooze request
    #[inline(always)]
    pub fn snzreqen24(&mut self) -> SNZREQEN24_W<SNZREQCR_SPEC> {
        SNZREQEN24_W::new(self, 24)
    }
    ///Bit 25 - Snooze Request Enable 25Enable RTC period snooze request
    #[inline(always)]
    pub fn snzreqen25(&mut self) -> SNZREQEN25_W<SNZREQCR_SPEC> {
        SNZREQEN25_W::new(self, 25)
    }
    ///Bit 28 - Snooze Request Enable 28Enable AGT1 underflow snooze request
    #[inline(always)]
    pub fn snzreqen28(&mut self) -> SNZREQEN28_W<SNZREQCR_SPEC> {
        SNZREQEN28_W::new(self, 28)
    }
    ///Bit 29 - Snooze Request Enable 29Enable AGT1 compare match A snooze request
    #[inline(always)]
    pub fn snzreqen29(&mut self) -> SNZREQEN29_W<SNZREQCR_SPEC> {
        SNZREQEN29_W::new(self, 29)
    }
    ///Bit 30 - Snooze Request Enable 30Enable AGT1 compare match B snooze request
    #[inline(always)]
    pub fn snzreqen30(&mut self) -> SNZREQEN30_W<SNZREQCR_SPEC> {
        SNZREQEN30_W::new(self, 30)
    }
}
/**Snooze Request Control Register

You can [`read`](crate::Reg::read) this register and get [`snzreqcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SNZREQCR_SPEC;
impl crate::RegisterSpec for SNZREQCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`snzreqcr::R`](R) reader structure
impl crate::Readable for SNZREQCR_SPEC {}
///`write(|w| ..)` method takes [`snzreqcr::W`](W) writer structure
impl crate::Writable for SNZREQCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SNZREQCR to value 0
impl crate::Resettable for SNZREQCR_SPEC {}
