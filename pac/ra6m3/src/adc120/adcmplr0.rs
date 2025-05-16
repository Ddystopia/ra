///Register `ADCMPLR0` reader
pub type R = crate::R<ADCMPLR0_SPEC>;
///Register `ADCMPLR0` writer
pub type W = crate::W<ADCMPLR0_SPEC>;
/**Comparison condition of AN000

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA00_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA00` reader - Comparison condition of AN000
pub type CMPLCHA00_R = crate::BitReader<CMPLCHA00_A>;
impl CMPLCHA00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA00_A {
        match self.bits {
            false => CMPLCHA00_A::_0,
            true => CMPLCHA00_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA00_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA00_A::_1
    }
}
///Field `CMPLCHA00` writer - Comparison condition of AN000
pub type CMPLCHA00_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA00_A>;
impl<'a, REG> CMPLCHA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA00_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA00_A::_1)
    }
}
/**Comparison condition of AN001

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA01_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA01` reader - Comparison condition of AN001
pub type CMPLCHA01_R = crate::BitReader<CMPLCHA01_A>;
impl CMPLCHA01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA01_A {
        match self.bits {
            false => CMPLCHA01_A::_0,
            true => CMPLCHA01_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA01_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA01_A::_1
    }
}
///Field `CMPLCHA01` writer - Comparison condition of AN001
pub type CMPLCHA01_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA01_A>;
impl<'a, REG> CMPLCHA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA01_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA01_A::_1)
    }
}
/**Comparison condition of AN002

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA02_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA02` reader - Comparison condition of AN002
pub type CMPLCHA02_R = crate::BitReader<CMPLCHA02_A>;
impl CMPLCHA02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA02_A {
        match self.bits {
            false => CMPLCHA02_A::_0,
            true => CMPLCHA02_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA02_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA02_A::_1
    }
}
///Field `CMPLCHA02` writer - Comparison condition of AN002
pub type CMPLCHA02_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA02_A>;
impl<'a, REG> CMPLCHA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA02_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA02_A::_1)
    }
}
/**Comparison condition of AN003

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA03_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA03` reader - Comparison condition of AN003
pub type CMPLCHA03_R = crate::BitReader<CMPLCHA03_A>;
impl CMPLCHA03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA03_A {
        match self.bits {
            false => CMPLCHA03_A::_0,
            true => CMPLCHA03_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA03_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA03_A::_1
    }
}
///Field `CMPLCHA03` writer - Comparison condition of AN003
pub type CMPLCHA03_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA03_A>;
impl<'a, REG> CMPLCHA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA03_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA03_A::_1)
    }
}
/**Comparison condition of AN004

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA04_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA04_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA04_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA04` reader - Comparison condition of AN004
pub type CMPLCHA04_R = crate::BitReader<CMPLCHA04_A>;
impl CMPLCHA04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA04_A {
        match self.bits {
            false => CMPLCHA04_A::_0,
            true => CMPLCHA04_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA04_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA04_A::_1
    }
}
///Field `CMPLCHA04` writer - Comparison condition of AN004
pub type CMPLCHA04_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA04_A>;
impl<'a, REG> CMPLCHA04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA04_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA04_A::_1)
    }
}
/**Comparison condition of AN005

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA05_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA05` reader - Comparison condition of AN005
pub type CMPLCHA05_R = crate::BitReader<CMPLCHA05_A>;
impl CMPLCHA05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA05_A {
        match self.bits {
            false => CMPLCHA05_A::_0,
            true => CMPLCHA05_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA05_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA05_A::_1
    }
}
///Field `CMPLCHA05` writer - Comparison condition of AN005
pub type CMPLCHA05_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA05_A>;
impl<'a, REG> CMPLCHA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA05_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA05_A::_1)
    }
}
/**Comparison condition of AN006

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA06_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA06` reader - Comparison condition of AN006
pub type CMPLCHA06_R = crate::BitReader<CMPLCHA06_A>;
impl CMPLCHA06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA06_A {
        match self.bits {
            false => CMPLCHA06_A::_0,
            true => CMPLCHA06_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA06_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA06_A::_1
    }
}
///Field `CMPLCHA06` writer - Comparison condition of AN006
pub type CMPLCHA06_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA06_A>;
impl<'a, REG> CMPLCHA06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA06_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA06_A::_1)
    }
}
/**Comparison condition of AN007

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA07_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA07` reader - Comparison condition of AN007
pub type CMPLCHA07_R = crate::BitReader<CMPLCHA07_A>;
impl CMPLCHA07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA07_A {
        match self.bits {
            false => CMPLCHA07_A::_0,
            true => CMPLCHA07_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA07_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA07_A::_1
    }
}
///Field `CMPLCHA07` writer - Comparison condition of AN007
pub type CMPLCHA07_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA07_A>;
impl<'a, REG> CMPLCHA07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA07_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA07_A::_1)
    }
}
impl R {
    ///Bit 0 - Comparison condition of AN000
    #[inline(always)]
    pub fn cmplcha00(&self) -> CMPLCHA00_R {
        CMPLCHA00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparison condition of AN001
    #[inline(always)]
    pub fn cmplcha01(&self) -> CMPLCHA01_R {
        CMPLCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Comparison condition of AN002
    #[inline(always)]
    pub fn cmplcha02(&self) -> CMPLCHA02_R {
        CMPLCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Comparison condition of AN003
    #[inline(always)]
    pub fn cmplcha03(&self) -> CMPLCHA03_R {
        CMPLCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Comparison condition of AN004
    #[inline(always)]
    pub fn cmplcha04(&self) -> CMPLCHA04_R {
        CMPLCHA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Comparison condition of AN005
    #[inline(always)]
    pub fn cmplcha05(&self) -> CMPLCHA05_R {
        CMPLCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Comparison condition of AN006
    #[inline(always)]
    pub fn cmplcha06(&self) -> CMPLCHA06_R {
        CMPLCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Comparison condition of AN007
    #[inline(always)]
    pub fn cmplcha07(&self) -> CMPLCHA07_R {
        CMPLCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparison condition of AN000
    #[inline(always)]
    pub fn cmplcha00(&mut self) -> CMPLCHA00_W<ADCMPLR0_SPEC> {
        CMPLCHA00_W::new(self, 0)
    }
    ///Bit 1 - Comparison condition of AN001
    #[inline(always)]
    pub fn cmplcha01(&mut self) -> CMPLCHA01_W<ADCMPLR0_SPEC> {
        CMPLCHA01_W::new(self, 1)
    }
    ///Bit 2 - Comparison condition of AN002
    #[inline(always)]
    pub fn cmplcha02(&mut self) -> CMPLCHA02_W<ADCMPLR0_SPEC> {
        CMPLCHA02_W::new(self, 2)
    }
    ///Bit 3 - Comparison condition of AN003
    #[inline(always)]
    pub fn cmplcha03(&mut self) -> CMPLCHA03_W<ADCMPLR0_SPEC> {
        CMPLCHA03_W::new(self, 3)
    }
    ///Bit 4 - Comparison condition of AN004
    #[inline(always)]
    pub fn cmplcha04(&mut self) -> CMPLCHA04_W<ADCMPLR0_SPEC> {
        CMPLCHA04_W::new(self, 4)
    }
    ///Bit 5 - Comparison condition of AN005
    #[inline(always)]
    pub fn cmplcha05(&mut self) -> CMPLCHA05_W<ADCMPLR0_SPEC> {
        CMPLCHA05_W::new(self, 5)
    }
    ///Bit 6 - Comparison condition of AN006
    #[inline(always)]
    pub fn cmplcha06(&mut self) -> CMPLCHA06_W<ADCMPLR0_SPEC> {
        CMPLCHA06_W::new(self, 6)
    }
    ///Bit 7 - Comparison condition of AN007
    #[inline(always)]
    pub fn cmplcha07(&mut self) -> CMPLCHA07_W<ADCMPLR0_SPEC> {
        CMPLCHA07_W::new(self, 7)
    }
}
/**A/D Compare Function Window A Comparison Condition Setting Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmplr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPLR0_SPEC;
impl crate::RegisterSpec for ADCMPLR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmplr0::R`](R) reader structure
impl crate::Readable for ADCMPLR0_SPEC {}
///`write(|w| ..)` method takes [`adcmplr0::W`](W) writer structure
impl crate::Writable for ADCMPLR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPLR0 to value 0
impl crate::Resettable for ADCMPLR0_SPEC {}
