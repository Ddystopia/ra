///Register `GTUPSR` reader
pub type R = crate::R<GTUPSR_SPEC>;
///Register `GTUPSR` writer
pub type W = crate::W<GTUPSR_SPEC>;
/**GTETRGA Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGAR_A {
    ///0: Disable counter count up on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<USGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Up Enable
pub type USGTRGAR_R = crate::BitReader<USGTRGAR_A>;
impl USGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGAR_A {
        match self.bits {
            false => USGTRGAR_A::_0,
            true => USGTRGAR_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGAR_A::_0
    }
    ///Enable counter count up on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGAR_A::_1
    }
}
///Field `USGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Up Enable
pub type USGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGAR_A>;
impl<'a, REG> USGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAR_A::_0)
    }
    ///Enable counter count up on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGAF_A {
    ///0: Disable counter count up on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTETRGA input.
    _1 = 1,
}
impl From<USGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Up Enable
pub type USGTRGAF_R = crate::BitReader<USGTRGAF_A>;
impl USGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGAF_A {
        match self.bits {
            false => USGTRGAF_A::_0,
            true => USGTRGAF_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGAF_A::_0
    }
    ///Enable counter count up on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGAF_A::_1
    }
}
///Field `USGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Up Enable
pub type USGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGAF_A>;
impl<'a, REG> USGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAF_A::_0)
    }
    ///Enable counter count up on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGBR_A {
    ///0: Disable counter count up on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTETRGB input.
    _1 = 1,
}
impl From<USGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Up Enable
pub type USGTRGBR_R = crate::BitReader<USGTRGBR_A>;
impl USGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGBR_A {
        match self.bits {
            false => USGTRGBR_A::_0,
            true => USGTRGBR_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGBR_A::_0
    }
    ///Enable counter count up on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGBR_A::_1
    }
}
///Field `USGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Up Enable
pub type USGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGBR_A>;
impl<'a, REG> USGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBR_A::_0)
    }
    ///Enable counter count up on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGBF_A {
    ///0: Disable counter count up on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTETRGB input.
    _1 = 1,
}
impl From<USGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Up Enable
pub type USGTRGBF_R = crate::BitReader<USGTRGBF_A>;
impl USGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGBF_A {
        match self.bits {
            false => USGTRGBF_A::_0,
            true => USGTRGBF_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGBF_A::_0
    }
    ///Enable counter count up on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGBF_A::_1
    }
}
///Field `USGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Up Enable
pub type USGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGBF_A>;
impl<'a, REG> USGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBF_A::_0)
    }
    ///Enable counter count up on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGCR_A {
    ///0: Disable counter count up on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<USGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Count Up Enable
pub type USGTRGCR_R = crate::BitReader<USGTRGCR_A>;
impl USGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGCR_A {
        match self.bits {
            false => USGTRGCR_A::_0,
            true => USGTRGCR_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGCR_A::_0
    }
    ///Enable counter count up on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGCR_A::_1
    }
}
///Field `USGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Count Up Enable
pub type USGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGCR_A>;
impl<'a, REG> USGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGCR_A::_0)
    }
    ///Enable counter count up on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGCF_A {
    ///0: Disable counter count up on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTETRGC input.
    _1 = 1,
}
impl From<USGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Count Up Enable
pub type USGTRGCF_R = crate::BitReader<USGTRGCF_A>;
impl USGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGCF_A {
        match self.bits {
            false => USGTRGCF_A::_0,
            true => USGTRGCF_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGCF_A::_0
    }
    ///Enable counter count up on the falling edge of GTETRGC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGCF_A::_1
    }
}
///Field `USGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Count Up Enable
pub type USGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGCF_A>;
impl<'a, REG> USGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGCF_A::_0)
    }
    ///Enable counter count up on the falling edge of GTETRGC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGDR_A {
    ///0: Disable counter count up on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<USGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Count Up Enable
pub type USGTRGDR_R = crate::BitReader<USGTRGDR_A>;
impl USGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGDR_A {
        match self.bits {
            false => USGTRGDR_A::_0,
            true => USGTRGDR_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGDR_A::_0
    }
    ///Enable counter count up on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGDR_A::_1
    }
}
///Field `USGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Count Up Enable
pub type USGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGDR_A>;
impl<'a, REG> USGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGDR_A::_0)
    }
    ///Enable counter count up on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USGTRGDF_A {
    ///0: Disable counter count up on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTETRGD input.
    _1 = 1,
}
impl From<USGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: USGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Count Up Enable
pub type USGTRGDF_R = crate::BitReader<USGTRGDF_A>;
impl USGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USGTRGDF_A {
        match self.bits {
            false => USGTRGDF_A::_0,
            true => USGTRGDF_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USGTRGDF_A::_0
    }
    ///Enable counter count up on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USGTRGDF_A::_1
    }
}
///Field `USGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Count Up Enable
pub type USGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, USGTRGDF_A>;
impl<'a, REG> USGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGDF_A::_0)
    }
    ///Enable counter count up on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCARBL_A {
    ///0: Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<USCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: USCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable
pub type USCARBL_R = crate::BitReader<USCARBL_A>;
impl USCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCARBL_A {
        match self.bits {
            false => USCARBL_A::_0,
            true => USCARBL_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCARBL_A::_0
    }
    ///Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCARBL_A::_1
    }
}
///Field `USCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable
pub type USCARBL_W<'a, REG> = crate::BitWriter<'a, REG, USCARBL_A>;
impl<'a, REG> USCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBL_A::_0)
    }
    ///Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCARBH_A {
    ///0: Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<USCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: USCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable
pub type USCARBH_R = crate::BitReader<USCARBH_A>;
impl USCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCARBH_A {
        match self.bits {
            false => USCARBH_A::_0,
            true => USCARBH_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCARBH_A::_0
    }
    ///Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCARBH_A::_1
    }
}
///Field `USCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable
pub type USCARBH_W<'a, REG> = crate::BitWriter<'a, REG, USCARBH_A>;
impl<'a, REG> USCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBH_A::_0)
    }
    ///Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCAFBL_A {
    ///0: Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<USCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: USCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable
pub type USCAFBL_R = crate::BitReader<USCAFBL_A>;
impl USCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCAFBL_A {
        match self.bits {
            false => USCAFBL_A::_0,
            true => USCAFBL_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCAFBL_A::_0
    }
    ///Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCAFBL_A::_1
    }
}
///Field `USCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable
pub type USCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, USCAFBL_A>;
impl<'a, REG> USCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBL_A::_0)
    }
    ///Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCAFBH_A {
    ///0: Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<USCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: USCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable
pub type USCAFBH_R = crate::BitReader<USCAFBH_A>;
impl USCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCAFBH_A {
        match self.bits {
            false => USCAFBH_A::_0,
            true => USCAFBH_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCAFBH_A::_0
    }
    ///Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCAFBH_A::_1
    }
}
///Field `USCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable
pub type USCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, USCAFBH_A>;
impl<'a, REG> USCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBH_A::_0)
    }
    ///Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBRAL_A {
    ///0: Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<USCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: USCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable
pub type USCBRAL_R = crate::BitReader<USCBRAL_A>;
impl USCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCBRAL_A {
        match self.bits {
            false => USCBRAL_A::_0,
            true => USCBRAL_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBRAL_A::_0
    }
    ///Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBRAL_A::_1
    }
}
///Field `USCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable
pub type USCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, USCBRAL_A>;
impl<'a, REG> USCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAL_A::_0)
    }
    ///Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBRAH_A {
    ///0: Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<USCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: USCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable
pub type USCBRAH_R = crate::BitReader<USCBRAH_A>;
impl USCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCBRAH_A {
        match self.bits {
            false => USCBRAH_A::_0,
            true => USCBRAH_A::_1,
        }
    }
    ///Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBRAH_A::_0
    }
    ///Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBRAH_A::_1
    }
}
///Field `USCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable
pub type USCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, USCBRAH_A>;
impl<'a, REG> USCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAH_A::_0)
    }
    ///Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBFAL_A {
    ///0: Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<USCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: USCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable
pub type USCBFAL_R = crate::BitReader<USCBFAL_A>;
impl USCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCBFAL_A {
        match self.bits {
            false => USCBFAL_A::_0,
            true => USCBFAL_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBFAL_A::_0
    }
    ///Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBFAL_A::_1
    }
}
///Field `USCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable
pub type USCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, USCBFAL_A>;
impl<'a, REG> USCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAL_A::_0)
    }
    ///Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCBFAH_A {
    ///0: Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<USCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: USCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable
pub type USCBFAH_R = crate::BitReader<USCBFAH_A>;
impl USCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USCBFAH_A {
        match self.bits {
            false => USCBFAH_A::_0,
            true => USCBFAH_A::_1,
        }
    }
    ///Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USCBFAH_A::_0
    }
    ///Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USCBFAH_A::_1
    }
}
///Field `USCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable
pub type USCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, USCBFAH_A>;
impl<'a, REG> USCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAH_A::_0)
    }
    ///Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCA_A {
    ///0: Disable counter count up on ELC_GPTA input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTA input.
    _1 = 1,
}
impl From<USELCA_A> for bool {
    #[inline(always)]
    fn from(variant: USELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCA` reader - ELC_GPTA Event Source Counter Count Up Enable
pub type USELCA_R = crate::BitReader<USELCA_A>;
impl USELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCA_A {
        match self.bits {
            false => USELCA_A::_0,
            true => USELCA_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCA_A::_0
    }
    ///Enable counter count up on ELC_GPTA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCA_A::_1
    }
}
///Field `USELCA` writer - ELC_GPTA Event Source Counter Count Up Enable
pub type USELCA_W<'a, REG> = crate::BitWriter<'a, REG, USELCA_A>;
impl<'a, REG> USELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCA_A::_0)
    }
    ///Enable counter count up on ELC_GPTA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCA_A::_1)
    }
}
/**ELC_GPTB Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCB_A {
    ///0: Disable counter count up on ELC_GPTB input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTB input.
    _1 = 1,
}
impl From<USELCB_A> for bool {
    #[inline(always)]
    fn from(variant: USELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCB` reader - ELC_GPTB Event Source Counter Count Up Enable
pub type USELCB_R = crate::BitReader<USELCB_A>;
impl USELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCB_A {
        match self.bits {
            false => USELCB_A::_0,
            true => USELCB_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCB_A::_0
    }
    ///Enable counter count up on ELC_GPTB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCB_A::_1
    }
}
///Field `USELCB` writer - ELC_GPTB Event Source Counter Count Up Enable
pub type USELCB_W<'a, REG> = crate::BitWriter<'a, REG, USELCB_A>;
impl<'a, REG> USELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCB_A::_0)
    }
    ///Enable counter count up on ELC_GPTB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCB_A::_1)
    }
}
/**ELC_GPTC Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCC_A {
    ///0: Disable counter count up on ELC_GPTC input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTC input.
    _1 = 1,
}
impl From<USELCC_A> for bool {
    #[inline(always)]
    fn from(variant: USELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCC` reader - ELC_GPTC Event Source Counter Count Up Enable
pub type USELCC_R = crate::BitReader<USELCC_A>;
impl USELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCC_A {
        match self.bits {
            false => USELCC_A::_0,
            true => USELCC_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCC_A::_0
    }
    ///Enable counter count up on ELC_GPTC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCC_A::_1
    }
}
///Field `USELCC` writer - ELC_GPTC Event Source Counter Count Up Enable
pub type USELCC_W<'a, REG> = crate::BitWriter<'a, REG, USELCC_A>;
impl<'a, REG> USELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCC_A::_0)
    }
    ///Enable counter count up on ELC_GPTC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCC_A::_1)
    }
}
/**ELC_GPTD Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCD_A {
    ///0: Disable counter count up on ELC_GPTD input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTD input
    _1 = 1,
}
impl From<USELCD_A> for bool {
    #[inline(always)]
    fn from(variant: USELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCD` reader - ELC_GPTD Event Source Counter Count Up Enable
pub type USELCD_R = crate::BitReader<USELCD_A>;
impl USELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCD_A {
        match self.bits {
            false => USELCD_A::_0,
            true => USELCD_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCD_A::_0
    }
    ///Enable counter count up on ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCD_A::_1
    }
}
///Field `USELCD` writer - ELC_GPTD Event Source Counter Count Up Enable
pub type USELCD_W<'a, REG> = crate::BitWriter<'a, REG, USELCD_A>;
impl<'a, REG> USELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCD_A::_0)
    }
    ///Enable counter count up on ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCD_A::_1)
    }
}
/**ELC_GPTE Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCE_A {
    ///0: Disable counter count up on ELC_GPTE input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTE input.put
    _1 = 1,
}
impl From<USELCE_A> for bool {
    #[inline(always)]
    fn from(variant: USELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCE` reader - ELC_GPTE Event Source Counter Count Up Enable
pub type USELCE_R = crate::BitReader<USELCE_A>;
impl USELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCE_A {
        match self.bits {
            false => USELCE_A::_0,
            true => USELCE_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCE_A::_0
    }
    ///Enable counter count up on ELC_GPTE input.put
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCE_A::_1
    }
}
///Field `USELCE` writer - ELC_GPTE Event Source Counter Count Up Enable
pub type USELCE_W<'a, REG> = crate::BitWriter<'a, REG, USELCE_A>;
impl<'a, REG> USELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCE_A::_0)
    }
    ///Enable counter count up on ELC_GPTE input.put
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCE_A::_1)
    }
}
/**ELC_GPTF Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCF_A {
    ///0: Disable counter count up on ELC_GPTF input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTF input.
    _1 = 1,
}
impl From<USELCF_A> for bool {
    #[inline(always)]
    fn from(variant: USELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCF` reader - ELC_GPTF Event Source Counter Count Up Enable
pub type USELCF_R = crate::BitReader<USELCF_A>;
impl USELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCF_A {
        match self.bits {
            false => USELCF_A::_0,
            true => USELCF_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCF_A::_0
    }
    ///Enable counter count up on ELC_GPTF input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCF_A::_1
    }
}
///Field `USELCF` writer - ELC_GPTF Event Source Counter Count Up Enable
pub type USELCF_W<'a, REG> = crate::BitWriter<'a, REG, USELCF_A>;
impl<'a, REG> USELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCF_A::_0)
    }
    ///Enable counter count up on ELC_GPTF input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCF_A::_1)
    }
}
/**ELC_GPTG Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCG_A {
    ///0: Disable counter count up on ELC_GPTG input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTG input.
    _1 = 1,
}
impl From<USELCG_A> for bool {
    #[inline(always)]
    fn from(variant: USELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCG` reader - ELC_GPTG Event Source Counter Count Up Enable
pub type USELCG_R = crate::BitReader<USELCG_A>;
impl USELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCG_A {
        match self.bits {
            false => USELCG_A::_0,
            true => USELCG_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCG_A::_0
    }
    ///Enable counter count up on ELC_GPTG input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCG_A::_1
    }
}
///Field `USELCG` writer - ELC_GPTG Event Source Counter Count Up Enable
pub type USELCG_W<'a, REG> = crate::BitWriter<'a, REG, USELCG_A>;
impl<'a, REG> USELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCG_A::_0)
    }
    ///Enable counter count up on ELC_GPTG input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCG_A::_1)
    }
}
/**ELC_GPTH Event Source Counter Count Up Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USELCH_A {
    ///0: Disable counter count up on ELC_GPTH input
    _0 = 0,
    ///1: Enable counter count up on ELC_GPTH input.
    _1 = 1,
}
impl From<USELCH_A> for bool {
    #[inline(always)]
    fn from(variant: USELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USELCH` reader - ELC_GPTH Event Source Counter Count Up Enable
pub type USELCH_R = crate::BitReader<USELCH_A>;
impl USELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USELCH_A {
        match self.bits {
            false => USELCH_A::_0,
            true => USELCH_A::_1,
        }
    }
    ///Disable counter count up on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USELCH_A::_0
    }
    ///Enable counter count up on ELC_GPTH input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USELCH_A::_1
    }
}
///Field `USELCH` writer - ELC_GPTH Event Source Counter Count Up Enable
pub type USELCH_W<'a, REG> = crate::BitWriter<'a, REG, USELCH_A>;
impl<'a, REG> USELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count up on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USELCH_A::_0)
    }
    ///Enable counter count up on ELC_GPTH input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USELCH_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgar(&self) -> USGTRGAR_R {
        USGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgaf(&self) -> USGTRGAF_R {
        USGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbr(&self) -> USGTRGBR_R {
        USGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbf(&self) -> USGTRGBF_R {
        USGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcr(&self) -> USGTRGCR_R {
        USGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcf(&self) -> USGTRGCF_R {
        USGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdr(&self) -> USGTRGDR_R {
        USGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdf(&self) -> USGTRGDF_R {
        USGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbl(&self) -> USCARBL_R {
        USCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbh(&self) -> USCARBH_R {
        USCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbl(&self) -> USCAFBL_R {
        USCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbh(&self) -> USCAFBH_R {
        USCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbral(&self) -> USCBRAL_R {
        USCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbrah(&self) -> USCBRAH_R {
        USCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfal(&self) -> USCBFAL_R {
        USCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfah(&self) -> USCBFAH_R {
        USCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselca(&self) -> USELCA_R {
        USELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcb(&self) -> USELCB_R {
        USELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcc(&self) -> USELCC_R {
        USELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcd(&self) -> USELCD_R {
        USELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselce(&self) -> USELCE_R {
        USELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcf(&self) -> USELCF_R {
        USELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcg(&self) -> USELCG_R {
        USELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselch(&self) -> USELCH_R {
        USELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgar(&mut self) -> USGTRGAR_W<GTUPSR_SPEC> {
        USGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgaf(&mut self) -> USGTRGAF_W<GTUPSR_SPEC> {
        USGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbr(&mut self) -> USGTRGBR_W<GTUPSR_SPEC> {
        USGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgbf(&mut self) -> USGTRGBF_W<GTUPSR_SPEC> {
        USGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcr(&mut self) -> USGTRGCR_W<GTUPSR_SPEC> {
        USGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgcf(&mut self) -> USGTRGCF_W<GTUPSR_SPEC> {
        USGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdr(&mut self) -> USGTRGDR_W<GTUPSR_SPEC> {
        USGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Up Enable
    #[inline(always)]
    pub fn usgtrgdf(&mut self) -> USGTRGDF_W<GTUPSR_SPEC> {
        USGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbl(&mut self) -> USCARBL_W<GTUPSR_SPEC> {
        USCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscarbh(&mut self) -> USCARBH_W<GTUPSR_SPEC> {
        USCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbl(&mut self) -> USCAFBL_W<GTUPSR_SPEC> {
        USCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscafbh(&mut self) -> USCAFBH_W<GTUPSR_SPEC> {
        USCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbral(&mut self) -> USCBRAL_W<GTUPSR_SPEC> {
        USCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbrah(&mut self) -> USCBRAH_W<GTUPSR_SPEC> {
        USCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfal(&mut self) -> USCBFAL_W<GTUPSR_SPEC> {
        USCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable
    #[inline(always)]
    pub fn uscbfah(&mut self) -> USCBFAH_W<GTUPSR_SPEC> {
        USCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselca(&mut self) -> USELCA_W<GTUPSR_SPEC> {
        USELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcb(&mut self) -> USELCB_W<GTUPSR_SPEC> {
        USELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcc(&mut self) -> USELCC_W<GTUPSR_SPEC> {
        USELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcd(&mut self) -> USELCD_W<GTUPSR_SPEC> {
        USELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselce(&mut self) -> USELCE_W<GTUPSR_SPEC> {
        USELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcf(&mut self) -> USELCF_W<GTUPSR_SPEC> {
        USELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselcg(&mut self) -> USELCG_W<GTUPSR_SPEC> {
        USELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Count Up Enable
    #[inline(always)]
    pub fn uselch(&mut self) -> USELCH_W<GTUPSR_SPEC> {
        USELCH_W::new(self, 23)
    }
}
/**General PWM Timer Up Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtupsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtupsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTUPSR_SPEC;
impl crate::RegisterSpec for GTUPSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtupsr::R`](R) reader structure
impl crate::Readable for GTUPSR_SPEC {}
///`write(|w| ..)` method takes [`gtupsr::W`](W) writer structure
impl crate::Writable for GTUPSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTUPSR to value 0
impl crate::Resettable for GTUPSR_SPEC {}
