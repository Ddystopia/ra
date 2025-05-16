///Register `GTICASR` reader
pub type R = crate::R<GTICASR_SPEC>;
///Register `GTICASR` writer
pub type W = crate::W<GTICASR_SPEC>;
/**GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGAR_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTETRGA input.
    _1 = 1,
}
impl From<ASGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGAR_R = crate::BitReader<ASGTRGAR_A>;
impl ASGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGAR_A {
        match self.bits {
            false => ASGTRGAR_A::_0,
            true => ASGTRGAR_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGAR_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGAR_A::_1
    }
}
///Field `ASGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGAR_A>;
impl<'a, REG> ASGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGAR_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGAF_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTETRGA input.
    _1 = 1,
}
impl From<ASGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGAF_R = crate::BitReader<ASGTRGAF_A>;
impl ASGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGAF_A {
        match self.bits {
            false => ASGTRGAF_A::_0,
            true => ASGTRGAF_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGAF_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGAF_A::_1
    }
}
///Field `ASGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGAF_A>;
impl<'a, REG> ASGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGAF_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGBR_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTETRGB input.
    _1 = 1,
}
impl From<ASGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGBR_R = crate::BitReader<ASGTRGBR_A>;
impl ASGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGBR_A {
        match self.bits {
            false => ASGTRGBR_A::_0,
            true => ASGTRGBR_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGBR_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGBR_A::_1
    }
}
///Field `ASGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGBR_A>;
impl<'a, REG> ASGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGBR_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGBF_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTETRGB input.
    _1 = 1,
}
impl From<ASGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGBF_R = crate::BitReader<ASGTRGBF_A>;
impl ASGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGBF_A {
        match self.bits {
            false => ASGTRGBF_A::_0,
            true => ASGTRGBF_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGBF_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGBF_A::_1
    }
}
///Field `ASGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGBF_A>;
impl<'a, REG> ASGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGBF_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGCR_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTETRGC input.
    _1 = 1,
}
impl From<ASGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGCR` reader - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGCR_R = crate::BitReader<ASGTRGCR_A>;
impl ASGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGCR_A {
        match self.bits {
            false => ASGTRGCR_A::_0,
            true => ASGTRGCR_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGCR_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGCR_A::_1
    }
}
///Field `ASGTRGCR` writer - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGCR_A>;
impl<'a, REG> ASGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGCR_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGCF_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<ASGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGCF` reader - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGCF_R = crate::BitReader<ASGTRGCF_A>;
impl ASGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGCF_A {
        match self.bits {
            false => ASGTRGCF_A::_0,
            true => ASGTRGCF_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGCF_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGCF_A::_1
    }
}
///Field `ASGTRGCF` writer - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGCF_A>;
impl<'a, REG> ASGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGCF_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGDR_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTETRGD input.
    _1 = 1,
}
impl From<ASGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGDR` reader - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGDR_R = crate::BitReader<ASGTRGDR_A>;
impl ASGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGDR_A {
        match self.bits {
            false => ASGTRGDR_A::_0,
            true => ASGTRGDR_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGDR_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGDR_A::_1
    }
}
///Field `ASGTRGDR` writer - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
pub type ASGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGDR_A>;
impl<'a, REG> ASGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGDR_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASGTRGDF_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTETRGD input.
    _1 = 1,
}
impl From<ASGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: ASGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASGTRGDF` reader - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGDF_R = crate::BitReader<ASGTRGDF_A>;
impl ASGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASGTRGDF_A {
        match self.bits {
            false => ASGTRGDF_A::_0,
            true => ASGTRGDF_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASGTRGDF_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASGTRGDF_A::_1
    }
}
///Field `ASGTRGDF` writer - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
pub type ASGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, ASGTRGDF_A>;
impl<'a, REG> ASGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGDF_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCARBL_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<ASCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
pub type ASCARBL_R = crate::BitReader<ASCARBL_A>;
impl ASCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCARBL_A {
        match self.bits {
            false => ASCARBL_A::_0,
            true => ASCARBL_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCARBL_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCARBL_A::_1
    }
}
///Field `ASCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
pub type ASCARBL_W<'a, REG> = crate::BitWriter<'a, REG, ASCARBL_A>;
impl<'a, REG> ASCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCARBL_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCARBH_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<ASCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable
pub type ASCARBH_R = crate::BitReader<ASCARBH_A>;
impl ASCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCARBH_A {
        match self.bits {
            false => ASCARBH_A::_0,
            true => ASCARBH_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCARBH_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCARBH_A::_1
    }
}
///Field `ASCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable
pub type ASCARBH_W<'a, REG> = crate::BitWriter<'a, REG, ASCARBH_A>;
impl<'a, REG> ASCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCARBH_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCAFBL_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<ASCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
pub type ASCAFBL_R = crate::BitReader<ASCAFBL_A>;
impl ASCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCAFBL_A {
        match self.bits {
            false => ASCAFBL_A::_0,
            true => ASCAFBL_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCAFBL_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCAFBL_A::_1
    }
}
///Field `ASCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
pub type ASCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, ASCAFBL_A>;
impl<'a, REG> ASCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCAFBL_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCAFBH_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<ASCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable
pub type ASCAFBH_R = crate::BitReader<ASCAFBH_A>;
impl ASCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCAFBH_A {
        match self.bits {
            false => ASCAFBH_A::_0,
            true => ASCAFBH_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCAFBH_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCAFBH_A::_1
    }
}
///Field `ASCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable
pub type ASCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, ASCAFBH_A>;
impl<'a, REG> ASCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCAFBH_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBRAL_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<ASCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
pub type ASCBRAL_R = crate::BitReader<ASCBRAL_A>;
impl ASCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCBRAL_A {
        match self.bits {
            false => ASCBRAL_A::_0,
            true => ASCBRAL_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBRAL_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBRAL_A::_1
    }
}
///Field `ASCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
pub type ASCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, ASCBRAL_A>;
impl<'a, REG> ASCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBRAL_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBRAH_A {
    ///0: Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<ASCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable
pub type ASCBRAH_R = crate::BitReader<ASCBRAH_A>;
impl ASCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCBRAH_A {
        match self.bits {
            false => ASCBRAH_A::_0,
            true => ASCBRAH_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBRAH_A::_0
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBRAH_A::_1
    }
}
///Field `ASCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable
pub type ASCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, ASCBRAH_A>;
impl<'a, REG> ASCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBRAH_A::_0)
    }
    ///Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBFAL_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<ASCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
pub type ASCBFAL_R = crate::BitReader<ASCBFAL_A>;
impl ASCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCBFAL_A {
        match self.bits {
            false => ASCBFAL_A::_0,
            true => ASCBFAL_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBFAL_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBFAL_A::_1
    }
}
///Field `ASCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
pub type ASCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, ASCBFAL_A>;
impl<'a, REG> ASCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBFAL_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASCBFAH_A {
    ///0: Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<ASCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: ASCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable
pub type ASCBFAH_R = crate::BitReader<ASCBFAH_A>;
impl ASCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASCBFAH_A {
        match self.bits {
            false => ASCBFAH_A::_0,
            true => ASCBFAH_A::_1,
        }
    }
    ///Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASCBFAH_A::_0
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASCBFAH_A::_1
    }
}
///Field `ASCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable
pub type ASCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, ASCBFAH_A>;
impl<'a, REG> ASCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBFAH_A::_0)
    }
    ///Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCA_A {
    ///0: Disable GTCCRA input capture on ELC_GPTA input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTA input.
    _1 = 1,
}
impl From<ASELCA_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCA` reader - ELC_GPTA Event Source GTCCRA Input Capture Enable
pub type ASELCA_R = crate::BitReader<ASELCA_A>;
impl ASELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCA_A {
        match self.bits {
            false => ASELCA_A::_0,
            true => ASELCA_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCA_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCA_A::_1
    }
}
///Field `ASELCA` writer - ELC_GPTA Event Source GTCCRA Input Capture Enable
pub type ASELCA_W<'a, REG> = crate::BitWriter<'a, REG, ASELCA_A>;
impl<'a, REG> ASELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCA_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCA_A::_1)
    }
}
/**ELC_GPTB Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCB_A {
    ///0: Disable GTCCRA input capture on ELC_GPTB input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTB input
    _1 = 1,
}
impl From<ASELCB_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCB` reader - ELC_GPTB Event Source GTCCRA Input Capture Enable
pub type ASELCB_R = crate::BitReader<ASELCB_A>;
impl ASELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCB_A {
        match self.bits {
            false => ASELCB_A::_0,
            true => ASELCB_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCB_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCB_A::_1
    }
}
///Field `ASELCB` writer - ELC_GPTB Event Source GTCCRA Input Capture Enable
pub type ASELCB_W<'a, REG> = crate::BitWriter<'a, REG, ASELCB_A>;
impl<'a, REG> ASELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCB_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCB_A::_1)
    }
}
/**ELC_GPTC Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCC_A {
    ///0: Disable GTCCRA input capture on ELC_GPTC input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTC input.
    _1 = 1,
}
impl From<ASELCC_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCC` reader - ELC_GPTC Event Source GTCCRA Input Capture Enable
pub type ASELCC_R = crate::BitReader<ASELCC_A>;
impl ASELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCC_A {
        match self.bits {
            false => ASELCC_A::_0,
            true => ASELCC_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCC_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCC_A::_1
    }
}
///Field `ASELCC` writer - ELC_GPTC Event Source GTCCRA Input Capture Enable
pub type ASELCC_W<'a, REG> = crate::BitWriter<'a, REG, ASELCC_A>;
impl<'a, REG> ASELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCC_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCC_A::_1)
    }
}
/**ELC_GPTD Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCD_A {
    ///0: Disable GTCCRA input capture on ELC_GPTD input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTD input.
    _1 = 1,
}
impl From<ASELCD_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCD` reader - ELC_GPTD Event Source GTCCRA Input Capture Enable
pub type ASELCD_R = crate::BitReader<ASELCD_A>;
impl ASELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCD_A {
        match self.bits {
            false => ASELCD_A::_0,
            true => ASELCD_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCD_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCD_A::_1
    }
}
///Field `ASELCD` writer - ELC_GPTD Event Source GTCCRA Input Capture Enable
pub type ASELCD_W<'a, REG> = crate::BitWriter<'a, REG, ASELCD_A>;
impl<'a, REG> ASELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCD_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCD_A::_1)
    }
}
/**ELC_GPTE Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCE_A {
    ///0: Disable GTCCRA input capture on ELC_GPTE input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTE input.
    _1 = 1,
}
impl From<ASELCE_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCE` reader - ELC_GPTE Event Source GTCCRA Input Capture Enable
pub type ASELCE_R = crate::BitReader<ASELCE_A>;
impl ASELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCE_A {
        match self.bits {
            false => ASELCE_A::_0,
            true => ASELCE_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCE_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTE input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCE_A::_1
    }
}
///Field `ASELCE` writer - ELC_GPTE Event Source GTCCRA Input Capture Enable
pub type ASELCE_W<'a, REG> = crate::BitWriter<'a, REG, ASELCE_A>;
impl<'a, REG> ASELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCE_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTE input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCE_A::_1)
    }
}
/**ELC_GPTF Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCF_A {
    ///0: Disable GTCCRA input capture on ELC_GPTF input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTF input.
    _1 = 1,
}
impl From<ASELCF_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCF` reader - ELC_GPTF Event Source GTCCRA Input Capture Enable
pub type ASELCF_R = crate::BitReader<ASELCF_A>;
impl ASELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCF_A {
        match self.bits {
            false => ASELCF_A::_0,
            true => ASELCF_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCF_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTF input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCF_A::_1
    }
}
///Field `ASELCF` writer - ELC_GPTF Event Source GTCCRA Input Capture Enable
pub type ASELCF_W<'a, REG> = crate::BitWriter<'a, REG, ASELCF_A>;
impl<'a, REG> ASELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCF_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTF input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCF_A::_1)
    }
}
/**ELC_GPTG Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCG_A {
    ///0: Disable GTCCRA input capture on ELC_GPTG input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTG input.
    _1 = 1,
}
impl From<ASELCG_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCG` reader - ELC_GPTG Event Source GTCCRA Input Capture Enable
pub type ASELCG_R = crate::BitReader<ASELCG_A>;
impl ASELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCG_A {
        match self.bits {
            false => ASELCG_A::_0,
            true => ASELCG_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCG_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTG input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCG_A::_1
    }
}
///Field `ASELCG` writer - ELC_GPTG Event Source GTCCRA Input Capture Enable
pub type ASELCG_W<'a, REG> = crate::BitWriter<'a, REG, ASELCG_A>;
impl<'a, REG> ASELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCG_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTG input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCG_A::_1)
    }
}
/**ELC_GPTH Event Source GTCCRA Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASELCH_A {
    ///0: Disable GTCCRA input capture on ELC_GPTH input
    _0 = 0,
    ///1: Enable GTCCRA input capture on ELC_GPTH input
    _1 = 1,
}
impl From<ASELCH_A> for bool {
    #[inline(always)]
    fn from(variant: ASELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASELCH` reader - ELC_GPTH Event Source GTCCRA Input Capture Enable
pub type ASELCH_R = crate::BitReader<ASELCH_A>;
impl ASELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASELCH_A {
        match self.bits {
            false => ASELCH_A::_0,
            true => ASELCH_A::_1,
        }
    }
    ///Disable GTCCRA input capture on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASELCH_A::_0
    }
    ///Enable GTCCRA input capture on ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASELCH_A::_1
    }
}
///Field `ASELCH` writer - ELC_GPTH Event Source GTCCRA Input Capture Enable
pub type ASELCH_W<'a, REG> = crate::BitWriter<'a, REG, ASELCH_A>;
impl<'a, REG> ASELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRA input capture on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCH_A::_0)
    }
    ///Enable GTCCRA input capture on ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASELCH_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgar(&self) -> ASGTRGAR_R {
        ASGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgaf(&self) -> ASGTRGAF_R {
        ASGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbr(&self) -> ASGTRGBR_R {
        ASGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbf(&self) -> ASGTRGBF_R {
        ASGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcr(&self) -> ASGTRGCR_R {
        ASGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcf(&self) -> ASGTRGCF_R {
        ASGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdr(&self) -> ASGTRGDR_R {
        ASGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdf(&self) -> ASGTRGDF_R {
        ASGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbl(&self) -> ASCARBL_R {
        ASCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbh(&self) -> ASCARBH_R {
        ASCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbl(&self) -> ASCAFBL_R {
        ASCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbh(&self) -> ASCAFBH_R {
        ASCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbral(&self) -> ASCBRAL_R {
        ASCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbrah(&self) -> ASCBRAH_R {
        ASCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfal(&self) -> ASCBFAL_R {
        ASCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfah(&self) -> ASCBFAH_R {
        ASCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselca(&self) -> ASELCA_R {
        ASELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcb(&self) -> ASELCB_R {
        ASELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcc(&self) -> ASELCC_R {
        ASELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcd(&self) -> ASELCD_R {
        ASELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselce(&self) -> ASELCE_R {
        ASELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcf(&self) -> ASELCF_R {
        ASELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcg(&self) -> ASELCG_R {
        ASELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselch(&self) -> ASELCH_R {
        ASELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgar(&mut self) -> ASGTRGAR_W<GTICASR_SPEC> {
        ASGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgaf(&mut self) -> ASGTRGAF_W<GTICASR_SPEC> {
        ASGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbr(&mut self) -> ASGTRGBR_W<GTICASR_SPEC> {
        ASGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgbf(&mut self) -> ASGTRGBF_W<GTICASR_SPEC> {
        ASGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcr(&mut self) -> ASGTRGCR_W<GTICASR_SPEC> {
        ASGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgcf(&mut self) -> ASGTRGCF_W<GTICASR_SPEC> {
        ASGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdr(&mut self) -> ASGTRGDR_W<GTICASR_SPEC> {
        ASGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn asgtrgdf(&mut self) -> ASGTRGDF_W<GTICASR_SPEC> {
        ASGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbl(&mut self) -> ASCARBL_W<GTICASR_SPEC> {
        ASCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascarbh(&mut self) -> ASCARBH_W<GTICASR_SPEC> {
        ASCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbl(&mut self) -> ASCAFBL_W<GTICASR_SPEC> {
        ASCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascafbh(&mut self) -> ASCAFBH_W<GTICASR_SPEC> {
        ASCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbral(&mut self) -> ASCBRAL_W<GTICASR_SPEC> {
        ASCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbrah(&mut self) -> ASCBRAH_W<GTICASR_SPEC> {
        ASCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfal(&mut self) -> ASCBFAL_W<GTICASR_SPEC> {
        ASCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn ascbfah(&mut self) -> ASCBFAH_W<GTICASR_SPEC> {
        ASCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselca(&mut self) -> ASELCA_W<GTICASR_SPEC> {
        ASELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcb(&mut self) -> ASELCB_W<GTICASR_SPEC> {
        ASELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcc(&mut self) -> ASELCC_W<GTICASR_SPEC> {
        ASELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcd(&mut self) -> ASELCD_W<GTICASR_SPEC> {
        ASELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselce(&mut self) -> ASELCE_W<GTICASR_SPEC> {
        ASELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcf(&mut self) -> ASELCF_W<GTICASR_SPEC> {
        ASELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselcg(&mut self) -> ASELCG_W<GTICASR_SPEC> {
        ASELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source GTCCRA Input Capture Enable
    #[inline(always)]
    pub fn aselch(&mut self) -> ASELCH_W<GTICASR_SPEC> {
        ASELCH_W::new(self, 23)
    }
}
/**General PWM Timer Input Capture Source Select Register A

You can [`read`](crate::Reg::read) this register and get [`gticasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTICASR_SPEC;
impl crate::RegisterSpec for GTICASR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gticasr::R`](R) reader structure
impl crate::Readable for GTICASR_SPEC {}
///`write(|w| ..)` method takes [`gticasr::W`](W) writer structure
impl crate::Writable for GTICASR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTICASR to value 0
impl crate::Resettable for GTICASR_SPEC {}
