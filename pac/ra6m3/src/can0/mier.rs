///Register `MIER` reader
pub type R = crate::R<MIER_SPEC>;
///Register `MIER` writer
pub type W = crate::W<MIER_SPEC>;
/**mailbox 0 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB0_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB0_A> for bool {
    #[inline(always)]
    fn from(variant: MB0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB0` reader - mailbox 0 Interrupt Enable
pub type MB0_R = crate::BitReader<MB0_A>;
impl MB0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB0_A {
        match self.bits {
            false => MB0_A::_0,
            true => MB0_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB0_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB0_A::_1
    }
}
///Field `MB0` writer - mailbox 0 Interrupt Enable
pub type MB0_W<'a, REG> = crate::BitWriter<'a, REG, MB0_A>;
impl<'a, REG> MB0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB0_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB0_A::_1)
    }
}
/**mailbox 1 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB1_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB1_A> for bool {
    #[inline(always)]
    fn from(variant: MB1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB1` reader - mailbox 1 Interrupt Enable
pub type MB1_R = crate::BitReader<MB1_A>;
impl MB1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB1_A {
        match self.bits {
            false => MB1_A::_0,
            true => MB1_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB1_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB1_A::_1
    }
}
///Field `MB1` writer - mailbox 1 Interrupt Enable
pub type MB1_W<'a, REG> = crate::BitWriter<'a, REG, MB1_A>;
impl<'a, REG> MB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB1_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB1_A::_1)
    }
}
/**mailbox 2 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB2_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB2_A> for bool {
    #[inline(always)]
    fn from(variant: MB2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB2` reader - mailbox 2 Interrupt Enable
pub type MB2_R = crate::BitReader<MB2_A>;
impl MB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB2_A {
        match self.bits {
            false => MB2_A::_0,
            true => MB2_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB2_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB2_A::_1
    }
}
///Field `MB2` writer - mailbox 2 Interrupt Enable
pub type MB2_W<'a, REG> = crate::BitWriter<'a, REG, MB2_A>;
impl<'a, REG> MB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB2_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB2_A::_1)
    }
}
/**mailbox 3 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB3_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB3_A> for bool {
    #[inline(always)]
    fn from(variant: MB3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB3` reader - mailbox 3 Interrupt Enable
pub type MB3_R = crate::BitReader<MB3_A>;
impl MB3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB3_A {
        match self.bits {
            false => MB3_A::_0,
            true => MB3_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB3_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB3_A::_1
    }
}
///Field `MB3` writer - mailbox 3 Interrupt Enable
pub type MB3_W<'a, REG> = crate::BitWriter<'a, REG, MB3_A>;
impl<'a, REG> MB3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB3_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB3_A::_1)
    }
}
/**mailbox 4 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB4_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB4_A> for bool {
    #[inline(always)]
    fn from(variant: MB4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB4` reader - mailbox 4 Interrupt Enable
pub type MB4_R = crate::BitReader<MB4_A>;
impl MB4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB4_A {
        match self.bits {
            false => MB4_A::_0,
            true => MB4_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB4_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB4_A::_1
    }
}
///Field `MB4` writer - mailbox 4 Interrupt Enable
pub type MB4_W<'a, REG> = crate::BitWriter<'a, REG, MB4_A>;
impl<'a, REG> MB4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB4_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB4_A::_1)
    }
}
/**mailbox 5 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB5_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB5_A> for bool {
    #[inline(always)]
    fn from(variant: MB5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB5` reader - mailbox 5 Interrupt Enable
pub type MB5_R = crate::BitReader<MB5_A>;
impl MB5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB5_A {
        match self.bits {
            false => MB5_A::_0,
            true => MB5_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB5_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB5_A::_1
    }
}
///Field `MB5` writer - mailbox 5 Interrupt Enable
pub type MB5_W<'a, REG> = crate::BitWriter<'a, REG, MB5_A>;
impl<'a, REG> MB5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB5_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB5_A::_1)
    }
}
/**mailbox 6 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB6_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB6_A> for bool {
    #[inline(always)]
    fn from(variant: MB6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB6` reader - mailbox 6 Interrupt Enable
pub type MB6_R = crate::BitReader<MB6_A>;
impl MB6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB6_A {
        match self.bits {
            false => MB6_A::_0,
            true => MB6_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB6_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB6_A::_1
    }
}
///Field `MB6` writer - mailbox 6 Interrupt Enable
pub type MB6_W<'a, REG> = crate::BitWriter<'a, REG, MB6_A>;
impl<'a, REG> MB6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB6_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB6_A::_1)
    }
}
/**mailbox 7 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB7_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB7_A> for bool {
    #[inline(always)]
    fn from(variant: MB7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB7` reader - mailbox 7 Interrupt Enable
pub type MB7_R = crate::BitReader<MB7_A>;
impl MB7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB7_A {
        match self.bits {
            false => MB7_A::_0,
            true => MB7_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB7_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB7_A::_1
    }
}
///Field `MB7` writer - mailbox 7 Interrupt Enable
pub type MB7_W<'a, REG> = crate::BitWriter<'a, REG, MB7_A>;
impl<'a, REG> MB7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB7_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB7_A::_1)
    }
}
/**mailbox 8 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB8_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB8_A> for bool {
    #[inline(always)]
    fn from(variant: MB8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB8` reader - mailbox 8 Interrupt Enable
pub type MB8_R = crate::BitReader<MB8_A>;
impl MB8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB8_A {
        match self.bits {
            false => MB8_A::_0,
            true => MB8_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB8_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB8_A::_1
    }
}
///Field `MB8` writer - mailbox 8 Interrupt Enable
pub type MB8_W<'a, REG> = crate::BitWriter<'a, REG, MB8_A>;
impl<'a, REG> MB8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB8_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB8_A::_1)
    }
}
/**mailbox 9 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB9_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB9_A> for bool {
    #[inline(always)]
    fn from(variant: MB9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB9` reader - mailbox 9 Interrupt Enable
pub type MB9_R = crate::BitReader<MB9_A>;
impl MB9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB9_A {
        match self.bits {
            false => MB9_A::_0,
            true => MB9_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB9_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB9_A::_1
    }
}
///Field `MB9` writer - mailbox 9 Interrupt Enable
pub type MB9_W<'a, REG> = crate::BitWriter<'a, REG, MB9_A>;
impl<'a, REG> MB9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB9_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB9_A::_1)
    }
}
/**mailbox 10 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB10_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB10_A> for bool {
    #[inline(always)]
    fn from(variant: MB10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB10` reader - mailbox 10 Interrupt Enable
pub type MB10_R = crate::BitReader<MB10_A>;
impl MB10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB10_A {
        match self.bits {
            false => MB10_A::_0,
            true => MB10_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB10_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB10_A::_1
    }
}
///Field `MB10` writer - mailbox 10 Interrupt Enable
pub type MB10_W<'a, REG> = crate::BitWriter<'a, REG, MB10_A>;
impl<'a, REG> MB10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB10_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB10_A::_1)
    }
}
/**mailbox 11 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB11_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB11_A> for bool {
    #[inline(always)]
    fn from(variant: MB11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB11` reader - mailbox 11 Interrupt Enable
pub type MB11_R = crate::BitReader<MB11_A>;
impl MB11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB11_A {
        match self.bits {
            false => MB11_A::_0,
            true => MB11_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB11_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB11_A::_1
    }
}
///Field `MB11` writer - mailbox 11 Interrupt Enable
pub type MB11_W<'a, REG> = crate::BitWriter<'a, REG, MB11_A>;
impl<'a, REG> MB11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB11_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB11_A::_1)
    }
}
/**mailbox 12 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB12_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB12_A> for bool {
    #[inline(always)]
    fn from(variant: MB12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB12` reader - mailbox 12 Interrupt Enable
pub type MB12_R = crate::BitReader<MB12_A>;
impl MB12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB12_A {
        match self.bits {
            false => MB12_A::_0,
            true => MB12_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB12_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB12_A::_1
    }
}
///Field `MB12` writer - mailbox 12 Interrupt Enable
pub type MB12_W<'a, REG> = crate::BitWriter<'a, REG, MB12_A>;
impl<'a, REG> MB12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB12_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB12_A::_1)
    }
}
/**mailbox 13 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB13_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB13_A> for bool {
    #[inline(always)]
    fn from(variant: MB13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB13` reader - mailbox 13 Interrupt Enable
pub type MB13_R = crate::BitReader<MB13_A>;
impl MB13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB13_A {
        match self.bits {
            false => MB13_A::_0,
            true => MB13_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB13_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB13_A::_1
    }
}
///Field `MB13` writer - mailbox 13 Interrupt Enable
pub type MB13_W<'a, REG> = crate::BitWriter<'a, REG, MB13_A>;
impl<'a, REG> MB13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB13_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB13_A::_1)
    }
}
/**mailbox 14 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB14_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB14_A> for bool {
    #[inline(always)]
    fn from(variant: MB14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB14` reader - mailbox 14 Interrupt Enable
pub type MB14_R = crate::BitReader<MB14_A>;
impl MB14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB14_A {
        match self.bits {
            false => MB14_A::_0,
            true => MB14_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB14_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB14_A::_1
    }
}
///Field `MB14` writer - mailbox 14 Interrupt Enable
pub type MB14_W<'a, REG> = crate::BitWriter<'a, REG, MB14_A>;
impl<'a, REG> MB14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB14_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB14_A::_1)
    }
}
/**mailbox 15 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB15_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB15_A> for bool {
    #[inline(always)]
    fn from(variant: MB15_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB15` reader - mailbox 15 Interrupt Enable
pub type MB15_R = crate::BitReader<MB15_A>;
impl MB15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB15_A {
        match self.bits {
            false => MB15_A::_0,
            true => MB15_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB15_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB15_A::_1
    }
}
///Field `MB15` writer - mailbox 15 Interrupt Enable
pub type MB15_W<'a, REG> = crate::BitWriter<'a, REG, MB15_A>;
impl<'a, REG> MB15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB15_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB15_A::_1)
    }
}
/**mailbox 16 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB16_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB16_A> for bool {
    #[inline(always)]
    fn from(variant: MB16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB16` reader - mailbox 16 Interrupt Enable
pub type MB16_R = crate::BitReader<MB16_A>;
impl MB16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB16_A {
        match self.bits {
            false => MB16_A::_0,
            true => MB16_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB16_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB16_A::_1
    }
}
///Field `MB16` writer - mailbox 16 Interrupt Enable
pub type MB16_W<'a, REG> = crate::BitWriter<'a, REG, MB16_A>;
impl<'a, REG> MB16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB16_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB16_A::_1)
    }
}
/**mailbox 17 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB17_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB17_A> for bool {
    #[inline(always)]
    fn from(variant: MB17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB17` reader - mailbox 17 Interrupt Enable
pub type MB17_R = crate::BitReader<MB17_A>;
impl MB17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB17_A {
        match self.bits {
            false => MB17_A::_0,
            true => MB17_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB17_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB17_A::_1
    }
}
///Field `MB17` writer - mailbox 17 Interrupt Enable
pub type MB17_W<'a, REG> = crate::BitWriter<'a, REG, MB17_A>;
impl<'a, REG> MB17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB17_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB17_A::_1)
    }
}
/**mailbox 18 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB18_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB18_A> for bool {
    #[inline(always)]
    fn from(variant: MB18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB18` reader - mailbox 18 Interrupt Enable
pub type MB18_R = crate::BitReader<MB18_A>;
impl MB18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB18_A {
        match self.bits {
            false => MB18_A::_0,
            true => MB18_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB18_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB18_A::_1
    }
}
///Field `MB18` writer - mailbox 18 Interrupt Enable
pub type MB18_W<'a, REG> = crate::BitWriter<'a, REG, MB18_A>;
impl<'a, REG> MB18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB18_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB18_A::_1)
    }
}
/**mailbox 19 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB19_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB19_A> for bool {
    #[inline(always)]
    fn from(variant: MB19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB19` reader - mailbox 19 Interrupt Enable
pub type MB19_R = crate::BitReader<MB19_A>;
impl MB19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB19_A {
        match self.bits {
            false => MB19_A::_0,
            true => MB19_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB19_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB19_A::_1
    }
}
///Field `MB19` writer - mailbox 19 Interrupt Enable
pub type MB19_W<'a, REG> = crate::BitWriter<'a, REG, MB19_A>;
impl<'a, REG> MB19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB19_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB19_A::_1)
    }
}
/**mailbox 20 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB20_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB20_A> for bool {
    #[inline(always)]
    fn from(variant: MB20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB20` reader - mailbox 20 Interrupt Enable
pub type MB20_R = crate::BitReader<MB20_A>;
impl MB20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB20_A {
        match self.bits {
            false => MB20_A::_0,
            true => MB20_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB20_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB20_A::_1
    }
}
///Field `MB20` writer - mailbox 20 Interrupt Enable
pub type MB20_W<'a, REG> = crate::BitWriter<'a, REG, MB20_A>;
impl<'a, REG> MB20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB20_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB20_A::_1)
    }
}
/**mailbox 21 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB21_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB21_A> for bool {
    #[inline(always)]
    fn from(variant: MB21_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB21` reader - mailbox 21 Interrupt Enable
pub type MB21_R = crate::BitReader<MB21_A>;
impl MB21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB21_A {
        match self.bits {
            false => MB21_A::_0,
            true => MB21_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB21_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB21_A::_1
    }
}
///Field `MB21` writer - mailbox 21 Interrupt Enable
pub type MB21_W<'a, REG> = crate::BitWriter<'a, REG, MB21_A>;
impl<'a, REG> MB21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB21_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB21_A::_1)
    }
}
/**mailbox 22 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB22_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB22_A> for bool {
    #[inline(always)]
    fn from(variant: MB22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB22` reader - mailbox 22 Interrupt Enable
pub type MB22_R = crate::BitReader<MB22_A>;
impl MB22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB22_A {
        match self.bits {
            false => MB22_A::_0,
            true => MB22_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB22_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB22_A::_1
    }
}
///Field `MB22` writer - mailbox 22 Interrupt Enable
pub type MB22_W<'a, REG> = crate::BitWriter<'a, REG, MB22_A>;
impl<'a, REG> MB22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB22_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB22_A::_1)
    }
}
/**mailbox 23 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB23_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB23_A> for bool {
    #[inline(always)]
    fn from(variant: MB23_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB23` reader - mailbox 23 Interrupt Enable
pub type MB23_R = crate::BitReader<MB23_A>;
impl MB23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB23_A {
        match self.bits {
            false => MB23_A::_0,
            true => MB23_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB23_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB23_A::_1
    }
}
///Field `MB23` writer - mailbox 23 Interrupt Enable
pub type MB23_W<'a, REG> = crate::BitWriter<'a, REG, MB23_A>;
impl<'a, REG> MB23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB23_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB23_A::_1)
    }
}
/**mailbox 24 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB24_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB24_A> for bool {
    #[inline(always)]
    fn from(variant: MB24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB24` reader - mailbox 24 Interrupt Enable
pub type MB24_R = crate::BitReader<MB24_A>;
impl MB24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB24_A {
        match self.bits {
            false => MB24_A::_0,
            true => MB24_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB24_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB24_A::_1
    }
}
///Field `MB24` writer - mailbox 24 Interrupt Enable
pub type MB24_W<'a, REG> = crate::BitWriter<'a, REG, MB24_A>;
impl<'a, REG> MB24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB24_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB24_A::_1)
    }
}
/**mailbox 25 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB25_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB25_A> for bool {
    #[inline(always)]
    fn from(variant: MB25_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB25` reader - mailbox 25 Interrupt Enable
pub type MB25_R = crate::BitReader<MB25_A>;
impl MB25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB25_A {
        match self.bits {
            false => MB25_A::_0,
            true => MB25_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB25_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB25_A::_1
    }
}
///Field `MB25` writer - mailbox 25 Interrupt Enable
pub type MB25_W<'a, REG> = crate::BitWriter<'a, REG, MB25_A>;
impl<'a, REG> MB25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB25_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB25_A::_1)
    }
}
/**mailbox 26 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB26_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB26_A> for bool {
    #[inline(always)]
    fn from(variant: MB26_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB26` reader - mailbox 26 Interrupt Enable
pub type MB26_R = crate::BitReader<MB26_A>;
impl MB26_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB26_A {
        match self.bits {
            false => MB26_A::_0,
            true => MB26_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB26_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB26_A::_1
    }
}
///Field `MB26` writer - mailbox 26 Interrupt Enable
pub type MB26_W<'a, REG> = crate::BitWriter<'a, REG, MB26_A>;
impl<'a, REG> MB26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB26_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB26_A::_1)
    }
}
/**mailbox 27 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB27_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB27_A> for bool {
    #[inline(always)]
    fn from(variant: MB27_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB27` reader - mailbox 27 Interrupt Enable
pub type MB27_R = crate::BitReader<MB27_A>;
impl MB27_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB27_A {
        match self.bits {
            false => MB27_A::_0,
            true => MB27_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB27_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB27_A::_1
    }
}
///Field `MB27` writer - mailbox 27 Interrupt Enable
pub type MB27_W<'a, REG> = crate::BitWriter<'a, REG, MB27_A>;
impl<'a, REG> MB27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB27_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB27_A::_1)
    }
}
/**mailbox 28 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB28_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB28_A> for bool {
    #[inline(always)]
    fn from(variant: MB28_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB28` reader - mailbox 28 Interrupt Enable
pub type MB28_R = crate::BitReader<MB28_A>;
impl MB28_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB28_A {
        match self.bits {
            false => MB28_A::_0,
            true => MB28_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB28_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB28_A::_1
    }
}
///Field `MB28` writer - mailbox 28 Interrupt Enable
pub type MB28_W<'a, REG> = crate::BitWriter<'a, REG, MB28_A>;
impl<'a, REG> MB28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB28_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB28_A::_1)
    }
}
/**mailbox 29 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB29_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB29_A> for bool {
    #[inline(always)]
    fn from(variant: MB29_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB29` reader - mailbox 29 Interrupt Enable
pub type MB29_R = crate::BitReader<MB29_A>;
impl MB29_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB29_A {
        match self.bits {
            false => MB29_A::_0,
            true => MB29_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB29_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB29_A::_1
    }
}
///Field `MB29` writer - mailbox 29 Interrupt Enable
pub type MB29_W<'a, REG> = crate::BitWriter<'a, REG, MB29_A>;
impl<'a, REG> MB29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB29_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB29_A::_1)
    }
}
/**mailbox 30 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB30_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB30_A> for bool {
    #[inline(always)]
    fn from(variant: MB30_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB30` reader - mailbox 30 Interrupt Enable
pub type MB30_R = crate::BitReader<MB30_A>;
impl MB30_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB30_A {
        match self.bits {
            false => MB30_A::_0,
            true => MB30_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB30_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB30_A::_1
    }
}
///Field `MB30` writer - mailbox 30 Interrupt Enable
pub type MB30_W<'a, REG> = crate::BitWriter<'a, REG, MB30_A>;
impl<'a, REG> MB30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB30_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB30_A::_1)
    }
}
/**mailbox 31 Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MB31_A {
    ///0: Interrupt disabled
    _0 = 0,
    ///1: Interrupt enabled
    _1 = 1,
}
impl From<MB31_A> for bool {
    #[inline(always)]
    fn from(variant: MB31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MB31` reader - mailbox 31 Interrupt Enable
pub type MB31_R = crate::BitReader<MB31_A>;
impl MB31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MB31_A {
        match self.bits {
            false => MB31_A::_0,
            true => MB31_A::_1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MB31_A::_0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MB31_A::_1
    }
}
///Field `MB31` writer - mailbox 31 Interrupt Enable
pub type MB31_W<'a, REG> = crate::BitWriter<'a, REG, MB31_A>;
impl<'a, REG> MB31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MB31_A::_0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MB31_A::_1)
    }
}
impl R {
    ///Bit 0 - mailbox 0 Interrupt Enable
    #[inline(always)]
    pub fn mb0(&self) -> MB0_R {
        MB0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - mailbox 1 Interrupt Enable
    #[inline(always)]
    pub fn mb1(&self) -> MB1_R {
        MB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - mailbox 2 Interrupt Enable
    #[inline(always)]
    pub fn mb2(&self) -> MB2_R {
        MB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - mailbox 3 Interrupt Enable
    #[inline(always)]
    pub fn mb3(&self) -> MB3_R {
        MB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - mailbox 4 Interrupt Enable
    #[inline(always)]
    pub fn mb4(&self) -> MB4_R {
        MB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - mailbox 5 Interrupt Enable
    #[inline(always)]
    pub fn mb5(&self) -> MB5_R {
        MB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - mailbox 6 Interrupt Enable
    #[inline(always)]
    pub fn mb6(&self) -> MB6_R {
        MB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - mailbox 7 Interrupt Enable
    #[inline(always)]
    pub fn mb7(&self) -> MB7_R {
        MB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - mailbox 8 Interrupt Enable
    #[inline(always)]
    pub fn mb8(&self) -> MB8_R {
        MB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - mailbox 9 Interrupt Enable
    #[inline(always)]
    pub fn mb9(&self) -> MB9_R {
        MB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - mailbox 10 Interrupt Enable
    #[inline(always)]
    pub fn mb10(&self) -> MB10_R {
        MB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - mailbox 11 Interrupt Enable
    #[inline(always)]
    pub fn mb11(&self) -> MB11_R {
        MB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - mailbox 12 Interrupt Enable
    #[inline(always)]
    pub fn mb12(&self) -> MB12_R {
        MB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - mailbox 13 Interrupt Enable
    #[inline(always)]
    pub fn mb13(&self) -> MB13_R {
        MB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - mailbox 14 Interrupt Enable
    #[inline(always)]
    pub fn mb14(&self) -> MB14_R {
        MB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - mailbox 15 Interrupt Enable
    #[inline(always)]
    pub fn mb15(&self) -> MB15_R {
        MB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - mailbox 16 Interrupt Enable
    #[inline(always)]
    pub fn mb16(&self) -> MB16_R {
        MB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - mailbox 17 Interrupt Enable
    #[inline(always)]
    pub fn mb17(&self) -> MB17_R {
        MB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - mailbox 18 Interrupt Enable
    #[inline(always)]
    pub fn mb18(&self) -> MB18_R {
        MB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - mailbox 19 Interrupt Enable
    #[inline(always)]
    pub fn mb19(&self) -> MB19_R {
        MB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - mailbox 20 Interrupt Enable
    #[inline(always)]
    pub fn mb20(&self) -> MB20_R {
        MB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - mailbox 21 Interrupt Enable
    #[inline(always)]
    pub fn mb21(&self) -> MB21_R {
        MB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - mailbox 22 Interrupt Enable
    #[inline(always)]
    pub fn mb22(&self) -> MB22_R {
        MB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - mailbox 23 Interrupt Enable
    #[inline(always)]
    pub fn mb23(&self) -> MB23_R {
        MB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - mailbox 24 Interrupt Enable
    #[inline(always)]
    pub fn mb24(&self) -> MB24_R {
        MB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - mailbox 25 Interrupt Enable
    #[inline(always)]
    pub fn mb25(&self) -> MB25_R {
        MB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - mailbox 26 Interrupt Enable
    #[inline(always)]
    pub fn mb26(&self) -> MB26_R {
        MB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - mailbox 27 Interrupt Enable
    #[inline(always)]
    pub fn mb27(&self) -> MB27_R {
        MB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - mailbox 28 Interrupt Enable
    #[inline(always)]
    pub fn mb28(&self) -> MB28_R {
        MB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - mailbox 29 Interrupt Enable
    #[inline(always)]
    pub fn mb29(&self) -> MB29_R {
        MB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - mailbox 30 Interrupt Enable
    #[inline(always)]
    pub fn mb30(&self) -> MB30_R {
        MB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - mailbox 31 Interrupt Enable
    #[inline(always)]
    pub fn mb31(&self) -> MB31_R {
        MB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - mailbox 0 Interrupt Enable
    #[inline(always)]
    pub fn mb0(&mut self) -> MB0_W<MIER_SPEC> {
        MB0_W::new(self, 0)
    }
    ///Bit 1 - mailbox 1 Interrupt Enable
    #[inline(always)]
    pub fn mb1(&mut self) -> MB1_W<MIER_SPEC> {
        MB1_W::new(self, 1)
    }
    ///Bit 2 - mailbox 2 Interrupt Enable
    #[inline(always)]
    pub fn mb2(&mut self) -> MB2_W<MIER_SPEC> {
        MB2_W::new(self, 2)
    }
    ///Bit 3 - mailbox 3 Interrupt Enable
    #[inline(always)]
    pub fn mb3(&mut self) -> MB3_W<MIER_SPEC> {
        MB3_W::new(self, 3)
    }
    ///Bit 4 - mailbox 4 Interrupt Enable
    #[inline(always)]
    pub fn mb4(&mut self) -> MB4_W<MIER_SPEC> {
        MB4_W::new(self, 4)
    }
    ///Bit 5 - mailbox 5 Interrupt Enable
    #[inline(always)]
    pub fn mb5(&mut self) -> MB5_W<MIER_SPEC> {
        MB5_W::new(self, 5)
    }
    ///Bit 6 - mailbox 6 Interrupt Enable
    #[inline(always)]
    pub fn mb6(&mut self) -> MB6_W<MIER_SPEC> {
        MB6_W::new(self, 6)
    }
    ///Bit 7 - mailbox 7 Interrupt Enable
    #[inline(always)]
    pub fn mb7(&mut self) -> MB7_W<MIER_SPEC> {
        MB7_W::new(self, 7)
    }
    ///Bit 8 - mailbox 8 Interrupt Enable
    #[inline(always)]
    pub fn mb8(&mut self) -> MB8_W<MIER_SPEC> {
        MB8_W::new(self, 8)
    }
    ///Bit 9 - mailbox 9 Interrupt Enable
    #[inline(always)]
    pub fn mb9(&mut self) -> MB9_W<MIER_SPEC> {
        MB9_W::new(self, 9)
    }
    ///Bit 10 - mailbox 10 Interrupt Enable
    #[inline(always)]
    pub fn mb10(&mut self) -> MB10_W<MIER_SPEC> {
        MB10_W::new(self, 10)
    }
    ///Bit 11 - mailbox 11 Interrupt Enable
    #[inline(always)]
    pub fn mb11(&mut self) -> MB11_W<MIER_SPEC> {
        MB11_W::new(self, 11)
    }
    ///Bit 12 - mailbox 12 Interrupt Enable
    #[inline(always)]
    pub fn mb12(&mut self) -> MB12_W<MIER_SPEC> {
        MB12_W::new(self, 12)
    }
    ///Bit 13 - mailbox 13 Interrupt Enable
    #[inline(always)]
    pub fn mb13(&mut self) -> MB13_W<MIER_SPEC> {
        MB13_W::new(self, 13)
    }
    ///Bit 14 - mailbox 14 Interrupt Enable
    #[inline(always)]
    pub fn mb14(&mut self) -> MB14_W<MIER_SPEC> {
        MB14_W::new(self, 14)
    }
    ///Bit 15 - mailbox 15 Interrupt Enable
    #[inline(always)]
    pub fn mb15(&mut self) -> MB15_W<MIER_SPEC> {
        MB15_W::new(self, 15)
    }
    ///Bit 16 - mailbox 16 Interrupt Enable
    #[inline(always)]
    pub fn mb16(&mut self) -> MB16_W<MIER_SPEC> {
        MB16_W::new(self, 16)
    }
    ///Bit 17 - mailbox 17 Interrupt Enable
    #[inline(always)]
    pub fn mb17(&mut self) -> MB17_W<MIER_SPEC> {
        MB17_W::new(self, 17)
    }
    ///Bit 18 - mailbox 18 Interrupt Enable
    #[inline(always)]
    pub fn mb18(&mut self) -> MB18_W<MIER_SPEC> {
        MB18_W::new(self, 18)
    }
    ///Bit 19 - mailbox 19 Interrupt Enable
    #[inline(always)]
    pub fn mb19(&mut self) -> MB19_W<MIER_SPEC> {
        MB19_W::new(self, 19)
    }
    ///Bit 20 - mailbox 20 Interrupt Enable
    #[inline(always)]
    pub fn mb20(&mut self) -> MB20_W<MIER_SPEC> {
        MB20_W::new(self, 20)
    }
    ///Bit 21 - mailbox 21 Interrupt Enable
    #[inline(always)]
    pub fn mb21(&mut self) -> MB21_W<MIER_SPEC> {
        MB21_W::new(self, 21)
    }
    ///Bit 22 - mailbox 22 Interrupt Enable
    #[inline(always)]
    pub fn mb22(&mut self) -> MB22_W<MIER_SPEC> {
        MB22_W::new(self, 22)
    }
    ///Bit 23 - mailbox 23 Interrupt Enable
    #[inline(always)]
    pub fn mb23(&mut self) -> MB23_W<MIER_SPEC> {
        MB23_W::new(self, 23)
    }
    ///Bit 24 - mailbox 24 Interrupt Enable
    #[inline(always)]
    pub fn mb24(&mut self) -> MB24_W<MIER_SPEC> {
        MB24_W::new(self, 24)
    }
    ///Bit 25 - mailbox 25 Interrupt Enable
    #[inline(always)]
    pub fn mb25(&mut self) -> MB25_W<MIER_SPEC> {
        MB25_W::new(self, 25)
    }
    ///Bit 26 - mailbox 26 Interrupt Enable
    #[inline(always)]
    pub fn mb26(&mut self) -> MB26_W<MIER_SPEC> {
        MB26_W::new(self, 26)
    }
    ///Bit 27 - mailbox 27 Interrupt Enable
    #[inline(always)]
    pub fn mb27(&mut self) -> MB27_W<MIER_SPEC> {
        MB27_W::new(self, 27)
    }
    ///Bit 28 - mailbox 28 Interrupt Enable
    #[inline(always)]
    pub fn mb28(&mut self) -> MB28_W<MIER_SPEC> {
        MB28_W::new(self, 28)
    }
    ///Bit 29 - mailbox 29 Interrupt Enable
    #[inline(always)]
    pub fn mb29(&mut self) -> MB29_W<MIER_SPEC> {
        MB29_W::new(self, 29)
    }
    ///Bit 30 - mailbox 30 Interrupt Enable
    #[inline(always)]
    pub fn mb30(&mut self) -> MB30_W<MIER_SPEC> {
        MB30_W::new(self, 30)
    }
    ///Bit 31 - mailbox 31 Interrupt Enable
    #[inline(always)]
    pub fn mb31(&mut self) -> MB31_W<MIER_SPEC> {
        MB31_W::new(self, 31)
    }
}
/**Mailbox Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`mier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MIER_SPEC;
impl crate::RegisterSpec for MIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mier::R`](R) reader structure
impl crate::Readable for MIER_SPEC {}
///`write(|w| ..)` method takes [`mier::W`](W) writer structure
impl crate::Writable for MIER_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MIER to value 0
impl crate::Resettable for MIER_SPEC {}
