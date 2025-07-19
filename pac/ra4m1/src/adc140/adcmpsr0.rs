///Register `ADCMPSR0` reader
pub type R = crate::R<ADCMPSR0_SPEC>;
///Register `ADCMPSR0` writer
pub type W = crate::W<ADCMPSR0_SPEC>;
/**Compare window A flag of AN000

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA00_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA00_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA00` reader - Compare window A flag of AN000

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA00_R = crate::BitReader<CMPSTCHA00_A>;
impl CMPSTCHA00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA00_A {
        match self.bits {
            false => CMPSTCHA00_A::_0,
            true => CMPSTCHA00_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA00_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA00_A::_1
    }
}
///Field `CMPSTCHA00` writer - Compare window A flag of AN000
pub type CMPSTCHA00_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA00_A>;
impl<'a, REG> CMPSTCHA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA00_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA00_A::_1)
    }
}
/**Compare window A flag of AN001

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA01_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA01_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA01` reader - Compare window A flag of AN001

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA01_R = crate::BitReader<CMPSTCHA01_A>;
impl CMPSTCHA01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA01_A {
        match self.bits {
            false => CMPSTCHA01_A::_0,
            true => CMPSTCHA01_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA01_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA01_A::_1
    }
}
///Field `CMPSTCHA01` writer - Compare window A flag of AN001
pub type CMPSTCHA01_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA01_A>;
impl<'a, REG> CMPSTCHA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA01_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA01_A::_1)
    }
}
/**Compare window A flag of AN002

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA02_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA02_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA02` reader - Compare window A flag of AN002

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA02_R = crate::BitReader<CMPSTCHA02_A>;
impl CMPSTCHA02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA02_A {
        match self.bits {
            false => CMPSTCHA02_A::_0,
            true => CMPSTCHA02_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA02_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA02_A::_1
    }
}
///Field `CMPSTCHA02` writer - Compare window A flag of AN002
pub type CMPSTCHA02_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA02_A>;
impl<'a, REG> CMPSTCHA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA02_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA02_A::_1)
    }
}
/**Compare window A flag of AN003

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA03_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA03_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA03` reader - Compare window A flag of AN003

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA03_R = crate::BitReader<CMPSTCHA03_A>;
impl CMPSTCHA03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA03_A {
        match self.bits {
            false => CMPSTCHA03_A::_0,
            true => CMPSTCHA03_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA03_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA03_A::_1
    }
}
///Field `CMPSTCHA03` writer - Compare window A flag of AN003
pub type CMPSTCHA03_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA03_A>;
impl<'a, REG> CMPSTCHA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA03_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA03_A::_1)
    }
}
/**Compare window A flag of AN004

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA04_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA04_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA04_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA04` reader - Compare window A flag of AN004

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA04_R = crate::BitReader<CMPSTCHA04_A>;
impl CMPSTCHA04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA04_A {
        match self.bits {
            false => CMPSTCHA04_A::_0,
            true => CMPSTCHA04_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA04_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA04_A::_1
    }
}
///Field `CMPSTCHA04` writer - Compare window A flag of AN004
pub type CMPSTCHA04_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA04_A>;
impl<'a, REG> CMPSTCHA04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA04_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA04_A::_1)
    }
}
/**Compare window A flag of AN005

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA05_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA05_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA05` reader - Compare window A flag of AN005

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA05_R = crate::BitReader<CMPSTCHA05_A>;
impl CMPSTCHA05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA05_A {
        match self.bits {
            false => CMPSTCHA05_A::_0,
            true => CMPSTCHA05_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA05_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA05_A::_1
    }
}
///Field `CMPSTCHA05` writer - Compare window A flag of AN005
pub type CMPSTCHA05_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA05_A>;
impl<'a, REG> CMPSTCHA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA05_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA05_A::_1)
    }
}
/**Compare window A flag of AN006

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA06_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA06_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA06` reader - Compare window A flag of AN006

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA06_R = crate::BitReader<CMPSTCHA06_A>;
impl CMPSTCHA06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA06_A {
        match self.bits {
            false => CMPSTCHA06_A::_0,
            true => CMPSTCHA06_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA06_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA06_A::_1
    }
}
///Field `CMPSTCHA06` writer - Compare window A flag of AN006
pub type CMPSTCHA06_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA06_A>;
impl<'a, REG> CMPSTCHA06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA06_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA06_A::_1)
    }
}
/**Compare window A flag of AN007

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA07_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA07_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA07` reader - Compare window A flag of AN007

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA07_R = crate::BitReader<CMPSTCHA07_A>;
impl CMPSTCHA07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA07_A {
        match self.bits {
            false => CMPSTCHA07_A::_0,
            true => CMPSTCHA07_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA07_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA07_A::_1
    }
}
///Field `CMPSTCHA07` writer - Compare window A flag of AN007
pub type CMPSTCHA07_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA07_A>;
impl<'a, REG> CMPSTCHA07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA07_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA07_A::_1)
    }
}
/**Compare window A flag of AN008

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA08_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA08_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA08_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA08` reader - Compare window A flag of AN008

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA08_R = crate::BitReader<CMPSTCHA08_A>;
impl CMPSTCHA08_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA08_A {
        match self.bits {
            false => CMPSTCHA08_A::_0,
            true => CMPSTCHA08_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA08_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA08_A::_1
    }
}
///Field `CMPSTCHA08` writer - Compare window A flag of AN008
pub type CMPSTCHA08_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA08_A>;
impl<'a, REG> CMPSTCHA08_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA08_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA08_A::_1)
    }
}
/**Compare window A flag of AN009

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA09_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA09_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA09_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA09` reader - Compare window A flag of AN009

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA09_R = crate::BitReader<CMPSTCHA09_A>;
impl CMPSTCHA09_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA09_A {
        match self.bits {
            false => CMPSTCHA09_A::_0,
            true => CMPSTCHA09_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA09_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA09_A::_1
    }
}
///Field `CMPSTCHA09` writer - Compare window A flag of AN009
pub type CMPSTCHA09_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA09_A>;
impl<'a, REG> CMPSTCHA09_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA09_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA09_A::_1)
    }
}
/**Compare window A flag of AN010

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA10_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA10_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA10_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA10` reader - Compare window A flag of AN010

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA10_R = crate::BitReader<CMPSTCHA10_A>;
impl CMPSTCHA10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA10_A {
        match self.bits {
            false => CMPSTCHA10_A::_0,
            true => CMPSTCHA10_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA10_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA10_A::_1
    }
}
///Field `CMPSTCHA10` writer - Compare window A flag of AN010
pub type CMPSTCHA10_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA10_A>;
impl<'a, REG> CMPSTCHA10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA10_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA10_A::_1)
    }
}
/**Compare window A flag of AN011

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA11_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA11_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA11_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA11` reader - Compare window A flag of AN011

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA11_R = crate::BitReader<CMPSTCHA11_A>;
impl CMPSTCHA11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA11_A {
        match self.bits {
            false => CMPSTCHA11_A::_0,
            true => CMPSTCHA11_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA11_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA11_A::_1
    }
}
///Field `CMPSTCHA11` writer - Compare window A flag of AN011
pub type CMPSTCHA11_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA11_A>;
impl<'a, REG> CMPSTCHA11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA11_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA11_A::_1)
    }
}
/**Compare window A flag of AN012

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA12_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA12_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA12_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA12` reader - Compare window A flag of AN012

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA12_R = crate::BitReader<CMPSTCHA12_A>;
impl CMPSTCHA12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA12_A {
        match self.bits {
            false => CMPSTCHA12_A::_0,
            true => CMPSTCHA12_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA12_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA12_A::_1
    }
}
///Field `CMPSTCHA12` writer - Compare window A flag of AN012
pub type CMPSTCHA12_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA12_A>;
impl<'a, REG> CMPSTCHA12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA12_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA12_A::_1)
    }
}
/**Compare window A flag of AN013

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA13_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA13_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA13_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA13` reader - Compare window A flag of AN013

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA13_R = crate::BitReader<CMPSTCHA13_A>;
impl CMPSTCHA13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA13_A {
        match self.bits {
            false => CMPSTCHA13_A::_0,
            true => CMPSTCHA13_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA13_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA13_A::_1
    }
}
///Field `CMPSTCHA13` writer - Compare window A flag of AN013
pub type CMPSTCHA13_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA13_A>;
impl<'a, REG> CMPSTCHA13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA13_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA13_A::_1)
    }
}
/**Compare window A flag of AN014

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA14_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA14_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA14_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA14` reader - Compare window A flag of AN014

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA14_R = crate::BitReader<CMPSTCHA14_A>;
impl CMPSTCHA14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA14_A {
        match self.bits {
            false => CMPSTCHA14_A::_0,
            true => CMPSTCHA14_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA14_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA14_A::_1
    }
}
///Field `CMPSTCHA14` writer - Compare window A flag of AN014
pub type CMPSTCHA14_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA14_A>;
impl<'a, REG> CMPSTCHA14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA14_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA14_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare window A flag of AN000
    #[inline(always)]
    pub fn cmpstcha00(&self) -> CMPSTCHA00_R {
        CMPSTCHA00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare window A flag of AN001
    #[inline(always)]
    pub fn cmpstcha01(&self) -> CMPSTCHA01_R {
        CMPSTCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare window A flag of AN002
    #[inline(always)]
    pub fn cmpstcha02(&self) -> CMPSTCHA02_R {
        CMPSTCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare window A flag of AN003
    #[inline(always)]
    pub fn cmpstcha03(&self) -> CMPSTCHA03_R {
        CMPSTCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare window A flag of AN004
    #[inline(always)]
    pub fn cmpstcha04(&self) -> CMPSTCHA04_R {
        CMPSTCHA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare window A flag of AN005
    #[inline(always)]
    pub fn cmpstcha05(&self) -> CMPSTCHA05_R {
        CMPSTCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare window A flag of AN006
    #[inline(always)]
    pub fn cmpstcha06(&self) -> CMPSTCHA06_R {
        CMPSTCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare window A flag of AN007
    #[inline(always)]
    pub fn cmpstcha07(&self) -> CMPSTCHA07_R {
        CMPSTCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Compare window A flag of AN008
    #[inline(always)]
    pub fn cmpstcha08(&self) -> CMPSTCHA08_R {
        CMPSTCHA08_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare window A flag of AN009
    #[inline(always)]
    pub fn cmpstcha09(&self) -> CMPSTCHA09_R {
        CMPSTCHA09_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare window A flag of AN010
    #[inline(always)]
    pub fn cmpstcha10(&self) -> CMPSTCHA10_R {
        CMPSTCHA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Compare window A flag of AN011
    #[inline(always)]
    pub fn cmpstcha11(&self) -> CMPSTCHA11_R {
        CMPSTCHA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Compare window A flag of AN012
    #[inline(always)]
    pub fn cmpstcha12(&self) -> CMPSTCHA12_R {
        CMPSTCHA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Compare window A flag of AN013
    #[inline(always)]
    pub fn cmpstcha13(&self) -> CMPSTCHA13_R {
        CMPSTCHA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Compare window A flag of AN014
    #[inline(always)]
    pub fn cmpstcha14(&self) -> CMPSTCHA14_R {
        CMPSTCHA14_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare window A flag of AN000
    #[inline(always)]
    pub fn cmpstcha00(&mut self) -> CMPSTCHA00_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA00_W::new(self, 0)
    }
    ///Bit 1 - Compare window A flag of AN001
    #[inline(always)]
    pub fn cmpstcha01(&mut self) -> CMPSTCHA01_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA01_W::new(self, 1)
    }
    ///Bit 2 - Compare window A flag of AN002
    #[inline(always)]
    pub fn cmpstcha02(&mut self) -> CMPSTCHA02_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA02_W::new(self, 2)
    }
    ///Bit 3 - Compare window A flag of AN003
    #[inline(always)]
    pub fn cmpstcha03(&mut self) -> CMPSTCHA03_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA03_W::new(self, 3)
    }
    ///Bit 4 - Compare window A flag of AN004
    #[inline(always)]
    pub fn cmpstcha04(&mut self) -> CMPSTCHA04_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA04_W::new(self, 4)
    }
    ///Bit 5 - Compare window A flag of AN005
    #[inline(always)]
    pub fn cmpstcha05(&mut self) -> CMPSTCHA05_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA05_W::new(self, 5)
    }
    ///Bit 6 - Compare window A flag of AN006
    #[inline(always)]
    pub fn cmpstcha06(&mut self) -> CMPSTCHA06_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA06_W::new(self, 6)
    }
    ///Bit 7 - Compare window A flag of AN007
    #[inline(always)]
    pub fn cmpstcha07(&mut self) -> CMPSTCHA07_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA07_W::new(self, 7)
    }
    ///Bit 8 - Compare window A flag of AN008
    #[inline(always)]
    pub fn cmpstcha08(&mut self) -> CMPSTCHA08_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA08_W::new(self, 8)
    }
    ///Bit 9 - Compare window A flag of AN009
    #[inline(always)]
    pub fn cmpstcha09(&mut self) -> CMPSTCHA09_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA09_W::new(self, 9)
    }
    ///Bit 10 - Compare window A flag of AN010
    #[inline(always)]
    pub fn cmpstcha10(&mut self) -> CMPSTCHA10_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA10_W::new(self, 10)
    }
    ///Bit 11 - Compare window A flag of AN011
    #[inline(always)]
    pub fn cmpstcha11(&mut self) -> CMPSTCHA11_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA11_W::new(self, 11)
    }
    ///Bit 12 - Compare window A flag of AN012
    #[inline(always)]
    pub fn cmpstcha12(&mut self) -> CMPSTCHA12_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA12_W::new(self, 12)
    }
    ///Bit 13 - Compare window A flag of AN013
    #[inline(always)]
    pub fn cmpstcha13(&mut self) -> CMPSTCHA13_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA13_W::new(self, 13)
    }
    ///Bit 14 - Compare window A flag of AN014
    #[inline(always)]
    pub fn cmpstcha14(&mut self) -> CMPSTCHA14_W<'_, ADCMPSR0_SPEC> {
        CMPSTCHA14_W::new(self, 14)
    }
}
/**A/D Compare Function Window A Channel Status Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPSR0_SPEC;
impl crate::RegisterSpec for ADCMPSR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpsr0::R`](R) reader structure
impl crate::Readable for ADCMPSR0_SPEC {}
///`write(|w| ..)` method takes [`adcmpsr0::W`](W) writer structure
impl crate::Writable for ADCMPSR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x7fff;
}
///`reset()` method sets ADCMPSR0 to value 0
impl crate::Resettable for ADCMPSR0_SPEC {}
