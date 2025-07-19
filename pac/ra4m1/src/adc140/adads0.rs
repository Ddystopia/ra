///Register `ADADS0` reader
pub type R = crate::R<ADADS0_SPEC>;
///Register `ADADS0` writer
pub type W = crate::W<ADADS0_SPEC>;
/**A/D-Converted Value Addition/Average Channel AN000 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS00_A {
    ///0: AN000 is not selected.
    _0 = 0,
    ///1: AN000 is selected.
    _1 = 1,
}
impl From<ADS00_A> for bool {
    #[inline(always)]
    fn from(variant: ADS00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS00` reader - A/D-Converted Value Addition/Average Channel AN000 Select
pub type ADS00_R = crate::BitReader<ADS00_A>;
impl ADS00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS00_A {
        match self.bits {
            false => ADS00_A::_0,
            true => ADS00_A::_1,
        }
    }
    ///AN000 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS00_A::_0
    }
    ///AN000 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS00_A::_1
    }
}
///Field `ADS00` writer - A/D-Converted Value Addition/Average Channel AN000 Select
pub type ADS00_W<'a, REG> = crate::BitWriter<'a, REG, ADS00_A>;
impl<'a, REG> ADS00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN000 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS00_A::_0)
    }
    ///AN000 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS00_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN001 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS01_A {
    ///0: AN001 is not selected.
    _0 = 0,
    ///1: AN001 is selected.
    _1 = 1,
}
impl From<ADS01_A> for bool {
    #[inline(always)]
    fn from(variant: ADS01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS01` reader - A/D-Converted Value Addition/Average Channel AN001 Select
pub type ADS01_R = crate::BitReader<ADS01_A>;
impl ADS01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS01_A {
        match self.bits {
            false => ADS01_A::_0,
            true => ADS01_A::_1,
        }
    }
    ///AN001 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS01_A::_0
    }
    ///AN001 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS01_A::_1
    }
}
///Field `ADS01` writer - A/D-Converted Value Addition/Average Channel AN001 Select
pub type ADS01_W<'a, REG> = crate::BitWriter<'a, REG, ADS01_A>;
impl<'a, REG> ADS01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN001 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS01_A::_0)
    }
    ///AN001 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS01_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN002 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS02_A {
    ///0: AN002 is not selected.
    _0 = 0,
    ///1: AN002 is selected.
    _1 = 1,
}
impl From<ADS02_A> for bool {
    #[inline(always)]
    fn from(variant: ADS02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS02` reader - A/D-Converted Value Addition/Average Channel AN002 Select
pub type ADS02_R = crate::BitReader<ADS02_A>;
impl ADS02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS02_A {
        match self.bits {
            false => ADS02_A::_0,
            true => ADS02_A::_1,
        }
    }
    ///AN002 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS02_A::_0
    }
    ///AN002 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS02_A::_1
    }
}
///Field `ADS02` writer - A/D-Converted Value Addition/Average Channel AN002 Select
pub type ADS02_W<'a, REG> = crate::BitWriter<'a, REG, ADS02_A>;
impl<'a, REG> ADS02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN002 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS02_A::_0)
    }
    ///AN002 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS02_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN003 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS03_A {
    ///0: AN003 is not selected.
    _0 = 0,
    ///1: AN003 is selected.
    _1 = 1,
}
impl From<ADS03_A> for bool {
    #[inline(always)]
    fn from(variant: ADS03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS03` reader - A/D-Converted Value Addition/Average Channel AN003 Select
pub type ADS03_R = crate::BitReader<ADS03_A>;
impl ADS03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS03_A {
        match self.bits {
            false => ADS03_A::_0,
            true => ADS03_A::_1,
        }
    }
    ///AN003 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS03_A::_0
    }
    ///AN003 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS03_A::_1
    }
}
///Field `ADS03` writer - A/D-Converted Value Addition/Average Channel AN003 Select
pub type ADS03_W<'a, REG> = crate::BitWriter<'a, REG, ADS03_A>;
impl<'a, REG> ADS03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN003 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS03_A::_0)
    }
    ///AN003 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS03_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN004 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS04_A {
    ///0: AN004 is not selected.
    _0 = 0,
    ///1: AN004 is selected.
    _1 = 1,
}
impl From<ADS04_A> for bool {
    #[inline(always)]
    fn from(variant: ADS04_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS04` reader - A/D-Converted Value Addition/Average Channel AN004 Select
pub type ADS04_R = crate::BitReader<ADS04_A>;
impl ADS04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS04_A {
        match self.bits {
            false => ADS04_A::_0,
            true => ADS04_A::_1,
        }
    }
    ///AN004 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS04_A::_0
    }
    ///AN004 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS04_A::_1
    }
}
///Field `ADS04` writer - A/D-Converted Value Addition/Average Channel AN004 Select
pub type ADS04_W<'a, REG> = crate::BitWriter<'a, REG, ADS04_A>;
impl<'a, REG> ADS04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN004 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS04_A::_0)
    }
    ///AN004 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS04_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN005 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS05_A {
    ///0: AN005 is not selected.
    _0 = 0,
    ///1: AN005 is selected.
    _1 = 1,
}
impl From<ADS05_A> for bool {
    #[inline(always)]
    fn from(variant: ADS05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS05` reader - A/D-Converted Value Addition/Average Channel AN005 Select
pub type ADS05_R = crate::BitReader<ADS05_A>;
impl ADS05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS05_A {
        match self.bits {
            false => ADS05_A::_0,
            true => ADS05_A::_1,
        }
    }
    ///AN005 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS05_A::_0
    }
    ///AN005 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS05_A::_1
    }
}
///Field `ADS05` writer - A/D-Converted Value Addition/Average Channel AN005 Select
pub type ADS05_W<'a, REG> = crate::BitWriter<'a, REG, ADS05_A>;
impl<'a, REG> ADS05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN005 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS05_A::_0)
    }
    ///AN005 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS05_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN006 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS06_A {
    ///0: AN006 is not selected.
    _0 = 0,
    ///1: AN006 is selected.
    _1 = 1,
}
impl From<ADS06_A> for bool {
    #[inline(always)]
    fn from(variant: ADS06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS06` reader - A/D-Converted Value Addition/Average Channel AN006 Select
pub type ADS06_R = crate::BitReader<ADS06_A>;
impl ADS06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS06_A {
        match self.bits {
            false => ADS06_A::_0,
            true => ADS06_A::_1,
        }
    }
    ///AN006 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS06_A::_0
    }
    ///AN006 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS06_A::_1
    }
}
///Field `ADS06` writer - A/D-Converted Value Addition/Average Channel AN006 Select
pub type ADS06_W<'a, REG> = crate::BitWriter<'a, REG, ADS06_A>;
impl<'a, REG> ADS06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN006 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS06_A::_0)
    }
    ///AN006 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS06_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN007 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS07_A {
    ///0: AN007 is not selected.
    _0 = 0,
    ///1: AN007 is selected.
    _1 = 1,
}
impl From<ADS07_A> for bool {
    #[inline(always)]
    fn from(variant: ADS07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS07` reader - A/D-Converted Value Addition/Average Channel AN007 Select
pub type ADS07_R = crate::BitReader<ADS07_A>;
impl ADS07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS07_A {
        match self.bits {
            false => ADS07_A::_0,
            true => ADS07_A::_1,
        }
    }
    ///AN007 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS07_A::_0
    }
    ///AN007 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS07_A::_1
    }
}
///Field `ADS07` writer - A/D-Converted Value Addition/Average Channel AN007 Select
pub type ADS07_W<'a, REG> = crate::BitWriter<'a, REG, ADS07_A>;
impl<'a, REG> ADS07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN007 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS07_A::_0)
    }
    ///AN007 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS07_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN008 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS08_A {
    ///0: AN008 is not selected.
    _0 = 0,
    ///1: AN008 is selected.
    _1 = 1,
}
impl From<ADS08_A> for bool {
    #[inline(always)]
    fn from(variant: ADS08_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS08` reader - A/D-Converted Value Addition/Average Channel AN008 Select
pub type ADS08_R = crate::BitReader<ADS08_A>;
impl ADS08_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS08_A {
        match self.bits {
            false => ADS08_A::_0,
            true => ADS08_A::_1,
        }
    }
    ///AN008 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS08_A::_0
    }
    ///AN008 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS08_A::_1
    }
}
///Field `ADS08` writer - A/D-Converted Value Addition/Average Channel AN008 Select
pub type ADS08_W<'a, REG> = crate::BitWriter<'a, REG, ADS08_A>;
impl<'a, REG> ADS08_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN008 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS08_A::_0)
    }
    ///AN008 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS08_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN009 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS09_A {
    ///0: AN009 is not selected.
    _0 = 0,
    ///1: AN009 is selected.
    _1 = 1,
}
impl From<ADS09_A> for bool {
    #[inline(always)]
    fn from(variant: ADS09_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS09` reader - A/D-Converted Value Addition/Average Channel AN009 Select
pub type ADS09_R = crate::BitReader<ADS09_A>;
impl ADS09_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS09_A {
        match self.bits {
            false => ADS09_A::_0,
            true => ADS09_A::_1,
        }
    }
    ///AN009 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS09_A::_0
    }
    ///AN009 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS09_A::_1
    }
}
///Field `ADS09` writer - A/D-Converted Value Addition/Average Channel AN009 Select
pub type ADS09_W<'a, REG> = crate::BitWriter<'a, REG, ADS09_A>;
impl<'a, REG> ADS09_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN009 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS09_A::_0)
    }
    ///AN009 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS09_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN010 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS10_A {
    ///0: AN010 is not selected.
    _0 = 0,
    ///1: AN010 is selected.
    _1 = 1,
}
impl From<ADS10_A> for bool {
    #[inline(always)]
    fn from(variant: ADS10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS10` reader - A/D-Converted Value Addition/Average Channel AN010 Select
pub type ADS10_R = crate::BitReader<ADS10_A>;
impl ADS10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS10_A {
        match self.bits {
            false => ADS10_A::_0,
            true => ADS10_A::_1,
        }
    }
    ///AN010 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS10_A::_0
    }
    ///AN010 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS10_A::_1
    }
}
///Field `ADS10` writer - A/D-Converted Value Addition/Average Channel AN010 Select
pub type ADS10_W<'a, REG> = crate::BitWriter<'a, REG, ADS10_A>;
impl<'a, REG> ADS10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN010 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS10_A::_0)
    }
    ///AN010 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS10_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN011 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS11_A {
    ///0: AN011 is not selected.
    _0 = 0,
    ///1: AN011 is selected.
    _1 = 1,
}
impl From<ADS11_A> for bool {
    #[inline(always)]
    fn from(variant: ADS11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS11` reader - A/D-Converted Value Addition/Average Channel AN011 Select
pub type ADS11_R = crate::BitReader<ADS11_A>;
impl ADS11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS11_A {
        match self.bits {
            false => ADS11_A::_0,
            true => ADS11_A::_1,
        }
    }
    ///AN011 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS11_A::_0
    }
    ///AN011 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS11_A::_1
    }
}
///Field `ADS11` writer - A/D-Converted Value Addition/Average Channel AN011 Select
pub type ADS11_W<'a, REG> = crate::BitWriter<'a, REG, ADS11_A>;
impl<'a, REG> ADS11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN011 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS11_A::_0)
    }
    ///AN011 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS11_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN012 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS12_A {
    ///0: AN012 is not selected.
    _0 = 0,
    ///1: AN012 is selected.
    _1 = 1,
}
impl From<ADS12_A> for bool {
    #[inline(always)]
    fn from(variant: ADS12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS12` reader - A/D-Converted Value Addition/Average Channel AN012 Select
pub type ADS12_R = crate::BitReader<ADS12_A>;
impl ADS12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS12_A {
        match self.bits {
            false => ADS12_A::_0,
            true => ADS12_A::_1,
        }
    }
    ///AN012 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS12_A::_0
    }
    ///AN012 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS12_A::_1
    }
}
///Field `ADS12` writer - A/D-Converted Value Addition/Average Channel AN012 Select
pub type ADS12_W<'a, REG> = crate::BitWriter<'a, REG, ADS12_A>;
impl<'a, REG> ADS12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN012 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS12_A::_0)
    }
    ///AN012 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS12_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN013 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS13_A {
    ///0: AN013 is not selected.
    _0 = 0,
    ///1: AN013 is selected.
    _1 = 1,
}
impl From<ADS13_A> for bool {
    #[inline(always)]
    fn from(variant: ADS13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS13` reader - A/D-Converted Value Addition/Average Channel AN013 Select
pub type ADS13_R = crate::BitReader<ADS13_A>;
impl ADS13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS13_A {
        match self.bits {
            false => ADS13_A::_0,
            true => ADS13_A::_1,
        }
    }
    ///AN013 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS13_A::_0
    }
    ///AN013 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS13_A::_1
    }
}
///Field `ADS13` writer - A/D-Converted Value Addition/Average Channel AN013 Select
pub type ADS13_W<'a, REG> = crate::BitWriter<'a, REG, ADS13_A>;
impl<'a, REG> ADS13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN013 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS13_A::_0)
    }
    ///AN013 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS13_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN014 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS14_A {
    ///0: AN014 is not selected.
    _0 = 0,
    ///1: AN014 is selected.
    _1 = 1,
}
impl From<ADS14_A> for bool {
    #[inline(always)]
    fn from(variant: ADS14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS14` reader - A/D-Converted Value Addition/Average Channel AN014 Select
pub type ADS14_R = crate::BitReader<ADS14_A>;
impl ADS14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS14_A {
        match self.bits {
            false => ADS14_A::_0,
            true => ADS14_A::_1,
        }
    }
    ///AN014 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS14_A::_0
    }
    ///AN014 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS14_A::_1
    }
}
///Field `ADS14` writer - A/D-Converted Value Addition/Average Channel AN014 Select
pub type ADS14_W<'a, REG> = crate::BitWriter<'a, REG, ADS14_A>;
impl<'a, REG> ADS14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN014 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS14_A::_0)
    }
    ///AN014 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS14_A::_1)
    }
}
impl R {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN000 Select
    #[inline(always)]
    pub fn ads00(&self) -> ADS00_R {
        ADS00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN001 Select
    #[inline(always)]
    pub fn ads01(&self) -> ADS01_R {
        ADS01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN002 Select
    #[inline(always)]
    pub fn ads02(&self) -> ADS02_R {
        ADS02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN003 Select
    #[inline(always)]
    pub fn ads03(&self) -> ADS03_R {
        ADS03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D-Converted Value Addition/Average Channel AN004 Select
    #[inline(always)]
    pub fn ads04(&self) -> ADS04_R {
        ADS04_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel AN005 Select
    #[inline(always)]
    pub fn ads05(&self) -> ADS05_R {
        ADS05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel AN006 Select
    #[inline(always)]
    pub fn ads06(&self) -> ADS06_R {
        ADS06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel AN007 Select
    #[inline(always)]
    pub fn ads07(&self) -> ADS07_R {
        ADS07_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D-Converted Value Addition/Average Channel AN008 Select
    #[inline(always)]
    pub fn ads08(&self) -> ADS08_R {
        ADS08_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D-Converted Value Addition/Average Channel AN009 Select
    #[inline(always)]
    pub fn ads09(&self) -> ADS09_R {
        ADS09_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - A/D-Converted Value Addition/Average Channel AN010 Select
    #[inline(always)]
    pub fn ads10(&self) -> ADS10_R {
        ADS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - A/D-Converted Value Addition/Average Channel AN011 Select
    #[inline(always)]
    pub fn ads11(&self) -> ADS11_R {
        ADS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - A/D-Converted Value Addition/Average Channel AN012 Select
    #[inline(always)]
    pub fn ads12(&self) -> ADS12_R {
        ADS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - A/D-Converted Value Addition/Average Channel AN013 Select
    #[inline(always)]
    pub fn ads13(&self) -> ADS13_R {
        ADS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - A/D-Converted Value Addition/Average Channel AN014 Select
    #[inline(always)]
    pub fn ads14(&self) -> ADS14_R {
        ADS14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN000 Select
    #[inline(always)]
    pub fn ads00(&mut self) -> ADS00_W<'_, ADADS0_SPEC> {
        ADS00_W::new(self, 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN001 Select
    #[inline(always)]
    pub fn ads01(&mut self) -> ADS01_W<'_, ADADS0_SPEC> {
        ADS01_W::new(self, 1)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN002 Select
    #[inline(always)]
    pub fn ads02(&mut self) -> ADS02_W<'_, ADADS0_SPEC> {
        ADS02_W::new(self, 2)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN003 Select
    #[inline(always)]
    pub fn ads03(&mut self) -> ADS03_W<'_, ADADS0_SPEC> {
        ADS03_W::new(self, 3)
    }
    ///Bit 4 - A/D-Converted Value Addition/Average Channel AN004 Select
    #[inline(always)]
    pub fn ads04(&mut self) -> ADS04_W<'_, ADADS0_SPEC> {
        ADS04_W::new(self, 4)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel AN005 Select
    #[inline(always)]
    pub fn ads05(&mut self) -> ADS05_W<'_, ADADS0_SPEC> {
        ADS05_W::new(self, 5)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel AN006 Select
    #[inline(always)]
    pub fn ads06(&mut self) -> ADS06_W<'_, ADADS0_SPEC> {
        ADS06_W::new(self, 6)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel AN007 Select
    #[inline(always)]
    pub fn ads07(&mut self) -> ADS07_W<'_, ADADS0_SPEC> {
        ADS07_W::new(self, 7)
    }
    ///Bit 8 - A/D-Converted Value Addition/Average Channel AN008 Select
    #[inline(always)]
    pub fn ads08(&mut self) -> ADS08_W<'_, ADADS0_SPEC> {
        ADS08_W::new(self, 8)
    }
    ///Bit 9 - A/D-Converted Value Addition/Average Channel AN009 Select
    #[inline(always)]
    pub fn ads09(&mut self) -> ADS09_W<'_, ADADS0_SPEC> {
        ADS09_W::new(self, 9)
    }
    ///Bit 10 - A/D-Converted Value Addition/Average Channel AN010 Select
    #[inline(always)]
    pub fn ads10(&mut self) -> ADS10_W<'_, ADADS0_SPEC> {
        ADS10_W::new(self, 10)
    }
    ///Bit 11 - A/D-Converted Value Addition/Average Channel AN011 Select
    #[inline(always)]
    pub fn ads11(&mut self) -> ADS11_W<'_, ADADS0_SPEC> {
        ADS11_W::new(self, 11)
    }
    ///Bit 12 - A/D-Converted Value Addition/Average Channel AN012 Select
    #[inline(always)]
    pub fn ads12(&mut self) -> ADS12_W<'_, ADADS0_SPEC> {
        ADS12_W::new(self, 12)
    }
    ///Bit 13 - A/D-Converted Value Addition/Average Channel AN013 Select
    #[inline(always)]
    pub fn ads13(&mut self) -> ADS13_W<'_, ADADS0_SPEC> {
        ADS13_W::new(self, 13)
    }
    ///Bit 14 - A/D-Converted Value Addition/Average Channel AN014 Select
    #[inline(always)]
    pub fn ads14(&mut self) -> ADS14_W<'_, ADADS0_SPEC> {
        ADS14_W::new(self, 14)
    }
}
/**A/D-Converted Value Addition/Average Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adads0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADADS0_SPEC;
impl crate::RegisterSpec for ADADS0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adads0::R`](R) reader structure
impl crate::Readable for ADADS0_SPEC {}
///`write(|w| ..)` method takes [`adads0::W`](W) writer structure
impl crate::Writable for ADADS0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADADS0 to value 0
impl crate::Resettable for ADADS0_SPEC {}
