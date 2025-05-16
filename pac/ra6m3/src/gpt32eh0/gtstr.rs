///Register `GTSTR` reader
pub type R = crate::R<GTSTR_SPEC>;
///Register `GTSTR` writer
pub type W = crate::W<GTSTR_SPEC>;
/**Channel 0 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT0_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32EH0.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT0_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT0` reader - Channel 0 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT0_R = crate::BitReader<CSTRT0_A>;
impl CSTRT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT0_A {
        match self.bits {
            false => CSTRT0_A::_0,
            true => CSTRT0_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT0_A::_0
    }
    ///GPT32EH0.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT0_A::_1
    }
}
///Field `CSTRT0` writer - Channel 0 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT0_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT0_A>;
impl<'a, REG> CSTRT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT0_A::_0)
    }
    ///GPT32EH0.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT0_A::_1)
    }
}
/**Channel 1 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT1_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32EH1.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT1_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT1` reader - Channel 1 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT1_R = crate::BitReader<CSTRT1_A>;
impl CSTRT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT1_A {
        match self.bits {
            false => CSTRT1_A::_0,
            true => CSTRT1_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT1_A::_0
    }
    ///GPT32EH1.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT1_A::_1
    }
}
///Field `CSTRT1` writer - Channel 1 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT1_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT1_A>;
impl<'a, REG> CSTRT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT1_A::_0)
    }
    ///GPT32EH1.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT1_A::_1)
    }
}
/**Channel 2 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT2_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32EH2.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT2_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT2` reader - Channel 2 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT2_R = crate::BitReader<CSTRT2_A>;
impl CSTRT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT2_A {
        match self.bits {
            false => CSTRT2_A::_0,
            true => CSTRT2_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT2_A::_0
    }
    ///GPT32EH2.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT2_A::_1
    }
}
///Field `CSTRT2` writer - Channel 2 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT2_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT2_A>;
impl<'a, REG> CSTRT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT2_A::_0)
    }
    ///GPT32EH2.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT2_A::_1)
    }
}
/**Channel 3 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT3_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32EH3.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT3_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT3` reader - Channel 3 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT3_R = crate::BitReader<CSTRT3_A>;
impl CSTRT3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT3_A {
        match self.bits {
            false => CSTRT3_A::_0,
            true => CSTRT3_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT3_A::_0
    }
    ///GPT32EH3.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT3_A::_1
    }
}
///Field `CSTRT3` writer - Channel 3 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT3_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT3_A>;
impl<'a, REG> CSTRT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT3_A::_0)
    }
    ///GPT32EH3.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT3_A::_1)
    }
}
/**Channel 4 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT4_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32E4.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT4_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT4` reader - Channel 4 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT4_R = crate::BitReader<CSTRT4_A>;
impl CSTRT4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT4_A {
        match self.bits {
            false => CSTRT4_A::_0,
            true => CSTRT4_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT4_A::_0
    }
    ///GPT32E4.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT4_A::_1
    }
}
///Field `CSTRT4` writer - Channel 4 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT4_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT4_A>;
impl<'a, REG> CSTRT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT4_A::_0)
    }
    ///GPT32E4.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT4_A::_1)
    }
}
/**Channel 5 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT5_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32E5.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT5_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT5` reader - Channel 5 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT5_R = crate::BitReader<CSTRT5_A>;
impl CSTRT5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT5_A {
        match self.bits {
            false => CSTRT5_A::_0,
            true => CSTRT5_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT5_A::_0
    }
    ///GPT32E5.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT5_A::_1
    }
}
///Field `CSTRT5` writer - Channel 5 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT5_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT5_A>;
impl<'a, REG> CSTRT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT5_A::_0)
    }
    ///GPT32E5.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT5_A::_1)
    }
}
/**Channel 6 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT6_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32E6.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT6_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT6` reader - Channel 6 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT6_R = crate::BitReader<CSTRT6_A>;
impl CSTRT6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT6_A {
        match self.bits {
            false => CSTRT6_A::_0,
            true => CSTRT6_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT6_A::_0
    }
    ///GPT32E6.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT6_A::_1
    }
}
///Field `CSTRT6` writer - Channel 6 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT6_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT6_A>;
impl<'a, REG> CSTRT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT6_A::_0)
    }
    ///GPT32E6.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT6_A::_1)
    }
}
/**Channel 7 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT7_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT32E7.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT7_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT7` reader - Channel 7 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT7_R = crate::BitReader<CSTRT7_A>;
impl CSTRT7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT7_A {
        match self.bits {
            false => CSTRT7_A::_0,
            true => CSTRT7_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT7_A::_0
    }
    ///GPT32E7.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT7_A::_1
    }
}
///Field `CSTRT7` writer - Channel 7 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT7_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT7_A>;
impl<'a, REG> CSTRT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT7_A::_0)
    }
    ///GPT32E7.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT7_A::_1)
    }
}
/**Channel 8 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT8_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT328.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT8_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT8` reader - Channel 8 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT8_R = crate::BitReader<CSTRT8_A>;
impl CSTRT8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT8_A {
        match self.bits {
            false => CSTRT8_A::_0,
            true => CSTRT8_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT8_A::_0
    }
    ///GPT328.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT8_A::_1
    }
}
///Field `CSTRT8` writer - Channel 8 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT8_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT8_A>;
impl<'a, REG> CSTRT8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT8_A::_0)
    }
    ///GPT328.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT8_A::_1)
    }
}
/**Channel 9 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT9_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT329.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT9_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT9` reader - Channel 9 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT9_R = crate::BitReader<CSTRT9_A>;
impl CSTRT9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT9_A {
        match self.bits {
            false => CSTRT9_A::_0,
            true => CSTRT9_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT9_A::_0
    }
    ///GPT329.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT9_A::_1
    }
}
///Field `CSTRT9` writer - Channel 9 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT9_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT9_A>;
impl<'a, REG> CSTRT9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT9_A::_0)
    }
    ///GPT329.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT9_A::_1)
    }
}
/**Channel 10 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT10_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT3210.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT10_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT10` reader - Channel 10 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT10_R = crate::BitReader<CSTRT10_A>;
impl CSTRT10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT10_A {
        match self.bits {
            false => CSTRT10_A::_0,
            true => CSTRT10_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT10_A::_0
    }
    ///GPT3210.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT10_A::_1
    }
}
///Field `CSTRT10` writer - Channel 10 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT10_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT10_A>;
impl<'a, REG> CSTRT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT10_A::_0)
    }
    ///GPT3210.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT10_A::_1)
    }
}
/**Channel 11 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT11_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT3211.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT11_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT11` reader - Channel 11 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT11_R = crate::BitReader<CSTRT11_A>;
impl CSTRT11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT11_A {
        match self.bits {
            false => CSTRT11_A::_0,
            true => CSTRT11_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT11_A::_0
    }
    ///GPT3211.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT11_A::_1
    }
}
///Field `CSTRT11` writer - Channel 11 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT11_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT11_A>;
impl<'a, REG> CSTRT11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT11_A::_0)
    }
    ///GPT3211.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT11_A::_1)
    }
}
/**Channel 12 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT12_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT3212.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT12_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT12` reader - Channel 12 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT12_R = crate::BitReader<CSTRT12_A>;
impl CSTRT12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT12_A {
        match self.bits {
            false => CSTRT12_A::_0,
            true => CSTRT12_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT12_A::_0
    }
    ///GPT3212.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT12_A::_1
    }
}
///Field `CSTRT12` writer - Channel 12 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT12_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT12_A>;
impl<'a, REG> CSTRT12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT12_A::_0)
    }
    ///GPT3212.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT12_A::_1)
    }
}
/**Channel 13 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT13_A {
    ///0: No effect (write) / counter stop (read)
    _0 = 0,
    ///1: GPT3213.GTCNT counter starts (write) / Counter running (read)
    _1 = 1,
}
impl From<CSTRT13_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT13` reader - Channel 13 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT13_R = crate::BitReader<CSTRT13_A>;
impl CSTRT13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT13_A {
        match self.bits {
            false => CSTRT13_A::_0,
            true => CSTRT13_A::_1,
        }
    }
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT13_A::_0
    }
    ///GPT3213.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT13_A::_1
    }
}
///Field `CSTRT13` writer - Channel 13 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
pub type CSTRT13_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT13_A>;
impl<'a, REG> CSTRT13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect (write) / counter stop (read)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT13_A::_0)
    }
    ///GPT3213.GTCNT counter starts (write) / Counter running (read)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT13_A::_1)
    }
}
impl R {
    ///Bit 0 - Channel 0 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt0(&self) -> CSTRT0_R {
        CSTRT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt1(&self) -> CSTRT1_R {
        CSTRT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 2 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt2(&self) -> CSTRT2_R {
        CSTRT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 3 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt3(&self) -> CSTRT3_R {
        CSTRT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Channel 4 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt4(&self) -> CSTRT4_R {
        CSTRT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Channel 5 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt5(&self) -> CSTRT5_R {
        CSTRT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Channel 6 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt6(&self) -> CSTRT6_R {
        CSTRT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Channel 7 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt7(&self) -> CSTRT7_R {
        CSTRT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Channel 8 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt8(&self) -> CSTRT8_R {
        CSTRT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Channel 9 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt9(&self) -> CSTRT9_R {
        CSTRT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Channel 10 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt10(&self) -> CSTRT10_R {
        CSTRT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel 11 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt11(&self) -> CSTRT11_R {
        CSTRT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Channel 12 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt12(&self) -> CSTRT12_R {
        CSTRT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Channel 13 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt13(&self) -> CSTRT13_R {
        CSTRT13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Channel 0 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt0(&mut self) -> CSTRT0_W<GTSTR_SPEC> {
        CSTRT0_W::new(self, 0)
    }
    ///Bit 1 - Channel 1 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt1(&mut self) -> CSTRT1_W<GTSTR_SPEC> {
        CSTRT1_W::new(self, 1)
    }
    ///Bit 2 - Channel 2 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt2(&mut self) -> CSTRT2_W<GTSTR_SPEC> {
        CSTRT2_W::new(self, 2)
    }
    ///Bit 3 - Channel 3 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt3(&mut self) -> CSTRT3_W<GTSTR_SPEC> {
        CSTRT3_W::new(self, 3)
    }
    ///Bit 4 - Channel 4 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt4(&mut self) -> CSTRT4_W<GTSTR_SPEC> {
        CSTRT4_W::new(self, 4)
    }
    ///Bit 5 - Channel 5 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt5(&mut self) -> CSTRT5_W<GTSTR_SPEC> {
        CSTRT5_W::new(self, 5)
    }
    ///Bit 6 - Channel 6 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt6(&mut self) -> CSTRT6_W<GTSTR_SPEC> {
        CSTRT6_W::new(self, 6)
    }
    ///Bit 7 - Channel 7 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt7(&mut self) -> CSTRT7_W<GTSTR_SPEC> {
        CSTRT7_W::new(self, 7)
    }
    ///Bit 8 - Channel 8 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt8(&mut self) -> CSTRT8_W<GTSTR_SPEC> {
        CSTRT8_W::new(self, 8)
    }
    ///Bit 9 - Channel 9 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt9(&mut self) -> CSTRT9_W<GTSTR_SPEC> {
        CSTRT9_W::new(self, 9)
    }
    ///Bit 10 - Channel 10 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt10(&mut self) -> CSTRT10_W<GTSTR_SPEC> {
        CSTRT10_W::new(self, 10)
    }
    ///Bit 11 - Channel 11 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt11(&mut self) -> CSTRT11_W<GTSTR_SPEC> {
        CSTRT11_W::new(self, 11)
    }
    ///Bit 12 - Channel 12 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt12(&mut self) -> CSTRT12_W<GTSTR_SPEC> {
        CSTRT12_W::new(self, 12)
    }
    ///Bit 13 - Channel 13 GTCNT Count StartRead data shows each channel's counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running.
    #[inline(always)]
    pub fn cstrt13(&mut self) -> CSTRT13_W<GTSTR_SPEC> {
        CSTRT13_W::new(self, 13)
    }
}
/**General PWM Timer Software Start Register

You can [`read`](crate::Reg::read) this register and get [`gtstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTSTR_SPEC;
impl crate::RegisterSpec for GTSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtstr::R`](R) reader structure
impl crate::Readable for GTSTR_SPEC {}
///`write(|w| ..)` method takes [`gtstr::W`](W) writer structure
impl crate::Writable for GTSTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSTR to value 0
impl crate::Resettable for GTSTR_SPEC {}
