///Register `GTSTP` reader
pub type R = crate::R<GTSTP_SPEC>;
///Register `GTSTP` writer
pub type W = crate::W<GTSTP_SPEC>;
/**Channel 0 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP0_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32EH0.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP0` reader - Channel 0 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP0_R = crate::BitReader<CSTOP0_A>;
impl CSTOP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP0_A {
        match self.bits {
            false => CSTOP0_A::_0,
            true => CSTOP0_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP0_A::_0
    }
    ///GPT32EH0.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP0_A::_1
    }
}
///Field `CSTOP0` writer - Channel 0 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP0_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP0_A>;
impl<'a, REG> CSTOP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP0_A::_0)
    }
    ///GPT32EH0.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP0_A::_1)
    }
}
/**Channel 1 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP1_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32EH1.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP1` reader - Channel 1 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP1_R = crate::BitReader<CSTOP1_A>;
impl CSTOP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP1_A {
        match self.bits {
            false => CSTOP1_A::_0,
            true => CSTOP1_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP1_A::_0
    }
    ///GPT32EH1.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP1_A::_1
    }
}
///Field `CSTOP1` writer - Channel 1 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP1_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP1_A>;
impl<'a, REG> CSTOP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP1_A::_0)
    }
    ///GPT32EH1.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP1_A::_1)
    }
}
/**Channel 2 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP2_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32EH2.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP2` reader - Channel 2 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP2_R = crate::BitReader<CSTOP2_A>;
impl CSTOP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP2_A {
        match self.bits {
            false => CSTOP2_A::_0,
            true => CSTOP2_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP2_A::_0
    }
    ///GPT32EH2.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP2_A::_1
    }
}
///Field `CSTOP2` writer - Channel 2 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP2_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP2_A>;
impl<'a, REG> CSTOP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP2_A::_0)
    }
    ///GPT32EH2.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP2_A::_1)
    }
}
/**Channel 3 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP3_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32EH3.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP3` reader - Channel 3 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP3_R = crate::BitReader<CSTOP3_A>;
impl CSTOP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP3_A {
        match self.bits {
            false => CSTOP3_A::_0,
            true => CSTOP3_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP3_A::_0
    }
    ///GPT32EH3.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP3_A::_1
    }
}
///Field `CSTOP3` writer - Channel 3 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP3_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP3_A>;
impl<'a, REG> CSTOP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP3_A::_0)
    }
    ///GPT32EH3.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP3_A::_1)
    }
}
/**Channel 4 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP4_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32E4.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP4` reader - Channel 4 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP4_R = crate::BitReader<CSTOP4_A>;
impl CSTOP4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP4_A {
        match self.bits {
            false => CSTOP4_A::_0,
            true => CSTOP4_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP4_A::_0
    }
    ///GPT32E4.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP4_A::_1
    }
}
///Field `CSTOP4` writer - Channel 4 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP4_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP4_A>;
impl<'a, REG> CSTOP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP4_A::_0)
    }
    ///GPT32E4.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP4_A::_1)
    }
}
/**Channel 5 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP5_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32E5.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP5` reader - Channel 5 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP5_R = crate::BitReader<CSTOP5_A>;
impl CSTOP5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP5_A {
        match self.bits {
            false => CSTOP5_A::_0,
            true => CSTOP5_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP5_A::_0
    }
    ///GPT32E5.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP5_A::_1
    }
}
///Field `CSTOP5` writer - Channel 5 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP5_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP5_A>;
impl<'a, REG> CSTOP5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP5_A::_0)
    }
    ///GPT32E5.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP5_A::_1)
    }
}
/**Channel 6 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP6_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32E6.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP6` reader - Channel 6 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP6_R = crate::BitReader<CSTOP6_A>;
impl CSTOP6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP6_A {
        match self.bits {
            false => CSTOP6_A::_0,
            true => CSTOP6_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP6_A::_0
    }
    ///GPT32E6.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP6_A::_1
    }
}
///Field `CSTOP6` writer - Channel 6 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP6_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP6_A>;
impl<'a, REG> CSTOP6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP6_A::_0)
    }
    ///GPT32E6.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP6_A::_1)
    }
}
/**Channel 7 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP7_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT32E7.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP7` reader - Channel 7 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP7_R = crate::BitReader<CSTOP7_A>;
impl CSTOP7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP7_A {
        match self.bits {
            false => CSTOP7_A::_0,
            true => CSTOP7_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP7_A::_0
    }
    ///GPT32E7.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP7_A::_1
    }
}
///Field `CSTOP7` writer - Channel 7 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP7_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP7_A>;
impl<'a, REG> CSTOP7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP7_A::_0)
    }
    ///GPT32E7.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP7_A::_1)
    }
}
/**Channel 8 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP8_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT328.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP8_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP8` reader - Channel 8 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP8_R = crate::BitReader<CSTOP8_A>;
impl CSTOP8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP8_A {
        match self.bits {
            false => CSTOP8_A::_0,
            true => CSTOP8_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP8_A::_0
    }
    ///GPT328.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP8_A::_1
    }
}
///Field `CSTOP8` writer - Channel 8 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP8_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP8_A>;
impl<'a, REG> CSTOP8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP8_A::_0)
    }
    ///GPT328.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP8_A::_1)
    }
}
/**Channel 9 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP9_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT329.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP9_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP9` reader - Channel 9 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP9_R = crate::BitReader<CSTOP9_A>;
impl CSTOP9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP9_A {
        match self.bits {
            false => CSTOP9_A::_0,
            true => CSTOP9_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP9_A::_0
    }
    ///GPT329.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP9_A::_1
    }
}
///Field `CSTOP9` writer - Channel 9 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP9_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP9_A>;
impl<'a, REG> CSTOP9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP9_A::_0)
    }
    ///GPT329.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP9_A::_1)
    }
}
/**Channel 10 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP10_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT3210.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP10_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP10` reader - Channel 10 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP10_R = crate::BitReader<CSTOP10_A>;
impl CSTOP10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP10_A {
        match self.bits {
            false => CSTOP10_A::_0,
            true => CSTOP10_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP10_A::_0
    }
    ///GPT3210.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP10_A::_1
    }
}
///Field `CSTOP10` writer - Channel 10 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP10_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP10_A>;
impl<'a, REG> CSTOP10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP10_A::_0)
    }
    ///GPT3210.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP10_A::_1)
    }
}
/**Channel 11 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP11_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT3211.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP11_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP11` reader - Channel 11 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP11_R = crate::BitReader<CSTOP11_A>;
impl CSTOP11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP11_A {
        match self.bits {
            false => CSTOP11_A::_0,
            true => CSTOP11_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP11_A::_0
    }
    ///GPT3211.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP11_A::_1
    }
}
///Field `CSTOP11` writer - Channel 11 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP11_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP11_A>;
impl<'a, REG> CSTOP11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP11_A::_0)
    }
    ///GPT3211.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP11_A::_1)
    }
}
/**Channel 12 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP12_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT3212.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP12_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP12` reader - Channel 12 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP12_R = crate::BitReader<CSTOP12_A>;
impl CSTOP12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP12_A {
        match self.bits {
            false => CSTOP12_A::_0,
            true => CSTOP12_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP12_A::_0
    }
    ///GPT3212.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP12_A::_1
    }
}
///Field `CSTOP12` writer - Channel 12 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP12_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP12_A>;
impl<'a, REG> CSTOP12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP12_A::_0)
    }
    ///GPT3212.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP12_A::_1)
    }
}
/**Channel 13 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP13_A {
    ///0: No effect (write) / counter running (read)
    _0 = 0,
    ///1: GPT3213.GTCNT counter stops (write) / Counter stop (read)
    _1 = 1,
}
impl From<CSTOP13_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP13` reader - Channel 13 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP13_R = crate::BitReader<CSTOP13_A>;
impl CSTOP13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP13_A {
        match self.bits {
            false => CSTOP13_A::_0,
            true => CSTOP13_A::_1,
        }
    }
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP13_A::_0
    }
    ///GPT3213.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP13_A::_1
    }
}
///Field `CSTOP13` writer - Channel 13 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
pub type CSTOP13_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP13_A>;
impl<'a, REG> CSTOP13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter running (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP13_A::_0)
    }
    ///GPT3213.GTCNT counter stops (write) / Counter stop (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP13_A::_1)
    }
}
impl R {
    ///Bit 0 - Channel 0 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop0(&self) -> CSTOP0_R {
        CSTOP0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop1(&self) -> CSTOP1_R {
        CSTOP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 2 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop2(&self) -> CSTOP2_R {
        CSTOP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 3 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop3(&self) -> CSTOP3_R {
        CSTOP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel 4 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop4(&self) -> CSTOP4_R {
        CSTOP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel 5 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop5(&self) -> CSTOP5_R {
        CSTOP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel 6 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop6(&self) -> CSTOP6_R {
        CSTOP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 7 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop7(&self) -> CSTOP7_R {
        CSTOP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel 8 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop8(&self) -> CSTOP8_R {
        CSTOP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel 9 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop9(&self) -> CSTOP9_R {
        CSTOP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel 10 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop10(&self) -> CSTOP10_R {
        CSTOP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel 11 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop11(&self) -> CSTOP11_R {
        CSTOP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel 12 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop12(&self) -> CSTOP12_R {
        CSTOP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel 13 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop13(&self) -> CSTOP13_R {
        CSTOP13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel 0 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop0(&mut self) -> CSTOP0_W<GTSTP_SPEC> {
        CSTOP0_W::new(self, 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop1(&mut self) -> CSTOP1_W<GTSTP_SPEC> {
        CSTOP1_W::new(self, 1)
    }
    ///Bit 2 - Channel 2 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop2(&mut self) -> CSTOP2_W<GTSTP_SPEC> {
        CSTOP2_W::new(self, 2)
    }
    ///Bit 3 - Channel 3 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop3(&mut self) -> CSTOP3_W<GTSTP_SPEC> {
        CSTOP3_W::new(self, 3)
    }
    ///Bit 4 - Channel 4 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop4(&mut self) -> CSTOP4_W<GTSTP_SPEC> {
        CSTOP4_W::new(self, 4)
    }
    ///Bit 5 - Channel 5 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop5(&mut self) -> CSTOP5_W<GTSTP_SPEC> {
        CSTOP5_W::new(self, 5)
    }
    ///Bit 6 - Channel 6 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop6(&mut self) -> CSTOP6_W<GTSTP_SPEC> {
        CSTOP6_W::new(self, 6)
    }
    ///Bit 7 - Channel 7 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop7(&mut self) -> CSTOP7_W<GTSTP_SPEC> {
        CSTOP7_W::new(self, 7)
    }
    ///Bit 8 - Channel 8 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop8(&mut self) -> CSTOP8_W<GTSTP_SPEC> {
        CSTOP8_W::new(self, 8)
    }
    ///Bit 9 - Channel 9 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop9(&mut self) -> CSTOP9_W<GTSTP_SPEC> {
        CSTOP9_W::new(self, 9)
    }
    ///Bit 10 - Channel 10 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop10(&mut self) -> CSTOP10_W<GTSTP_SPEC> {
        CSTOP10_W::new(self, 10)
    }
    ///Bit 11 - Channel 11 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop11(&mut self) -> CSTOP11_W<GTSTP_SPEC> {
        CSTOP11_W::new(self, 11)
    }
    ///Bit 12 - Channel 12 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop12(&mut self) -> CSTOP12_W<GTSTP_SPEC> {
        CSTOP12_W::new(self, 12)
    }
    ///Bit 13 - Channel 13 GTCNT Count StopRead data shows each channel's counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop.
    #[inline(always)]
    pub fn cstop13(&mut self) -> CSTOP13_W<GTSTP_SPEC> {
        CSTOP13_W::new(self, 13)
    }
}
/**General PWM Timer Software Stop Register

You can [`read`](crate::Reg::read) this register and get [`gtstp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTSTP_SPEC;
impl crate::RegisterSpec for GTSTP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtstp::R`](R) reader structure
impl crate::Readable for GTSTP_SPEC {}
///`write(|w| ..)` method takes [`gtstp::W`](W) writer structure
impl crate::Writable for GTSTP_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSTP to value 0xffff_ffff
impl crate::Resettable for GTSTP_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
