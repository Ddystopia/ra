///Register `ADANSA0` reader
pub type R = crate::R<ADANSA0_SPEC>;
///Register `ADANSA0` writer
pub type W = crate::W<ADANSA0_SPEC>;
/**AN000 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA00_A {
    ///0: AN000 is not subjected to conversion.
    _0 = 0,
    ///1: AN000 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA00` reader - AN000 Select
pub type ANSA00_R = crate::BitReader<ANSA00_A>;
impl ANSA00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA00_A {
        match self.bits {
            false => ANSA00_A::_0,
            true => ANSA00_A::_1,
        }
    }
    ///AN000 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA00_A::_0
    }
    ///AN000 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA00_A::_1
    }
}
///Field `ANSA00` writer - AN000 Select
pub type ANSA00_W<'a, REG> = crate::BitWriter<'a, REG, ANSA00_A>;
impl<'a, REG> ANSA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN000 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA00_A::_0)
    }
    ///AN000 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA00_A::_1)
    }
}
/**AN001 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA01_A {
    ///0: AN001 is not subjected to conversion.
    _0 = 0,
    ///1: AN001 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA01` reader - AN001 Select
pub type ANSA01_R = crate::BitReader<ANSA01_A>;
impl ANSA01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA01_A {
        match self.bits {
            false => ANSA01_A::_0,
            true => ANSA01_A::_1,
        }
    }
    ///AN001 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA01_A::_0
    }
    ///AN001 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA01_A::_1
    }
}
///Field `ANSA01` writer - AN001 Select
pub type ANSA01_W<'a, REG> = crate::BitWriter<'a, REG, ANSA01_A>;
impl<'a, REG> ANSA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN001 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA01_A::_0)
    }
    ///AN001 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA01_A::_1)
    }
}
/**AN002 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA02_A {
    ///0: AN002 is not subjected to conversion.
    _0 = 0,
    ///1: AN002 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA02` reader - AN002 Select
pub type ANSA02_R = crate::BitReader<ANSA02_A>;
impl ANSA02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA02_A {
        match self.bits {
            false => ANSA02_A::_0,
            true => ANSA02_A::_1,
        }
    }
    ///AN002 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA02_A::_0
    }
    ///AN002 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA02_A::_1
    }
}
///Field `ANSA02` writer - AN002 Select
pub type ANSA02_W<'a, REG> = crate::BitWriter<'a, REG, ANSA02_A>;
impl<'a, REG> ANSA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN002 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA02_A::_0)
    }
    ///AN002 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA02_A::_1)
    }
}
/**AN003 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA03_A {
    ///0: AN003 is not subjected to conversion.
    _0 = 0,
    ///1: AN003 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA03` reader - AN003 Select
pub type ANSA03_R = crate::BitReader<ANSA03_A>;
impl ANSA03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA03_A {
        match self.bits {
            false => ANSA03_A::_0,
            true => ANSA03_A::_1,
        }
    }
    ///AN003 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA03_A::_0
    }
    ///AN003 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA03_A::_1
    }
}
///Field `ANSA03` writer - AN003 Select
pub type ANSA03_W<'a, REG> = crate::BitWriter<'a, REG, ANSA03_A>;
impl<'a, REG> ANSA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN003 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA03_A::_0)
    }
    ///AN003 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA03_A::_1)
    }
}
/**AN004 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA04_A {
    ///0: AN004 is not subjected to conversion.
    _0 = 0,
    ///1: AN004 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA04_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA04_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA04` reader - AN004 Select
pub type ANSA04_R = crate::BitReader<ANSA04_A>;
impl ANSA04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA04_A {
        match self.bits {
            false => ANSA04_A::_0,
            true => ANSA04_A::_1,
        }
    }
    ///AN004 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA04_A::_0
    }
    ///AN004 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA04_A::_1
    }
}
///Field `ANSA04` writer - AN004 Select
pub type ANSA04_W<'a, REG> = crate::BitWriter<'a, REG, ANSA04_A>;
impl<'a, REG> ANSA04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN004 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA04_A::_0)
    }
    ///AN004 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA04_A::_1)
    }
}
/**AN005 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA05_A {
    ///0: AN005 is not subjected to conversion.
    _0 = 0,
    ///1: AN005 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA05` reader - AN005 Select
pub type ANSA05_R = crate::BitReader<ANSA05_A>;
impl ANSA05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA05_A {
        match self.bits {
            false => ANSA05_A::_0,
            true => ANSA05_A::_1,
        }
    }
    ///AN005 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA05_A::_0
    }
    ///AN005 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA05_A::_1
    }
}
///Field `ANSA05` writer - AN005 Select
pub type ANSA05_W<'a, REG> = crate::BitWriter<'a, REG, ANSA05_A>;
impl<'a, REG> ANSA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN005 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA05_A::_0)
    }
    ///AN005 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA05_A::_1)
    }
}
/**AN006 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA06_A {
    ///0: AN006 is not subjected to conversion.
    _0 = 0,
    ///1: AN006 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA06` reader - AN006 Select
pub type ANSA06_R = crate::BitReader<ANSA06_A>;
impl ANSA06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA06_A {
        match self.bits {
            false => ANSA06_A::_0,
            true => ANSA06_A::_1,
        }
    }
    ///AN006 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA06_A::_0
    }
    ///AN006 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA06_A::_1
    }
}
///Field `ANSA06` writer - AN006 Select
pub type ANSA06_W<'a, REG> = crate::BitWriter<'a, REG, ANSA06_A>;
impl<'a, REG> ANSA06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN006 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA06_A::_0)
    }
    ///AN006 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA06_A::_1)
    }
}
/**AN007 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA07_A {
    ///0: AN007 is not subjected to conversion.
    _0 = 0,
    ///1: AN007 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA07` reader - AN007 Select
pub type ANSA07_R = crate::BitReader<ANSA07_A>;
impl ANSA07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA07_A {
        match self.bits {
            false => ANSA07_A::_0,
            true => ANSA07_A::_1,
        }
    }
    ///AN007 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA07_A::_0
    }
    ///AN007 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA07_A::_1
    }
}
///Field `ANSA07` writer - AN007 Select
pub type ANSA07_W<'a, REG> = crate::BitWriter<'a, REG, ANSA07_A>;
impl<'a, REG> ANSA07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN007 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA07_A::_0)
    }
    ///AN007 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA07_A::_1)
    }
}
/**AN008 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA08_A {
    ///0: AN008 is not subjected to conversion.
    _0 = 0,
    ///1: AN008 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA08_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA08_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA08` reader - AN008 Select
pub type ANSA08_R = crate::BitReader<ANSA08_A>;
impl ANSA08_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA08_A {
        match self.bits {
            false => ANSA08_A::_0,
            true => ANSA08_A::_1,
        }
    }
    ///AN008 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA08_A::_0
    }
    ///AN008 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA08_A::_1
    }
}
///Field `ANSA08` writer - AN008 Select
pub type ANSA08_W<'a, REG> = crate::BitWriter<'a, REG, ANSA08_A>;
impl<'a, REG> ANSA08_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN008 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA08_A::_0)
    }
    ///AN008 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA08_A::_1)
    }
}
/**AN009 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA09_A {
    ///0: AN009 is not subjected to conversion.
    _0 = 0,
    ///1: AN009 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA09_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA09_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA09` reader - AN009 Select
pub type ANSA09_R = crate::BitReader<ANSA09_A>;
impl ANSA09_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA09_A {
        match self.bits {
            false => ANSA09_A::_0,
            true => ANSA09_A::_1,
        }
    }
    ///AN009 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA09_A::_0
    }
    ///AN009 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA09_A::_1
    }
}
///Field `ANSA09` writer - AN009 Select
pub type ANSA09_W<'a, REG> = crate::BitWriter<'a, REG, ANSA09_A>;
impl<'a, REG> ANSA09_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN009 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA09_A::_0)
    }
    ///AN009 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA09_A::_1)
    }
}
/**AN010 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA010_A {
    ///0: AN010 is not subjected to conversion.
    _0 = 0,
    ///1: AN010 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA010_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA010_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA010` reader - AN010 Select
pub type ANSA010_R = crate::BitReader<ANSA010_A>;
impl ANSA010_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA010_A {
        match self.bits {
            false => ANSA010_A::_0,
            true => ANSA010_A::_1,
        }
    }
    ///AN010 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA010_A::_0
    }
    ///AN010 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA010_A::_1
    }
}
///Field `ANSA010` writer - AN010 Select
pub type ANSA010_W<'a, REG> = crate::BitWriter<'a, REG, ANSA010_A>;
impl<'a, REG> ANSA010_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN010 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA010_A::_0)
    }
    ///AN010 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA010_A::_1)
    }
}
/**AN011 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA011_A {
    ///0: AN011 is not subjected to conversion.
    _0 = 0,
    ///1: AN011 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA011_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA011_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA011` reader - AN011 Select
pub type ANSA011_R = crate::BitReader<ANSA011_A>;
impl ANSA011_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA011_A {
        match self.bits {
            false => ANSA011_A::_0,
            true => ANSA011_A::_1,
        }
    }
    ///AN011 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA011_A::_0
    }
    ///AN011 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA011_A::_1
    }
}
///Field `ANSA011` writer - AN011 Select
pub type ANSA011_W<'a, REG> = crate::BitWriter<'a, REG, ANSA011_A>;
impl<'a, REG> ANSA011_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN011 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA011_A::_0)
    }
    ///AN011 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA011_A::_1)
    }
}
/**AN012 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA012_A {
    ///0: AN012 is not subjected to conversion.
    _0 = 0,
    ///1: AN012 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA012_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA012_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA012` reader - AN012 Select
pub type ANSA012_R = crate::BitReader<ANSA012_A>;
impl ANSA012_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA012_A {
        match self.bits {
            false => ANSA012_A::_0,
            true => ANSA012_A::_1,
        }
    }
    ///AN012 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA012_A::_0
    }
    ///AN012 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA012_A::_1
    }
}
///Field `ANSA012` writer - AN012 Select
pub type ANSA012_W<'a, REG> = crate::BitWriter<'a, REG, ANSA012_A>;
impl<'a, REG> ANSA012_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN012 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA012_A::_0)
    }
    ///AN012 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA012_A::_1)
    }
}
/**AN013 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA013_A {
    ///0: AN013 is not subjected to conversion.
    _0 = 0,
    ///1: AN013 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA013_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA013_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA013` reader - AN013 Select
pub type ANSA013_R = crate::BitReader<ANSA013_A>;
impl ANSA013_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA013_A {
        match self.bits {
            false => ANSA013_A::_0,
            true => ANSA013_A::_1,
        }
    }
    ///AN013 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA013_A::_0
    }
    ///AN013 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA013_A::_1
    }
}
///Field `ANSA013` writer - AN013 Select
pub type ANSA013_W<'a, REG> = crate::BitWriter<'a, REG, ANSA013_A>;
impl<'a, REG> ANSA013_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN013 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA013_A::_0)
    }
    ///AN013 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA013_A::_1)
    }
}
/**AN014 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA014_A {
    ///0: AN014 is not subjected to conversion.
    _0 = 0,
    ///1: AN014 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA014_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA014_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA014` reader - AN014 Select
pub type ANSA014_R = crate::BitReader<ANSA014_A>;
impl ANSA014_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA014_A {
        match self.bits {
            false => ANSA014_A::_0,
            true => ANSA014_A::_1,
        }
    }
    ///AN014 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA014_A::_0
    }
    ///AN014 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA014_A::_1
    }
}
///Field `ANSA014` writer - AN014 Select
pub type ANSA014_W<'a, REG> = crate::BitWriter<'a, REG, ANSA014_A>;
impl<'a, REG> ANSA014_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN014 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA014_A::_0)
    }
    ///AN014 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA014_A::_1)
    }
}
impl R {
    ///Bit 0 - AN000 Select
    #[inline(always)]
    pub fn ansa00(&self) -> ANSA00_R {
        ANSA00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN001 Select
    #[inline(always)]
    pub fn ansa01(&self) -> ANSA01_R {
        ANSA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN002 Select
    #[inline(always)]
    pub fn ansa02(&self) -> ANSA02_R {
        ANSA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN003 Select
    #[inline(always)]
    pub fn ansa03(&self) -> ANSA03_R {
        ANSA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AN004 Select
    #[inline(always)]
    pub fn ansa04(&self) -> ANSA04_R {
        ANSA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AN005 Select
    #[inline(always)]
    pub fn ansa05(&self) -> ANSA05_R {
        ANSA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AN006 Select
    #[inline(always)]
    pub fn ansa06(&self) -> ANSA06_R {
        ANSA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AN007 Select
    #[inline(always)]
    pub fn ansa07(&self) -> ANSA07_R {
        ANSA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AN008 Select
    #[inline(always)]
    pub fn ansa08(&self) -> ANSA08_R {
        ANSA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AN009 Select
    #[inline(always)]
    pub fn ansa09(&self) -> ANSA09_R {
        ANSA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AN010 Select
    #[inline(always)]
    pub fn ansa010(&self) -> ANSA010_R {
        ANSA010_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AN011 Select
    #[inline(always)]
    pub fn ansa011(&self) -> ANSA011_R {
        ANSA011_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AN012 Select
    #[inline(always)]
    pub fn ansa012(&self) -> ANSA012_R {
        ANSA012_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AN013 Select
    #[inline(always)]
    pub fn ansa013(&self) -> ANSA013_R {
        ANSA013_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AN014 Select
    #[inline(always)]
    pub fn ansa014(&self) -> ANSA014_R {
        ANSA014_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN000 Select
    #[inline(always)]
    pub fn ansa00(&mut self) -> ANSA00_W<'_, ADANSA0_SPEC> {
        ANSA00_W::new(self, 0)
    }
    ///Bit 1 - AN001 Select
    #[inline(always)]
    pub fn ansa01(&mut self) -> ANSA01_W<'_, ADANSA0_SPEC> {
        ANSA01_W::new(self, 1)
    }
    ///Bit 2 - AN002 Select
    #[inline(always)]
    pub fn ansa02(&mut self) -> ANSA02_W<'_, ADANSA0_SPEC> {
        ANSA02_W::new(self, 2)
    }
    ///Bit 3 - AN003 Select
    #[inline(always)]
    pub fn ansa03(&mut self) -> ANSA03_W<'_, ADANSA0_SPEC> {
        ANSA03_W::new(self, 3)
    }
    ///Bit 4 - AN004 Select
    #[inline(always)]
    pub fn ansa04(&mut self) -> ANSA04_W<'_, ADANSA0_SPEC> {
        ANSA04_W::new(self, 4)
    }
    ///Bit 5 - AN005 Select
    #[inline(always)]
    pub fn ansa05(&mut self) -> ANSA05_W<'_, ADANSA0_SPEC> {
        ANSA05_W::new(self, 5)
    }
    ///Bit 6 - AN006 Select
    #[inline(always)]
    pub fn ansa06(&mut self) -> ANSA06_W<'_, ADANSA0_SPEC> {
        ANSA06_W::new(self, 6)
    }
    ///Bit 7 - AN007 Select
    #[inline(always)]
    pub fn ansa07(&mut self) -> ANSA07_W<'_, ADANSA0_SPEC> {
        ANSA07_W::new(self, 7)
    }
    ///Bit 8 - AN008 Select
    #[inline(always)]
    pub fn ansa08(&mut self) -> ANSA08_W<'_, ADANSA0_SPEC> {
        ANSA08_W::new(self, 8)
    }
    ///Bit 9 - AN009 Select
    #[inline(always)]
    pub fn ansa09(&mut self) -> ANSA09_W<'_, ADANSA0_SPEC> {
        ANSA09_W::new(self, 9)
    }
    ///Bit 10 - AN010 Select
    #[inline(always)]
    pub fn ansa010(&mut self) -> ANSA010_W<'_, ADANSA0_SPEC> {
        ANSA010_W::new(self, 10)
    }
    ///Bit 11 - AN011 Select
    #[inline(always)]
    pub fn ansa011(&mut self) -> ANSA011_W<'_, ADANSA0_SPEC> {
        ANSA011_W::new(self, 11)
    }
    ///Bit 12 - AN012 Select
    #[inline(always)]
    pub fn ansa012(&mut self) -> ANSA012_W<'_, ADANSA0_SPEC> {
        ANSA012_W::new(self, 12)
    }
    ///Bit 13 - AN013 Select
    #[inline(always)]
    pub fn ansa013(&mut self) -> ANSA013_W<'_, ADANSA0_SPEC> {
        ANSA013_W::new(self, 13)
    }
    ///Bit 14 - AN014 Select
    #[inline(always)]
    pub fn ansa014(&mut self) -> ANSA014_W<'_, ADANSA0_SPEC> {
        ANSA014_W::new(self, 14)
    }
}
/**A/D Channel Select Register A0

You can [`read`](crate::Reg::read) this register and get [`adansa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADANSA0_SPEC;
impl crate::RegisterSpec for ADANSA0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adansa0::R`](R) reader structure
impl crate::Readable for ADANSA0_SPEC {}
///`write(|w| ..)` method takes [`adansa0::W`](W) writer structure
impl crate::Writable for ADANSA0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSA0 to value 0
impl crate::Resettable for ADANSA0_SPEC {}
