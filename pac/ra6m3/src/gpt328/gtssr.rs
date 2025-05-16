///Register `GTSSR` reader
pub type R = crate::R<GTSSR_SPEC>;
///Register `GTSSR` writer
pub type W = crate::W<GTSSR_SPEC>;
/**GTETRGA Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGAR_A {
    ///0: Disable counter start on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTETRGA input.
    _1 = 1,
}
impl From<SSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Start Enable
pub type SSGTRGAR_R = crate::BitReader<SSGTRGAR_A>;
impl SSGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGAR_A {
        match self.bits {
            false => SSGTRGAR_A::_0,
            true => SSGTRGAR_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGAR_A::_0
    }
    ///Enable counter start on the rising edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGAR_A::_1
    }
}
///Field `SSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Start Enable
pub type SSGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGAR_A>;
impl<'a, REG> SSGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAR_A::_0)
    }
    ///Enable counter start on the rising edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGAF_A {
    ///0: Disable counter start on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<SSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Start Enable
pub type SSGTRGAF_R = crate::BitReader<SSGTRGAF_A>;
impl SSGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGAF_A {
        match self.bits {
            false => SSGTRGAF_A::_0,
            true => SSGTRGAF_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGAF_A::_0
    }
    ///Enable counter start on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGAF_A::_1
    }
}
///Field `SSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Start Enable
pub type SSGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGAF_A>;
impl<'a, REG> SSGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAF_A::_0)
    }
    ///Enable counter start on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGBR_A {
    ///0: Disable counter start on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTETRGB input.
    _1 = 1,
}
impl From<SSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Start Enable
pub type SSGTRGBR_R = crate::BitReader<SSGTRGBR_A>;
impl SSGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGBR_A {
        match self.bits {
            false => SSGTRGBR_A::_0,
            true => SSGTRGBR_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGBR_A::_0
    }
    ///Enable counter start on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGBR_A::_1
    }
}
///Field `SSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Start Enable
pub type SSGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGBR_A>;
impl<'a, REG> SSGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBR_A::_0)
    }
    ///Enable counter start on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGBF_A {
    ///0: Disable counter start on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<SSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Start Enable
pub type SSGTRGBF_R = crate::BitReader<SSGTRGBF_A>;
impl SSGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGBF_A {
        match self.bits {
            false => SSGTRGBF_A::_0,
            true => SSGTRGBF_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGBF_A::_0
    }
    ///Enable counter start on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGBF_A::_1
    }
}
///Field `SSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Start Enable
pub type SSGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGBF_A>;
impl<'a, REG> SSGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBF_A::_0)
    }
    ///Enable counter start on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGCR_A {
    ///0: Disable counter start on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<SSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Start Enable
pub type SSGTRGCR_R = crate::BitReader<SSGTRGCR_A>;
impl SSGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGCR_A {
        match self.bits {
            false => SSGTRGCR_A::_0,
            true => SSGTRGCR_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGCR_A::_0
    }
    ///Enable counter start on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGCR_A::_1
    }
}
///Field `SSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Start Enable
pub type SSGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGCR_A>;
impl<'a, REG> SSGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGCR_A::_0)
    }
    ///Enable counter start on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGCF_A {
    ///0: Disable counter start on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<SSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Start Enable
pub type SSGTRGCF_R = crate::BitReader<SSGTRGCF_A>;
impl SSGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGCF_A {
        match self.bits {
            false => SSGTRGCF_A::_0,
            true => SSGTRGCF_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGCF_A::_0
    }
    ///Enable counter start on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGCF_A::_1
    }
}
///Field `SSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Start Enable
pub type SSGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGCF_A>;
impl<'a, REG> SSGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGCF_A::_0)
    }
    ///Enable counter start on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGDR_A {
    ///0: Disable counter start on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<SSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Start Enable
pub type SSGTRGDR_R = crate::BitReader<SSGTRGDR_A>;
impl SSGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGDR_A {
        match self.bits {
            false => SSGTRGDR_A::_0,
            true => SSGTRGDR_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGDR_A::_0
    }
    ///Enable counter start on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGDR_A::_1
    }
}
///Field `SSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Start Enable
pub type SSGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGDR_A>;
impl<'a, REG> SSGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGDR_A::_0)
    }
    ///Enable counter start on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSGTRGDF_A {
    ///0: Disable counter start on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTETRGD input.
    _1 = 1,
}
impl From<SSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: SSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Start Enable
pub type SSGTRGDF_R = crate::BitReader<SSGTRGDF_A>;
impl SSGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSGTRGDF_A {
        match self.bits {
            false => SSGTRGDF_A::_0,
            true => SSGTRGDF_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSGTRGDF_A::_0
    }
    ///Enable counter start on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSGTRGDF_A::_1
    }
}
///Field `SSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Start Enable
pub type SSGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, SSGTRGDF_A>;
impl<'a, REG> SSGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGDF_A::_0)
    }
    ///Enable counter start on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCARBL_A {
    ///0: Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<SSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable
pub type SSCARBL_R = crate::BitReader<SSCARBL_A>;
impl SSCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCARBL_A {
        match self.bits {
            false => SSCARBL_A::_0,
            true => SSCARBL_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCARBL_A::_0
    }
    ///Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCARBL_A::_1
    }
}
///Field `SSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable
pub type SSCARBL_W<'a, REG> = crate::BitWriter<'a, REG, SSCARBL_A>;
impl<'a, REG> SSCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBL_A::_0)
    }
    ///Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCARBH_A {
    ///0: Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 1
    _1 = 1,
}
impl From<SSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable
pub type SSCARBH_R = crate::BitReader<SSCARBH_A>;
impl SSCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCARBH_A {
        match self.bits {
            false => SSCARBH_A::_0,
            true => SSCARBH_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCARBH_A::_0
    }
    ///Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCARBH_A::_1
    }
}
///Field `SSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable
pub type SSCARBH_W<'a, REG> = crate::BitWriter<'a, REG, SSCARBH_A>;
impl<'a, REG> SSCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBH_A::_0)
    }
    ///Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCAFBL_A {
    ///0: Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<SSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable
pub type SSCAFBL_R = crate::BitReader<SSCAFBL_A>;
impl SSCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCAFBL_A {
        match self.bits {
            false => SSCAFBL_A::_0,
            true => SSCAFBL_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCAFBL_A::_0
    }
    ///Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCAFBL_A::_1
    }
}
///Field `SSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable
pub type SSCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, SSCAFBL_A>;
impl<'a, REG> SSCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBL_A::_0)
    }
    ///Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCAFBH_A {
    ///0: Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<SSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable
pub type SSCAFBH_R = crate::BitReader<SSCAFBH_A>;
impl SSCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCAFBH_A {
        match self.bits {
            false => SSCAFBH_A::_0,
            true => SSCAFBH_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCAFBH_A::_0
    }
    ///Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCAFBH_A::_1
    }
}
///Field `SSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable
pub type SSCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, SSCAFBH_A>;
impl<'a, REG> SSCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBH_A::_0)
    }
    ///Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBRAL_A {
    ///0: Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<SSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable
pub type SSCBRAL_R = crate::BitReader<SSCBRAL_A>;
impl SSCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCBRAL_A {
        match self.bits {
            false => SSCBRAL_A::_0,
            true => SSCBRAL_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBRAL_A::_0
    }
    ///Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBRAL_A::_1
    }
}
///Field `SSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable
pub type SSCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, SSCBRAL_A>;
impl<'a, REG> SSCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAL_A::_0)
    }
    ///Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBRAH_A {
    ///0: Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<SSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable
pub type SSCBRAH_R = crate::BitReader<SSCBRAH_A>;
impl SSCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCBRAH_A {
        match self.bits {
            false => SSCBRAH_A::_0,
            true => SSCBRAH_A::_1,
        }
    }
    ///Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBRAH_A::_0
    }
    ///Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBRAH_A::_1
    }
}
///Field `SSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable
pub type SSCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, SSCBRAH_A>;
impl<'a, REG> SSCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAH_A::_0)
    }
    ///Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBFAL_A {
    ///0: Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<SSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable
pub type SSCBFAL_R = crate::BitReader<SSCBFAL_A>;
impl SSCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCBFAL_A {
        match self.bits {
            false => SSCBFAL_A::_0,
            true => SSCBFAL_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBFAL_A::_0
    }
    ///Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBFAL_A::_1
    }
}
///Field `SSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable
pub type SSCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, SSCBFAL_A>;
impl<'a, REG> SSCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAL_A::_0)
    }
    ///Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCBFAH_A {
    ///0: Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<SSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: SSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable
pub type SSCBFAH_R = crate::BitReader<SSCBFAH_A>;
impl SSCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSCBFAH_A {
        match self.bits {
            false => SSCBFAH_A::_0,
            true => SSCBFAH_A::_1,
        }
    }
    ///Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSCBFAH_A::_0
    }
    ///Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSCBFAH_A::_1
    }
}
///Field `SSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable
pub type SSCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, SSCBFAH_A>;
impl<'a, REG> SSCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAH_A::_0)
    }
    ///Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCA_A {
    ///0: Disable counter start on ELC_GPTA input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTA input.
    _1 = 1,
}
impl From<SSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCA` reader - ELC_GPTA Event Source Counter Start Enable
pub type SSELCA_R = crate::BitReader<SSELCA_A>;
impl SSELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCA_A {
        match self.bits {
            false => SSELCA_A::_0,
            true => SSELCA_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCA_A::_0
    }
    ///Enable counter start on ELC_GPTA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCA_A::_1
    }
}
///Field `SSELCA` writer - ELC_GPTA Event Source Counter Start Enable
pub type SSELCA_W<'a, REG> = crate::BitWriter<'a, REG, SSELCA_A>;
impl<'a, REG> SSELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCA_A::_0)
    }
    ///Enable counter start on ELC_GPTA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCA_A::_1)
    }
}
/**ELC_GPTB Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCB_A {
    ///0: Disable counter start on ELC_GPTB input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTB input.
    _1 = 1,
}
impl From<SSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCB` reader - ELC_GPTB Event Source Counter Start Enable
pub type SSELCB_R = crate::BitReader<SSELCB_A>;
impl SSELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCB_A {
        match self.bits {
            false => SSELCB_A::_0,
            true => SSELCB_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCB_A::_0
    }
    ///Enable counter start on ELC_GPTB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCB_A::_1
    }
}
///Field `SSELCB` writer - ELC_GPTB Event Source Counter Start Enable
pub type SSELCB_W<'a, REG> = crate::BitWriter<'a, REG, SSELCB_A>;
impl<'a, REG> SSELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCB_A::_0)
    }
    ///Enable counter start on ELC_GPTB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCB_A::_1)
    }
}
/**ELC_GPTC Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCC_A {
    ///0: Disable counter start on ELC_GPTC input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTC input.
    _1 = 1,
}
impl From<SSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCC` reader - ELC_GPTC Event Source Counter Start Enable
pub type SSELCC_R = crate::BitReader<SSELCC_A>;
impl SSELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCC_A {
        match self.bits {
            false => SSELCC_A::_0,
            true => SSELCC_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCC_A::_0
    }
    ///Enable counter start on ELC_GPTC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCC_A::_1
    }
}
///Field `SSELCC` writer - ELC_GPTC Event Source Counter Start Enable
pub type SSELCC_W<'a, REG> = crate::BitWriter<'a, REG, SSELCC_A>;
impl<'a, REG> SSELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCC_A::_0)
    }
    ///Enable counter start on ELC_GPTC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCC_A::_1)
    }
}
/**ELC_GPTD Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCD_A {
    ///0: Disable counter start on ELC_GPTD input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTD input.
    _1 = 1,
}
impl From<SSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCD` reader - ELC_GPTD Event Source Counter Start Enable
pub type SSELCD_R = crate::BitReader<SSELCD_A>;
impl SSELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCD_A {
        match self.bits {
            false => SSELCD_A::_0,
            true => SSELCD_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCD_A::_0
    }
    ///Enable counter start on ELC_GPTD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCD_A::_1
    }
}
///Field `SSELCD` writer - ELC_GPTD Event Source Counter Start Enable
pub type SSELCD_W<'a, REG> = crate::BitWriter<'a, REG, SSELCD_A>;
impl<'a, REG> SSELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCD_A::_0)
    }
    ///Enable counter start on ELC_GPTD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCD_A::_1)
    }
}
/**ELC_GPTE Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCE_A {
    ///0: Disable counter start on ELC_GPTE input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTE input
    _1 = 1,
}
impl From<SSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCE` reader - ELC_GPTE Event Source Counter Start Enable
pub type SSELCE_R = crate::BitReader<SSELCE_A>;
impl SSELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCE_A {
        match self.bits {
            false => SSELCE_A::_0,
            true => SSELCE_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCE_A::_0
    }
    ///Enable counter start on ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCE_A::_1
    }
}
///Field `SSELCE` writer - ELC_GPTE Event Source Counter Start Enable
pub type SSELCE_W<'a, REG> = crate::BitWriter<'a, REG, SSELCE_A>;
impl<'a, REG> SSELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCE_A::_0)
    }
    ///Enable counter start on ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCE_A::_1)
    }
}
/**ELC_GPTF Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCF_A {
    ///0: Disable counter start on ELC_GPTF input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTF input
    _1 = 1,
}
impl From<SSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCF` reader - ELC_GPTF Event Source Counter Start Enable
pub type SSELCF_R = crate::BitReader<SSELCF_A>;
impl SSELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCF_A {
        match self.bits {
            false => SSELCF_A::_0,
            true => SSELCF_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCF_A::_0
    }
    ///Enable counter start on ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCF_A::_1
    }
}
///Field `SSELCF` writer - ELC_GPTF Event Source Counter Start Enable
pub type SSELCF_W<'a, REG> = crate::BitWriter<'a, REG, SSELCF_A>;
impl<'a, REG> SSELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCF_A::_0)
    }
    ///Enable counter start on ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCF_A::_1)
    }
}
/**ELC_GPTG Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCG_A {
    ///0: Disable counter start on ELC_GPTG input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTG input.
    _1 = 1,
}
impl From<SSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCG` reader - ELC_GPTG Event Source Counter Start Enable
pub type SSELCG_R = crate::BitReader<SSELCG_A>;
impl SSELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCG_A {
        match self.bits {
            false => SSELCG_A::_0,
            true => SSELCG_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCG_A::_0
    }
    ///Enable counter start on ELC_GPTG input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCG_A::_1
    }
}
///Field `SSELCG` writer - ELC_GPTG Event Source Counter Start Enable
pub type SSELCG_W<'a, REG> = crate::BitWriter<'a, REG, SSELCG_A>;
impl<'a, REG> SSELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCG_A::_0)
    }
    ///Enable counter start on ELC_GPTG input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCG_A::_1)
    }
}
/**ELC_GPTH Event Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSELCH_A {
    ///0: Disable counter start on ELC_GPTH input
    _0 = 0,
    ///1: Enable counter start on ELC_GPTH input.
    _1 = 1,
}
impl From<SSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: SSELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SSELCH` reader - ELC_GPTH Event Source Counter Start Enable
pub type SSELCH_R = crate::BitReader<SSELCH_A>;
impl SSELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSELCH_A {
        match self.bits {
            false => SSELCH_A::_0,
            true => SSELCH_A::_1,
        }
    }
    ///Disable counter start on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSELCH_A::_0
    }
    ///Enable counter start on ELC_GPTH input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSELCH_A::_1
    }
}
///Field `SSELCH` writer - ELC_GPTH Event Source Counter Start Enable
pub type SSELCH_W<'a, REG> = crate::BitWriter<'a, REG, SSELCH_A>;
impl<'a, REG> SSELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCH_A::_0)
    }
    ///Enable counter start on ELC_GPTH input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SSELCH_A::_1)
    }
}
/**Software Source Counter Start Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTRT_A {
    ///0: Disable counter start by the GTSTR register
    _0 = 0,
    ///1: Enable counter start by the GTSTR register
    _1 = 1,
}
impl From<CSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CSTRT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTRT` reader - Software Source Counter Start Enable
pub type CSTRT_R = crate::BitReader<CSTRT_A>;
impl CSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTRT_A {
        match self.bits {
            false => CSTRT_A::_0,
            true => CSTRT_A::_1,
        }
    }
    ///Disable counter start by the GTSTR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTRT_A::_0
    }
    ///Enable counter start by the GTSTR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTRT_A::_1
    }
}
///Field `CSTRT` writer - Software Source Counter Start Enable
pub type CSTRT_W<'a, REG> = crate::BitWriter<'a, REG, CSTRT_A>;
impl<'a, REG> CSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter start by the GTSTR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT_A::_0)
    }
    ///Enable counter start by the GTSTR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTRT_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgar(&self) -> SSGTRGAR_R {
        SSGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgaf(&self) -> SSGTRGAF_R {
        SSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbr(&self) -> SSGTRGBR_R {
        SSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbf(&self) -> SSGTRGBF_R {
        SSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcr(&self) -> SSGTRGCR_R {
        SSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcf(&self) -> SSGTRGCF_R {
        SSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdr(&self) -> SSGTRGDR_R {
        SSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdf(&self) -> SSGTRGDF_R {
        SSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbl(&self) -> SSCARBL_R {
        SSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbh(&self) -> SSCARBH_R {
        SSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbl(&self) -> SSCAFBL_R {
        SSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbh(&self) -> SSCAFBH_R {
        SSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbral(&self) -> SSCBRAL_R {
        SSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbrah(&self) -> SSCBRAH_R {
        SSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfal(&self) -> SSCBFAL_R {
        SSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfah(&self) -> SSCBFAH_R {
        SSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselca(&self) -> SSELCA_R {
        SSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcb(&self) -> SSELCB_R {
        SSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcc(&self) -> SSELCC_R {
        SSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcd(&self) -> SSELCD_R {
        SSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselce(&self) -> SSELCE_R {
        SSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcf(&self) -> SSELCF_R {
        SSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcg(&self) -> SSELCG_R {
        SSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselch(&self) -> SSELCH_R {
        SSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - Software Source Counter Start Enable
    #[inline(always)]
    pub fn cstrt(&self) -> CSTRT_R {
        CSTRT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgar(&mut self) -> SSGTRGAR_W<GTSSR_SPEC> {
        SSGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgaf(&mut self) -> SSGTRGAF_W<GTSSR_SPEC> {
        SSGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbr(&mut self) -> SSGTRGBR_W<GTSSR_SPEC> {
        SSGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgbf(&mut self) -> SSGTRGBF_W<GTSSR_SPEC> {
        SSGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcr(&mut self) -> SSGTRGCR_W<GTSSR_SPEC> {
        SSGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgcf(&mut self) -> SSGTRGCF_W<GTSSR_SPEC> {
        SSGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdr(&mut self) -> SSGTRGDR_W<GTSSR_SPEC> {
        SSGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Start Enable
    #[inline(always)]
    pub fn ssgtrgdf(&mut self) -> SSGTRGDF_W<GTSSR_SPEC> {
        SSGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbl(&mut self) -> SSCARBL_W<GTSSR_SPEC> {
        SSCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscarbh(&mut self) -> SSCARBH_W<GTSSR_SPEC> {
        SSCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbl(&mut self) -> SSCAFBL_W<GTSSR_SPEC> {
        SSCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscafbh(&mut self) -> SSCAFBH_W<GTSSR_SPEC> {
        SSCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbral(&mut self) -> SSCBRAL_W<GTSSR_SPEC> {
        SSCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbrah(&mut self) -> SSCBRAH_W<GTSSR_SPEC> {
        SSCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfal(&mut self) -> SSCBFAL_W<GTSSR_SPEC> {
        SSCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable
    #[inline(always)]
    pub fn sscbfah(&mut self) -> SSCBFAH_W<GTSSR_SPEC> {
        SSCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselca(&mut self) -> SSELCA_W<GTSSR_SPEC> {
        SSELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcb(&mut self) -> SSELCB_W<GTSSR_SPEC> {
        SSELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcc(&mut self) -> SSELCC_W<GTSSR_SPEC> {
        SSELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcd(&mut self) -> SSELCD_W<GTSSR_SPEC> {
        SSELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselce(&mut self) -> SSELCE_W<GTSSR_SPEC> {
        SSELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcf(&mut self) -> SSELCF_W<GTSSR_SPEC> {
        SSELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselcg(&mut self) -> SSELCG_W<GTSSR_SPEC> {
        SSELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Start Enable
    #[inline(always)]
    pub fn sselch(&mut self) -> SSELCH_W<GTSSR_SPEC> {
        SSELCH_W::new(self, 23)
    }
    ///Bit 31 - Software Source Counter Start Enable
    #[inline(always)]
    pub fn cstrt(&mut self) -> CSTRT_W<GTSSR_SPEC> {
        CSTRT_W::new(self, 31)
    }
}
/**General PWM Timer Start Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTSSR_SPEC;
impl crate::RegisterSpec for GTSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtssr::R`](R) reader structure
impl crate::Readable for GTSSR_SPEC {}
///`write(|w| ..)` method takes [`gtssr::W`](W) writer structure
impl crate::Writable for GTSSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTSSR to value 0
impl crate::Resettable for GTSSR_SPEC {}
