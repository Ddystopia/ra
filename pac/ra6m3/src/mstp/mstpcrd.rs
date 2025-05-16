///Register `MSTPCRD` reader
pub type R = crate::R<MSTPCRD_SPEC>;
///Register `MSTPCRD` writer
pub type W = crate::W<MSTPCRD_SPEC>;
/**AGT1 Module StopNote: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD2_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD2_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD2` reader - AGT1 Module StopNote: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1.
pub type MSTPD2_R = crate::BitReader<MSTPD2_A>;
impl MSTPD2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD2_A {
        match self.bits {
            false => MSTPD2_A::_0,
            true => MSTPD2_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD2_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD2_A::_1
    }
}
///Field `MSTPD2` writer - AGT1 Module StopNote: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1.
pub type MSTPD2_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD2_A>;
impl<'a, REG> MSTPD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD2_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD2_A::_1)
    }
}
/**AGT0 Module StopNote: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0.

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD3_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD3` reader - AGT0 Module StopNote: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0.
pub type MSTPD3_R = crate::BitReader<MSTPD3_A>;
impl MSTPD3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD3_A {
        match self.bits {
            false => MSTPD3_A::_0,
            true => MSTPD3_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD3_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD3_A::_1
    }
}
///Field `MSTPD3` writer - AGT0 Module StopNote: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0.
pub type MSTPD3_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD3_A>;
impl<'a, REG> MSTPD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD3_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD3_A::_1)
    }
}
/**GPT ch7-ch0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD5_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD5_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD5` reader - GPT ch7-ch0 Module Stop
pub type MSTPD5_R = crate::BitReader<MSTPD5_A>;
impl MSTPD5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD5_A {
        match self.bits {
            false => MSTPD5_A::_0,
            true => MSTPD5_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD5_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD5_A::_1
    }
}
///Field `MSTPD5` writer - GPT ch7-ch0 Module Stop
pub type MSTPD5_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD5_A>;
impl<'a, REG> MSTPD5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD5_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD5_A::_1)
    }
}
/**GPT ch13-ch8 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD6_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD6_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD6_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD6` reader - GPT ch13-ch8 Module Stop
pub type MSTPD6_R = crate::BitReader<MSTPD6_A>;
impl MSTPD6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD6_A {
        match self.bits {
            false => MSTPD6_A::_0,
            true => MSTPD6_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD6_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD6_A::_1
    }
}
///Field `MSTPD6` writer - GPT ch13-ch8 Module Stop
pub type MSTPD6_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD6_A>;
impl<'a, REG> MSTPD6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD6_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD6_A::_1)
    }
}
/**PGI Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD14_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD14` reader - PGI Module Stop
pub type MSTPD14_R = crate::BitReader<MSTPD14_A>;
impl MSTPD14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD14_A {
        match self.bits {
            false => MSTPD14_A::_0,
            true => MSTPD14_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD14_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD14_A::_1
    }
}
///Field `MSTPD14` writer - PGI Module Stop
pub type MSTPD14_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD14_A>;
impl<'a, REG> MSTPD14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD14_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD14_A::_1)
    }
}
/**12-bit A/D Converter 1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD15_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD15_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD15_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD15` reader - 12-bit A/D Converter 1 Module Stop
pub type MSTPD15_R = crate::BitReader<MSTPD15_A>;
impl MSTPD15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD15_A {
        match self.bits {
            false => MSTPD15_A::_0,
            true => MSTPD15_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD15_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD15_A::_1
    }
}
///Field `MSTPD15` writer - 12-bit A/D Converter 1 Module Stop
pub type MSTPD15_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD15_A>;
impl<'a, REG> MSTPD15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD15_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD15_A::_1)
    }
}
/**12-bit A/D Converter 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD16_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD16_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD16` reader - 12-bit A/D Converter 0 Module Stop
pub type MSTPD16_R = crate::BitReader<MSTPD16_A>;
impl MSTPD16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD16_A {
        match self.bits {
            false => MSTPD16_A::_0,
            true => MSTPD16_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD16_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD16_A::_1
    }
}
///Field `MSTPD16` writer - 12-bit A/D Converter 0 Module Stop
pub type MSTPD16_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD16_A>;
impl<'a, REG> MSTPD16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD16_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD16_A::_1)
    }
}
/**D/A Converter 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD20_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD20_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD20` reader - D/A Converter 0 Module Stop
pub type MSTPD20_R = crate::BitReader<MSTPD20_A>;
impl MSTPD20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD20_A {
        match self.bits {
            false => MSTPD20_A::_0,
            true => MSTPD20_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD20_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD20_A::_1
    }
}
///Field `MSTPD20` writer - D/A Converter 0 Module Stop
pub type MSTPD20_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD20_A>;
impl<'a, REG> MSTPD20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD20_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD20_A::_1)
    }
}
/**Temperature Sensor Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD22_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD22_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD22` reader - Temperature Sensor Module Stop
pub type MSTPD22_R = crate::BitReader<MSTPD22_A>;
impl MSTPD22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD22_A {
        match self.bits {
            false => MSTPD22_A::_0,
            true => MSTPD22_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD22_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD22_A::_1
    }
}
///Field `MSTPD22` writer - Temperature Sensor Module Stop
pub type MSTPD22_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD22_A>;
impl<'a, REG> MSTPD22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD22_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD22_A::_1)
    }
}
/**Comparator-OC5 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD23_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD23_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD23_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD23` reader - Comparator-OC5 Module Stop
pub type MSTPD23_R = crate::BitReader<MSTPD23_A>;
impl MSTPD23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD23_A {
        match self.bits {
            false => MSTPD23_A::_0,
            true => MSTPD23_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD23_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD23_A::_1
    }
}
///Field `MSTPD23` writer - Comparator-OC5 Module Stop
pub type MSTPD23_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD23_A>;
impl<'a, REG> MSTPD23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD23_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD23_A::_1)
    }
}
/**Comparator-OC4 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD24_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD24_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD24` reader - Comparator-OC4 Module Stop
pub type MSTPD24_R = crate::BitReader<MSTPD24_A>;
impl MSTPD24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD24_A {
        match self.bits {
            false => MSTPD24_A::_0,
            true => MSTPD24_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD24_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD24_A::_1
    }
}
///Field `MSTPD24` writer - Comparator-OC4 Module Stop
pub type MSTPD24_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD24_A>;
impl<'a, REG> MSTPD24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD24_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD24_A::_1)
    }
}
/**Comparator-OC3 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD25_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD25_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD25_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD25` reader - Comparator-OC3 Module Stop
pub type MSTPD25_R = crate::BitReader<MSTPD25_A>;
impl MSTPD25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD25_A {
        match self.bits {
            false => MSTPD25_A::_0,
            true => MSTPD25_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD25_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD25_A::_1
    }
}
///Field `MSTPD25` writer - Comparator-OC3 Module Stop
pub type MSTPD25_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD25_A>;
impl<'a, REG> MSTPD25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD25_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD25_A::_1)
    }
}
/**Comparator-OC2 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD26_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD26_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD26_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD26` reader - Comparator-OC2 Module Stop
pub type MSTPD26_R = crate::BitReader<MSTPD26_A>;
impl MSTPD26_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD26_A {
        match self.bits {
            false => MSTPD26_A::_0,
            true => MSTPD26_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD26_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD26_A::_1
    }
}
///Field `MSTPD26` writer - Comparator-OC2 Module Stop
pub type MSTPD26_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD26_A>;
impl<'a, REG> MSTPD26_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD26_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD26_A::_1)
    }
}
/**Comparator-OC1 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD27_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD27_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD27_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD27` reader - Comparator-OC1 Module Stop
pub type MSTPD27_R = crate::BitReader<MSTPD27_A>;
impl MSTPD27_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD27_A {
        match self.bits {
            false => MSTPD27_A::_0,
            true => MSTPD27_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD27_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD27_A::_1
    }
}
///Field `MSTPD27` writer - Comparator-OC1 Module Stop
pub type MSTPD27_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD27_A>;
impl<'a, REG> MSTPD27_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD27_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD27_A::_1)
    }
}
/**Comparator-OC0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD28_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD28_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD28_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD28` reader - Comparator-OC0 Module Stop
pub type MSTPD28_R = crate::BitReader<MSTPD28_A>;
impl MSTPD28_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD28_A {
        match self.bits {
            false => MSTPD28_A::_0,
            true => MSTPD28_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD28_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD28_A::_1
    }
}
///Field `MSTPD28` writer - Comparator-OC0 Module Stop
pub type MSTPD28_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD28_A>;
impl<'a, REG> MSTPD28_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD28_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD28_A::_1)
    }
}
impl R {
    ///Bit 2 - AGT1 Module StopNote: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1.
    #[inline(always)]
    pub fn mstpd2(&self) -> MSTPD2_R {
        MSTPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AGT0 Module StopNote: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0.
    #[inline(always)]
    pub fn mstpd3(&self) -> MSTPD3_R {
        MSTPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - GPT ch7-ch0 Module Stop
    #[inline(always)]
    pub fn mstpd5(&self) -> MSTPD5_R {
        MSTPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPT ch13-ch8 Module Stop
    #[inline(always)]
    pub fn mstpd6(&self) -> MSTPD6_R {
        MSTPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - PGI Module Stop
    #[inline(always)]
    pub fn mstpd14(&self) -> MSTPD14_R {
        MSTPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - 12-bit A/D Converter 1 Module Stop
    #[inline(always)]
    pub fn mstpd15(&self) -> MSTPD15_R {
        MSTPD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 12-bit A/D Converter 0 Module Stop
    #[inline(always)]
    pub fn mstpd16(&self) -> MSTPD16_R {
        MSTPD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - D/A Converter 0 Module Stop
    #[inline(always)]
    pub fn mstpd20(&self) -> MSTPD20_R {
        MSTPD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Temperature Sensor Module Stop
    #[inline(always)]
    pub fn mstpd22(&self) -> MSTPD22_R {
        MSTPD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Comparator-OC5 Module Stop
    #[inline(always)]
    pub fn mstpd23(&self) -> MSTPD23_R {
        MSTPD23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Comparator-OC4 Module Stop
    #[inline(always)]
    pub fn mstpd24(&self) -> MSTPD24_R {
        MSTPD24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Comparator-OC3 Module Stop
    #[inline(always)]
    pub fn mstpd25(&self) -> MSTPD25_R {
        MSTPD25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Comparator-OC2 Module Stop
    #[inline(always)]
    pub fn mstpd26(&self) -> MSTPD26_R {
        MSTPD26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Comparator-OC1 Module Stop
    #[inline(always)]
    pub fn mstpd27(&self) -> MSTPD27_R {
        MSTPD27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Comparator-OC0 Module Stop
    #[inline(always)]
    pub fn mstpd28(&self) -> MSTPD28_R {
        MSTPD28_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - AGT1 Module StopNote: AGT1 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT1.
    #[inline(always)]
    pub fn mstpd2(&mut self) -> MSTPD2_W<MSTPCRD_SPEC> {
        MSTPD2_W::new(self, 2)
    }
    ///Bit 3 - AGT0 Module StopNote: AGT0 is in the module stop state when the count source is either of PCLKB, PCLKB/2 or PCLKB/8. In case the count source is sub-clock or LOCO, this bit should be set to 1 except when accessing the registers of AGT0.
    #[inline(always)]
    pub fn mstpd3(&mut self) -> MSTPD3_W<MSTPCRD_SPEC> {
        MSTPD3_W::new(self, 3)
    }
    ///Bit 5 - GPT ch7-ch0 Module Stop
    #[inline(always)]
    pub fn mstpd5(&mut self) -> MSTPD5_W<MSTPCRD_SPEC> {
        MSTPD5_W::new(self, 5)
    }
    ///Bit 6 - GPT ch13-ch8 Module Stop
    #[inline(always)]
    pub fn mstpd6(&mut self) -> MSTPD6_W<MSTPCRD_SPEC> {
        MSTPD6_W::new(self, 6)
    }
    ///Bit 14 - PGI Module Stop
    #[inline(always)]
    pub fn mstpd14(&mut self) -> MSTPD14_W<MSTPCRD_SPEC> {
        MSTPD14_W::new(self, 14)
    }
    ///Bit 15 - 12-bit A/D Converter 1 Module Stop
    #[inline(always)]
    pub fn mstpd15(&mut self) -> MSTPD15_W<MSTPCRD_SPEC> {
        MSTPD15_W::new(self, 15)
    }
    ///Bit 16 - 12-bit A/D Converter 0 Module Stop
    #[inline(always)]
    pub fn mstpd16(&mut self) -> MSTPD16_W<MSTPCRD_SPEC> {
        MSTPD16_W::new(self, 16)
    }
    ///Bit 20 - D/A Converter 0 Module Stop
    #[inline(always)]
    pub fn mstpd20(&mut self) -> MSTPD20_W<MSTPCRD_SPEC> {
        MSTPD20_W::new(self, 20)
    }
    ///Bit 22 - Temperature Sensor Module Stop
    #[inline(always)]
    pub fn mstpd22(&mut self) -> MSTPD22_W<MSTPCRD_SPEC> {
        MSTPD22_W::new(self, 22)
    }
    ///Bit 23 - Comparator-OC5 Module Stop
    #[inline(always)]
    pub fn mstpd23(&mut self) -> MSTPD23_W<MSTPCRD_SPEC> {
        MSTPD23_W::new(self, 23)
    }
    ///Bit 24 - Comparator-OC4 Module Stop
    #[inline(always)]
    pub fn mstpd24(&mut self) -> MSTPD24_W<MSTPCRD_SPEC> {
        MSTPD24_W::new(self, 24)
    }
    ///Bit 25 - Comparator-OC3 Module Stop
    #[inline(always)]
    pub fn mstpd25(&mut self) -> MSTPD25_W<MSTPCRD_SPEC> {
        MSTPD25_W::new(self, 25)
    }
    ///Bit 26 - Comparator-OC2 Module Stop
    #[inline(always)]
    pub fn mstpd26(&mut self) -> MSTPD26_W<MSTPCRD_SPEC> {
        MSTPD26_W::new(self, 26)
    }
    ///Bit 27 - Comparator-OC1 Module Stop
    #[inline(always)]
    pub fn mstpd27(&mut self) -> MSTPD27_W<MSTPCRD_SPEC> {
        MSTPD27_W::new(self, 27)
    }
    ///Bit 28 - Comparator-OC0 Module Stop
    #[inline(always)]
    pub fn mstpd28(&mut self) -> MSTPD28_W<MSTPCRD_SPEC> {
        MSTPD28_W::new(self, 28)
    }
}
/**Module Stop Control Register D

You can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSTPCRD_SPEC;
impl crate::RegisterSpec for MSTPCRD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mstpcrd::R`](R) reader structure
impl crate::Readable for MSTPCRD_SPEC {}
///`write(|w| ..)` method takes [`mstpcrd::W`](W) writer structure
impl crate::Writable for MSTPCRD_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRD to value 0xffff_ffff
impl crate::Resettable for MSTPCRD_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
