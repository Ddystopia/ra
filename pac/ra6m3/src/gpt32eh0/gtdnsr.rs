///Register `GTDNSR` reader
pub type R = crate::R<GTDNSR_SPEC>;
///Register `GTDNSR` writer
pub type W = crate::W<GTDNSR_SPEC>;
/**GTETRGA Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGAR_A {
    ///0: Disable counter count down on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<DSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGAR_R = crate::BitReader<DSGTRGAR_A>;
impl DSGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGAR_A {
        match self.bits {
            false => DSGTRGAR_A::_0,
            true => DSGTRGAR_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGAR_A::_0
    }
    ///Enable counter count down on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGAR_A::_1
    }
}
///Field `DSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGAR_A>;
impl<'a, REG> DSGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGAR_A::_0)
    }
    ///Enable counter count down on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGAF_A {
    ///0: Disable counter count down on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTETRGA input.
    _1 = 1,
}
impl From<DSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGAF_R = crate::BitReader<DSGTRGAF_A>;
impl DSGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGAF_A {
        match self.bits {
            false => DSGTRGAF_A::_0,
            true => DSGTRGAF_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGAF_A::_0
    }
    ///Enable counter count down on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGAF_A::_1
    }
}
///Field `DSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGAF_A>;
impl<'a, REG> DSGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGAF_A::_0)
    }
    ///Enable counter count down on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGBR_A {
    ///0: Disable counter count down on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTETRGB input.
    _1 = 1,
}
impl From<DSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGBR_R = crate::BitReader<DSGTRGBR_A>;
impl DSGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGBR_A {
        match self.bits {
            false => DSGTRGBR_A::_0,
            true => DSGTRGBR_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGBR_A::_0
    }
    ///Enable counter count down on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGBR_A::_1
    }
}
///Field `DSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGBR_A>;
impl<'a, REG> DSGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGBR_A::_0)
    }
    ///Enable counter count down on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGBF_A {
    ///0: Disable counter count down on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTETRGB input.
    _1 = 1,
}
impl From<DSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGBF_R = crate::BitReader<DSGTRGBF_A>;
impl DSGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGBF_A {
        match self.bits {
            false => DSGTRGBF_A::_0,
            true => DSGTRGBF_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGBF_A::_0
    }
    ///Enable counter count down on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGBF_A::_1
    }
}
///Field `DSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGBF_A>;
impl<'a, REG> DSGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGBF_A::_0)
    }
    ///Enable counter count down on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGCR_A {
    ///0: Disable counter count down on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<DSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGCR_R = crate::BitReader<DSGTRGCR_A>;
impl DSGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGCR_A {
        match self.bits {
            false => DSGTRGCR_A::_0,
            true => DSGTRGCR_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGCR_A::_0
    }
    ///Enable counter count down on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGCR_A::_1
    }
}
///Field `DSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGCR_A>;
impl<'a, REG> DSGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGCR_A::_0)
    }
    ///Enable counter count down on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGCF_A {
    ///0: Disable counter count down on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTETRGC input.
    _1 = 1,
}
impl From<DSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGCF_R = crate::BitReader<DSGTRGCF_A>;
impl DSGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGCF_A {
        match self.bits {
            false => DSGTRGCF_A::_0,
            true => DSGTRGCF_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGCF_A::_0
    }
    ///Enable counter count down on the falling edge of GTETRGC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGCF_A::_1
    }
}
///Field `DSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGCF_A>;
impl<'a, REG> DSGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGCF_A::_0)
    }
    ///Enable counter count down on the falling edge of GTETRGC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGDR_A {
    ///0: Disable counter count down on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTETRGD input.
    _1 = 1,
}
impl From<DSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGDR_R = crate::BitReader<DSGTRGDR_A>;
impl DSGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGDR_A {
        match self.bits {
            false => DSGTRGDR_A::_0,
            true => DSGTRGDR_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGDR_A::_0
    }
    ///Enable counter count down on the rising edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGDR_A::_1
    }
}
///Field `DSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Count Down Enable
pub type DSGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGDR_A>;
impl<'a, REG> DSGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGDR_A::_0)
    }
    ///Enable counter count down on the rising edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSGTRGDF_A {
    ///0: Disable counter count down on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTETRGD input.
    _1 = 1,
}
impl From<DSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: DSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGDF_R = crate::BitReader<DSGTRGDF_A>;
impl DSGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSGTRGDF_A {
        match self.bits {
            false => DSGTRGDF_A::_0,
            true => DSGTRGDF_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSGTRGDF_A::_0
    }
    ///Enable counter count down on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSGTRGDF_A::_1
    }
}
///Field `DSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Count Down Enable
pub type DSGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, DSGTRGDF_A>;
impl<'a, REG> DSGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGDF_A::_0)
    }
    ///Enable counter count down on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCARBL_A {
    ///0: Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<DSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable
pub type DSCARBL_R = crate::BitReader<DSCARBL_A>;
impl DSCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCARBL_A {
        match self.bits {
            false => DSCARBL_A::_0,
            true => DSCARBL_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCARBL_A::_0
    }
    ///Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCARBL_A::_1
    }
}
///Field `DSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable
pub type DSCARBL_W<'a, REG> = crate::BitWriter<'a, REG, DSCARBL_A>;
impl<'a, REG> DSCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCARBL_A::_0)
    }
    ///Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCARBH_A {
    ///0: Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<DSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable
pub type DSCARBH_R = crate::BitReader<DSCARBH_A>;
impl DSCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCARBH_A {
        match self.bits {
            false => DSCARBH_A::_0,
            true => DSCARBH_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCARBH_A::_0
    }
    ///Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCARBH_A::_1
    }
}
///Field `DSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable
pub type DSCARBH_W<'a, REG> = crate::BitWriter<'a, REG, DSCARBH_A>;
impl<'a, REG> DSCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCARBH_A::_0)
    }
    ///Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCAFBL_A {
    ///0: Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0
    _1 = 1,
}
impl From<DSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable
pub type DSCAFBL_R = crate::BitReader<DSCAFBL_A>;
impl DSCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCAFBL_A {
        match self.bits {
            false => DSCAFBL_A::_0,
            true => DSCAFBL_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCAFBL_A::_0
    }
    ///Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCAFBL_A::_1
    }
}
///Field `DSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable
pub type DSCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, DSCAFBL_A>;
impl<'a, REG> DSCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCAFBL_A::_0)
    }
    ///Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCAFBH_A {
    ///0: Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<DSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable
pub type DSCAFBH_R = crate::BitReader<DSCAFBH_A>;
impl DSCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCAFBH_A {
        match self.bits {
            false => DSCAFBH_A::_0,
            true => DSCAFBH_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCAFBH_A::_0
    }
    ///Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCAFBH_A::_1
    }
}
///Field `DSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable
pub type DSCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, DSCAFBH_A>;
impl<'a, REG> DSCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCAFBH_A::_0)
    }
    ///Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBRAL_A {
    ///0: Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<DSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable
pub type DSCBRAL_R = crate::BitReader<DSCBRAL_A>;
impl DSCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCBRAL_A {
        match self.bits {
            false => DSCBRAL_A::_0,
            true => DSCBRAL_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBRAL_A::_0
    }
    ///Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBRAL_A::_1
    }
}
///Field `DSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable
pub type DSCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, DSCBRAL_A>;
impl<'a, REG> DSCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBRAL_A::_0)
    }
    ///Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBRAH_A {
    ///0: Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<DSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable
pub type DSCBRAH_R = crate::BitReader<DSCBRAH_A>;
impl DSCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCBRAH_A {
        match self.bits {
            false => DSCBRAH_A::_0,
            true => DSCBRAH_A::_1,
        }
    }
    ///Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBRAH_A::_0
    }
    ///Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBRAH_A::_1
    }
}
///Field `DSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable
pub type DSCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, DSCBRAH_A>;
impl<'a, REG> DSCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBRAH_A::_0)
    }
    ///Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBFAL_A {
    ///0: Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<DSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable
pub type DSCBFAL_R = crate::BitReader<DSCBFAL_A>;
impl DSCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCBFAL_A {
        match self.bits {
            false => DSCBFAL_A::_0,
            true => DSCBFAL_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBFAL_A::_0
    }
    ///Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBFAL_A::_1
    }
}
///Field `DSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable
pub type DSCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, DSCBFAL_A>;
impl<'a, REG> DSCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBFAL_A::_0)
    }
    ///Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCBFAH_A {
    ///0: Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<DSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: DSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable
pub type DSCBFAH_R = crate::BitReader<DSCBFAH_A>;
impl DSCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSCBFAH_A {
        match self.bits {
            false => DSCBFAH_A::_0,
            true => DSCBFAH_A::_1,
        }
    }
    ///Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCBFAH_A::_0
    }
    ///Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCBFAH_A::_1
    }
}
///Field `DSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable
pub type DSCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, DSCBFAH_A>;
impl<'a, REG> DSCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBFAH_A::_0)
    }
    ///Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCA_A {
    ///0: Disable counter count down on ELC_GPTA input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTA input.
    _1 = 1,
}
impl From<DSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCA` reader - ELC_GPTA Event Source Counter Count Down Enable
pub type DSELCA_R = crate::BitReader<DSELCA_A>;
impl DSELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCA_A {
        match self.bits {
            false => DSELCA_A::_0,
            true => DSELCA_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCA_A::_0
    }
    ///Enable counter count down on ELC_GPTA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCA_A::_1
    }
}
///Field `DSELCA` writer - ELC_GPTA Event Source Counter Count Down Enable
pub type DSELCA_W<'a, REG> = crate::BitWriter<'a, REG, DSELCA_A>;
impl<'a, REG> DSELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCA_A::_0)
    }
    ///Enable counter count down on ELC_GPTA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCA_A::_1)
    }
}
/**ELC_GPTB Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCB_A {
    ///0: Disable counter count down on ELC_GPTB input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTB input.
    _1 = 1,
}
impl From<DSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCB` reader - ELC_GPTB Event Source Counter Count Down Enable
pub type DSELCB_R = crate::BitReader<DSELCB_A>;
impl DSELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCB_A {
        match self.bits {
            false => DSELCB_A::_0,
            true => DSELCB_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCB_A::_0
    }
    ///Enable counter count down on ELC_GPTB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCB_A::_1
    }
}
///Field `DSELCB` writer - ELC_GPTB Event Source Counter Count Down Enable
pub type DSELCB_W<'a, REG> = crate::BitWriter<'a, REG, DSELCB_A>;
impl<'a, REG> DSELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCB_A::_0)
    }
    ///Enable counter count down on ELC_GPTB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCB_A::_1)
    }
}
/**ELC_GPTC Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCC_A {
    ///0: Disable counter count down on ELC_GPTC input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTC input.
    _1 = 1,
}
impl From<DSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCC` reader - ELC_GPTC Event Source Counter Count Down Enable
pub type DSELCC_R = crate::BitReader<DSELCC_A>;
impl DSELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCC_A {
        match self.bits {
            false => DSELCC_A::_0,
            true => DSELCC_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCC_A::_0
    }
    ///Enable counter count down on ELC_GPTC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCC_A::_1
    }
}
///Field `DSELCC` writer - ELC_GPTC Event Source Counter Count Down Enable
pub type DSELCC_W<'a, REG> = crate::BitWriter<'a, REG, DSELCC_A>;
impl<'a, REG> DSELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCC_A::_0)
    }
    ///Enable counter count down on ELC_GPTC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCC_A::_1)
    }
}
/**ELC_GPTD Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCD_A {
    ///0: Disable counter count down on ELC_GPTD input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTD input.
    _1 = 1,
}
impl From<DSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCD` reader - ELC_GPTD Event Source Counter Count Down Enable
pub type DSELCD_R = crate::BitReader<DSELCD_A>;
impl DSELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCD_A {
        match self.bits {
            false => DSELCD_A::_0,
            true => DSELCD_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCD_A::_0
    }
    ///Enable counter count down on ELC_GPTD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCD_A::_1
    }
}
///Field `DSELCD` writer - ELC_GPTD Event Source Counter Count Down Enable
pub type DSELCD_W<'a, REG> = crate::BitWriter<'a, REG, DSELCD_A>;
impl<'a, REG> DSELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCD_A::_0)
    }
    ///Enable counter count down on ELC_GPTD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCD_A::_1)
    }
}
/**ELC_GPTE Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCE_A {
    ///0: Disable counter count down on ELC_GPTE input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTE input.
    _1 = 1,
}
impl From<DSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCE` reader - ELC_GPTE Event Source Counter Count Down Enable
pub type DSELCE_R = crate::BitReader<DSELCE_A>;
impl DSELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCE_A {
        match self.bits {
            false => DSELCE_A::_0,
            true => DSELCE_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCE_A::_0
    }
    ///Enable counter count down on ELC_GPTE input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCE_A::_1
    }
}
///Field `DSELCE` writer - ELC_GPTE Event Source Counter Count Down Enable
pub type DSELCE_W<'a, REG> = crate::BitWriter<'a, REG, DSELCE_A>;
impl<'a, REG> DSELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCE_A::_0)
    }
    ///Enable counter count down on ELC_GPTE input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCE_A::_1)
    }
}
/**ELC_GPTF Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCF_A {
    ///0: Disable counter count down on ELC_GPTF input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTF input.
    _1 = 1,
}
impl From<DSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCF` reader - ELC_GPTF Event Source Counter Count Down Enable
pub type DSELCF_R = crate::BitReader<DSELCF_A>;
impl DSELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCF_A {
        match self.bits {
            false => DSELCF_A::_0,
            true => DSELCF_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCF_A::_0
    }
    ///Enable counter count down on ELC_GPTF input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCF_A::_1
    }
}
///Field `DSELCF` writer - ELC_GPTF Event Source Counter Count Down Enable
pub type DSELCF_W<'a, REG> = crate::BitWriter<'a, REG, DSELCF_A>;
impl<'a, REG> DSELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCF_A::_0)
    }
    ///Enable counter count down on ELC_GPTF input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCF_A::_1)
    }
}
/**ELC_GPTG Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCG_A {
    ///0: Disable counter count down on ELC_GPTG input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTG input.
    _1 = 1,
}
impl From<DSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCG` reader - ELC_GPTG Event Source Counter Count Down Enable
pub type DSELCG_R = crate::BitReader<DSELCG_A>;
impl DSELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCG_A {
        match self.bits {
            false => DSELCG_A::_0,
            true => DSELCG_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCG_A::_0
    }
    ///Enable counter count down on ELC_GPTG input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCG_A::_1
    }
}
///Field `DSELCG` writer - ELC_GPTG Event Source Counter Count Down Enable
pub type DSELCG_W<'a, REG> = crate::BitWriter<'a, REG, DSELCG_A>;
impl<'a, REG> DSELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCG_A::_0)
    }
    ///Enable counter count down on ELC_GPTG input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCG_A::_1)
    }
}
/**ELC_GPTH Event Source Counter Count Down Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSELCH_A {
    ///0: Disable counter count down on ELC_GPTH input
    _0 = 0,
    ///1: Enable counter count down on ELC_GPTH input.
    _1 = 1,
}
impl From<DSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: DSELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DSELCH` reader - ELC_GPTH Event Source Counter Count Down Enable
pub type DSELCH_R = crate::BitReader<DSELCH_A>;
impl DSELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSELCH_A {
        match self.bits {
            false => DSELCH_A::_0,
            true => DSELCH_A::_1,
        }
    }
    ///Disable counter count down on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSELCH_A::_0
    }
    ///Enable counter count down on ELC_GPTH input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSELCH_A::_1
    }
}
///Field `DSELCH` writer - ELC_GPTH Event Source Counter Count Down Enable
pub type DSELCH_W<'a, REG> = crate::BitWriter<'a, REG, DSELCH_A>;
impl<'a, REG> DSELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter count down on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCH_A::_0)
    }
    ///Enable counter count down on ELC_GPTH input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSELCH_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgar(&self) -> DSGTRGAR_R {
        DSGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgaf(&self) -> DSGTRGAF_R {
        DSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbr(&self) -> DSGTRGBR_R {
        DSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbf(&self) -> DSGTRGBF_R {
        DSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcr(&self) -> DSGTRGCR_R {
        DSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcf(&self) -> DSGTRGCF_R {
        DSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdr(&self) -> DSGTRGDR_R {
        DSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdf(&self) -> DSGTRGDF_R {
        DSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbl(&self) -> DSCARBL_R {
        DSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbh(&self) -> DSCARBH_R {
        DSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbl(&self) -> DSCAFBL_R {
        DSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbh(&self) -> DSCAFBH_R {
        DSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbral(&self) -> DSCBRAL_R {
        DSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbrah(&self) -> DSCBRAH_R {
        DSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfal(&self) -> DSCBFAL_R {
        DSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfah(&self) -> DSCBFAH_R {
        DSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselca(&self) -> DSELCA_R {
        DSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcb(&self) -> DSELCB_R {
        DSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcc(&self) -> DSELCC_R {
        DSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcd(&self) -> DSELCD_R {
        DSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselce(&self) -> DSELCE_R {
        DSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcf(&self) -> DSELCF_R {
        DSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcg(&self) -> DSELCG_R {
        DSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselch(&self) -> DSELCH_R {
        DSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgar(&mut self) -> DSGTRGAR_W<GTDNSR_SPEC> {
        DSGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgaf(&mut self) -> DSGTRGAF_W<GTDNSR_SPEC> {
        DSGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbr(&mut self) -> DSGTRGBR_W<GTDNSR_SPEC> {
        DSGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgbf(&mut self) -> DSGTRGBF_W<GTDNSR_SPEC> {
        DSGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcr(&mut self) -> DSGTRGCR_W<GTDNSR_SPEC> {
        DSGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgcf(&mut self) -> DSGTRGCF_W<GTDNSR_SPEC> {
        DSGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdr(&mut self) -> DSGTRGDR_W<GTDNSR_SPEC> {
        DSGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Count Down Enable
    #[inline(always)]
    pub fn dsgtrgdf(&mut self) -> DSGTRGDF_W<GTDNSR_SPEC> {
        DSGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbl(&mut self) -> DSCARBL_W<GTDNSR_SPEC> {
        DSCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscarbh(&mut self) -> DSCARBH_W<GTDNSR_SPEC> {
        DSCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbl(&mut self) -> DSCAFBL_W<GTDNSR_SPEC> {
        DSCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscafbh(&mut self) -> DSCAFBH_W<GTDNSR_SPEC> {
        DSCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbral(&mut self) -> DSCBRAL_W<GTDNSR_SPEC> {
        DSCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbrah(&mut self) -> DSCBRAH_W<GTDNSR_SPEC> {
        DSCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfal(&mut self) -> DSCBFAL_W<GTDNSR_SPEC> {
        DSCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable
    #[inline(always)]
    pub fn dscbfah(&mut self) -> DSCBFAH_W<GTDNSR_SPEC> {
        DSCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselca(&mut self) -> DSELCA_W<GTDNSR_SPEC> {
        DSELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcb(&mut self) -> DSELCB_W<GTDNSR_SPEC> {
        DSELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcc(&mut self) -> DSELCC_W<GTDNSR_SPEC> {
        DSELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcd(&mut self) -> DSELCD_W<GTDNSR_SPEC> {
        DSELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselce(&mut self) -> DSELCE_W<GTDNSR_SPEC> {
        DSELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcf(&mut self) -> DSELCF_W<GTDNSR_SPEC> {
        DSELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselcg(&mut self) -> DSELCG_W<GTDNSR_SPEC> {
        DSELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Count Down Enable
    #[inline(always)]
    pub fn dselch(&mut self) -> DSELCH_W<GTDNSR_SPEC> {
        DSELCH_W::new(self, 23)
    }
}
/**General PWM Timer Down Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtdnsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdnsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDNSR_SPEC;
impl crate::RegisterSpec for GTDNSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtdnsr::R`](R) reader structure
impl crate::Readable for GTDNSR_SPEC {}
///`write(|w| ..)` method takes [`gtdnsr::W`](W) writer structure
impl crate::Writable for GTDNSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDNSR to value 0
impl crate::Resettable for GTDNSR_SPEC {}
