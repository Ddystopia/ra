///Register `MSTPCRB` reader
pub type R = crate::R<MSTPCRB_SPEC>;
///Register `MSTPCRB` writer
pub type W = crate::W<MSTPCRB_SPEC>;
/**RCAN1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB1_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB1` reader - RCAN1 Module Stop
pub type MSTPB1_R = crate::BitReader<MSTPB1_A>;
impl MSTPB1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB1_A {
        match self.bits {
            false => MSTPB1_A::_0,
            true => MSTPB1_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB1_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB1_A::_1
    }
}
///Field `MSTPB1` writer - RCAN1 Module Stop
pub type MSTPB1_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB1_A>;
impl<'a, REG> MSTPB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB1_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB1_A::_1)
    }
}
/**RCAN0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB2_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB2` reader - RCAN0 Module Stop
pub type MSTPB2_R = crate::BitReader<MSTPB2_A>;
impl MSTPB2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB2_A {
        match self.bits {
            false => MSTPB2_A::_0,
            true => MSTPB2_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB2_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB2_A::_1
    }
}
///Field `MSTPB2` writer - RCAN0 Module Stop
pub type MSTPB2_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB2_A>;
impl<'a, REG> MSTPB2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB2_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB2_A::_1)
    }
}
/**IrDA Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB5_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB5_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB5` reader - IrDA Module Stop
pub type MSTPB5_R = crate::BitReader<MSTPB5_A>;
impl MSTPB5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB5_A {
        match self.bits {
            false => MSTPB5_A::_0,
            true => MSTPB5_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB5_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB5_A::_1
    }
}
///Field `MSTPB5` writer - IrDA Module Stop
pub type MSTPB5_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB5_A>;
impl<'a, REG> MSTPB5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB5_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB5_A::_1)
    }
}
/**Queued Serial Peripheral Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB6_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB6` reader - Queued Serial Peripheral Interface Module Stop
pub type MSTPB6_R = crate::BitReader<MSTPB6_A>;
impl MSTPB6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB6_A {
        match self.bits {
            false => MSTPB6_A::_0,
            true => MSTPB6_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB6_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB6_A::_1
    }
}
///Field `MSTPB6` writer - Queued Serial Peripheral Interface Module Stop
pub type MSTPB6_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB6_A>;
impl<'a, REG> MSTPB6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB6_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB6_A::_1)
    }
}
/**I2C Bus Interface 2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB7_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB7_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB7` reader - I2C Bus Interface 2 Module Stop
pub type MSTPB7_R = crate::BitReader<MSTPB7_A>;
impl MSTPB7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB7_A {
        match self.bits {
            false => MSTPB7_A::_0,
            true => MSTPB7_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB7_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB7_A::_1
    }
}
///Field `MSTPB7` writer - I2C Bus Interface 2 Module Stop
pub type MSTPB7_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB7_A>;
impl<'a, REG> MSTPB7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB7_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB7_A::_1)
    }
}
/**I2C Bus Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB8_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB8_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB8` reader - I2C Bus Interface 1 Module Stop
pub type MSTPB8_R = crate::BitReader<MSTPB8_A>;
impl MSTPB8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB8_A {
        match self.bits {
            false => MSTPB8_A::_0,
            true => MSTPB8_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB8_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB8_A::_1
    }
}
///Field `MSTPB8` writer - I2C Bus Interface 1 Module Stop
pub type MSTPB8_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB8_A>;
impl<'a, REG> MSTPB8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB8_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB8_A::_1)
    }
}
/**I2C Bus Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB9_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB9_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB9` reader - I2C Bus Interface 0 Module Stop
pub type MSTPB9_R = crate::BitReader<MSTPB9_A>;
impl MSTPB9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB9_A {
        match self.bits {
            false => MSTPB9_A::_0,
            true => MSTPB9_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB9_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB9_A::_1
    }
}
///Field `MSTPB9` writer - I2C Bus Interface 0 Module Stop
pub type MSTPB9_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB9_A>;
impl<'a, REG> MSTPB9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB9_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB9_A::_1)
    }
}
/**Universal Serial Bus 2.0 FS Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB11_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB11_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB11` reader - Universal Serial Bus 2.0 FS Interface Module Stop
pub type MSTPB11_R = crate::BitReader<MSTPB11_A>;
impl MSTPB11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB11_A {
        match self.bits {
            false => MSTPB11_A::_0,
            true => MSTPB11_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB11_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB11_A::_1
    }
}
///Field `MSTPB11` writer - Universal Serial Bus 2.0 FS Interface Module Stop
pub type MSTPB11_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB11_A>;
impl<'a, REG> MSTPB11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB11_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB11_A::_1)
    }
}
/**Universal Serial Bus 2.0 HS Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB12_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB12_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB12` reader - Universal Serial Bus 2.0 HS Interface Module Stop
pub type MSTPB12_R = crate::BitReader<MSTPB12_A>;
impl MSTPB12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB12_A {
        match self.bits {
            false => MSTPB12_A::_0,
            true => MSTPB12_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB12_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB12_A::_1
    }
}
///Field `MSTPB12` writer - Universal Serial Bus 2.0 HS Interface Module Stop
pub type MSTPB12_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB12_A>;
impl<'a, REG> MSTPB12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB12_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB12_A::_1)
    }
}
/**EPTPC and PTPEDMAC Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB13_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB13_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB13` reader - EPTPC and PTPEDMAC Module Stop
pub type MSTPB13_R = crate::BitReader<MSTPB13_A>;
impl MSTPB13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB13_A {
        match self.bits {
            false => MSTPB13_A::_0,
            true => MSTPB13_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB13_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB13_A::_1
    }
}
///Field `MSTPB13` writer - EPTPC and PTPEDMAC Module Stop
pub type MSTPB13_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB13_A>;
impl<'a, REG> MSTPB13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB13_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB13_A::_1)
    }
}
/**ETHERC0 and EDMAC0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB15_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB15_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB15_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB15` reader - ETHERC0 and EDMAC0 Module Stop
pub type MSTPB15_R = crate::BitReader<MSTPB15_A>;
impl MSTPB15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB15_A {
        match self.bits {
            false => MSTPB15_A::_0,
            true => MSTPB15_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB15_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB15_A::_1
    }
}
///Field `MSTPB15` writer - ETHERC0 and EDMAC0 Module Stop
pub type MSTPB15_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB15_A>;
impl<'a, REG> MSTPB15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB15_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB15_A::_1)
    }
}
/**Serial Peripheral Interface Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB18_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB18_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB18` reader - Serial Peripheral Interface Module Stop
pub type MSTPB18_R = crate::BitReader<MSTPB18_A>;
impl MSTPB18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB18_A {
        match self.bits {
            false => MSTPB18_A::_0,
            true => MSTPB18_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB18_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB18_A::_1
    }
}
///Field `MSTPB18` writer - Serial Peripheral Interface Module Stop
pub type MSTPB18_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB18_A>;
impl<'a, REG> MSTPB18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB18_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB18_A::_1)
    }
}
/**Serial Peripheral Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB19_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB19_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB19` reader - Serial Peripheral Interface 0 Module Stop
pub type MSTPB19_R = crate::BitReader<MSTPB19_A>;
impl MSTPB19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB19_A {
        match self.bits {
            false => MSTPB19_A::_0,
            true => MSTPB19_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB19_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB19_A::_1
    }
}
///Field `MSTPB19` writer - Serial Peripheral Interface 0 Module Stop
pub type MSTPB19_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB19_A>;
impl<'a, REG> MSTPB19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB19_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB19_A::_1)
    }
}
/**Serial Communication Interface 9 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB22_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB22` reader - Serial Communication Interface 9 Module Stop
pub type MSTPB22_R = crate::BitReader<MSTPB22_A>;
impl MSTPB22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB22_A {
        match self.bits {
            false => MSTPB22_A::_0,
            true => MSTPB22_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB22_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB22_A::_1
    }
}
///Field `MSTPB22` writer - Serial Communication Interface 9 Module Stop
pub type MSTPB22_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB22_A>;
impl<'a, REG> MSTPB22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB22_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB22_A::_1)
    }
}
/**Serial Communication Interface 8 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB23_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB23_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB23_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB23` reader - Serial Communication Interface 8 Module Stop
pub type MSTPB23_R = crate::BitReader<MSTPB23_A>;
impl MSTPB23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB23_A {
        match self.bits {
            false => MSTPB23_A::_0,
            true => MSTPB23_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB23_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB23_A::_1
    }
}
///Field `MSTPB23` writer - Serial Communication Interface 8 Module Stop
pub type MSTPB23_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB23_A>;
impl<'a, REG> MSTPB23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB23_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB23_A::_1)
    }
}
/**Serial Communication Interface 7 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB24_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB24_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB24` reader - Serial Communication Interface 7 Module Stop
pub type MSTPB24_R = crate::BitReader<MSTPB24_A>;
impl MSTPB24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB24_A {
        match self.bits {
            false => MSTPB24_A::_0,
            true => MSTPB24_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB24_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB24_A::_1
    }
}
///Field `MSTPB24` writer - Serial Communication Interface 7 Module Stop
pub type MSTPB24_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB24_A>;
impl<'a, REG> MSTPB24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB24_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB24_A::_1)
    }
}
/**Serial Communication Interface 6 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB25_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB25_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB25_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB25` reader - Serial Communication Interface 6 Module Stop
pub type MSTPB25_R = crate::BitReader<MSTPB25_A>;
impl MSTPB25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB25_A {
        match self.bits {
            false => MSTPB25_A::_0,
            true => MSTPB25_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB25_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB25_A::_1
    }
}
///Field `MSTPB25` writer - Serial Communication Interface 6 Module Stop
pub type MSTPB25_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB25_A>;
impl<'a, REG> MSTPB25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB25_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB25_A::_1)
    }
}
/**Serial Communication Interface 5 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB26_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB26_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB26_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB26` reader - Serial Communication Interface 5 Module Stop
pub type MSTPB26_R = crate::BitReader<MSTPB26_A>;
impl MSTPB26_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB26_A {
        match self.bits {
            false => MSTPB26_A::_0,
            true => MSTPB26_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB26_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB26_A::_1
    }
}
///Field `MSTPB26` writer - Serial Communication Interface 5 Module Stop
pub type MSTPB26_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB26_A>;
impl<'a, REG> MSTPB26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB26_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB26_A::_1)
    }
}
/**Serial Communication Interface 4 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB27_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB27_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB27_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB27` reader - Serial Communication Interface 4 Module Stop
pub type MSTPB27_R = crate::BitReader<MSTPB27_A>;
impl MSTPB27_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB27_A {
        match self.bits {
            false => MSTPB27_A::_0,
            true => MSTPB27_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB27_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB27_A::_1
    }
}
///Field `MSTPB27` writer - Serial Communication Interface 4 Module Stop
pub type MSTPB27_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB27_A>;
impl<'a, REG> MSTPB27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB27_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB27_A::_1)
    }
}
/**Serial Communication Interface 3 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB28_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB28_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB28_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB28` reader - Serial Communication Interface 3 Module Stop
pub type MSTPB28_R = crate::BitReader<MSTPB28_A>;
impl MSTPB28_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB28_A {
        match self.bits {
            false => MSTPB28_A::_0,
            true => MSTPB28_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB28_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB28_A::_1
    }
}
///Field `MSTPB28` writer - Serial Communication Interface 3 Module Stop
pub type MSTPB28_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB28_A>;
impl<'a, REG> MSTPB28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB28_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB28_A::_1)
    }
}
/**Serial Communication Interface 2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB29_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB29_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB29_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB29` reader - Serial Communication Interface 2 Module Stop
pub type MSTPB29_R = crate::BitReader<MSTPB29_A>;
impl MSTPB29_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB29_A {
        match self.bits {
            false => MSTPB29_A::_0,
            true => MSTPB29_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB29_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB29_A::_1
    }
}
///Field `MSTPB29` writer - Serial Communication Interface 2 Module Stop
pub type MSTPB29_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB29_A>;
impl<'a, REG> MSTPB29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB29_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB29_A::_1)
    }
}
/**Serial Communication Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB30_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB30_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB30_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB30` reader - Serial Communication Interface 1 Module Stop
pub type MSTPB30_R = crate::BitReader<MSTPB30_A>;
impl MSTPB30_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB30_A {
        match self.bits {
            false => MSTPB30_A::_0,
            true => MSTPB30_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB30_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB30_A::_1
    }
}
///Field `MSTPB30` writer - Serial Communication Interface 1 Module Stop
pub type MSTPB30_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB30_A>;
impl<'a, REG> MSTPB30_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB30_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB30_A::_1)
    }
}
/**Serial Communication Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPB31_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPB31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPB31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPB31` reader - Serial Communication Interface 0 Module Stop
pub type MSTPB31_R = crate::BitReader<MSTPB31_A>;
impl MSTPB31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPB31_A {
        match self.bits {
            false => MSTPB31_A::_0,
            true => MSTPB31_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPB31_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPB31_A::_1
    }
}
///Field `MSTPB31` writer - Serial Communication Interface 0 Module Stop
pub type MSTPB31_W<'a, REG> = crate::BitWriter<'a, REG, MSTPB31_A>;
impl<'a, REG> MSTPB31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB31_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPB31_A::_1)
    }
}
impl R {
    ///Bit 1 - RCAN1 Module Stop
    #[inline(always)]
    pub fn mstpb1(&self) -> MSTPB1_R {
        MSTPB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RCAN0 Module Stop
    #[inline(always)]
    pub fn mstpb2(&self) -> MSTPB2_R {
        MSTPB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - IrDA Module Stop
    #[inline(always)]
    pub fn mstpb5(&self) -> MSTPB5_R {
        MSTPB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Queued Serial Peripheral Interface Module Stop
    #[inline(always)]
    pub fn mstpb6(&self) -> MSTPB6_R {
        MSTPB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C Bus Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb7(&self) -> MSTPB7_R {
        MSTPB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I2C Bus Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb8(&self) -> MSTPB8_R {
        MSTPB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - I2C Bus Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb9(&self) -> MSTPB9_R {
        MSTPB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop
    #[inline(always)]
    pub fn mstpb11(&self) -> MSTPB11_R {
        MSTPB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Universal Serial Bus 2.0 HS Interface Module Stop
    #[inline(always)]
    pub fn mstpb12(&self) -> MSTPB12_R {
        MSTPB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EPTPC and PTPEDMAC Module Stop
    #[inline(always)]
    pub fn mstpb13(&self) -> MSTPB13_R {
        MSTPB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - ETHERC0 and EDMAC0 Module Stop
    #[inline(always)]
    pub fn mstpb15(&self) -> MSTPB15_R {
        MSTPB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - Serial Peripheral Interface Module Stop
    #[inline(always)]
    pub fn mstpb18(&self) -> MSTPB18_R {
        MSTPB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Serial Peripheral Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb19(&self) -> MSTPB19_R {
        MSTPB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Serial Communication Interface 9 Module Stop
    #[inline(always)]
    pub fn mstpb22(&self) -> MSTPB22_R {
        MSTPB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Serial Communication Interface 8 Module Stop
    #[inline(always)]
    pub fn mstpb23(&self) -> MSTPB23_R {
        MSTPB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Serial Communication Interface 7 Module Stop
    #[inline(always)]
    pub fn mstpb24(&self) -> MSTPB24_R {
        MSTPB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Serial Communication Interface 6 Module Stop
    #[inline(always)]
    pub fn mstpb25(&self) -> MSTPB25_R {
        MSTPB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Serial Communication Interface 5 Module Stop
    #[inline(always)]
    pub fn mstpb26(&self) -> MSTPB26_R {
        MSTPB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Serial Communication Interface 4 Module Stop
    #[inline(always)]
    pub fn mstpb27(&self) -> MSTPB27_R {
        MSTPB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Serial Communication Interface 3 Module Stop
    #[inline(always)]
    pub fn mstpb28(&self) -> MSTPB28_R {
        MSTPB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Serial Communication Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb29(&self) -> MSTPB29_R {
        MSTPB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Serial Communication Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb30(&self) -> MSTPB30_R {
        MSTPB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Serial Communication Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb31(&self) -> MSTPB31_R {
        MSTPB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - RCAN1 Module Stop
    #[inline(always)]
    pub fn mstpb1(&mut self) -> MSTPB1_W<MSTPCRB_SPEC> {
        MSTPB1_W::new(self, 1)
    }
    ///Bit 2 - RCAN0 Module Stop
    #[inline(always)]
    pub fn mstpb2(&mut self) -> MSTPB2_W<MSTPCRB_SPEC> {
        MSTPB2_W::new(self, 2)
    }
    ///Bit 5 - IrDA Module Stop
    #[inline(always)]
    pub fn mstpb5(&mut self) -> MSTPB5_W<MSTPCRB_SPEC> {
        MSTPB5_W::new(self, 5)
    }
    ///Bit 6 - Queued Serial Peripheral Interface Module Stop
    #[inline(always)]
    pub fn mstpb6(&mut self) -> MSTPB6_W<MSTPCRB_SPEC> {
        MSTPB6_W::new(self, 6)
    }
    ///Bit 7 - I2C Bus Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb7(&mut self) -> MSTPB7_W<MSTPCRB_SPEC> {
        MSTPB7_W::new(self, 7)
    }
    ///Bit 8 - I2C Bus Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb8(&mut self) -> MSTPB8_W<MSTPCRB_SPEC> {
        MSTPB8_W::new(self, 8)
    }
    ///Bit 9 - I2C Bus Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb9(&mut self) -> MSTPB9_W<MSTPCRB_SPEC> {
        MSTPB9_W::new(self, 9)
    }
    ///Bit 11 - Universal Serial Bus 2.0 FS Interface Module Stop
    #[inline(always)]
    pub fn mstpb11(&mut self) -> MSTPB11_W<MSTPCRB_SPEC> {
        MSTPB11_W::new(self, 11)
    }
    ///Bit 12 - Universal Serial Bus 2.0 HS Interface Module Stop
    #[inline(always)]
    pub fn mstpb12(&mut self) -> MSTPB12_W<MSTPCRB_SPEC> {
        MSTPB12_W::new(self, 12)
    }
    ///Bit 13 - EPTPC and PTPEDMAC Module Stop
    #[inline(always)]
    pub fn mstpb13(&mut self) -> MSTPB13_W<MSTPCRB_SPEC> {
        MSTPB13_W::new(self, 13)
    }
    ///Bit 15 - ETHERC0 and EDMAC0 Module Stop
    #[inline(always)]
    pub fn mstpb15(&mut self) -> MSTPB15_W<MSTPCRB_SPEC> {
        MSTPB15_W::new(self, 15)
    }
    ///Bit 18 - Serial Peripheral Interface Module Stop
    #[inline(always)]
    pub fn mstpb18(&mut self) -> MSTPB18_W<MSTPCRB_SPEC> {
        MSTPB18_W::new(self, 18)
    }
    ///Bit 19 - Serial Peripheral Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb19(&mut self) -> MSTPB19_W<MSTPCRB_SPEC> {
        MSTPB19_W::new(self, 19)
    }
    ///Bit 22 - Serial Communication Interface 9 Module Stop
    #[inline(always)]
    pub fn mstpb22(&mut self) -> MSTPB22_W<MSTPCRB_SPEC> {
        MSTPB22_W::new(self, 22)
    }
    ///Bit 23 - Serial Communication Interface 8 Module Stop
    #[inline(always)]
    pub fn mstpb23(&mut self) -> MSTPB23_W<MSTPCRB_SPEC> {
        MSTPB23_W::new(self, 23)
    }
    ///Bit 24 - Serial Communication Interface 7 Module Stop
    #[inline(always)]
    pub fn mstpb24(&mut self) -> MSTPB24_W<MSTPCRB_SPEC> {
        MSTPB24_W::new(self, 24)
    }
    ///Bit 25 - Serial Communication Interface 6 Module Stop
    #[inline(always)]
    pub fn mstpb25(&mut self) -> MSTPB25_W<MSTPCRB_SPEC> {
        MSTPB25_W::new(self, 25)
    }
    ///Bit 26 - Serial Communication Interface 5 Module Stop
    #[inline(always)]
    pub fn mstpb26(&mut self) -> MSTPB26_W<MSTPCRB_SPEC> {
        MSTPB26_W::new(self, 26)
    }
    ///Bit 27 - Serial Communication Interface 4 Module Stop
    #[inline(always)]
    pub fn mstpb27(&mut self) -> MSTPB27_W<MSTPCRB_SPEC> {
        MSTPB27_W::new(self, 27)
    }
    ///Bit 28 - Serial Communication Interface 3 Module Stop
    #[inline(always)]
    pub fn mstpb28(&mut self) -> MSTPB28_W<MSTPCRB_SPEC> {
        MSTPB28_W::new(self, 28)
    }
    ///Bit 29 - Serial Communication Interface 2 Module Stop
    #[inline(always)]
    pub fn mstpb29(&mut self) -> MSTPB29_W<MSTPCRB_SPEC> {
        MSTPB29_W::new(self, 29)
    }
    ///Bit 30 - Serial Communication Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpb30(&mut self) -> MSTPB30_W<MSTPCRB_SPEC> {
        MSTPB30_W::new(self, 30)
    }
    ///Bit 31 - Serial Communication Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpb31(&mut self) -> MSTPB31_W<MSTPCRB_SPEC> {
        MSTPB31_W::new(self, 31)
    }
}
/**Module Stop Control Register B

You can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSTPCRB_SPEC;
impl crate::RegisterSpec for MSTPCRB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mstpcrb::R`](R) reader structure
impl crate::Readable for MSTPCRB_SPEC {}
///`write(|w| ..)` method takes [`mstpcrb::W`](W) writer structure
impl crate::Writable for MSTPCRB_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRB to value 0xffff_ffff
impl crate::Resettable for MSTPCRB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
