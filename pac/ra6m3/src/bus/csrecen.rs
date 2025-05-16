///Register `CSRECEN` reader
pub type R = crate::R<CSRECEN_SPEC>;
///Register `CSRECEN` writer
pub type W = crate::W<CSRECEN_SPEC>;
/**Separate Bus Recovery Cycle Insertion Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN0_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN0_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN0` reader - Separate Bus Recovery Cycle Insertion Enable 0
pub type RCVEN0_R = crate::BitReader<RCVEN0_A>;
impl RCVEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN0_A {
        match self.bits {
            false => RCVEN0_A::_0,
            true => RCVEN0_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN0_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN0_A::_1
    }
}
///Field `RCVEN0` writer - Separate Bus Recovery Cycle Insertion Enable 0
pub type RCVEN0_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN0_A>;
impl<'a, REG> RCVEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN0_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN0_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN1_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN1_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN1` reader - Separate Bus Recovery Cycle Insertion Enable 1
pub type RCVEN1_R = crate::BitReader<RCVEN1_A>;
impl RCVEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN1_A {
        match self.bits {
            false => RCVEN1_A::_0,
            true => RCVEN1_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN1_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN1_A::_1
    }
}
///Field `RCVEN1` writer - Separate Bus Recovery Cycle Insertion Enable 1
pub type RCVEN1_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN1_A>;
impl<'a, REG> RCVEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN1_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN1_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN2_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN2_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN2` reader - Separate Bus Recovery Cycle Insertion Enable 2
pub type RCVEN2_R = crate::BitReader<RCVEN2_A>;
impl RCVEN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN2_A {
        match self.bits {
            false => RCVEN2_A::_0,
            true => RCVEN2_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN2_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN2_A::_1
    }
}
///Field `RCVEN2` writer - Separate Bus Recovery Cycle Insertion Enable 2
pub type RCVEN2_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN2_A>;
impl<'a, REG> RCVEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN2_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN2_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 3

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN3_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN3_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN3` reader - Separate Bus Recovery Cycle Insertion Enable 3
pub type RCVEN3_R = crate::BitReader<RCVEN3_A>;
impl RCVEN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN3_A {
        match self.bits {
            false => RCVEN3_A::_0,
            true => RCVEN3_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN3_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN3_A::_1
    }
}
///Field `RCVEN3` writer - Separate Bus Recovery Cycle Insertion Enable 3
pub type RCVEN3_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN3_A>;
impl<'a, REG> RCVEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN3_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN3_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 4

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN4_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN4_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN4` reader - Separate Bus Recovery Cycle Insertion Enable 4
pub type RCVEN4_R = crate::BitReader<RCVEN4_A>;
impl RCVEN4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN4_A {
        match self.bits {
            false => RCVEN4_A::_0,
            true => RCVEN4_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN4_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN4_A::_1
    }
}
///Field `RCVEN4` writer - Separate Bus Recovery Cycle Insertion Enable 4
pub type RCVEN4_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN4_A>;
impl<'a, REG> RCVEN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN4_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN4_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 5

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN5_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN5_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN5` reader - Separate Bus Recovery Cycle Insertion Enable 5
pub type RCVEN5_R = crate::BitReader<RCVEN5_A>;
impl RCVEN5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN5_A {
        match self.bits {
            false => RCVEN5_A::_0,
            true => RCVEN5_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN5_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN5_A::_1
    }
}
///Field `RCVEN5` writer - Separate Bus Recovery Cycle Insertion Enable 5
pub type RCVEN5_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN5_A>;
impl<'a, REG> RCVEN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN5_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN5_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN6_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN6_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN6` reader - Separate Bus Recovery Cycle Insertion Enable 6
pub type RCVEN6_R = crate::BitReader<RCVEN6_A>;
impl RCVEN6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN6_A {
        match self.bits {
            false => RCVEN6_A::_0,
            true => RCVEN6_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN6_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN6_A::_1
    }
}
///Field `RCVEN6` writer - Separate Bus Recovery Cycle Insertion Enable 6
pub type RCVEN6_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN6_A>;
impl<'a, REG> RCVEN6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN6_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN6_A::_1)
    }
}
/**Separate Bus Recovery Cycle Insertion Enable 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVEN7_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVEN7_A> for bool {
    #[inline(always)]
    fn from(variant: RCVEN7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVEN7` reader - Separate Bus Recovery Cycle Insertion Enable 7
pub type RCVEN7_R = crate::BitReader<RCVEN7_A>;
impl RCVEN7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVEN7_A {
        match self.bits {
            false => RCVEN7_A::_0,
            true => RCVEN7_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVEN7_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVEN7_A::_1
    }
}
///Field `RCVEN7` writer - Separate Bus Recovery Cycle Insertion Enable 7
pub type RCVEN7_W<'a, REG> = crate::BitWriter<'a, REG, RCVEN7_A>;
impl<'a, REG> RCVEN7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN7_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVEN7_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM0_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM0_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM0` reader - Multiplexed Bus Recovery Cycle Insertion Enable 0
pub type RCVENM0_R = crate::BitReader<RCVENM0_A>;
impl RCVENM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM0_A {
        match self.bits {
            false => RCVENM0_A::_0,
            true => RCVENM0_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM0_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM0_A::_1
    }
}
///Field `RCVENM0` writer - Multiplexed Bus Recovery Cycle Insertion Enable 0
pub type RCVENM0_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM0_A>;
impl<'a, REG> RCVENM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM0_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM0_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 1

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM1_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM1_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM1` reader - Multiplexed Bus Recovery Cycle Insertion Enable 1
pub type RCVENM1_R = crate::BitReader<RCVENM1_A>;
impl RCVENM1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM1_A {
        match self.bits {
            false => RCVENM1_A::_0,
            true => RCVENM1_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM1_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM1_A::_1
    }
}
///Field `RCVENM1` writer - Multiplexed Bus Recovery Cycle Insertion Enable 1
pub type RCVENM1_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM1_A>;
impl<'a, REG> RCVENM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM1_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM1_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 2

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM2_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM2_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM2` reader - Multiplexed Bus Recovery Cycle Insertion Enable 2
pub type RCVENM2_R = crate::BitReader<RCVENM2_A>;
impl RCVENM2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM2_A {
        match self.bits {
            false => RCVENM2_A::_0,
            true => RCVENM2_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM2_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM2_A::_1
    }
}
///Field `RCVENM2` writer - Multiplexed Bus Recovery Cycle Insertion Enable 2
pub type RCVENM2_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM2_A>;
impl<'a, REG> RCVENM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM2_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM2_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 3

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM3_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM3_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM3` reader - Multiplexed Bus Recovery Cycle Insertion Enable 3
pub type RCVENM3_R = crate::BitReader<RCVENM3_A>;
impl RCVENM3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM3_A {
        match self.bits {
            false => RCVENM3_A::_0,
            true => RCVENM3_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM3_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM3_A::_1
    }
}
///Field `RCVENM3` writer - Multiplexed Bus Recovery Cycle Insertion Enable 3
pub type RCVENM3_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM3_A>;
impl<'a, REG> RCVENM3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM3_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM3_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 4

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM4_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM4_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM4` reader - Multiplexed Bus Recovery Cycle Insertion Enable 4
pub type RCVENM4_R = crate::BitReader<RCVENM4_A>;
impl RCVENM4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM4_A {
        match self.bits {
            false => RCVENM4_A::_0,
            true => RCVENM4_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM4_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM4_A::_1
    }
}
///Field `RCVENM4` writer - Multiplexed Bus Recovery Cycle Insertion Enable 4
pub type RCVENM4_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM4_A>;
impl<'a, REG> RCVENM4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM4_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM4_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 5

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM5_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM5_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM5` reader - Multiplexed Bus Recovery Cycle Insertion Enable 5
pub type RCVENM5_R = crate::BitReader<RCVENM5_A>;
impl RCVENM5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM5_A {
        match self.bits {
            false => RCVENM5_A::_0,
            true => RCVENM5_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM5_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM5_A::_1
    }
}
///Field `RCVENM5` writer - Multiplexed Bus Recovery Cycle Insertion Enable 5
pub type RCVENM5_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM5_A>;
impl<'a, REG> RCVENM5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM5_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM5_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 6

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM6_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM6_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM6` reader - Multiplexed Bus Recovery Cycle Insertion Enable 6
pub type RCVENM6_R = crate::BitReader<RCVENM6_A>;
impl RCVENM6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM6_A {
        match self.bits {
            false => RCVENM6_A::_0,
            true => RCVENM6_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM6_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM6_A::_1
    }
}
///Field `RCVENM6` writer - Multiplexed Bus Recovery Cycle Insertion Enable 6
pub type RCVENM6_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM6_A>;
impl<'a, REG> RCVENM6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM6_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM6_A::_1)
    }
}
/**Multiplexed Bus Recovery Cycle Insertion Enable 7

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCVENM7_A {
    ///0: Recovery cycle insertion is disabled.
    _0 = 0,
    ///1: Recovery cycle insertion is enabled.
    _1 = 1,
}
impl From<RCVENM7_A> for bool {
    #[inline(always)]
    fn from(variant: RCVENM7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCVENM7` reader - Multiplexed Bus Recovery Cycle Insertion Enable 7
pub type RCVENM7_R = crate::BitReader<RCVENM7_A>;
impl RCVENM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCVENM7_A {
        match self.bits {
            false => RCVENM7_A::_0,
            true => RCVENM7_A::_1,
        }
    }
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCVENM7_A::_0
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCVENM7_A::_1
    }
}
///Field `RCVENM7` writer - Multiplexed Bus Recovery Cycle Insertion Enable 7
pub type RCVENM7_W<'a, REG> = crate::BitWriter<'a, REG, RCVENM7_A>;
impl<'a, REG> RCVENM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Recovery cycle insertion is disabled.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM7_A::_0)
    }
    ///Recovery cycle insertion is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCVENM7_A::_1)
    }
}
impl R {
    ///Bit 0 - Separate Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcven0(&self) -> RCVEN0_R {
        RCVEN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Separate Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcven1(&self) -> RCVEN1_R {
        RCVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Separate Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcven2(&self) -> RCVEN2_R {
        RCVEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Separate Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcven3(&self) -> RCVEN3_R {
        RCVEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Separate Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcven4(&self) -> RCVEN4_R {
        RCVEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Separate Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcven5(&self) -> RCVEN5_R {
        RCVEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Separate Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcven6(&self) -> RCVEN6_R {
        RCVEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Separate Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcven7(&self) -> RCVEN7_R {
        RCVEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Multiplexed Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcvenm0(&self) -> RCVENM0_R {
        RCVENM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Multiplexed Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcvenm1(&self) -> RCVENM1_R {
        RCVENM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Multiplexed Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcvenm2(&self) -> RCVENM2_R {
        RCVENM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Multiplexed Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcvenm3(&self) -> RCVENM3_R {
        RCVENM3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Multiplexed Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcvenm4(&self) -> RCVENM4_R {
        RCVENM4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Multiplexed Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcvenm5(&self) -> RCVENM5_R {
        RCVENM5_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Multiplexed Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcvenm6(&self) -> RCVENM6_R {
        RCVENM6_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Multiplexed Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcvenm7(&self) -> RCVENM7_R {
        RCVENM7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Separate Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcven0(&mut self) -> RCVEN0_W<CSRECEN_SPEC> {
        RCVEN0_W::new(self, 0)
    }
    ///Bit 1 - Separate Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcven1(&mut self) -> RCVEN1_W<CSRECEN_SPEC> {
        RCVEN1_W::new(self, 1)
    }
    ///Bit 2 - Separate Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcven2(&mut self) -> RCVEN2_W<CSRECEN_SPEC> {
        RCVEN2_W::new(self, 2)
    }
    ///Bit 3 - Separate Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcven3(&mut self) -> RCVEN3_W<CSRECEN_SPEC> {
        RCVEN3_W::new(self, 3)
    }
    ///Bit 4 - Separate Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcven4(&mut self) -> RCVEN4_W<CSRECEN_SPEC> {
        RCVEN4_W::new(self, 4)
    }
    ///Bit 5 - Separate Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcven5(&mut self) -> RCVEN5_W<CSRECEN_SPEC> {
        RCVEN5_W::new(self, 5)
    }
    ///Bit 6 - Separate Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcven6(&mut self) -> RCVEN6_W<CSRECEN_SPEC> {
        RCVEN6_W::new(self, 6)
    }
    ///Bit 7 - Separate Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcven7(&mut self) -> RCVEN7_W<CSRECEN_SPEC> {
        RCVEN7_W::new(self, 7)
    }
    ///Bit 8 - Multiplexed Bus Recovery Cycle Insertion Enable 0
    #[inline(always)]
    pub fn rcvenm0(&mut self) -> RCVENM0_W<CSRECEN_SPEC> {
        RCVENM0_W::new(self, 8)
    }
    ///Bit 9 - Multiplexed Bus Recovery Cycle Insertion Enable 1
    #[inline(always)]
    pub fn rcvenm1(&mut self) -> RCVENM1_W<CSRECEN_SPEC> {
        RCVENM1_W::new(self, 9)
    }
    ///Bit 10 - Multiplexed Bus Recovery Cycle Insertion Enable 2
    #[inline(always)]
    pub fn rcvenm2(&mut self) -> RCVENM2_W<CSRECEN_SPEC> {
        RCVENM2_W::new(self, 10)
    }
    ///Bit 11 - Multiplexed Bus Recovery Cycle Insertion Enable 3
    #[inline(always)]
    pub fn rcvenm3(&mut self) -> RCVENM3_W<CSRECEN_SPEC> {
        RCVENM3_W::new(self, 11)
    }
    ///Bit 12 - Multiplexed Bus Recovery Cycle Insertion Enable 4
    #[inline(always)]
    pub fn rcvenm4(&mut self) -> RCVENM4_W<CSRECEN_SPEC> {
        RCVENM4_W::new(self, 12)
    }
    ///Bit 13 - Multiplexed Bus Recovery Cycle Insertion Enable 5
    #[inline(always)]
    pub fn rcvenm5(&mut self) -> RCVENM5_W<CSRECEN_SPEC> {
        RCVENM5_W::new(self, 13)
    }
    ///Bit 14 - Multiplexed Bus Recovery Cycle Insertion Enable 6
    #[inline(always)]
    pub fn rcvenm6(&mut self) -> RCVENM6_W<CSRECEN_SPEC> {
        RCVENM6_W::new(self, 14)
    }
    ///Bit 15 - Multiplexed Bus Recovery Cycle Insertion Enable 7
    #[inline(always)]
    pub fn rcvenm7(&mut self) -> RCVENM7_W<CSRECEN_SPEC> {
        RCVENM7_W::new(self, 15)
    }
}
/**CS Recovery Cycle Insertion Enable Register

You can [`read`](crate::Reg::read) this register and get [`csrecen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrecen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CSRECEN_SPEC;
impl crate::RegisterSpec for CSRECEN_SPEC {
    type Ux = u16;
}
///`read()` method returns [`csrecen::R`](R) reader structure
impl crate::Readable for CSRECEN_SPEC {}
///`write(|w| ..)` method takes [`csrecen::W`](W) writer structure
impl crate::Writable for CSRECEN_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSRECEN to value 0x3e3e
impl crate::Resettable for CSRECEN_SPEC {
    const RESET_VALUE: u16 = 0x3e3e;
}
