///Register `GTICBSR` reader
pub type R = crate::R<GTICBSR_SPEC>;
///Register `GTICBSR` writer
pub type W = crate::W<GTICBSR_SPEC>;
/**GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGAR_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTETRGA input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTETRGA input.
    _1 = 1,
}
impl From<BSGTRGAR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGAR` reader - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGAR_R = crate::BitReader<BSGTRGAR_A>;
impl BSGTRGAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGAR_A {
        match self.bits {
            false => BSGTRGAR_A::_0,
            true => BSGTRGAR_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGAR_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGAR_A::_1
    }
}
///Field `BSGTRGAR` writer - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGAR_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGAR_A>;
impl<'a, REG> BSGTRGAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGAR_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGAR_A::_1)
    }
}
/**GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGAF_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTETRGA input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTETRGA input.
    _1 = 1,
}
impl From<BSGTRGAF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGAF` reader - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGAF_R = crate::BitReader<BSGTRGAF_A>;
impl BSGTRGAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGAF_A {
        match self.bits {
            false => BSGTRGAF_A::_0,
            true => BSGTRGAF_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGAF_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGAF_A::_1
    }
}
///Field `BSGTRGAF` writer - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGAF_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGAF_A>;
impl<'a, REG> BSGTRGAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTETRGA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGAF_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGAF_A::_1)
    }
}
/**GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGBR_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTETRGB input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTETRGB input.
    _1 = 1,
}
impl From<BSGTRGBR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGBR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGBR` reader - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGBR_R = crate::BitReader<BSGTRGBR_A>;
impl BSGTRGBR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGBR_A {
        match self.bits {
            false => BSGTRGBR_A::_0,
            true => BSGTRGBR_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGBR_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGBR_A::_1
    }
}
///Field `BSGTRGBR` writer - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGBR_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGBR_A>;
impl<'a, REG> BSGTRGBR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGBR_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGBR_A::_1)
    }
}
/**GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGBF_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTETRGB input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTETRGB input.
    _1 = 1,
}
impl From<BSGTRGBF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGBF` reader - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGBF_R = crate::BitReader<BSGTRGBF_A>;
impl BSGTRGBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGBF_A {
        match self.bits {
            false => BSGTRGBF_A::_0,
            true => BSGTRGBF_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGBF_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGBF_A::_1
    }
}
///Field `BSGTRGBF` writer - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGBF_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGBF_A>;
impl<'a, REG> BSGTRGBF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTETRGB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGBF_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGBF_A::_1)
    }
}
/**GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGCR_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTETRGC input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTETRGC input.
    _1 = 1,
}
impl From<BSGTRGCR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGCR` reader - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGCR_R = crate::BitReader<BSGTRGCR_A>;
impl BSGTRGCR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGCR_A {
        match self.bits {
            false => BSGTRGCR_A::_0,
            true => BSGTRGCR_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGCR_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGCR_A::_1
    }
}
///Field `BSGTRGCR` writer - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGCR_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGCR_A>;
impl<'a, REG> BSGTRGCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGCR_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGCR_A::_1)
    }
}
/**GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGCF_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTETRGC input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTETRGC input.
    _1 = 1,
}
impl From<BSGTRGCF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGCF` reader - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGCF_R = crate::BitReader<BSGTRGCF_A>;
impl BSGTRGCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGCF_A {
        match self.bits {
            false => BSGTRGCF_A::_0,
            true => BSGTRGCF_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGCF_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGC input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGCF_A::_1
    }
}
///Field `BSGTRGCF` writer - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGCF_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGCF_A>;
impl<'a, REG> BSGTRGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTETRGC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGCF_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGC input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGCF_A::_1)
    }
}
/**GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGDR_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTETRGD input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTETRGD input.
    _1 = 1,
}
impl From<BSGTRGDR_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGDR` reader - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGDR_R = crate::BitReader<BSGTRGDR_A>;
impl BSGTRGDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGDR_A {
        match self.bits {
            false => BSGTRGDR_A::_0,
            true => BSGTRGDR_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGDR_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGDR_A::_1
    }
}
///Field `BSGTRGDR` writer - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable
pub type BSGTRGDR_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGDR_A>;
impl<'a, REG> BSGTRGDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGDR_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGDR_A::_1)
    }
}
/**GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSGTRGDF_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTETRGD input
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTETRGD input.
    _1 = 1,
}
impl From<BSGTRGDF_A> for bool {
    #[inline(always)]
    fn from(variant: BSGTRGDF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSGTRGDF` reader - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGDF_R = crate::BitReader<BSGTRGDF_A>;
impl BSGTRGDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSGTRGDF_A {
        match self.bits {
            false => BSGTRGDF_A::_0,
            true => BSGTRGDF_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSGTRGDF_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSGTRGDF_A::_1
    }
}
///Field `BSGTRGDF` writer - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable
pub type BSGTRGDF_W<'a, REG> = crate::BitWriter<'a, REG, BSGTRGDF_A>;
impl<'a, REG> BSGTRGDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTETRGD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGDF_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTETRGD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSGTRGDF_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCARBL_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<BSCARBL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCARBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCARBL` reader - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
pub type BSCARBL_R = crate::BitReader<BSCARBL_A>;
impl BSCARBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCARBL_A {
        match self.bits {
            false => BSCARBL_A::_0,
            true => BSCARBL_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCARBL_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCARBL_A::_1
    }
}
///Field `BSCARBL` writer - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
pub type BSCARBL_W<'a, REG> = crate::BitWriter<'a, REG, BSCARBL_A>;
impl<'a, REG> BSCARBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCARBL_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCARBL_A::_1)
    }
}
/**GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCARBH_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<BSCARBH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCARBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCARBH` reader - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable
pub type BSCARBH_R = crate::BitReader<BSCARBH_A>;
impl BSCARBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCARBH_A {
        match self.bits {
            false => BSCARBH_A::_0,
            true => BSCARBH_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCARBH_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCARBH_A::_1
    }
}
///Field `BSCARBH` writer - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable
pub type BSCARBH_W<'a, REG> = crate::BitWriter<'a, REG, BSCARBH_A>;
impl<'a, REG> BSCARBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCARBH_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCARBH_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCAFBL_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0.
    _1 = 1,
}
impl From<BSCAFBL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCAFBL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCAFBL` reader - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
pub type BSCAFBL_R = crate::BitReader<BSCAFBL_A>;
impl BSCAFBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCAFBL_A {
        match self.bits {
            false => BSCAFBL_A::_0,
            true => BSCAFBL_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCAFBL_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCAFBL_A::_1
    }
}
///Field `BSCAFBL` writer - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
pub type BSCAFBL_W<'a, REG> = crate::BitWriter<'a, REG, BSCAFBL_A>;
impl<'a, REG> BSCAFBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCAFBL_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCAFBL_A::_1)
    }
}
/**GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCAFBH_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1.
    _1 = 1,
}
impl From<BSCAFBH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCAFBH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCAFBH` reader - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable
pub type BSCAFBH_R = crate::BitReader<BSCAFBH_A>;
impl BSCAFBH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCAFBH_A {
        match self.bits {
            false => BSCAFBH_A::_0,
            true => BSCAFBH_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCAFBH_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCAFBH_A::_1
    }
}
///Field `BSCAFBH` writer - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable
pub type BSCAFBH_W<'a, REG> = crate::BitWriter<'a, REG, BSCAFBH_A>;
impl<'a, REG> BSCAFBH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCAFBH_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCAFBH_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBRAL_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<BSCBRAL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBRAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCBRAL` reader - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
pub type BSCBRAL_R = crate::BitReader<BSCBRAL_A>;
impl BSCBRAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCBRAL_A {
        match self.bits {
            false => BSCBRAL_A::_0,
            true => BSCBRAL_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBRAL_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBRAL_A::_1
    }
}
///Field `BSCBRAL` writer - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
pub type BSCBRAL_W<'a, REG> = crate::BitWriter<'a, REG, BSCBRAL_A>;
impl<'a, REG> BSCBRAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBRAL_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBRAL_A::_1)
    }
}
/**GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBRAH_A {
    ///0: Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<BSCBRAH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBRAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCBRAH` reader - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable
pub type BSCBRAH_R = crate::BitReader<BSCBRAH_A>;
impl BSCBRAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCBRAH_A {
        match self.bits {
            false => BSCBRAH_A::_0,
            true => BSCBRAH_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBRAH_A::_0
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBRAH_A::_1
    }
}
///Field `BSCBRAH` writer - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable
pub type BSCBRAH_W<'a, REG> = crate::BitWriter<'a, REG, BSCBRAH_A>;
impl<'a, REG> BSCBRAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBRAH_A::_0)
    }
    ///Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBRAH_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBFAL_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0.
    _1 = 1,
}
impl From<BSCBFAL_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBFAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCBFAL` reader - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
pub type BSCBFAL_R = crate::BitReader<BSCBFAL_A>;
impl BSCBFAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCBFAL_A {
        match self.bits {
            false => BSCBFAL_A::_0,
            true => BSCBFAL_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBFAL_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBFAL_A::_1
    }
}
///Field `BSCBFAL` writer - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
pub type BSCBFAL_W<'a, REG> = crate::BitWriter<'a, REG, BSCBFAL_A>;
impl<'a, REG> BSCBFAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBFAL_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBFAL_A::_1)
    }
}
/**GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSCBFAH_A {
    ///0: Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1
    _0 = 0,
    ///1: Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1.
    _1 = 1,
}
impl From<BSCBFAH_A> for bool {
    #[inline(always)]
    fn from(variant: BSCBFAH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSCBFAH` reader - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable
pub type BSCBFAH_R = crate::BitReader<BSCBFAH_A>;
impl BSCBFAH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSCBFAH_A {
        match self.bits {
            false => BSCBFAH_A::_0,
            true => BSCBFAH_A::_1,
        }
    }
    ///Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSCBFAH_A::_0
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSCBFAH_A::_1
    }
}
///Field `BSCBFAH` writer - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable
pub type BSCBFAH_W<'a, REG> = crate::BitWriter<'a, REG, BSCBFAH_A>;
impl<'a, REG> BSCBFAH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBFAH_A::_0)
    }
    ///Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSCBFAH_A::_1)
    }
}
/**ELC_GPTA Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCA_A {
    ///0: Disable GTCCRB input capture on ELC_GPTA input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTA input.
    _1 = 1,
}
impl From<BSELCA_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCA` reader - ELC_GPTA Event Source GTCCRB Input Capture Enable
pub type BSELCA_R = crate::BitReader<BSELCA_A>;
impl BSELCA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCA_A {
        match self.bits {
            false => BSELCA_A::_0,
            true => BSELCA_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTA input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCA_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTA input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCA_A::_1
    }
}
///Field `BSELCA` writer - ELC_GPTA Event Source GTCCRB Input Capture Enable
pub type BSELCA_W<'a, REG> = crate::BitWriter<'a, REG, BSELCA_A>;
impl<'a, REG> BSELCA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTA input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCA_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTA input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCA_A::_1)
    }
}
/**ELC_GPTB Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCB_A {
    ///0: Disable GTCCRB input capture on ELC_GPTB input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTB input.
    _1 = 1,
}
impl From<BSELCB_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCB_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCB` reader - ELC_GPTB Event Source GTCCRB Input Capture Enable
pub type BSELCB_R = crate::BitReader<BSELCB_A>;
impl BSELCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCB_A {
        match self.bits {
            false => BSELCB_A::_0,
            true => BSELCB_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTB input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCB_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTB input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCB_A::_1
    }
}
///Field `BSELCB` writer - ELC_GPTB Event Source GTCCRB Input Capture Enable
pub type BSELCB_W<'a, REG> = crate::BitWriter<'a, REG, BSELCB_A>;
impl<'a, REG> BSELCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTB input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCB_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTB input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCB_A::_1)
    }
}
/**ELC_GPTC Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCC_A {
    ///0: Disable GTCCRB input capture on ELC_GPTC input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTC input
    _1 = 1,
}
impl From<BSELCC_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCC` reader - ELC_GPTC Event Source GTCCRB Input Capture Enable
pub type BSELCC_R = crate::BitReader<BSELCC_A>;
impl BSELCC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCC_A {
        match self.bits {
            false => BSELCC_A::_0,
            true => BSELCC_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTC input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCC_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTC input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCC_A::_1
    }
}
///Field `BSELCC` writer - ELC_GPTC Event Source GTCCRB Input Capture Enable
pub type BSELCC_W<'a, REG> = crate::BitWriter<'a, REG, BSELCC_A>;
impl<'a, REG> BSELCC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTC input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCC_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTC input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCC_A::_1)
    }
}
/**ELC_GPTD Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCD_A {
    ///0: Disable GTCCRB input capture on ELC_GPTD input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTD input.
    _1 = 1,
}
impl From<BSELCD_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCD` reader - ELC_GPTD Event Source GTCCRB Input Capture Enable
pub type BSELCD_R = crate::BitReader<BSELCD_A>;
impl BSELCD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCD_A {
        match self.bits {
            false => BSELCD_A::_0,
            true => BSELCD_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTD input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCD_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTD input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCD_A::_1
    }
}
///Field `BSELCD` writer - ELC_GPTD Event Source GTCCRB Input Capture Enable
pub type BSELCD_W<'a, REG> = crate::BitWriter<'a, REG, BSELCD_A>;
impl<'a, REG> BSELCD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTD input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCD_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTD input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCD_A::_1)
    }
}
/**ELC_GPTE Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCE_A {
    ///0: Disable GTCCRB input capture on ELC_GPTE input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTE input
    _1 = 1,
}
impl From<BSELCE_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCE` reader - ELC_GPTE Event Source GTCCRB Input Capture Enable
pub type BSELCE_R = crate::BitReader<BSELCE_A>;
impl BSELCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCE_A {
        match self.bits {
            false => BSELCE_A::_0,
            true => BSELCE_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTE input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCE_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTE input
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCE_A::_1
    }
}
///Field `BSELCE` writer - ELC_GPTE Event Source GTCCRB Input Capture Enable
pub type BSELCE_W<'a, REG> = crate::BitWriter<'a, REG, BSELCE_A>;
impl<'a, REG> BSELCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTE input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCE_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTE input
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCE_A::_1)
    }
}
/**ELC_GPTF Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCF_A {
    ///0: Disable GTCCRB input capture on ELC_GPTF input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTF input.
    _1 = 1,
}
impl From<BSELCF_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCF` reader - ELC_GPTF Event Source GTCCRB Input Capture Enable
pub type BSELCF_R = crate::BitReader<BSELCF_A>;
impl BSELCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCF_A {
        match self.bits {
            false => BSELCF_A::_0,
            true => BSELCF_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTF input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCF_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTF input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCF_A::_1
    }
}
///Field `BSELCF` writer - ELC_GPTF Event Source GTCCRB Input Capture Enable
pub type BSELCF_W<'a, REG> = crate::BitWriter<'a, REG, BSELCF_A>;
impl<'a, REG> BSELCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTF input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCF_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTF input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCF_A::_1)
    }
}
/**ELC_GPTG Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCG_A {
    ///0: Disable GTCCRB input capture on ELC_GPTG input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTG input.
    _1 = 1,
}
impl From<BSELCG_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCG` reader - ELC_GPTG Event Source GTCCRB Input Capture Enable
pub type BSELCG_R = crate::BitReader<BSELCG_A>;
impl BSELCG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCG_A {
        match self.bits {
            false => BSELCG_A::_0,
            true => BSELCG_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTG input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCG_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTG input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCG_A::_1
    }
}
///Field `BSELCG` writer - ELC_GPTG Event Source GTCCRB Input Capture Enable
pub type BSELCG_W<'a, REG> = crate::BitWriter<'a, REG, BSELCG_A>;
impl<'a, REG> BSELCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTG input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCG_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTG input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCG_A::_1)
    }
}
/**ELC_GPTH Event Source GTCCRB Input Capture Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSELCH_A {
    ///0: Disable GTCCRB input capture on ELC_GPTH input
    _0 = 0,
    ///1: Enable GTCCRB input capture on ELC_GPTH input.
    _1 = 1,
}
impl From<BSELCH_A> for bool {
    #[inline(always)]
    fn from(variant: BSELCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSELCH` reader - ELC_GPTH Event Source GTCCRB Input Capture Enable
pub type BSELCH_R = crate::BitReader<BSELCH_A>;
impl BSELCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BSELCH_A {
        match self.bits {
            false => BSELCH_A::_0,
            true => BSELCH_A::_1,
        }
    }
    ///Disable GTCCRB input capture on ELC_GPTH input
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSELCH_A::_0
    }
    ///Enable GTCCRB input capture on ELC_GPTH input.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSELCH_A::_1
    }
}
///Field `BSELCH` writer - ELC_GPTH Event Source GTCCRB Input Capture Enable
pub type BSELCH_W<'a, REG> = crate::BitWriter<'a, REG, BSELCH_A>;
impl<'a, REG> BSELCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable GTCCRB input capture on ELC_GPTH input
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCH_A::_0)
    }
    ///Enable GTCCRB input capture on ELC_GPTH input.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BSELCH_A::_1)
    }
}
impl R {
    ///Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgar(&self) -> BSGTRGAR_R {
        BSGTRGAR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgaf(&self) -> BSGTRGAF_R {
        BSGTRGAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgbr(&self) -> BSGTRGBR_R {
        BSGTRGBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgbf(&self) -> BSGTRGBF_R {
        BSGTRGBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgcr(&self) -> BSGTRGCR_R {
        BSGTRGCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgcf(&self) -> BSGTRGCF_R {
        BSGTRGCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgdr(&self) -> BSGTRGDR_R {
        BSGTRGDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgdf(&self) -> BSGTRGDF_R {
        BSGTRGDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscarbl(&self) -> BSCARBL_R {
        BSCARBL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscarbh(&self) -> BSCARBH_R {
        BSCARBH_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscafbl(&self) -> BSCAFBL_R {
        BSCAFBL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscafbh(&self) -> BSCAFBH_R {
        BSCAFBH_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbral(&self) -> BSCBRAL_R {
        BSCBRAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbrah(&self) -> BSCBRAH_R {
        BSCBRAH_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbfal(&self) -> BSCBFAL_R {
        BSCBFAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbfah(&self) -> BSCBFAH_R {
        BSCBFAH_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselca(&self) -> BSELCA_R {
        BSELCA_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcb(&self) -> BSELCB_R {
        BSELCB_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcc(&self) -> BSELCC_R {
        BSELCC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcd(&self) -> BSELCD_R {
        BSELCD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ELC_GPTE Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselce(&self) -> BSELCE_R {
        BSELCE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ELC_GPTF Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcf(&self) -> BSELCF_R {
        BSELCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ELC_GPTG Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcg(&self) -> BSELCG_R {
        BSELCG_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ELC_GPTH Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselch(&self) -> BSELCH_R {
        BSELCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgar(&mut self) -> BSGTRGAR_W<GTICBSR_SPEC> {
        BSGTRGAR_W::new(self, 0)
    }
    ///Bit 1 - GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgaf(&mut self) -> BSGTRGAF_W<GTICBSR_SPEC> {
        BSGTRGAF_W::new(self, 1)
    }
    ///Bit 2 - GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgbr(&mut self) -> BSGTRGBR_W<GTICBSR_SPEC> {
        BSGTRGBR_W::new(self, 2)
    }
    ///Bit 3 - GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgbf(&mut self) -> BSGTRGBF_W<GTICBSR_SPEC> {
        BSGTRGBF_W::new(self, 3)
    }
    ///Bit 4 - GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgcr(&mut self) -> BSGTRGCR_W<GTICBSR_SPEC> {
        BSGTRGCR_W::new(self, 4)
    }
    ///Bit 5 - GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgcf(&mut self) -> BSGTRGCF_W<GTICBSR_SPEC> {
        BSGTRGCF_W::new(self, 5)
    }
    ///Bit 6 - GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgdr(&mut self) -> BSGTRGDR_W<GTICBSR_SPEC> {
        BSGTRGDR_W::new(self, 6)
    }
    ///Bit 7 - GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bsgtrgdf(&mut self) -> BSGTRGDF_W<GTICBSR_SPEC> {
        BSGTRGDF_W::new(self, 7)
    }
    ///Bit 8 - GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscarbl(&mut self) -> BSCARBL_W<GTICBSR_SPEC> {
        BSCARBL_W::new(self, 8)
    }
    ///Bit 9 - GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscarbh(&mut self) -> BSCARBH_W<GTICBSR_SPEC> {
        BSCARBH_W::new(self, 9)
    }
    ///Bit 10 - GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscafbl(&mut self) -> BSCAFBL_W<GTICBSR_SPEC> {
        BSCAFBL_W::new(self, 10)
    }
    ///Bit 11 - GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscafbh(&mut self) -> BSCAFBH_W<GTICBSR_SPEC> {
        BSCAFBH_W::new(self, 11)
    }
    ///Bit 12 - GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbral(&mut self) -> BSCBRAL_W<GTICBSR_SPEC> {
        BSCBRAL_W::new(self, 12)
    }
    ///Bit 13 - GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbrah(&mut self) -> BSCBRAH_W<GTICBSR_SPEC> {
        BSCBRAH_W::new(self, 13)
    }
    ///Bit 14 - GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbfal(&mut self) -> BSCBFAL_W<GTICBSR_SPEC> {
        BSCBFAL_W::new(self, 14)
    }
    ///Bit 15 - GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bscbfah(&mut self) -> BSCBFAH_W<GTICBSR_SPEC> {
        BSCBFAH_W::new(self, 15)
    }
    ///Bit 16 - ELC_GPTA Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselca(&mut self) -> BSELCA_W<GTICBSR_SPEC> {
        BSELCA_W::new(self, 16)
    }
    ///Bit 17 - ELC_GPTB Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcb(&mut self) -> BSELCB_W<GTICBSR_SPEC> {
        BSELCB_W::new(self, 17)
    }
    ///Bit 18 - ELC_GPTC Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcc(&mut self) -> BSELCC_W<GTICBSR_SPEC> {
        BSELCC_W::new(self, 18)
    }
    ///Bit 19 - ELC_GPTD Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcd(&mut self) -> BSELCD_W<GTICBSR_SPEC> {
        BSELCD_W::new(self, 19)
    }
    ///Bit 20 - ELC_GPTE Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselce(&mut self) -> BSELCE_W<GTICBSR_SPEC> {
        BSELCE_W::new(self, 20)
    }
    ///Bit 21 - ELC_GPTF Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcf(&mut self) -> BSELCF_W<GTICBSR_SPEC> {
        BSELCF_W::new(self, 21)
    }
    ///Bit 22 - ELC_GPTG Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselcg(&mut self) -> BSELCG_W<GTICBSR_SPEC> {
        BSELCG_W::new(self, 22)
    }
    ///Bit 23 - ELC_GPTH Event Source GTCCRB Input Capture Enable
    #[inline(always)]
    pub fn bselch(&mut self) -> BSELCH_W<GTICBSR_SPEC> {
        BSELCH_W::new(self, 23)
    }
}
/**General PWM Timer Input Capture Source Select Register B

You can [`read`](crate::Reg::read) this register and get [`gticbsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticbsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTICBSR_SPEC;
impl crate::RegisterSpec for GTICBSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gticbsr::R`](R) reader structure
impl crate::Readable for GTICBSR_SPEC {}
///`write(|w| ..)` method takes [`gticbsr::W`](W) writer structure
impl crate::Writable for GTICBSR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTICBSR to value 0
impl crate::Resettable for GTICBSR_SPEC {}
