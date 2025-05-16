///Register `GTPSR` reader
pub type R = crate::R<GTPSR_SPEC>;
///Register `GTPSR` writer
pub type W = crate::W<GTPSR_SPEC>;
/**GTETRGA Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGAR_A {
    ///0: Disable counter stop on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTETRGA input
    _1 = 1,
}
impl From<PSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGAR` reader - GTETRGA Pin Rising Input Source Counter Stop Enable
pub type PSGTRGAR_R = crate::BitReader<PSGTRGAR_A>;
impl PSGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGAR_A {
        match self.bits {
            false => PSGTRGAR_A::_0,
            true => PSGTRGAR_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGAR_A::_0
    }
    ///Enable counter stop on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGAR_A::_1
    }
}
///Field `PSGTRGAR` writer - GTETRGA Pin Rising Input Source Counter Stop Enable
pub type PSGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGAR_A>;
impl<'a, REG> PSGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAR_A::_0)
    }
    ///Enable counter stop on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGAF_A {
    ///0: Disable counter stop on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTETRGA input
    _1 = 1,
}
impl From<PSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGAF` reader - GTETRGA Pin Falling Input Source Counter Stop Enable
pub type PSGTRGAF_R = crate::BitReader<PSGTRGAF_A>;
impl PSGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGAF_A {
        match self.bits {
            false => PSGTRGAF_A::_0,
            true => PSGTRGAF_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGAF_A::_0
    }
    ///Enable counter stop on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGAF_A::_1
    }
}
///Field `PSGTRGAF` writer - GTETRGA Pin Falling Input Source Counter Stop Enable
pub type PSGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGAF_A>;
impl<'a, REG> PSGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAF_A::_0)
    }
    ///Enable counter stop on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGBR_A {
    ///0: Disable counter stop on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTETRGB input
    _1 = 1,
}
impl From<PSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGBR` reader - GTETRGB Pin Rising Input Source Counter Stop Enable
pub type PSGTRGBR_R = crate::BitReader<PSGTRGBR_A>;
impl PSGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGBR_A {
        match self.bits {
            false => PSGTRGBR_A::_0,
            true => PSGTRGBR_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGBR_A::_0
    }
    ///Enable counter stop on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGBR_A::_1
    }
}
///Field `PSGTRGBR` writer - GTETRGB Pin Rising Input Source Counter Stop Enable
pub type PSGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGBR_A>;
impl<'a, REG> PSGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBR_A::_0)
    }
    ///Enable counter stop on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGBF_A {
    ///0: Disable counter stop on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTETRGB input
    _1 = 1,
}
impl From<PSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGBF` reader - GTETRGB Pin Falling Input Source Counter Stop Enable
pub type PSGTRGBF_R = crate::BitReader<PSGTRGBF_A>;
impl PSGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGBF_A {
        match self.bits {
            false => PSGTRGBF_A::_0,
            true => PSGTRGBF_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGBF_A::_0
    }
    ///Enable counter stop on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGBF_A::_1
    }
}
///Field `PSGTRGBF` writer - GTETRGB Pin Falling Input Source Counter Stop Enable
pub type PSGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGBF_A>;
impl<'a, REG> PSGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBF_A::_0)
    }
    ///Enable counter stop on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGCR_A {
    ///0: Disable counter stop on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTETRGC input
    _1 = 1,
}
impl From<PSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGCR` reader - GTETRGC Pin Rising Input Source Counter Stop Enable
pub type PSGTRGCR_R = crate::BitReader<PSGTRGCR_A>;
impl PSGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGCR_A {
        match self.bits {
            false => PSGTRGCR_A::_0,
            true => PSGTRGCR_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGCR_A::_0
    }
    ///Enable counter stop on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGCR_A::_1
    }
}
///Field `PSGTRGCR` writer - GTETRGC Pin Rising Input Source Counter Stop Enable
pub type PSGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGCR_A>;
impl<'a, REG> PSGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGCR_A::_0)
    }
    ///Enable counter stop on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGCF_A {
    ///0: Disable counter stop on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTETRGC input
    _1 = 1,
}
impl From<PSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGCF` reader - GTETRGC Pin Falling Input Source Counter Stop Enable
pub type PSGTRGCF_R = crate::BitReader<PSGTRGCF_A>;
impl PSGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGCF_A {
        match self.bits {
            false => PSGTRGCF_A::_0,
            true => PSGTRGCF_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGCF_A::_0
    }
    ///Enable counter stop on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGCF_A::_1
    }
}
///Field `PSGTRGCF` writer - GTETRGC Pin Falling Input Source Counter Stop Enable
pub type PSGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGCF_A>;
impl<'a, REG> PSGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGCF_A::_0)
    }
    ///Enable counter stop on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGDR_A {
    ///0: Disable counter stop on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTETRGD input
    _1 = 1,
}
impl From<PSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGDR` reader - GTETRGD Pin Rising Input Source Counter Stop Enable
pub type PSGTRGDR_R = crate::BitReader<PSGTRGDR_A>;
impl PSGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGDR_A {
        match self.bits {
            false => PSGTRGDR_A::_0,
            true => PSGTRGDR_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGDR_A::_0
    }
    ///Enable counter stop on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGDR_A::_1
    }
}
///Field `PSGTRGDR` writer - GTETRGD Pin Rising Input Source Counter Stop Enable
pub type PSGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGDR_A>;
impl<'a, REG> PSGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGDR_A::_0)
    }
    ///Enable counter stop on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSGTRGDF_A {
    ///0: Disable counter stop on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTETRGD input
    _1 = 1,
}
impl From<PSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: PSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSGTRGDF` reader - GTETRGD Pin Falling Input Source Counter Stop Enable
pub type PSGTRGDF_R = crate::BitReader<PSGTRGDF_A>;
impl PSGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSGTRGDF_A {
        match self.bits {
            false => PSGTRGDF_A::_0,
            true => PSGTRGDF_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSGTRGDF_A::_0
    }
    ///Enable counter stop on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSGTRGDF_A::_1
    }
}
///Field `PSGTRGDF` writer - GTETRGD Pin Falling Input Source Counter Stop Enable
pub type PSGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, PSGTRGDF_A>;
impl<'a, REG> PSGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGDF_A::_0)
    }
    ///Enable counter stop on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCARBL_A {
    ///0: Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0
    _1 = 1,
}
impl From<PSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable
pub type PSCARBL_R = crate::BitReader<PSCARBL_A>;
impl PSCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCARBL_A {
        match self.bits {
            false => PSCARBL_A::_0,
            true => PSCARBL_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCARBL_A::_0
    }
    ///Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCARBL_A::_1
    }
}
///Field `PSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable
pub type PSCARBL_W<'a, REG> = crate::BitWriter<'a, REG, PSCARBL_A>;
impl<'a, REG> PSCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBL_A::_0)
    }
    ///Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCARBH_A {
    ///0: Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1
    _1 = 1,
}
impl From<PSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable
pub type PSCARBH_R = crate::BitReader<PSCARBH_A>;
impl PSCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCARBH_A {
        match self.bits {
            false => PSCARBH_A::_0,
            true => PSCARBH_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCARBH_A::_0
    }
    ///Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCARBH_A::_1
    }
}
///Field `PSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable
pub type PSCARBH_W<'a, REG> = crate::BitWriter<'a, REG, PSCARBH_A>;
impl<'a, REG> PSCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBH_A::_0)
    }
    ///Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCAFBL_A {
    ///0: Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0
    _1 = 1,
}
impl From<PSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable
pub type PSCAFBL_R = crate::BitReader<PSCAFBL_A>;
impl PSCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCAFBL_A {
        match self.bits {
            false => PSCAFBL_A::_0,
            true => PSCAFBL_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCAFBL_A::_0
    }
    ///Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCAFBL_A::_1
    }
}
///Field `PSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable
pub type PSCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, PSCAFBL_A>;
impl<'a, REG> PSCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBL_A::_0)
    }
    ///Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCAFBH_A {
    ///0: Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1
    _1 = 1,
}
impl From<PSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable
pub type PSCAFBH_R = crate::BitReader<PSCAFBH_A>;
impl PSCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCAFBH_A {
        match self.bits {
            false => PSCAFBH_A::_0,
            true => PSCAFBH_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCAFBH_A::_0
    }
    ///Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCAFBH_A::_1
    }
}
///Field `PSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable
pub type PSCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, PSCAFBH_A>;
impl<'a, REG> PSCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBH_A::_0)
    }
    ///Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBRAL_A {
    ///0: Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0
    _1 = 1,
}
impl From<PSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable
pub type PSCBRAL_R = crate::BitReader<PSCBRAL_A>;
impl PSCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCBRAL_A {
        match self.bits {
            false => PSCBRAL_A::_0,
            true => PSCBRAL_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBRAL_A::_0
    }
    ///Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBRAL_A::_1
    }
}
///Field `PSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable
pub type PSCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, PSCBRAL_A>;
impl<'a, REG> PSCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAL_A::_0)
    }
    ///Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBRAH_A {
    ///0: Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1
    _1 = 1,
}
impl From<PSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable
pub type PSCBRAH_R = crate::BitReader<PSCBRAH_A>;
impl PSCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCBRAH_A {
        match self.bits {
            false => PSCBRAH_A::_0,
            true => PSCBRAH_A::_1,
        }
    }
    ///Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBRAH_A::_0
    }
    ///Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBRAH_A::_1
    }
}
///Field `PSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable
pub type PSCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, PSCBRAH_A>;
impl<'a, REG> PSCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAH_A::_0)
    }
    ///Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBFAL_A {
    ///0: Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0
    _1 = 1,
}
impl From<PSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable
pub type PSCBFAL_R = crate::BitReader<PSCBFAL_A>;
impl PSCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCBFAL_A {
        match self.bits {
            false => PSCBFAL_A::_0,
            true => PSCBFAL_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBFAL_A::_0
    }
    ///Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBFAL_A::_1
    }
}
///Field `PSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable
pub type PSCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, PSCBFAL_A>;
impl<'a, REG> PSCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAL_A::_0)
    }
    ///Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSCBFAH_A {
    ///0: Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1
    _1 = 1,
}
impl From<PSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: PSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable
pub type PSCBFAH_R = crate::BitReader<PSCBFAH_A>;
impl PSCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSCBFAH_A {
        match self.bits {
            false => PSCBFAH_A::_0,
            true => PSCBFAH_A::_1,
        }
    }
    ///Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSCBFAH_A::_0
    }
    ///Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSCBFAH_A::_1
    }
}
///Field `PSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable
pub type PSCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, PSCBFAH_A>;
impl<'a, REG> PSCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAH_A::_0)
    }
    ///Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCA_A {
    ///0: Disable counter stop on ELC_GPTA input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTA input
    _1 = 1,
}
impl From<PSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCA` reader - ELC_GPTA Event Source Counter Stop Enable
pub type PSELCA_R = crate::BitReader<PSELCA_A>;
impl PSELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCA_A {
        match self.bits {
            false => PSELCA_A::_0,
            true => PSELCA_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCA_A::_0
    }
    ///Enable counter stop on ELC_GPTA input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCA_A::_1
    }
}
///Field `PSELCA` writer - ELC_GPTA Event Source Counter Stop Enable
pub type PSELCA_W<'a, REG> = crate::BitWriter<'a, REG, PSELCA_A>;
impl<'a, REG> PSELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCA_A::_0)
    }
    ///Enable counter stop on ELC_GPTA input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCA_A::_1)
    }
}
/**ELC_GPTB Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCB_A {
    ///0: Disable counter stop on ELC_GPTB input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTB input
    _1 = 1,
}
impl From<PSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCB` reader - ELC_GPTB Event Source Counter Stop Enable
pub type PSELCB_R = crate::BitReader<PSELCB_A>;
impl PSELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCB_A {
        match self.bits {
            false => PSELCB_A::_0,
            true => PSELCB_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCB_A::_0
    }
    ///Enable counter stop on ELC_GPTB input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCB_A::_1
    }
}
///Field `PSELCB` writer - ELC_GPTB Event Source Counter Stop Enable
pub type PSELCB_W<'a, REG> = crate::BitWriter<'a, REG, PSELCB_A>;
impl<'a, REG> PSELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCB_A::_0)
    }
    ///Enable counter stop on ELC_GPTB input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCB_A::_1)
    }
}
/**ELC_GPTC Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCC_A {
    ///0: Disable counter stop on ELC_GPTC input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTC input
    _1 = 1,
}
impl From<PSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCC` reader - ELC_GPTC Event Source Counter Stop Enable
pub type PSELCC_R = crate::BitReader<PSELCC_A>;
impl PSELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCC_A {
        match self.bits {
            false => PSELCC_A::_0,
            true => PSELCC_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCC_A::_0
    }
    ///Enable counter stop on ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCC_A::_1
    }
}
///Field `PSELCC` writer - ELC_GPTC Event Source Counter Stop Enable
pub type PSELCC_W<'a, REG> = crate::BitWriter<'a, REG, PSELCC_A>;
impl<'a, REG> PSELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCC_A::_0)
    }
    ///Enable counter stop on ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCC_A::_1)
    }
}
/**ELC_GPTD Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCD_A {
    ///0: Disable counter stop on ELC_GPTD input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTD input
    _1 = 1,
}
impl From<PSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCD` reader - ELC_GPTD Event Source Counter Stop Enable
pub type PSELCD_R = crate::BitReader<PSELCD_A>;
impl PSELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCD_A {
        match self.bits {
            false => PSELCD_A::_0,
            true => PSELCD_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCD_A::_0
    }
    ///Enable counter stop on ELC_GPTD input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCD_A::_1
    }
}
///Field `PSELCD` writer - ELC_GPTD Event Source Counter Stop Enable
pub type PSELCD_W<'a, REG> = crate::BitWriter<'a, REG, PSELCD_A>;
impl<'a, REG> PSELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCD_A::_0)
    }
    ///Enable counter stop on ELC_GPTD input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCD_A::_1)
    }
}
/**ELC_GPTE Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCE_A {
    ///0: Disable counter stop on ELC_GPTE input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTE input
    _1 = 1,
}
impl From<PSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCE` reader - ELC_GPTE Event Source Counter Stop Enable
pub type PSELCE_R = crate::BitReader<PSELCE_A>;
impl PSELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCE_A {
        match self.bits {
            false => PSELCE_A::_0,
            true => PSELCE_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCE_A::_0
    }
    ///Enable counter stop on ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCE_A::_1
    }
}
///Field `PSELCE` writer - ELC_GPTE Event Source Counter Stop Enable
pub type PSELCE_W<'a, REG> = crate::BitWriter<'a, REG, PSELCE_A>;
impl<'a, REG> PSELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCE_A::_0)
    }
    ///Enable counter stop on ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCE_A::_1)
    }
}
/**ELC_GPTF Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCF_A {
    ///0: Disable counter stop on ELC_GPTF input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTF input
    _1 = 1,
}
impl From<PSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCF` reader - ELC_GPTF Event Source Counter Stop Enable
pub type PSELCF_R = crate::BitReader<PSELCF_A>;
impl PSELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCF_A {
        match self.bits {
            false => PSELCF_A::_0,
            true => PSELCF_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCF_A::_0
    }
    ///Enable counter stop on ELC_GPTF input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCF_A::_1
    }
}
///Field `PSELCF` writer - ELC_GPTF Event Source Counter Stop Enable
pub type PSELCF_W<'a, REG> = crate::BitWriter<'a, REG, PSELCF_A>;
impl<'a, REG> PSELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCF_A::_0)
    }
    ///Enable counter stop on ELC_GPTF input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCF_A::_1)
    }
}
/**ELC_GPTG Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCG_A {
    ///0: Disable counter stop on ELC_GPTG input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTG input
    _1 = 1,
}
impl From<PSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCG` reader - ELC_GPTG Event Source Counter Stop Enable
pub type PSELCG_R = crate::BitReader<PSELCG_A>;
impl PSELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCG_A {
        match self.bits {
            false => PSELCG_A::_0,
            true => PSELCG_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCG_A::_0
    }
    ///Enable counter stop on ELC_GPTG input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCG_A::_1
    }
}
///Field `PSELCG` writer - ELC_GPTG Event Source Counter Stop Enable
pub type PSELCG_W<'a, REG> = crate::BitWriter<'a, REG, PSELCG_A>;
impl<'a, REG> PSELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCG_A::_0)
    }
    ///Enable counter stop on ELC_GPTG input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCG_A::_1)
    }
}
/**ELC_GPTH Event Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSELCH_A {
    ///0: Disable counter stop on ELC_GPTH input
    _0 = 0,
    ///1: Enable counter stop on ELC_GPTH input
    _1 = 1,
}
impl From<PSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: PSELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PSELCH` reader - ELC_GPTH Event Source Counter Stop Enable
pub type PSELCH_R = crate::BitReader<PSELCH_A>;
impl PSELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSELCH_A {
        match self.bits {
            false => PSELCH_A::_0,
            true => PSELCH_A::_1,
        }
    }
    ///Disable counter stop on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSELCH_A::_0
    }
    ///Enable counter stop on ELC_GPTH input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSELCH_A::_1
    }
}
///Field `PSELCH` writer - ELC_GPTH Event Source Counter Stop Enable
pub type PSELCH_W<'a, REG> = crate::BitWriter<'a, REG, PSELCH_A>;
impl<'a, REG> PSELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCH_A::_0)
    }
    ///Enable counter stop on ELC_GPTH input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PSELCH_A::_1)
    }
}
/**Software Source Counter Stop Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSTOP_A {
    ///0: Disable counter stop by the GTSTP register
    _0 = 0,
    ///1: Enable counter stop by the GTSTP register
    _1 = 1,
}
impl From<CSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: CSTOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSTOP` reader - Software Source Counter Stop Enable
pub type CSTOP_R = crate::BitReader<CSTOP_A>;
impl CSTOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSTOP_A {
        match self.bits {
            false => CSTOP_A::_0,
            true => CSTOP_A::_1,
        }
    }
    ///Disable counter stop by the GTSTP register
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CSTOP_A::_0
    }
    ///Enable counter stop by the GTSTP register
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOP_A::_1
    }
}
///Field `CSTOP` writer - Software Source Counter Stop Enable
pub type CSTOP_W<'a, REG> = crate::BitWriter<'a, REG, CSTOP_A>;
impl<'a, REG> CSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable counter stop by the GTSTP register
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP_A::_0)
    }
    ///Enable counter stop by the GTSTP register
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CSTOP_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgar(&self) -> PSGTRGAR_R {
        PSGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgaf(&self) -> PSGTRGAF_R {
        PSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbr(&self) -> PSGTRGBR_R {
        PSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbf(&self) -> PSGTRGBF_R {
        PSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcr(&self) -> PSGTRGCR_R {
        PSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcf(&self) -> PSGTRGCF_R {
        PSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdr(&self) -> PSGTRGDR_R {
        PSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdf(&self) -> PSGTRGDF_R {
        PSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbl(&self) -> PSCARBL_R {
        PSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbh(&self) -> PSCARBH_R {
        PSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbl(&self) -> PSCAFBL_R {
        PSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbh(&self) -> PSCAFBH_R {
        PSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbral(&self) -> PSCBRAL_R {
        PSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbrah(&self) -> PSCBRAH_R {
        PSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfal(&self) -> PSCBFAL_R {
        PSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfah(&self) -> PSCBFAH_R {
        PSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselca(&self) -> PSELCA_R {
        PSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcb(&self) -> PSELCB_R {
        PSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcc(&self) -> PSELCC_R {
        PSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcd(&self) -> PSELCD_R {
        PSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselce(&self) -> PSELCE_R {
        PSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcf(&self) -> PSELCF_R {
        PSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcg(&self) -> PSELCG_R {
        PSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselch(&self) -> PSELCH_R {
        PSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - Software Source Counter Stop Enable
    #[inline(always)]
    pub fn cstop(&self) -> CSTOP_R {
        CSTOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgar(&mut self) -> PSGTRGAR_W<GTPSR_SPEC> {
        PSGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgaf(&mut self) -> PSGTRGAF_W<GTPSR_SPEC> {
        PSGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbr(&mut self) -> PSGTRGBR_W<GTPSR_SPEC> {
        PSGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgbf(&mut self) -> PSGTRGBF_W<GTPSR_SPEC> {
        PSGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcr(&mut self) -> PSGTRGCR_W<GTPSR_SPEC> {
        PSGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgcf(&mut self) -> PSGTRGCF_W<GTPSR_SPEC> {
        PSGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdr(&mut self) -> PSGTRGDR_W<GTPSR_SPEC> {
        PSGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source Counter Stop Enable
    #[inline(always)]
    pub fn psgtrgdf(&mut self) -> PSGTRGDF_W<GTPSR_SPEC> {
        PSGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbl(&mut self) -> PSCARBL_W<GTPSR_SPEC> {
        PSCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscarbh(&mut self) -> PSCARBH_W<GTPSR_SPEC> {
        PSCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbl(&mut self) -> PSCAFBL_W<GTPSR_SPEC> {
        PSCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscafbh(&mut self) -> PSCAFBH_W<GTPSR_SPEC> {
        PSCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbral(&mut self) -> PSCBRAL_W<GTPSR_SPEC> {
        PSCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbrah(&mut self) -> PSCBRAH_W<GTPSR_SPEC> {
        PSCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfal(&mut self) -> PSCBFAL_W<GTPSR_SPEC> {
        PSCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable
    #[inline(always)]
    pub fn pscbfah(&mut self) -> PSCBFAH_W<GTPSR_SPEC> {
        PSCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselca(&mut self) -> PSELCA_W<GTPSR_SPEC> {
        PSELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcb(&mut self) -> PSELCB_W<GTPSR_SPEC> {
        PSELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcc(&mut self) -> PSELCC_W<GTPSR_SPEC> {
        PSELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcd(&mut self) -> PSELCD_W<GTPSR_SPEC> {
        PSELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselce(&mut self) -> PSELCE_W<GTPSR_SPEC> {
        PSELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcf(&mut self) -> PSELCF_W<GTPSR_SPEC> {
        PSELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselcg(&mut self) -> PSELCG_W<GTPSR_SPEC> {
        PSELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source Counter Stop Enable
    #[inline(always)]
    pub fn pselch(&mut self) -> PSELCH_W<GTPSR_SPEC> {
        PSELCH_W::new(self, 23)
    }
    ///Bit 31 - Software Source Counter Stop Enable
    #[inline(always)]
    pub fn cstop(&mut self) -> CSTOP_W<GTPSR_SPEC> {
        CSTOP_W::new(self, 31)
    }
}
/**General PWM Timer Stop Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTPSR_SPEC;
impl crate::RegisterSpec for GTPSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gtpsr::R`](R) reader structure
impl crate::Readable for GTPSR_SPEC {}
///`write(|w| ..)` method takes [`gtpsr::W`](W) writer structure
impl crate::Writable for GTPSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTPSR to value 0
impl crate::Resettable for GTPSR_SPEC {}
