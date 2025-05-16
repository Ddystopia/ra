///Register `MSTPCRC` reader
pub type R = crate::R<MSTPCRC_SPEC>;
///Register `MSTPCRC` writer
pub type W = crate::W<MSTPCRC_SPEC>;
/**CAC Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC0_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC0` reader - CAC Module Stop
pub type MSTPC0_R = crate::BitReader<MSTPC0_A>;
impl MSTPC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC0_A {
        match self.bits {
            false => MSTPC0_A::_0,
            true => MSTPC0_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC0_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC0_A::_1
    }
}
///Field `MSTPC0` writer - CAC Module Stop
pub type MSTPC0_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC0_A>;
impl<'a, REG> MSTPC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC0_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC0_A::_1)
    }
}
/**CRC Calculator Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC1_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC1` reader - CRC Calculator Module Stop
pub type MSTPC1_R = crate::BitReader<MSTPC1_A>;
impl MSTPC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC1_A {
        match self.bits {
            false => MSTPC1_A::_0,
            true => MSTPC1_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC1_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC1_A::_1
    }
}
///Field `MSTPC1` writer - CRC Calculator Module Stop
pub type MSTPC1_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC1_A>;
impl<'a, REG> MSTPC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC1_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC1_A::_1)
    }
}
/**Parallel Data Capture Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC2_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC2` reader - Parallel Data Capture Module Stop
pub type MSTPC2_R = crate::BitReader<MSTPC2_A>;
impl MSTPC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC2_A {
        match self.bits {
            false => MSTPC2_A::_0,
            true => MSTPC2_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC2_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC2_A::_1
    }
}
///Field `MSTPC2` writer - Parallel Data Capture Module Stop
pub type MSTPC2_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC2_A>;
impl<'a, REG> MSTPC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC2_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC2_A::_1)
    }
}
/**Capacitive Touch Sensing Unit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC3_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC3` reader - Capacitive Touch Sensing Unit Module Stop
pub type MSTPC3_R = crate::BitReader<MSTPC3_A>;
impl MSTPC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC3_A {
        match self.bits {
            false => MSTPC3_A::_0,
            true => MSTPC3_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC3_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC3_A::_1
    }
}
///Field `MSTPC3` writer - Capacitive Touch Sensing Unit Module Stop
pub type MSTPC3_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC3_A>;
impl<'a, REG> MSTPC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC3_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC3_A::_1)
    }
}
/**Grafic LCD Controler Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC4_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC4_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC4` reader - Grafic LCD Controler Module Stop
pub type MSTPC4_R = crate::BitReader<MSTPC4_A>;
impl MSTPC4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC4_A {
        match self.bits {
            false => MSTPC4_A::_0,
            true => MSTPC4_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC4_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC4_A::_1
    }
}
///Field `MSTPC4` writer - Grafic LCD Controler Module Stop
pub type MSTPC4_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC4_A>;
impl<'a, REG> MSTPC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC4_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC4_A::_1)
    }
}
/**JPEG codec engine Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC5_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC5_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC5` reader - JPEG codec engine Module Stop
pub type MSTPC5_R = crate::BitReader<MSTPC5_A>;
impl MSTPC5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC5_A {
        match self.bits {
            false => MSTPC5_A::_0,
            true => MSTPC5_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC5_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC5_A::_1
    }
}
///Field `MSTPC5` writer - JPEG codec engine Module Stop
pub type MSTPC5_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC5_A>;
impl<'a, REG> MSTPC5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC5_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC5_A::_1)
    }
}
/**2DG engine Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC6_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC6` reader - 2DG engine Module Stop
pub type MSTPC6_R = crate::BitReader<MSTPC6_A>;
impl MSTPC6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC6_A {
        match self.bits {
            false => MSTPC6_A::_0,
            true => MSTPC6_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC6_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC6_A::_1
    }
}
///Field `MSTPC6` writer - 2DG engine Module Stop
pub type MSTPC6_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC6_A>;
impl<'a, REG> MSTPC6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC6_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC6_A::_1)
    }
}
/**Synchronous Serial Interface 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC7_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC7_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC7_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC7` reader - Synchronous Serial Interface 1 Module Stop
pub type MSTPC7_R = crate::BitReader<MSTPC7_A>;
impl MSTPC7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC7_A {
        match self.bits {
            false => MSTPC7_A::_0,
            true => MSTPC7_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC7_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC7_A::_1
    }
}
///Field `MSTPC7` writer - Synchronous Serial Interface 1 Module Stop
pub type MSTPC7_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC7_A>;
impl<'a, REG> MSTPC7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC7_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC7_A::_1)
    }
}
/**Synchronous Serial Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC8_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC8_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC8` reader - Synchronous Serial Interface 0 Module Stop
pub type MSTPC8_R = crate::BitReader<MSTPC8_A>;
impl MSTPC8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC8_A {
        match self.bits {
            false => MSTPC8_A::_0,
            true => MSTPC8_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC8_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC8_A::_1
    }
}
///Field `MSTPC8` writer - Synchronous Serial Interface 0 Module Stop
pub type MSTPC8_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC8_A>;
impl<'a, REG> MSTPC8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC8_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC8_A::_1)
    }
}
/**Sampling Rate Converter Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC9_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC9_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC9_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC9` reader - Sampling Rate Converter Module Stop
pub type MSTPC9_R = crate::BitReader<MSTPC9_A>;
impl MSTPC9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC9_A {
        match self.bits {
            false => MSTPC9_A::_0,
            true => MSTPC9_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC9_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC9_A::_1
    }
}
///Field `MSTPC9` writer - Sampling Rate Converter Module Stop
pub type MSTPC9_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC9_A>;
impl<'a, REG> MSTPC9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC9_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC9_A::_1)
    }
}
/**Synchronous Digital Hierarchy/ Multi Media Card 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC11_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC11_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC11_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC11` reader - Synchronous Digital Hierarchy/ Multi Media Card 1 Module Stop
pub type MSTPC11_R = crate::BitReader<MSTPC11_A>;
impl MSTPC11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC11_A {
        match self.bits {
            false => MSTPC11_A::_0,
            true => MSTPC11_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC11_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC11_A::_1
    }
}
///Field `MSTPC11` writer - Synchronous Digital Hierarchy/ Multi Media Card 1 Module Stop
pub type MSTPC11_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC11_A>;
impl<'a, REG> MSTPC11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC11_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC11_A::_1)
    }
}
/**Synchronous Digital Hierarchy/ Multi Media Card 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC12_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC12_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC12_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC12` reader - Synchronous Digital Hierarchy/ Multi Media Card 0 Module Stop
pub type MSTPC12_R = crate::BitReader<MSTPC12_A>;
impl MSTPC12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC12_A {
        match self.bits {
            false => MSTPC12_A::_0,
            true => MSTPC12_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC12_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC12_A::_1
    }
}
///Field `MSTPC12` writer - Synchronous Digital Hierarchy/ Multi Media Card 0 Module Stop
pub type MSTPC12_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC12_A>;
impl<'a, REG> MSTPC12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC12_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC12_A::_1)
    }
}
/**Data Operation Circuit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC13_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC13_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC13` reader - Data Operation Circuit Module Stop
pub type MSTPC13_R = crate::BitReader<MSTPC13_A>;
impl MSTPC13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC13_A {
        match self.bits {
            false => MSTPC13_A::_0,
            true => MSTPC13_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC13_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC13_A::_1
    }
}
///Field `MSTPC13` writer - Data Operation Circuit Module Stop
pub type MSTPC13_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC13_A>;
impl<'a, REG> MSTPC13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC13_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC13_A::_1)
    }
}
/**Event Link Controller Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC14_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC14` reader - Event Link Controller Module Stop
pub type MSTPC14_R = crate::BitReader<MSTPC14_A>;
impl MSTPC14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC14_A {
        match self.bits {
            false => MSTPC14_A::_0,
            true => MSTPC14_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC14_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC14_A::_1
    }
}
///Field `MSTPC14` writer - Event Link Controller Module Stop
pub type MSTPC14_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC14_A>;
impl<'a, REG> MSTPC14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC14_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC14_A::_1)
    }
}
/**TSIP Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC31_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC31` reader - TSIP Module Stop
pub type MSTPC31_R = crate::BitReader<MSTPC31_A>;
impl MSTPC31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC31_A {
        match self.bits {
            false => MSTPC31_A::_0,
            true => MSTPC31_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC31_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC31_A::_1
    }
}
///Field `MSTPC31` writer - TSIP Module Stop
pub type MSTPC31_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC31_A>;
impl<'a, REG> MSTPC31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC31_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC31_A::_1)
    }
}
impl R {
    ///Bit 0 - CAC Module Stop
    #[inline(always)]
    pub fn mstpc0(&self) -> MSTPC0_R {
        MSTPC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CRC Calculator Module Stop
    #[inline(always)]
    pub fn mstpc1(&self) -> MSTPC1_R {
        MSTPC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Parallel Data Capture Module Stop
    #[inline(always)]
    pub fn mstpc2(&self) -> MSTPC2_R {
        MSTPC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capacitive Touch Sensing Unit Module Stop
    #[inline(always)]
    pub fn mstpc3(&self) -> MSTPC3_R {
        MSTPC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Grafic LCD Controler Module Stop
    #[inline(always)]
    pub fn mstpc4(&self) -> MSTPC4_R {
        MSTPC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JPEG codec engine Module Stop
    #[inline(always)]
    pub fn mstpc5(&self) -> MSTPC5_R {
        MSTPC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 2DG engine Module Stop
    #[inline(always)]
    pub fn mstpc6(&self) -> MSTPC6_R {
        MSTPC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronous Serial Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpc7(&self) -> MSTPC7_R {
        MSTPC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Synchronous Serial Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpc8(&self) -> MSTPC8_R {
        MSTPC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Sampling Rate Converter Module Stop
    #[inline(always)]
    pub fn mstpc9(&self) -> MSTPC9_R {
        MSTPC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Synchronous Digital Hierarchy/ Multi Media Card 1 Module Stop
    #[inline(always)]
    pub fn mstpc11(&self) -> MSTPC11_R {
        MSTPC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Synchronous Digital Hierarchy/ Multi Media Card 0 Module Stop
    #[inline(always)]
    pub fn mstpc12(&self) -> MSTPC12_R {
        MSTPC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Data Operation Circuit Module Stop
    #[inline(always)]
    pub fn mstpc13(&self) -> MSTPC13_R {
        MSTPC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Event Link Controller Module Stop
    #[inline(always)]
    pub fn mstpc14(&self) -> MSTPC14_R {
        MSTPC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 31 - TSIP Module Stop
    #[inline(always)]
    pub fn mstpc31(&self) -> MSTPC31_R {
        MSTPC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CAC Module Stop
    #[inline(always)]
    pub fn mstpc0(&mut self) -> MSTPC0_W<MSTPCRC_SPEC> {
        MSTPC0_W::new(self, 0)
    }
    ///Bit 1 - CRC Calculator Module Stop
    #[inline(always)]
    pub fn mstpc1(&mut self) -> MSTPC1_W<MSTPCRC_SPEC> {
        MSTPC1_W::new(self, 1)
    }
    ///Bit 2 - Parallel Data Capture Module Stop
    #[inline(always)]
    pub fn mstpc2(&mut self) -> MSTPC2_W<MSTPCRC_SPEC> {
        MSTPC2_W::new(self, 2)
    }
    ///Bit 3 - Capacitive Touch Sensing Unit Module Stop
    #[inline(always)]
    pub fn mstpc3(&mut self) -> MSTPC3_W<MSTPCRC_SPEC> {
        MSTPC3_W::new(self, 3)
    }
    ///Bit 4 - Grafic LCD Controler Module Stop
    #[inline(always)]
    pub fn mstpc4(&mut self) -> MSTPC4_W<MSTPCRC_SPEC> {
        MSTPC4_W::new(self, 4)
    }
    ///Bit 5 - JPEG codec engine Module Stop
    #[inline(always)]
    pub fn mstpc5(&mut self) -> MSTPC5_W<MSTPCRC_SPEC> {
        MSTPC5_W::new(self, 5)
    }
    ///Bit 6 - 2DG engine Module Stop
    #[inline(always)]
    pub fn mstpc6(&mut self) -> MSTPC6_W<MSTPCRC_SPEC> {
        MSTPC6_W::new(self, 6)
    }
    ///Bit 7 - Synchronous Serial Interface 1 Module Stop
    #[inline(always)]
    pub fn mstpc7(&mut self) -> MSTPC7_W<MSTPCRC_SPEC> {
        MSTPC7_W::new(self, 7)
    }
    ///Bit 8 - Synchronous Serial Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpc8(&mut self) -> MSTPC8_W<MSTPCRC_SPEC> {
        MSTPC8_W::new(self, 8)
    }
    ///Bit 9 - Sampling Rate Converter Module Stop
    #[inline(always)]
    pub fn mstpc9(&mut self) -> MSTPC9_W<MSTPCRC_SPEC> {
        MSTPC9_W::new(self, 9)
    }
    ///Bit 11 - Synchronous Digital Hierarchy/ Multi Media Card 1 Module Stop
    #[inline(always)]
    pub fn mstpc11(&mut self) -> MSTPC11_W<MSTPCRC_SPEC> {
        MSTPC11_W::new(self, 11)
    }
    ///Bit 12 - Synchronous Digital Hierarchy/ Multi Media Card 0 Module Stop
    #[inline(always)]
    pub fn mstpc12(&mut self) -> MSTPC12_W<MSTPCRC_SPEC> {
        MSTPC12_W::new(self, 12)
    }
    ///Bit 13 - Data Operation Circuit Module Stop
    #[inline(always)]
    pub fn mstpc13(&mut self) -> MSTPC13_W<MSTPCRC_SPEC> {
        MSTPC13_W::new(self, 13)
    }
    ///Bit 14 - Event Link Controller Module Stop
    #[inline(always)]
    pub fn mstpc14(&mut self) -> MSTPC14_W<MSTPCRC_SPEC> {
        MSTPC14_W::new(self, 14)
    }
    ///Bit 31 - TSIP Module Stop
    #[inline(always)]
    pub fn mstpc31(&mut self) -> MSTPC31_W<MSTPCRC_SPEC> {
        MSTPC31_W::new(self, 31)
    }
}
/**Module Stop Control Register C

You can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSTPCRC_SPEC;
impl crate::RegisterSpec for MSTPCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mstpcrc::R`](R) reader structure
impl crate::Readable for MSTPCRC_SPEC {}
///`write(|w| ..)` method takes [`mstpcrc::W`](W) writer structure
impl crate::Writable for MSTPCRC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRC to value 0xffff_ffff
impl crate::Resettable for MSTPCRC_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
