///Register `WUPEN` reader
pub type R = crate::R<WUPEN_SPEC>;
///Register `WUPEN` writer
pub type W = crate::W<WUPEN_SPEC>;
/**IRQ0 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN0_A {
    ///0: S/W standby returns by IRQ0 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ0 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN0` reader - IRQ0 interrupt S/W standby returns enable bit
pub type IRQWUPEN0_R = crate::BitReader<IRQWUPEN0_A>;
impl IRQWUPEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN0_A {
        match self.bits {
            false => IRQWUPEN0_A::_0,
            true => IRQWUPEN0_A::_1,
        }
    }
    ///S/W standby returns by IRQ0 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN0_A::_0
    }
    ///S/W standby returns by IRQ0 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN0_A::_1
    }
}
///Field `IRQWUPEN0` writer - IRQ0 interrupt S/W standby returns enable bit
pub type IRQWUPEN0_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN0_A>;
impl<'a, REG> IRQWUPEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ0 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN0_A::_0)
    }
    ///S/W standby returns by IRQ0 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN0_A::_1)
    }
}
/**IRQ1 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN1_A {
    ///0: S/W standby returns by IRQ1 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ1 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN1` reader - IRQ1 interrupt S/W standby returns enable bit
pub type IRQWUPEN1_R = crate::BitReader<IRQWUPEN1_A>;
impl IRQWUPEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN1_A {
        match self.bits {
            false => IRQWUPEN1_A::_0,
            true => IRQWUPEN1_A::_1,
        }
    }
    ///S/W standby returns by IRQ1 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN1_A::_0
    }
    ///S/W standby returns by IRQ1 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN1_A::_1
    }
}
///Field `IRQWUPEN1` writer - IRQ1 interrupt S/W standby returns enable bit
pub type IRQWUPEN1_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN1_A>;
impl<'a, REG> IRQWUPEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ1 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN1_A::_0)
    }
    ///S/W standby returns by IRQ1 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN1_A::_1)
    }
}
/**IRQ2 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN2_A {
    ///0: S/W standby returns by IRQ2 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ2 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN2` reader - IRQ2 interrupt S/W standby returns enable bit
pub type IRQWUPEN2_R = crate::BitReader<IRQWUPEN2_A>;
impl IRQWUPEN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN2_A {
        match self.bits {
            false => IRQWUPEN2_A::_0,
            true => IRQWUPEN2_A::_1,
        }
    }
    ///S/W standby returns by IRQ2 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN2_A::_0
    }
    ///S/W standby returns by IRQ2 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN2_A::_1
    }
}
///Field `IRQWUPEN2` writer - IRQ2 interrupt S/W standby returns enable bit
pub type IRQWUPEN2_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN2_A>;
impl<'a, REG> IRQWUPEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ2 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN2_A::_0)
    }
    ///S/W standby returns by IRQ2 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN2_A::_1)
    }
}
/**IRQ3 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN3_A {
    ///0: S/W standby returns by IRQ3 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ3 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN3` reader - IRQ3 interrupt S/W standby returns enable bit
pub type IRQWUPEN3_R = crate::BitReader<IRQWUPEN3_A>;
impl IRQWUPEN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN3_A {
        match self.bits {
            false => IRQWUPEN3_A::_0,
            true => IRQWUPEN3_A::_1,
        }
    }
    ///S/W standby returns by IRQ3 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN3_A::_0
    }
    ///S/W standby returns by IRQ3 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN3_A::_1
    }
}
///Field `IRQWUPEN3` writer - IRQ3 interrupt S/W standby returns enable bit
pub type IRQWUPEN3_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN3_A>;
impl<'a, REG> IRQWUPEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ3 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN3_A::_0)
    }
    ///S/W standby returns by IRQ3 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN3_A::_1)
    }
}
/**IRQ4 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN4_A {
    ///0: S/W standby returns by IRQ4 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ4 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN4` reader - IRQ4 interrupt S/W standby returns enable bit
pub type IRQWUPEN4_R = crate::BitReader<IRQWUPEN4_A>;
impl IRQWUPEN4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN4_A {
        match self.bits {
            false => IRQWUPEN4_A::_0,
            true => IRQWUPEN4_A::_1,
        }
    }
    ///S/W standby returns by IRQ4 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN4_A::_0
    }
    ///S/W standby returns by IRQ4 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN4_A::_1
    }
}
///Field `IRQWUPEN4` writer - IRQ4 interrupt S/W standby returns enable bit
pub type IRQWUPEN4_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN4_A>;
impl<'a, REG> IRQWUPEN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ4 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN4_A::_0)
    }
    ///S/W standby returns by IRQ4 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN4_A::_1)
    }
}
/**IRQ5 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN5_A {
    ///0: S/W standby returns by IRQ5 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ5 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN5` reader - IRQ5 interrupt S/W standby returns enable bit
pub type IRQWUPEN5_R = crate::BitReader<IRQWUPEN5_A>;
impl IRQWUPEN5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN5_A {
        match self.bits {
            false => IRQWUPEN5_A::_0,
            true => IRQWUPEN5_A::_1,
        }
    }
    ///S/W standby returns by IRQ5 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN5_A::_0
    }
    ///S/W standby returns by IRQ5 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN5_A::_1
    }
}
///Field `IRQWUPEN5` writer - IRQ5 interrupt S/W standby returns enable bit
pub type IRQWUPEN5_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN5_A>;
impl<'a, REG> IRQWUPEN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ5 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN5_A::_0)
    }
    ///S/W standby returns by IRQ5 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN5_A::_1)
    }
}
/**IRQ6 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN6_A {
    ///0: S/W standby returns by IRQ6 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ6 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN6_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN6` reader - IRQ6 interrupt S/W standby returns enable bit
pub type IRQWUPEN6_R = crate::BitReader<IRQWUPEN6_A>;
impl IRQWUPEN6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN6_A {
        match self.bits {
            false => IRQWUPEN6_A::_0,
            true => IRQWUPEN6_A::_1,
        }
    }
    ///S/W standby returns by IRQ6 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN6_A::_0
    }
    ///S/W standby returns by IRQ6 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN6_A::_1
    }
}
///Field `IRQWUPEN6` writer - IRQ6 interrupt S/W standby returns enable bit
pub type IRQWUPEN6_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN6_A>;
impl<'a, REG> IRQWUPEN6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ6 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN6_A::_0)
    }
    ///S/W standby returns by IRQ6 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN6_A::_1)
    }
}
/**IRQ7 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN7_A {
    ///0: S/W standby returns by IRQ7 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ7 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN7_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN7` reader - IRQ7 interrupt S/W standby returns enable bit
pub type IRQWUPEN7_R = crate::BitReader<IRQWUPEN7_A>;
impl IRQWUPEN7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN7_A {
        match self.bits {
            false => IRQWUPEN7_A::_0,
            true => IRQWUPEN7_A::_1,
        }
    }
    ///S/W standby returns by IRQ7 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN7_A::_0
    }
    ///S/W standby returns by IRQ7 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN7_A::_1
    }
}
///Field `IRQWUPEN7` writer - IRQ7 interrupt S/W standby returns enable bit
pub type IRQWUPEN7_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN7_A>;
impl<'a, REG> IRQWUPEN7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ7 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN7_A::_0)
    }
    ///S/W standby returns by IRQ7 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN7_A::_1)
    }
}
/**IRQ8 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN8_A {
    ///0: S/W standby returns by IRQ8 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ8 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN8_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN8` reader - IRQ8 interrupt S/W standby returns enable bit
pub type IRQWUPEN8_R = crate::BitReader<IRQWUPEN8_A>;
impl IRQWUPEN8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN8_A {
        match self.bits {
            false => IRQWUPEN8_A::_0,
            true => IRQWUPEN8_A::_1,
        }
    }
    ///S/W standby returns by IRQ8 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN8_A::_0
    }
    ///S/W standby returns by IRQ8 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN8_A::_1
    }
}
///Field `IRQWUPEN8` writer - IRQ8 interrupt S/W standby returns enable bit
pub type IRQWUPEN8_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN8_A>;
impl<'a, REG> IRQWUPEN8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ8 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN8_A::_0)
    }
    ///S/W standby returns by IRQ8 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN8_A::_1)
    }
}
/**IRQ9 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN9_A {
    ///0: S/W standby returns by IRQ9 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ9 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN9_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN9` reader - IRQ9 interrupt S/W standby returns enable bit
pub type IRQWUPEN9_R = crate::BitReader<IRQWUPEN9_A>;
impl IRQWUPEN9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN9_A {
        match self.bits {
            false => IRQWUPEN9_A::_0,
            true => IRQWUPEN9_A::_1,
        }
    }
    ///S/W standby returns by IRQ9 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN9_A::_0
    }
    ///S/W standby returns by IRQ9 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN9_A::_1
    }
}
///Field `IRQWUPEN9` writer - IRQ9 interrupt S/W standby returns enable bit
pub type IRQWUPEN9_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN9_A>;
impl<'a, REG> IRQWUPEN9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ9 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN9_A::_0)
    }
    ///S/W standby returns by IRQ9 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN9_A::_1)
    }
}
/**IRQ10 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN10_A {
    ///0: S/W standby returns by IRQ10 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ10 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN10_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN10` reader - IRQ10 interrupt S/W standby returns enable bit
pub type IRQWUPEN10_R = crate::BitReader<IRQWUPEN10_A>;
impl IRQWUPEN10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN10_A {
        match self.bits {
            false => IRQWUPEN10_A::_0,
            true => IRQWUPEN10_A::_1,
        }
    }
    ///S/W standby returns by IRQ10 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN10_A::_0
    }
    ///S/W standby returns by IRQ10 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN10_A::_1
    }
}
///Field `IRQWUPEN10` writer - IRQ10 interrupt S/W standby returns enable bit
pub type IRQWUPEN10_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN10_A>;
impl<'a, REG> IRQWUPEN10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ10 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN10_A::_0)
    }
    ///S/W standby returns by IRQ10 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN10_A::_1)
    }
}
/**IRQ11 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN11_A {
    ///0: S/W standby returns by IRQ11 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ11 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN11_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN11` reader - IRQ11 interrupt S/W standby returns enable bit
pub type IRQWUPEN11_R = crate::BitReader<IRQWUPEN11_A>;
impl IRQWUPEN11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN11_A {
        match self.bits {
            false => IRQWUPEN11_A::_0,
            true => IRQWUPEN11_A::_1,
        }
    }
    ///S/W standby returns by IRQ11 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN11_A::_0
    }
    ///S/W standby returns by IRQ11 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN11_A::_1
    }
}
///Field `IRQWUPEN11` writer - IRQ11 interrupt S/W standby returns enable bit
pub type IRQWUPEN11_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN11_A>;
impl<'a, REG> IRQWUPEN11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ11 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN11_A::_0)
    }
    ///S/W standby returns by IRQ11 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN11_A::_1)
    }
}
/**IRQ12 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN12_A {
    ///0: S/W standby returns by IRQ12 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ12 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN12_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN12` reader - IRQ12 interrupt S/W standby returns enable bit
pub type IRQWUPEN12_R = crate::BitReader<IRQWUPEN12_A>;
impl IRQWUPEN12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN12_A {
        match self.bits {
            false => IRQWUPEN12_A::_0,
            true => IRQWUPEN12_A::_1,
        }
    }
    ///S/W standby returns by IRQ12 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN12_A::_0
    }
    ///S/W standby returns by IRQ12 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN12_A::_1
    }
}
///Field `IRQWUPEN12` writer - IRQ12 interrupt S/W standby returns enable bit
pub type IRQWUPEN12_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN12_A>;
impl<'a, REG> IRQWUPEN12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ12 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN12_A::_0)
    }
    ///S/W standby returns by IRQ12 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN12_A::_1)
    }
}
/**IRQ13 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN13_A {
    ///0: S/W standby returns by IRQ13 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ13 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN13_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN13` reader - IRQ13 interrupt S/W standby returns enable bit
pub type IRQWUPEN13_R = crate::BitReader<IRQWUPEN13_A>;
impl IRQWUPEN13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN13_A {
        match self.bits {
            false => IRQWUPEN13_A::_0,
            true => IRQWUPEN13_A::_1,
        }
    }
    ///S/W standby returns by IRQ13 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN13_A::_0
    }
    ///S/W standby returns by IRQ13 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN13_A::_1
    }
}
///Field `IRQWUPEN13` writer - IRQ13 interrupt S/W standby returns enable bit
pub type IRQWUPEN13_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN13_A>;
impl<'a, REG> IRQWUPEN13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ13 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN13_A::_0)
    }
    ///S/W standby returns by IRQ13 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN13_A::_1)
    }
}
/**IRQ14 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN14_A {
    ///0: S/W standby returns by IRQ14 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ14 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN14_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN14` reader - IRQ14 interrupt S/W standby returns enable bit
pub type IRQWUPEN14_R = crate::BitReader<IRQWUPEN14_A>;
impl IRQWUPEN14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN14_A {
        match self.bits {
            false => IRQWUPEN14_A::_0,
            true => IRQWUPEN14_A::_1,
        }
    }
    ///S/W standby returns by IRQ14 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN14_A::_0
    }
    ///S/W standby returns by IRQ14 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN14_A::_1
    }
}
///Field `IRQWUPEN14` writer - IRQ14 interrupt S/W standby returns enable bit
pub type IRQWUPEN14_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN14_A>;
impl<'a, REG> IRQWUPEN14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ14 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN14_A::_0)
    }
    ///S/W standby returns by IRQ14 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN14_A::_1)
    }
}
/**IRQ15 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQWUPEN15_A {
    ///0: S/W standby returns by IRQ15 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IRQ15 interrupt is enabled
    _1 = 1,
}
impl From<IRQWUPEN15_A> for bool {
    #[inline(always)]
    fn from(variant: IRQWUPEN15_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRQWUPEN15` reader - IRQ15 interrupt S/W standby returns enable bit
pub type IRQWUPEN15_R = crate::BitReader<IRQWUPEN15_A>;
impl IRQWUPEN15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRQWUPEN15_A {
        match self.bits {
            false => IRQWUPEN15_A::_0,
            true => IRQWUPEN15_A::_1,
        }
    }
    ///S/W standby returns by IRQ15 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQWUPEN15_A::_0
    }
    ///S/W standby returns by IRQ15 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQWUPEN15_A::_1
    }
}
///Field `IRQWUPEN15` writer - IRQ15 interrupt S/W standby returns enable bit
pub type IRQWUPEN15_W<'a, REG> = crate::BitWriter<'a, REG, IRQWUPEN15_A>;
impl<'a, REG> IRQWUPEN15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IRQ15 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN15_A::_0)
    }
    ///S/W standby returns by IRQ15 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IRQWUPEN15_A::_1)
    }
}
/**IWDT interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDTWUPEN_A {
    ///0: S/W standby returns by IWDT interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IWDT interrupt is enabled
    _1 = 1,
}
impl From<IWDTWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IWDTWUPEN` reader - IWDT interrupt S/W standby returns enable bit
pub type IWDTWUPEN_R = crate::BitReader<IWDTWUPEN_A>;
impl IWDTWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IWDTWUPEN_A {
        match self.bits {
            false => IWDTWUPEN_A::_0,
            true => IWDTWUPEN_A::_1,
        }
    }
    ///S/W standby returns by IWDT interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTWUPEN_A::_0
    }
    ///S/W standby returns by IWDT interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTWUPEN_A::_1
    }
}
///Field `IWDTWUPEN` writer - IWDT interrupt S/W standby returns enable bit
pub type IWDTWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, IWDTWUPEN_A>;
impl<'a, REG> IWDTWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IWDT interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTWUPEN_A::_0)
    }
    ///S/W standby returns by IWDT interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDTWUPEN_A::_1)
    }
}
/**Key interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYWUPEN_A {
    ///0: S/W standby returns by KEY interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by KEY interrupt is enabled
    _1 = 1,
}
impl From<KEYWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: KEYWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `KEYWUPEN` reader - Key interrupt S/W standby returns enable bit
pub type KEYWUPEN_R = crate::BitReader<KEYWUPEN_A>;
impl KEYWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KEYWUPEN_A {
        match self.bits {
            false => KEYWUPEN_A::_0,
            true => KEYWUPEN_A::_1,
        }
    }
    ///S/W standby returns by KEY interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KEYWUPEN_A::_0
    }
    ///S/W standby returns by KEY interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KEYWUPEN_A::_1
    }
}
///Field `KEYWUPEN` writer - Key interrupt S/W standby returns enable bit
pub type KEYWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, KEYWUPEN_A>;
impl<'a, REG> KEYWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by KEY interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KEYWUPEN_A::_0)
    }
    ///S/W standby returns by KEY interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KEYWUPEN_A::_1)
    }
}
/**LVD1 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD1WUPEN_A {
    ///0: S/W standby returns by LVD1 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by LVD1 interrupt is enabled
    _1 = 1,
}
impl From<LVD1WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD1WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD1WUPEN` reader - LVD1 interrupt S/W standby returns enable bit
pub type LVD1WUPEN_R = crate::BitReader<LVD1WUPEN_A>;
impl LVD1WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD1WUPEN_A {
        match self.bits {
            false => LVD1WUPEN_A::_0,
            true => LVD1WUPEN_A::_1,
        }
    }
    ///S/W standby returns by LVD1 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD1WUPEN_A::_0
    }
    ///S/W standby returns by LVD1 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD1WUPEN_A::_1
    }
}
///Field `LVD1WUPEN` writer - LVD1 interrupt S/W standby returns enable bit
pub type LVD1WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, LVD1WUPEN_A>;
impl<'a, REG> LVD1WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by LVD1 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1WUPEN_A::_0)
    }
    ///S/W standby returns by LVD1 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD1WUPEN_A::_1)
    }
}
/**LVD2 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVD2WUPEN_A {
    ///0: S/W standby returns by LVD2 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by LVD2 interrupt is enabled
    _1 = 1,
}
impl From<LVD2WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LVD2WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LVD2WUPEN` reader - LVD2 interrupt S/W standby returns enable bit
pub type LVD2WUPEN_R = crate::BitReader<LVD2WUPEN_A>;
impl LVD2WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LVD2WUPEN_A {
        match self.bits {
            false => LVD2WUPEN_A::_0,
            true => LVD2WUPEN_A::_1,
        }
    }
    ///S/W standby returns by LVD2 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVD2WUPEN_A::_0
    }
    ///S/W standby returns by LVD2 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVD2WUPEN_A::_1
    }
}
///Field `LVD2WUPEN` writer - LVD2 interrupt S/W standby returns enable bit
pub type LVD2WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, LVD2WUPEN_A>;
impl<'a, REG> LVD2WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by LVD2 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2WUPEN_A::_0)
    }
    ///S/W standby returns by LVD2 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(LVD2WUPEN_A::_1)
    }
}
/**ACMPHS0 interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMPHS0WUPEN_A {
    ///0: S/W standby returns by ACMPHS0 interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by ACMPHS0 interrupt is enabled
    _1 = 1,
}
impl From<ACMPHS0WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMPHS0WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ACMPHS0WUPEN` reader - ACMPHS0 interrupt S/W standby returns enable bit
pub type ACMPHS0WUPEN_R = crate::BitReader<ACMPHS0WUPEN_A>;
impl ACMPHS0WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACMPHS0WUPEN_A {
        match self.bits {
            false => ACMPHS0WUPEN_A::_0,
            true => ACMPHS0WUPEN_A::_1,
        }
    }
    ///S/W standby returns by ACMPHS0 interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACMPHS0WUPEN_A::_0
    }
    ///S/W standby returns by ACMPHS0 interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACMPHS0WUPEN_A::_1
    }
}
///Field `ACMPHS0WUPEN` writer - ACMPHS0 interrupt S/W standby returns enable bit
pub type ACMPHS0WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, ACMPHS0WUPEN_A>;
impl<'a, REG> ACMPHS0WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by ACMPHS0 interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ACMPHS0WUPEN_A::_0)
    }
    ///S/W standby returns by ACMPHS0 interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ACMPHS0WUPEN_A::_1)
    }
}
/**RTC alarm interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCALMWUPEN_A {
    ///0: S/W standby returns by RTC alarm interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by RTC alarm interrupt is enabled
    _1 = 1,
}
impl From<RTCALMWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCALMWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCALMWUPEN` reader - RTC alarm interrupt S/W standby returns enable bit
pub type RTCALMWUPEN_R = crate::BitReader<RTCALMWUPEN_A>;
impl RTCALMWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCALMWUPEN_A {
        match self.bits {
            false => RTCALMWUPEN_A::_0,
            true => RTCALMWUPEN_A::_1,
        }
    }
    ///S/W standby returns by RTC alarm interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCALMWUPEN_A::_0
    }
    ///S/W standby returns by RTC alarm interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCALMWUPEN_A::_1
    }
}
///Field `RTCALMWUPEN` writer - RTC alarm interrupt S/W standby returns enable bit
pub type RTCALMWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCALMWUPEN_A>;
impl<'a, REG> RTCALMWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by RTC alarm interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCALMWUPEN_A::_0)
    }
    ///S/W standby returns by RTC alarm interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCALMWUPEN_A::_1)
    }
}
/**RCT period interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCPRDWUPEN_A {
    ///0: S/W standby returns by RTC period interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by RTC period interrupt is enabled
    _1 = 1,
}
impl From<RTCPRDWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCPRDWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCPRDWUPEN` reader - RCT period interrupt S/W standby returns enable bit
pub type RTCPRDWUPEN_R = crate::BitReader<RTCPRDWUPEN_A>;
impl RTCPRDWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTCPRDWUPEN_A {
        match self.bits {
            false => RTCPRDWUPEN_A::_0,
            true => RTCPRDWUPEN_A::_1,
        }
    }
    ///S/W standby returns by RTC period interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCPRDWUPEN_A::_0
    }
    ///S/W standby returns by RTC period interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCPRDWUPEN_A::_1
    }
}
///Field `RTCPRDWUPEN` writer - RCT period interrupt S/W standby returns enable bit
pub type RTCPRDWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCPRDWUPEN_A>;
impl<'a, REG> RTCPRDWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by RTC period interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRDWUPEN_A::_0)
    }
    ///S/W standby returns by RTC period interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRDWUPEN_A::_1)
    }
}
/**USBHS interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBHSWUPEN_A {
    ///0: S/W standby returns by USBHS interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by USBHS interrupt is enabled
    _1 = 1,
}
impl From<USBHSWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBHSWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USBHSWUPEN` reader - USBHS interrupt S/W standby returns enable bit
pub type USBHSWUPEN_R = crate::BitReader<USBHSWUPEN_A>;
impl USBHSWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBHSWUPEN_A {
        match self.bits {
            false => USBHSWUPEN_A::_0,
            true => USBHSWUPEN_A::_1,
        }
    }
    ///S/W standby returns by USBHS interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBHSWUPEN_A::_0
    }
    ///S/W standby returns by USBHS interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBHSWUPEN_A::_1
    }
}
///Field `USBHSWUPEN` writer - USBHS interrupt S/W standby returns enable bit
pub type USBHSWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, USBHSWUPEN_A>;
impl<'a, REG> USBHSWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by USBHS interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBHSWUPEN_A::_0)
    }
    ///S/W standby returns by USBHS interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBHSWUPEN_A::_1)
    }
}
/**USBFS interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSWUPEN_A {
    ///0: S/W standby returns by USBFS interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by USBFS interrupt is enabled
    _1 = 1,
}
impl From<USBFSWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USBFSWUPEN` reader - USBFS interrupt S/W standby returns enable bit
pub type USBFSWUPEN_R = crate::BitReader<USBFSWUPEN_A>;
impl USBFSWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBFSWUPEN_A {
        match self.bits {
            false => USBFSWUPEN_A::_0,
            true => USBFSWUPEN_A::_1,
        }
    }
    ///S/W standby returns by USBFS interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBFSWUPEN_A::_0
    }
    ///S/W standby returns by USBFS interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBFSWUPEN_A::_1
    }
}
///Field `USBFSWUPEN` writer - USBFS interrupt S/W standby returns enable bit
pub type USBFSWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, USBFSWUPEN_A>;
impl<'a, REG> USBFSWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by USBFS interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSWUPEN_A::_0)
    }
    ///S/W standby returns by USBFS interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSWUPEN_A::_1)
    }
}
/**AGT1 underflow interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1UDWUPEN_A {
    ///0: S/W standby returns by AGT1 underflow interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by AGT1 underflow interrupt is enabled
    _1 = 1,
}
impl From<AGT1UDWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1UDWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1UDWUPEN` reader - AGT1 underflow interrupt S/W standby returns enable bit
pub type AGT1UDWUPEN_R = crate::BitReader<AGT1UDWUPEN_A>;
impl AGT1UDWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AGT1UDWUPEN_A {
        match self.bits {
            false => AGT1UDWUPEN_A::_0,
            true => AGT1UDWUPEN_A::_1,
        }
    }
    ///S/W standby returns by AGT1 underflow interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1UDWUPEN_A::_0
    }
    ///S/W standby returns by AGT1 underflow interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1UDWUPEN_A::_1
    }
}
///Field `AGT1UDWUPEN` writer - AGT1 underflow interrupt S/W standby returns enable bit
pub type AGT1UDWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, AGT1UDWUPEN_A>;
impl<'a, REG> AGT1UDWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by AGT1 underflow interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1UDWUPEN_A::_0)
    }
    ///S/W standby returns by AGT1 underflow interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1UDWUPEN_A::_1)
    }
}
/**AGT1 compare match A interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CAWUPEN_A {
    ///0: S/W standby returns by AGT1 compare match A interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by AGT1 compare match A interrupt is enabled
    _1 = 1,
}
impl From<AGT1CAWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1CAWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1CAWUPEN` reader - AGT1 compare match A interrupt S/W standby returns enable bit
pub type AGT1CAWUPEN_R = crate::BitReader<AGT1CAWUPEN_A>;
impl AGT1CAWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AGT1CAWUPEN_A {
        match self.bits {
            false => AGT1CAWUPEN_A::_0,
            true => AGT1CAWUPEN_A::_1,
        }
    }
    ///S/W standby returns by AGT1 compare match A interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1CAWUPEN_A::_0
    }
    ///S/W standby returns by AGT1 compare match A interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1CAWUPEN_A::_1
    }
}
///Field `AGT1CAWUPEN` writer - AGT1 compare match A interrupt S/W standby returns enable bit
pub type AGT1CAWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, AGT1CAWUPEN_A>;
impl<'a, REG> AGT1CAWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by AGT1 compare match A interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1CAWUPEN_A::_0)
    }
    ///S/W standby returns by AGT1 compare match A interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1CAWUPEN_A::_1)
    }
}
/**AGT1 compare match B interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AGT1CBWUPEN_A {
    ///0: S/W standby returns by AGT1 compare match B interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by AGT1 compare match B interrupt is enabled
    _1 = 1,
}
impl From<AGT1CBWUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: AGT1CBWUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AGT1CBWUPEN` reader - AGT1 compare match B interrupt S/W standby returns enable bit
pub type AGT1CBWUPEN_R = crate::BitReader<AGT1CBWUPEN_A>;
impl AGT1CBWUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AGT1CBWUPEN_A {
        match self.bits {
            false => AGT1CBWUPEN_A::_0,
            true => AGT1CBWUPEN_A::_1,
        }
    }
    ///S/W standby returns by AGT1 compare match B interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AGT1CBWUPEN_A::_0
    }
    ///S/W standby returns by AGT1 compare match B interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AGT1CBWUPEN_A::_1
    }
}
///Field `AGT1CBWUPEN` writer - AGT1 compare match B interrupt S/W standby returns enable bit
pub type AGT1CBWUPEN_W<'a, REG> = crate::BitWriter<'a, REG, AGT1CBWUPEN_A>;
impl<'a, REG> AGT1CBWUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by AGT1 compare match B interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1CBWUPEN_A::_0)
    }
    ///S/W standby returns by AGT1 compare match B interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(AGT1CBWUPEN_A::_1)
    }
}
/**IIC0 address match interrupt S/W standby returns enable bit

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IIC0WUPEN_A {
    ///0: S/W standby returns by IIC0 address match interrupt is disabled
    _0 = 0,
    ///1: S/W standby returns by IIC0 address match interrupt is enabled
    _1 = 1,
}
impl From<IIC0WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IIC0WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IIC0WUPEN` reader - IIC0 address match interrupt S/W standby returns enable bit
pub type IIC0WUPEN_R = crate::BitReader<IIC0WUPEN_A>;
impl IIC0WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IIC0WUPEN_A {
        match self.bits {
            false => IIC0WUPEN_A::_0,
            true => IIC0WUPEN_A::_1,
        }
    }
    ///S/W standby returns by IIC0 address match interrupt is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IIC0WUPEN_A::_0
    }
    ///S/W standby returns by IIC0 address match interrupt is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IIC0WUPEN_A::_1
    }
}
///Field `IIC0WUPEN` writer - IIC0 address match interrupt S/W standby returns enable bit
pub type IIC0WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, IIC0WUPEN_A>;
impl<'a, REG> IIC0WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///S/W standby returns by IIC0 address match interrupt is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IIC0WUPEN_A::_0)
    }
    ///S/W standby returns by IIC0 address match interrupt is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IIC0WUPEN_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen0(&self) -> IRQWUPEN0_R {
        IRQWUPEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen1(&self) -> IRQWUPEN1_R {
        IRQWUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen2(&self) -> IRQWUPEN2_R {
        IRQWUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen3(&self) -> IRQWUPEN3_R {
        IRQWUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen4(&self) -> IRQWUPEN4_R {
        IRQWUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen5(&self) -> IRQWUPEN5_R {
        IRQWUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen6(&self) -> IRQWUPEN6_R {
        IRQWUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen7(&self) -> IRQWUPEN7_R {
        IRQWUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IRQ8 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen8(&self) -> IRQWUPEN8_R {
        IRQWUPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IRQ9 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen9(&self) -> IRQWUPEN9_R {
        IRQWUPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IRQ10 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen10(&self) -> IRQWUPEN10_R {
        IRQWUPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - IRQ11 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen11(&self) -> IRQWUPEN11_R {
        IRQWUPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IRQ12 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen12(&self) -> IRQWUPEN12_R {
        IRQWUPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - IRQ13 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen13(&self) -> IRQWUPEN13_R {
        IRQWUPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - IRQ14 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen14(&self) -> IRQWUPEN14_R {
        IRQWUPEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - IRQ15 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen15(&self) -> IRQWUPEN15_R {
        IRQWUPEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IWDT interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn iwdtwupen(&self) -> IWDTWUPEN_R {
        IWDTWUPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Key interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn keywupen(&self) -> KEYWUPEN_R {
        KEYWUPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LVD1 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn lvd1wupen(&self) -> LVD1WUPEN_R {
        LVD1WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - LVD2 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn lvd2wupen(&self) -> LVD2WUPEN_R {
        LVD2WUPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - ACMPHS0 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn acmphs0wupen(&self) -> ACMPHS0WUPEN_R {
        ACMPHS0WUPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - RTC alarm interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn rtcalmwupen(&self) -> RTCALMWUPEN_R {
        RTCALMWUPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - RCT period interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn rtcprdwupen(&self) -> RTCPRDWUPEN_R {
        RTCPRDWUPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USBHS interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn usbhswupen(&self) -> USBHSWUPEN_R {
        USBHSWUPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USBFS interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn usbfswupen(&self) -> USBFSWUPEN_R {
        USBFSWUPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - AGT1 underflow interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn agt1udwupen(&self) -> AGT1UDWUPEN_R {
        AGT1UDWUPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - AGT1 compare match A interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn agt1cawupen(&self) -> AGT1CAWUPEN_R {
        AGT1CAWUPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AGT1 compare match B interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn agt1cbwupen(&self) -> AGT1CBWUPEN_R {
        AGT1CBWUPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - IIC0 address match interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn iic0wupen(&self) -> IIC0WUPEN_R {
        IIC0WUPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ0 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen0(&mut self) -> IRQWUPEN0_W<WUPEN_SPEC> {
        IRQWUPEN0_W::new(self, 0)
    }
    ///Bit 1 - IRQ1 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen1(&mut self) -> IRQWUPEN1_W<WUPEN_SPEC> {
        IRQWUPEN1_W::new(self, 1)
    }
    ///Bit 2 - IRQ2 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen2(&mut self) -> IRQWUPEN2_W<WUPEN_SPEC> {
        IRQWUPEN2_W::new(self, 2)
    }
    ///Bit 3 - IRQ3 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen3(&mut self) -> IRQWUPEN3_W<WUPEN_SPEC> {
        IRQWUPEN3_W::new(self, 3)
    }
    ///Bit 4 - IRQ4 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen4(&mut self) -> IRQWUPEN4_W<WUPEN_SPEC> {
        IRQWUPEN4_W::new(self, 4)
    }
    ///Bit 5 - IRQ5 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen5(&mut self) -> IRQWUPEN5_W<WUPEN_SPEC> {
        IRQWUPEN5_W::new(self, 5)
    }
    ///Bit 6 - IRQ6 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen6(&mut self) -> IRQWUPEN6_W<WUPEN_SPEC> {
        IRQWUPEN6_W::new(self, 6)
    }
    ///Bit 7 - IRQ7 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen7(&mut self) -> IRQWUPEN7_W<WUPEN_SPEC> {
        IRQWUPEN7_W::new(self, 7)
    }
    ///Bit 8 - IRQ8 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen8(&mut self) -> IRQWUPEN8_W<WUPEN_SPEC> {
        IRQWUPEN8_W::new(self, 8)
    }
    ///Bit 9 - IRQ9 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen9(&mut self) -> IRQWUPEN9_W<WUPEN_SPEC> {
        IRQWUPEN9_W::new(self, 9)
    }
    ///Bit 10 - IRQ10 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen10(&mut self) -> IRQWUPEN10_W<WUPEN_SPEC> {
        IRQWUPEN10_W::new(self, 10)
    }
    ///Bit 11 - IRQ11 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen11(&mut self) -> IRQWUPEN11_W<WUPEN_SPEC> {
        IRQWUPEN11_W::new(self, 11)
    }
    ///Bit 12 - IRQ12 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen12(&mut self) -> IRQWUPEN12_W<WUPEN_SPEC> {
        IRQWUPEN12_W::new(self, 12)
    }
    ///Bit 13 - IRQ13 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen13(&mut self) -> IRQWUPEN13_W<WUPEN_SPEC> {
        IRQWUPEN13_W::new(self, 13)
    }
    ///Bit 14 - IRQ14 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen14(&mut self) -> IRQWUPEN14_W<WUPEN_SPEC> {
        IRQWUPEN14_W::new(self, 14)
    }
    ///Bit 15 - IRQ15 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn irqwupen15(&mut self) -> IRQWUPEN15_W<WUPEN_SPEC> {
        IRQWUPEN15_W::new(self, 15)
    }
    ///Bit 16 - IWDT interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn iwdtwupen(&mut self) -> IWDTWUPEN_W<WUPEN_SPEC> {
        IWDTWUPEN_W::new(self, 16)
    }
    ///Bit 17 - Key interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn keywupen(&mut self) -> KEYWUPEN_W<WUPEN_SPEC> {
        KEYWUPEN_W::new(self, 17)
    }
    ///Bit 18 - LVD1 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn lvd1wupen(&mut self) -> LVD1WUPEN_W<WUPEN_SPEC> {
        LVD1WUPEN_W::new(self, 18)
    }
    ///Bit 19 - LVD2 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn lvd2wupen(&mut self) -> LVD2WUPEN_W<WUPEN_SPEC> {
        LVD2WUPEN_W::new(self, 19)
    }
    ///Bit 22 - ACMPHS0 interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn acmphs0wupen(&mut self) -> ACMPHS0WUPEN_W<WUPEN_SPEC> {
        ACMPHS0WUPEN_W::new(self, 22)
    }
    ///Bit 24 - RTC alarm interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn rtcalmwupen(&mut self) -> RTCALMWUPEN_W<WUPEN_SPEC> {
        RTCALMWUPEN_W::new(self, 24)
    }
    ///Bit 25 - RCT period interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn rtcprdwupen(&mut self) -> RTCPRDWUPEN_W<WUPEN_SPEC> {
        RTCPRDWUPEN_W::new(self, 25)
    }
    ///Bit 26 - USBHS interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn usbhswupen(&mut self) -> USBHSWUPEN_W<WUPEN_SPEC> {
        USBHSWUPEN_W::new(self, 26)
    }
    ///Bit 27 - USBFS interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn usbfswupen(&mut self) -> USBFSWUPEN_W<WUPEN_SPEC> {
        USBFSWUPEN_W::new(self, 27)
    }
    ///Bit 28 - AGT1 underflow interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn agt1udwupen(&mut self) -> AGT1UDWUPEN_W<WUPEN_SPEC> {
        AGT1UDWUPEN_W::new(self, 28)
    }
    ///Bit 29 - AGT1 compare match A interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn agt1cawupen(&mut self) -> AGT1CAWUPEN_W<WUPEN_SPEC> {
        AGT1CAWUPEN_W::new(self, 29)
    }
    ///Bit 30 - AGT1 compare match B interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn agt1cbwupen(&mut self) -> AGT1CBWUPEN_W<WUPEN_SPEC> {
        AGT1CBWUPEN_W::new(self, 30)
    }
    ///Bit 31 - IIC0 address match interrupt S/W standby returns enable bit
    #[inline(always)]
    pub fn iic0wupen(&mut self) -> IIC0WUPEN_W<WUPEN_SPEC> {
        IIC0WUPEN_W::new(self, 31)
    }
}
/**Wake Up interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`wupen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WUPEN_SPEC;
impl crate::RegisterSpec for WUPEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wupen::R`](R) reader structure
impl crate::Readable for WUPEN_SPEC {}
///`write(|w| ..)` method takes [`wupen::W`](W) writer structure
impl crate::Writable for WUPEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUPEN to value 0
impl crate::Resettable for WUPEN_SPEC {}
