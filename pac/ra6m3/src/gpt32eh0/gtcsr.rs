///Register `GTCSR` reader
pub type R = crate::R<GTCSR_SPEC>;
///Register `GTCSR` writer
pub type W = crate::W<GTCSR_SPEC>;
/**GTETRGA Pin Rising Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGAR_A {
    ///0: Disable counter clear on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<CSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Clear Enable
pub type CSGTRGAR_R = crate::BitReader<CSGTRGAR_A>;
impl CSGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGAR_A {
        match self.bits {
            false => CSGTRGAR_A::_0,
            true => CSGTRGAR_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGAR_A::_0
    }
    ///Enable counter clear on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGAR_A::_1
    }
}
///Field `CSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Clear Enable
pub type CSGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGAR_A>;
impl<'a, REG> CSGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAR_A::_0)
    }
    ///Enable counter clear on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGAF_A {
    ///0: Disable counter clear on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<CSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Clear Enable
pub type CSGTRGAF_R = crate::BitReader<CSGTRGAF_A>;
impl CSGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGAF_A {
        match self.bits {
            false => CSGTRGAF_A::_0,
            true => CSGTRGAF_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGAF_A::_0
    }
    ///Enable counter clear on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGAF_A::_1
    }
}
///Field `CSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Clear Enable
pub type CSGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGAF_A>;
impl<'a, REG> CSGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAF_A::_0)
    }
    ///Enable counter clear on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGBR_A {
    ///0: Disable counter clear on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<CSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Clear Enable
pub type CSGTRGBR_R = crate::BitReader<CSGTRGBR_A>;
impl CSGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGBR_A {
        match self.bits {
            false => CSGTRGBR_A::_0,
            true => CSGTRGBR_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGBR_A::_0
    }
    ///Enable counter clear on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGBR_A::_1
    }
}
///Field `CSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Clear Enable
pub type CSGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGBR_A>;
impl<'a, REG> CSGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBR_A::_0)
    }
    ///Enable counter clear on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGBF_A {
    ///0: Disable counter clear on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<CSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Clear Enable
pub type CSGTRGBF_R = crate::BitReader<CSGTRGBF_A>;
impl CSGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGBF_A {
        match self.bits {
            false => CSGTRGBF_A::_0,
            true => CSGTRGBF_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGBF_A::_0
    }
    ///Enable counter clear on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGBF_A::_1
    }
}
///Field `CSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Clear Enable
pub type CSGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGBF_A>;
impl<'a, REG> CSGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBF_A::_0)
    }
    ///Enable counter clear on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGCR_A {
    ///0: Disable counter clear on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<CSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Clear Enable
pub type CSGTRGCR_R = crate::BitReader<CSGTRGCR_A>;
impl CSGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGCR_A {
        match self.bits {
            false => CSGTRGCR_A::_0,
            true => CSGTRGCR_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGCR_A::_0
    }
    ///Enable counter clear on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGCR_A::_1
    }
}
///Field `CSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Clear Enable
pub type CSGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGCR_A>;
impl<'a, REG> CSGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGCR_A::_0)
    }
    ///Enable counter clear on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGCF_A {
    ///0: Disable counter clear on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<CSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Clear Enable
pub type CSGTRGCF_R = crate::BitReader<CSGTRGCF_A>;
impl CSGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGCF_A {
        match self.bits {
            false => CSGTRGCF_A::_0,
            true => CSGTRGCF_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGCF_A::_0
    }
    ///Enable counter clear on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGCF_A::_1
    }
}
///Field `CSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Clear Enable
pub type CSGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGCF_A>;
impl<'a, REG> CSGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGCF_A::_0)
    }
    ///Enable counter clear on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGDR_A {
    ///0: Disable counter clear on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<CSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Clear Enable
pub type CSGTRGDR_R = crate::BitReader<CSGTRGDR_A>;
impl CSGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGDR_A {
        match self.bits {
            false => CSGTRGDR_A::_0,
            true => CSGTRGDR_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGDR_A::_0
    }
    ///Enable counter clear on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGDR_A::_1
    }
}
///Field `CSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Clear Enable
pub type CSGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGDR_A>;
impl<'a, REG> CSGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGDR_A::_0)
    }
    ///Enable counter clear on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSGTRGDF_A {
    ///0: Disable counter clear on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<CSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: CSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Clear Enable
pub type CSGTRGDF_R = crate::BitReader<CSGTRGDF_A>;
impl CSGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSGTRGDF_A {
        match self.bits {
            false => CSGTRGDF_A::_0,
            true => CSGTRGDF_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSGTRGDF_A::_0
    }
    ///Enable counter clear on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSGTRGDF_A::_1
    }
}
///Field `CSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Clear Enable
pub type CSGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, CSGTRGDF_A>;
impl<'a, REG> CSGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGDF_A::_0)
    }
    ///Enable counter clear on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCARBL_A {
    ///0: Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0
    _1 = 1,
}
impl From<CSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable
pub type CSCARBL_R = crate::BitReader<CSCARBL_A>;
impl CSCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCARBL_A {
        match self.bits {
            false => CSCARBL_A::_0,
            true => CSCARBL_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCARBL_A::_0
    }
    ///Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCARBL_A::_1
    }
}
///Field `CSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable
pub type CSCARBL_W<'a, REG> = crate::BitWriter<'a, REG, CSCARBL_A>;
impl<'a, REG> CSCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBL_A::_0)
    }
    ///Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCARBH_A {
    ///0: Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1
    _1 = 1,
}
impl From<CSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable
pub type CSCARBH_R = crate::BitReader<CSCARBH_A>;
impl CSCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCARBH_A {
        match self.bits {
            false => CSCARBH_A::_0,
            true => CSCARBH_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCARBH_A::_0
    }
    ///Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCARBH_A::_1
    }
}
///Field `CSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable
pub type CSCARBH_W<'a, REG> = crate::BitWriter<'a, REG, CSCARBH_A>;
impl<'a, REG> CSCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBH_A::_0)
    }
    ///Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCAFBL_A {
    ///0: Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0
    _1 = 1,
}
impl From<CSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable
pub type CSCAFBL_R = crate::BitReader<CSCAFBL_A>;
impl CSCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCAFBL_A {
        match self.bits {
            false => CSCAFBL_A::_0,
            true => CSCAFBL_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCAFBL_A::_0
    }
    ///Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCAFBL_A::_1
    }
}
///Field `CSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable
pub type CSCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, CSCAFBL_A>;
impl<'a, REG> CSCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBL_A::_0)
    }
    ///Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCAFBH_A {
    ///0: Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1
    _1 = 1,
}
impl From<CSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable
pub type CSCAFBH_R = crate::BitReader<CSCAFBH_A>;
impl CSCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCAFBH_A {
        match self.bits {
            false => CSCAFBH_A::_0,
            true => CSCAFBH_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCAFBH_A::_0
    }
    ///Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCAFBH_A::_1
    }
}
///Field `CSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable
pub type CSCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, CSCAFBH_A>;
impl<'a, REG> CSCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBH_A::_0)
    }
    ///Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBRAL_A {
    ///0: Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0
    _1 = 1,
}
impl From<CSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable
pub type CSCBRAL_R = crate::BitReader<CSCBRAL_A>;
impl CSCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCBRAL_A {
        match self.bits {
            false => CSCBRAL_A::_0,
            true => CSCBRAL_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBRAL_A::_0
    }
    ///Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBRAL_A::_1
    }
}
///Field `CSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable
pub type CSCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, CSCBRAL_A>;
impl<'a, REG> CSCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAL_A::_0)
    }
    ///Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBRAH_A {
    ///0: Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1
    _1 = 1,
}
impl From<CSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable
pub type CSCBRAH_R = crate::BitReader<CSCBRAH_A>;
impl CSCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCBRAH_A {
        match self.bits {
            false => CSCBRAH_A::_0,
            true => CSCBRAH_A::_1,
        }
    }
    ///Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBRAH_A::_0
    }
    ///Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBRAH_A::_1
    }
}
///Field `CSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable
pub type CSCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, CSCBRAH_A>;
impl<'a, REG> CSCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAH_A::_0)
    }
    ///Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBFAL_A {
    ///0: Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0
    _1 = 1,
}
impl From<CSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable
pub type CSCBFAL_R = crate::BitReader<CSCBFAL_A>;
impl CSCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCBFAL_A {
        match self.bits {
            false => CSCBFAL_A::_0,
            true => CSCBFAL_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBFAL_A::_0
    }
    ///Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBFAL_A::_1
    }
}
///Field `CSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable
pub type CSCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, CSCBFAL_A>;
impl<'a, REG> CSCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAL_A::_0)
    }
    ///Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSCBFAH_A {
    ///0: Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1
    _1 = 1,
}
impl From<CSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: CSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable
pub type CSCBFAH_R = crate::BitReader<CSCBFAH_A>;
impl CSCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSCBFAH_A {
        match self.bits {
            false => CSCBFAH_A::_0,
            true => CSCBFAH_A::_1,
        }
    }
    ///Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSCBFAH_A::_0
    }
    ///Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSCBFAH_A::_1
    }
}
///Field `CSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable
pub type CSCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, CSCBFAH_A>;
impl<'a, REG> CSCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAH_A::_0)
    }
    ///Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCA_A {
    ///0: Disable counter clear on ELC_GPTA input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTA input
    _1 = 1,
}
impl From<CSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCA` reader - ELC_GPTA Event Source Counter Clear Enable
pub type CSELCA_R = crate::BitReader<CSELCA_A>;
impl CSELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCA_A {
        match self.bits {
            false => CSELCA_A::_0,
            true => CSELCA_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCA_A::_0
    }
    ///Enable counter clear on ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCA_A::_1
    }
}
///Field `CSELCA` writer - ELC_GPTA Event Source Counter Clear Enable
pub type CSELCA_W<'a, REG> = crate::BitWriter<'a, REG, CSELCA_A>;
impl<'a, REG> CSELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCA_A::_0)
    }
    ///Enable counter clear on ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCA_A::_1)
    }
}
/**ELC_GPTB Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCB_A {
    ///0: Disable counter clear on ELC_GPTB input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTB input
    _1 = 1,
}
impl From<CSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCB` reader - ELC_GPTB Event Source Counter Clear Enable
pub type CSELCB_R = crate::BitReader<CSELCB_A>;
impl CSELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCB_A {
        match self.bits {
            false => CSELCB_A::_0,
            true => CSELCB_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCB_A::_0
    }
    ///Enable counter clear on ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCB_A::_1
    }
}
///Field `CSELCB` writer - ELC_GPTB Event Source Counter Clear Enable
pub type CSELCB_W<'a, REG> = crate::BitWriter<'a, REG, CSELCB_A>;
impl<'a, REG> CSELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCB_A::_0)
    }
    ///Enable counter clear on ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCB_A::_1)
    }
}
/**ELC_GPTC Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCC_A {
    ///0: Disable counter clear on ELC_GPTC input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTC input
    _1 = 1,
}
impl From<CSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCC` reader - ELC_GPTC Event Source Counter Clear Enable
pub type CSELCC_R = crate::BitReader<CSELCC_A>;
impl CSELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCC_A {
        match self.bits {
            false => CSELCC_A::_0,
            true => CSELCC_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCC_A::_0
    }
    ///Enable counter clear on ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCC_A::_1
    }
}
///Field `CSELCC` writer - ELC_GPTC Event Source Counter Clear Enable
pub type CSELCC_W<'a, REG> = crate::BitWriter<'a, REG, CSELCC_A>;
impl<'a, REG> CSELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCC_A::_0)
    }
    ///Enable counter clear on ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCC_A::_1)
    }
}
/**ELC_GPTD Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCD_A {
    ///0: Disable counter clear on ELC_GPTD input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTD input
    _1 = 1,
}
impl From<CSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCD` reader - ELC_GPTD Event Source Counter Clear Enable
pub type CSELCD_R = crate::BitReader<CSELCD_A>;
impl CSELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCD_A {
        match self.bits {
            false => CSELCD_A::_0,
            true => CSELCD_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCD_A::_0
    }
    ///Enable counter clear on ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCD_A::_1
    }
}
///Field `CSELCD` writer - ELC_GPTD Event Source Counter Clear Enable
pub type CSELCD_W<'a, REG> = crate::BitWriter<'a, REG, CSELCD_A>;
impl<'a, REG> CSELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCD_A::_0)
    }
    ///Enable counter clear on ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCD_A::_1)
    }
}
/**ELC_GPTE Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCE_A {
    ///0: Disable counter clear on ELC_GPTE input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTE input
    _1 = 1,
}
impl From<CSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCE` reader - ELC_GPTE Event Source Counter Clear Enable
pub type CSELCE_R = crate::BitReader<CSELCE_A>;
impl CSELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCE_A {
        match self.bits {
            false => CSELCE_A::_0,
            true => CSELCE_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCE_A::_0
    }
    ///Enable counter clear on ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCE_A::_1
    }
}
///Field `CSELCE` writer - ELC_GPTE Event Source Counter Clear Enable
pub type CSELCE_W<'a, REG> = crate::BitWriter<'a, REG, CSELCE_A>;
impl<'a, REG> CSELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCE_A::_0)
    }
    ///Enable counter clear on ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCE_A::_1)
    }
}
/**ELC_GPTF Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCF_A {
    ///0: Disable counter clear on ELC_GPTF input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTF input
    _1 = 1,
}
impl From<CSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCF` reader - ELC_GPTF Event Source Counter Clear Enable
pub type CSELCF_R = crate::BitReader<CSELCF_A>;
impl CSELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCF_A {
        match self.bits {
            false => CSELCF_A::_0,
            true => CSELCF_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCF_A::_0
    }
    ///Enable counter clear on ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCF_A::_1
    }
}
///Field `CSELCF` writer - ELC_GPTF Event Source Counter Clear Enable
pub type CSELCF_W<'a, REG> = crate::BitWriter<'a, REG, CSELCF_A>;
impl<'a, REG> CSELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCF_A::_0)
    }
    ///Enable counter clear on ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCF_A::_1)
    }
}
/**ELC_GPTG Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCG_A {
    ///0: Disable counter clear on ELC_GPTG input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTG input
    _1 = 1,
}
impl From<CSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCG` reader - ELC_GPTG Event Source Counter Clear Enable
pub type CSELCG_R = crate::BitReader<CSELCG_A>;
impl CSELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCG_A {
        match self.bits {
            false => CSELCG_A::_0,
            true => CSELCG_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCG_A::_0
    }
    ///Enable counter clear on ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCG_A::_1
    }
}
///Field `CSELCG` writer - ELC_GPTG Event Source Counter Clear Enable
pub type CSELCG_W<'a, REG> = crate::BitWriter<'a, REG, CSELCG_A>;
impl<'a, REG> CSELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCG_A::_0)
    }
    ///Enable counter clear on ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCG_A::_1)
    }
}
/**ELC_GPTH Event Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSELCH_A {
    ///0: Disable counter clear on ELC_GPTH input
    _0 = 0,
    ///1: Enable counter clear on ELC_GPTH input
    _1 = 1,
}
impl From<CSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: CSELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSELCH` reader - ELC_GPTH Event Source Counter Clear Enable
pub type CSELCH_R = crate::BitReader<CSELCH_A>;
impl CSELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSELCH_A {
        match self.bits {
            false => CSELCH_A::_0,
            true => CSELCH_A::_1,
        }
    }
    ///Disable counter clear on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSELCH_A::_0
    }
    ///Enable counter clear on ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSELCH_A::_1
    }
}
///Field `CSELCH` writer - ELC_GPTH Event Source Counter Clear Enable
pub type CSELCH_W<'a, REG> = crate::BitWriter<'a, REG, CSELCH_A>;
impl<'a, REG> CSELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCH_A::_0)
    }
    ///Enable counter clear on ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSELCH_A::_1)
    }
}
/**Software Source Counter Clear Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLR_A {
    ///0: Disable counter clear by the GTCLR register
    _0 = 0,
    ///1: Enable counter clear by the GTCLR register
    _1 = 1,
}
impl From<CCLR_A> for bool {
    #[inline(always)]
    fn from(variant: CCLR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCLR` reader - Software Source Counter Clear Enable
pub type CCLR_R = crate::BitReader<CCLR_A>;
impl CCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCLR_A {
        match self.bits {
            false => CCLR_A::_0,
            true => CCLR_A::_1,
        }
    }
    ///Disable counter clear by the GTCLR register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCLR_A::_0
    }
    ///Enable counter clear by the GTCLR register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCLR_A::_1
    }
}
///Field `CCLR` writer - Software Source Counter Clear Enable
pub type CCLR_W<'a, REG> = crate::BitWriter<'a, REG, CCLR_A>;
impl<'a, REG> CCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter clear by the GTCLR register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR_A::_0)
    }
    ///Enable counter clear by the GTCLR register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CCLR_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgar(&self) -> CSGTRGAR_R {
        CSGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgaf(&self) -> CSGTRGAF_R {
        CSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgbr(&self) -> CSGTRGBR_R {
        CSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgbf(&self) -> CSGTRGBF_R {
        CSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgcr(&self) -> CSGTRGCR_R {
        CSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgcf(&self) -> CSGTRGCF_R {
        CSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgdr(&self) -> CSGTRGDR_R {
        CSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgdf(&self) -> CSGTRGDF_R {
        CSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscarbl(&self) -> CSCARBL_R {
        CSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscarbh(&self) -> CSCARBH_R {
        CSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscafbl(&self) -> CSCAFBL_R {
        CSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscafbh(&self) -> CSCAFBH_R {
        CSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbral(&self) -> CSCBRAL_R {
        CSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbrah(&self) -> CSCBRAH_R {
        CSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbfal(&self) -> CSCBFAL_R {
        CSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbfah(&self) -> CSCBFAH_R {
        CSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselca(&self) -> CSELCA_R {
        CSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcb(&self) -> CSELCB_R {
        CSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcc(&self) -> CSELCC_R {
        CSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcd(&self) -> CSELCD_R {
        CSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselce(&self) -> CSELCE_R {
        CSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcf(&self) -> CSELCF_R {
        CSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcg(&self) -> CSELCG_R {
        CSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselch(&self) -> CSELCH_R {
        CSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - Software Source Counter Clear Enable
    #[inline(always)]
    pub fn cclr(&self) -> CCLR_R {
        CCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgar(&mut self) -> CSGTRGAR_W<GTCSR_SPEC> {
        CSGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgaf(&mut self) -> CSGTRGAF_W<GTCSR_SPEC> {
        CSGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgbr(&mut self) -> CSGTRGBR_W<GTCSR_SPEC> {
        CSGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgbf(&mut self) -> CSGTRGBF_W<GTCSR_SPEC> {
        CSGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgcr(&mut self) -> CSGTRGCR_W<GTCSR_SPEC> {
        CSGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgcf(&mut self) -> CSGTRGCF_W<GTCSR_SPEC> {
        CSGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgdr(&mut self) -> CSGTRGDR_W<GTCSR_SPEC> {
        CSGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Clear Enable
    #[inline(always)]
    pub fn csgtrgdf(&mut self) -> CSGTRGDF_W<GTCSR_SPEC> {
        CSGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscarbl(&mut self) -> CSCARBL_W<GTCSR_SPEC> {
        CSCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscarbh(&mut self) -> CSCARBH_W<GTCSR_SPEC> {
        CSCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscafbl(&mut self) -> CSCAFBL_W<GTCSR_SPEC> {
        CSCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscafbh(&mut self) -> CSCAFBH_W<GTCSR_SPEC> {
        CSCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbral(&mut self) -> CSCBRAL_W<GTCSR_SPEC> {
        CSCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbrah(&mut self) -> CSCBRAH_W<GTCSR_SPEC> {
        CSCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbfal(&mut self) -> CSCBFAL_W<GTCSR_SPEC> {
        CSCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable
    #[inline(always)]
    pub fn cscbfah(&mut self) -> CSCBFAH_W<GTCSR_SPEC> {
        CSCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselca(&mut self) -> CSELCA_W<GTCSR_SPEC> {
        CSELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcb(&mut self) -> CSELCB_W<GTCSR_SPEC> {
        CSELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcc(&mut self) -> CSELCC_W<GTCSR_SPEC> {
        CSELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcd(&mut self) -> CSELCD_W<GTCSR_SPEC> {
        CSELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselce(&mut self) -> CSELCE_W<GTCSR_SPEC> {
        CSELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcf(&mut self) -> CSELCF_W<GTCSR_SPEC> {
        CSELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselcg(&mut self) -> CSELCG_W<GTCSR_SPEC> {
        CSELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Clear Enable
    #[inline(always)]
    pub fn cselch(&mut self) -> CSELCH_W<GTCSR_SPEC> {
        CSELCH_W::new(self, 23)
    }
    ///Bit 31 - Software Source Counter Clear Enable
    #[inline(always)]
    pub fn cclr(&mut self) -> CCLR_W<GTCSR_SPEC> {
        CCLR_W::new(self, 31)
    }
}
/**General PWM Timer Clear Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTCSR_SPEC;
impl crate::RegisterSpec for GTCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtcsr::R`](R) reader structure
impl crate::Readable for GTCSR_SPEC {}
///`write(|w| ..)` method takes [`gtcsr::W`](W) writer structure
impl crate::Writable for GTCSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTCSR to value 0
impl crate::Resettable for GTCSR_SPEC {}
