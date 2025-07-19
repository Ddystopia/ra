///Register `ADANSB0` reader
pub type R = crate::R<ADANSB0_SPEC>;
///Register `ADANSB0` writer
pub type W = crate::W<ADANSB0_SPEC>;
/**AN000 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB00_A {
    ///0: AN000 is not subjected to conversion.
    _0 = 0,
    ///1: AN000 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB00` reader - AN000 Select
pub type ANSB00_R = crate::BitReader<ANSB00_A>;
impl ANSB00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB00_A {
        match self.bits {
            false => ANSB00_A::_0,
            true => ANSB00_A::_1,
        }
    }
    ///AN000 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB00_A::_0
    }
    ///AN000 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB00_A::_1
    }
}
///Field `ANSB00` writer - AN000 Select
pub type ANSB00_W<'a, REG> = crate::BitWriter<'a, REG, ANSB00_A>;
impl<'a, REG> ANSB00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN000 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB00_A::_0)
    }
    ///AN000 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB00_A::_1)
    }
}
/**AN001 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB01_A {
    ///0: AN001 is not subjected to conversion.
    _0 = 0,
    ///1: AN001 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB01` reader - AN001 Select
pub type ANSB01_R = crate::BitReader<ANSB01_A>;
impl ANSB01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB01_A {
        match self.bits {
            false => ANSB01_A::_0,
            true => ANSB01_A::_1,
        }
    }
    ///AN001 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB01_A::_0
    }
    ///AN001 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB01_A::_1
    }
}
///Field `ANSB01` writer - AN001 Select
pub type ANSB01_W<'a, REG> = crate::BitWriter<'a, REG, ANSB01_A>;
impl<'a, REG> ANSB01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN001 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB01_A::_0)
    }
    ///AN001 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB01_A::_1)
    }
}
/**AN002 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB02_A {
    ///0: AN002 is not subjected to conversion.
    _0 = 0,
    ///1: AN002 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB02` reader - AN002 Select
pub type ANSB02_R = crate::BitReader<ANSB02_A>;
impl ANSB02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB02_A {
        match self.bits {
            false => ANSB02_A::_0,
            true => ANSB02_A::_1,
        }
    }
    ///AN002 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB02_A::_0
    }
    ///AN002 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB02_A::_1
    }
}
///Field `ANSB02` writer - AN002 Select
pub type ANSB02_W<'a, REG> = crate::BitWriter<'a, REG, ANSB02_A>;
impl<'a, REG> ANSB02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN002 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB02_A::_0)
    }
    ///AN002 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB02_A::_1)
    }
}
/**AN003 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB03_A {
    ///0: AN003 is not subjected to conversion.
    _0 = 0,
    ///1: AN003 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB03` reader - AN003 Select
pub type ANSB03_R = crate::BitReader<ANSB03_A>;
impl ANSB03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB03_A {
        match self.bits {
            false => ANSB03_A::_0,
            true => ANSB03_A::_1,
        }
    }
    ///AN003 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB03_A::_0
    }
    ///AN003 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB03_A::_1
    }
}
///Field `ANSB03` writer - AN003 Select
pub type ANSB03_W<'a, REG> = crate::BitWriter<'a, REG, ANSB03_A>;
impl<'a, REG> ANSB03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN003 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB03_A::_0)
    }
    ///AN003 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB03_A::_1)
    }
}
/**AN004 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB04_A {
    ///0: AN004 is not subjected to conversion.
    _0 = 0,
    ///1: AN004 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB04_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB04_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB04` reader - AN004 Select
pub type ANSB04_R = crate::BitReader<ANSB04_A>;
impl ANSB04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB04_A {
        match self.bits {
            false => ANSB04_A::_0,
            true => ANSB04_A::_1,
        }
    }
    ///AN004 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB04_A::_0
    }
    ///AN004 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB04_A::_1
    }
}
///Field `ANSB04` writer - AN004 Select
pub type ANSB04_W<'a, REG> = crate::BitWriter<'a, REG, ANSB04_A>;
impl<'a, REG> ANSB04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN004 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB04_A::_0)
    }
    ///AN004 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB04_A::_1)
    }
}
/**AN005 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB05_A {
    ///0: AN005 is not subjected to conversion.
    _0 = 0,
    ///1: AN005 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB05` reader - AN005 Select
pub type ANSB05_R = crate::BitReader<ANSB05_A>;
impl ANSB05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB05_A {
        match self.bits {
            false => ANSB05_A::_0,
            true => ANSB05_A::_1,
        }
    }
    ///AN005 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB05_A::_0
    }
    ///AN005 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB05_A::_1
    }
}
///Field `ANSB05` writer - AN005 Select
pub type ANSB05_W<'a, REG> = crate::BitWriter<'a, REG, ANSB05_A>;
impl<'a, REG> ANSB05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN005 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB05_A::_0)
    }
    ///AN005 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB05_A::_1)
    }
}
/**AN006 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB06_A {
    ///0: AN006 is not subjected to conversion.
    _0 = 0,
    ///1: AN006 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB06` reader - AN006 Select
pub type ANSB06_R = crate::BitReader<ANSB06_A>;
impl ANSB06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB06_A {
        match self.bits {
            false => ANSB06_A::_0,
            true => ANSB06_A::_1,
        }
    }
    ///AN006 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB06_A::_0
    }
    ///AN006 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB06_A::_1
    }
}
///Field `ANSB06` writer - AN006 Select
pub type ANSB06_W<'a, REG> = crate::BitWriter<'a, REG, ANSB06_A>;
impl<'a, REG> ANSB06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN006 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB06_A::_0)
    }
    ///AN006 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB06_A::_1)
    }
}
/**AN007 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB07_A {
    ///0: AN007 is not subjected to conversion.
    _0 = 0,
    ///1: AN007 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB07` reader - AN007 Select
pub type ANSB07_R = crate::BitReader<ANSB07_A>;
impl ANSB07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB07_A {
        match self.bits {
            false => ANSB07_A::_0,
            true => ANSB07_A::_1,
        }
    }
    ///AN007 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB07_A::_0
    }
    ///AN007 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB07_A::_1
    }
}
///Field `ANSB07` writer - AN007 Select
pub type ANSB07_W<'a, REG> = crate::BitWriter<'a, REG, ANSB07_A>;
impl<'a, REG> ANSB07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN007 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB07_A::_0)
    }
    ///AN007 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB07_A::_1)
    }
}
/**AN008 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB08_A {
    ///0: AN008 is not subjected to conversion.
    _0 = 0,
    ///1: AN008 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB08_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB08_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB08` reader - AN008 Select
pub type ANSB08_R = crate::BitReader<ANSB08_A>;
impl ANSB08_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB08_A {
        match self.bits {
            false => ANSB08_A::_0,
            true => ANSB08_A::_1,
        }
    }
    ///AN008 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB08_A::_0
    }
    ///AN008 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB08_A::_1
    }
}
///Field `ANSB08` writer - AN008 Select
pub type ANSB08_W<'a, REG> = crate::BitWriter<'a, REG, ANSB08_A>;
impl<'a, REG> ANSB08_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN008 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB08_A::_0)
    }
    ///AN008 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB08_A::_1)
    }
}
/**AN009 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB09_A {
    ///0: AN009 is not subjected to conversion.
    _0 = 0,
    ///1: AN009 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB09_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB09_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB09` reader - AN009 Select
pub type ANSB09_R = crate::BitReader<ANSB09_A>;
impl ANSB09_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB09_A {
        match self.bits {
            false => ANSB09_A::_0,
            true => ANSB09_A::_1,
        }
    }
    ///AN009 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB09_A::_0
    }
    ///AN009 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB09_A::_1
    }
}
///Field `ANSB09` writer - AN009 Select
pub type ANSB09_W<'a, REG> = crate::BitWriter<'a, REG, ANSB09_A>;
impl<'a, REG> ANSB09_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN009 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB09_A::_0)
    }
    ///AN009 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB09_A::_1)
    }
}
/**AN010 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB10_A {
    ///0: AN010 is not subjected to conversion.
    _0 = 0,
    ///1: AN010 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB10_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB10` reader - AN010 Select
pub type ANSB10_R = crate::BitReader<ANSB10_A>;
impl ANSB10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB10_A {
        match self.bits {
            false => ANSB10_A::_0,
            true => ANSB10_A::_1,
        }
    }
    ///AN010 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB10_A::_0
    }
    ///AN010 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB10_A::_1
    }
}
///Field `ANSB10` writer - AN010 Select
pub type ANSB10_W<'a, REG> = crate::BitWriter<'a, REG, ANSB10_A>;
impl<'a, REG> ANSB10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN010 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB10_A::_0)
    }
    ///AN010 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB10_A::_1)
    }
}
/**AN011 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB11_A {
    ///0: AN011 is not subjected to conversion.
    _0 = 0,
    ///1: AN011 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB11_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB11` reader - AN011 Select
pub type ANSB11_R = crate::BitReader<ANSB11_A>;
impl ANSB11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB11_A {
        match self.bits {
            false => ANSB11_A::_0,
            true => ANSB11_A::_1,
        }
    }
    ///AN011 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB11_A::_0
    }
    ///AN011 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB11_A::_1
    }
}
///Field `ANSB11` writer - AN011 Select
pub type ANSB11_W<'a, REG> = crate::BitWriter<'a, REG, ANSB11_A>;
impl<'a, REG> ANSB11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN011 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB11_A::_0)
    }
    ///AN011 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB11_A::_1)
    }
}
/**AN012 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB12_A {
    ///0: AN012 is not subjected to conversion.
    _0 = 0,
    ///1: AN012 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB12_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB12` reader - AN012 Select
pub type ANSB12_R = crate::BitReader<ANSB12_A>;
impl ANSB12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB12_A {
        match self.bits {
            false => ANSB12_A::_0,
            true => ANSB12_A::_1,
        }
    }
    ///AN012 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB12_A::_0
    }
    ///AN012 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB12_A::_1
    }
}
///Field `ANSB12` writer - AN012 Select
pub type ANSB12_W<'a, REG> = crate::BitWriter<'a, REG, ANSB12_A>;
impl<'a, REG> ANSB12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN012 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB12_A::_0)
    }
    ///AN012 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB12_A::_1)
    }
}
/**AN013 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB13_A {
    ///0: AN013 is not subjected to conversion.
    _0 = 0,
    ///1: AN013 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB13_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB13` reader - AN013 Select
pub type ANSB13_R = crate::BitReader<ANSB13_A>;
impl ANSB13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB13_A {
        match self.bits {
            false => ANSB13_A::_0,
            true => ANSB13_A::_1,
        }
    }
    ///AN013 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB13_A::_0
    }
    ///AN013 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB13_A::_1
    }
}
///Field `ANSB13` writer - AN013 Select
pub type ANSB13_W<'a, REG> = crate::BitWriter<'a, REG, ANSB13_A>;
impl<'a, REG> ANSB13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN013 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB13_A::_0)
    }
    ///AN013 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB13_A::_1)
    }
}
/**AN014 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB14_A {
    ///0: AN014 is not subjected to conversion.
    _0 = 0,
    ///1: AN014 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB14_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB14` reader - AN014 Select
pub type ANSB14_R = crate::BitReader<ANSB14_A>;
impl ANSB14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB14_A {
        match self.bits {
            false => ANSB14_A::_0,
            true => ANSB14_A::_1,
        }
    }
    ///AN014 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB14_A::_0
    }
    ///AN014 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB14_A::_1
    }
}
///Field `ANSB14` writer - AN014 Select
pub type ANSB14_W<'a, REG> = crate::BitWriter<'a, REG, ANSB14_A>;
impl<'a, REG> ANSB14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN014 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB14_A::_0)
    }
    ///AN014 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB14_A::_1)
    }
}
impl R {
    ///Bit 0 - AN000 Select
    #[inline(always)]
    pub fn ansb00(&self) -> ANSB00_R {
        ANSB00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN001 Select
    #[inline(always)]
    pub fn ansb01(&self) -> ANSB01_R {
        ANSB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN002 Select
    #[inline(always)]
    pub fn ansb02(&self) -> ANSB02_R {
        ANSB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN003 Select
    #[inline(always)]
    pub fn ansb03(&self) -> ANSB03_R {
        ANSB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AN004 Select
    #[inline(always)]
    pub fn ansb04(&self) -> ANSB04_R {
        ANSB04_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AN005 Select
    #[inline(always)]
    pub fn ansb05(&self) -> ANSB05_R {
        ANSB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AN006 Select
    #[inline(always)]
    pub fn ansb06(&self) -> ANSB06_R {
        ANSB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AN007 Select
    #[inline(always)]
    pub fn ansb07(&self) -> ANSB07_R {
        ANSB07_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AN008 Select
    #[inline(always)]
    pub fn ansb08(&self) -> ANSB08_R {
        ANSB08_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AN009 Select
    #[inline(always)]
    pub fn ansb09(&self) -> ANSB09_R {
        ANSB09_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - AN010 Select
    #[inline(always)]
    pub fn ansb10(&self) -> ANSB10_R {
        ANSB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - AN011 Select
    #[inline(always)]
    pub fn ansb11(&self) -> ANSB11_R {
        ANSB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - AN012 Select
    #[inline(always)]
    pub fn ansb12(&self) -> ANSB12_R {
        ANSB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - AN013 Select
    #[inline(always)]
    pub fn ansb13(&self) -> ANSB13_R {
        ANSB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - AN014 Select
    #[inline(always)]
    pub fn ansb14(&self) -> ANSB14_R {
        ANSB14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN000 Select
    #[inline(always)]
    pub fn ansb00(&mut self) -> ANSB00_W<'_, ADANSB0_SPEC> {
        ANSB00_W::new(self, 0)
    }
    ///Bit 1 - AN001 Select
    #[inline(always)]
    pub fn ansb01(&mut self) -> ANSB01_W<'_, ADANSB0_SPEC> {
        ANSB01_W::new(self, 1)
    }
    ///Bit 2 - AN002 Select
    #[inline(always)]
    pub fn ansb02(&mut self) -> ANSB02_W<'_, ADANSB0_SPEC> {
        ANSB02_W::new(self, 2)
    }
    ///Bit 3 - AN003 Select
    #[inline(always)]
    pub fn ansb03(&mut self) -> ANSB03_W<'_, ADANSB0_SPEC> {
        ANSB03_W::new(self, 3)
    }
    ///Bit 4 - AN004 Select
    #[inline(always)]
    pub fn ansb04(&mut self) -> ANSB04_W<'_, ADANSB0_SPEC> {
        ANSB04_W::new(self, 4)
    }
    ///Bit 5 - AN005 Select
    #[inline(always)]
    pub fn ansb05(&mut self) -> ANSB05_W<'_, ADANSB0_SPEC> {
        ANSB05_W::new(self, 5)
    }
    ///Bit 6 - AN006 Select
    #[inline(always)]
    pub fn ansb06(&mut self) -> ANSB06_W<'_, ADANSB0_SPEC> {
        ANSB06_W::new(self, 6)
    }
    ///Bit 7 - AN007 Select
    #[inline(always)]
    pub fn ansb07(&mut self) -> ANSB07_W<'_, ADANSB0_SPEC> {
        ANSB07_W::new(self, 7)
    }
    ///Bit 8 - AN008 Select
    #[inline(always)]
    pub fn ansb08(&mut self) -> ANSB08_W<'_, ADANSB0_SPEC> {
        ANSB08_W::new(self, 8)
    }
    ///Bit 9 - AN009 Select
    #[inline(always)]
    pub fn ansb09(&mut self) -> ANSB09_W<'_, ADANSB0_SPEC> {
        ANSB09_W::new(self, 9)
    }
    ///Bit 10 - AN010 Select
    #[inline(always)]
    pub fn ansb10(&mut self) -> ANSB10_W<'_, ADANSB0_SPEC> {
        ANSB10_W::new(self, 10)
    }
    ///Bit 11 - AN011 Select
    #[inline(always)]
    pub fn ansb11(&mut self) -> ANSB11_W<'_, ADANSB0_SPEC> {
        ANSB11_W::new(self, 11)
    }
    ///Bit 12 - AN012 Select
    #[inline(always)]
    pub fn ansb12(&mut self) -> ANSB12_W<'_, ADANSB0_SPEC> {
        ANSB12_W::new(self, 12)
    }
    ///Bit 13 - AN013 Select
    #[inline(always)]
    pub fn ansb13(&mut self) -> ANSB13_W<'_, ADANSB0_SPEC> {
        ANSB13_W::new(self, 13)
    }
    ///Bit 14 - AN014 Select
    #[inline(always)]
    pub fn ansb14(&mut self) -> ANSB14_W<'_, ADANSB0_SPEC> {
        ANSB14_W::new(self, 14)
    }
}
/**A/D Channel Select Register B0

You can [`read`](crate::Reg::read) this register and get [`adansb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADANSB0_SPEC;
impl crate::RegisterSpec for ADANSB0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adansb0::R`](R) reader structure
impl crate::Readable for ADANSB0_SPEC {}
///`write(|w| ..)` method takes [`adansb0::W`](W) writer structure
impl crate::Writable for ADANSB0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSB0 to value 0
impl crate::Resettable for ADANSB0_SPEC {}
