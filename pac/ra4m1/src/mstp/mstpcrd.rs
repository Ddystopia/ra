///Register `MSTPCRD` reader
pub type R = crate::R<MSTPCRD_SPEC>;
///Register `MSTPCRD` writer
pub type W = crate::W<MSTPCRD_SPEC>;
/**Asynchronous General Purpose Timer 1 Module Stop

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
///Field `MSTPD2` reader - Asynchronous General Purpose Timer 1 Module Stop
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
///Field `MSTPD2` writer - Asynchronous General Purpose Timer 1 Module Stop
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
/**Asynchronous General Purpose Timer 0 Module Stop

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
///Field `MSTPD3` reader - Asynchronous General Purpose Timer 0 Module Stop
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
///Field `MSTPD3` writer - Asynchronous General Purpose Timer 0 Module Stop
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
/**General PWM Timer 323 to 320 Module Stop

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
///Field `MSTPD5` reader - General PWM Timer 323 to 320 Module Stop
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
///Field `MSTPD5` writer - General PWM Timer 323 to 320 Module Stop
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
/**General PWM Timer 169 to 164 Module Stop

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
///Field `MSTPD6` reader - General PWM Timer 169 to 164 Module Stop
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
///Field `MSTPD6` writer - General PWM Timer 169 to 164 Module Stop
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
/**Port Output Enable for GPT Module Stop

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
///Field `MSTPD14` reader - Port Output Enable for GPT Module Stop
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
///Field `MSTPD14` writer - Port Output Enable for GPT Module Stop
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
/**14-Bit A/D Converter Module Stop

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
///Field `MSTPD16` reader - 14-Bit A/D Converter Module Stop
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
///Field `MSTPD16` writer - 14-Bit A/D Converter Module Stop
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
/**8-bit D/A Converter Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD19_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD19_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD19` reader - 8-bit D/A Converter Module Stop
pub type MSTPD19_R = crate::BitReader<MSTPD19_A>;
impl MSTPD19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD19_A {
        match self.bits {
            false => MSTPD19_A::_0,
            true => MSTPD19_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD19_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD19_A::_1
    }
}
///Field `MSTPD19` writer - 8-bit D/A Converter Module Stop
pub type MSTPD19_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD19_A>;
impl<'a, REG> MSTPD19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD19_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD19_A::_1)
    }
}
/**12-Bit D/A Converter Module Stop

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
///Field `MSTPD20` reader - 12-Bit D/A Converter Module Stop
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
///Field `MSTPD20` writer - 12-Bit D/A Converter Module Stop
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
/**Low-Power Analog Comparator Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD29_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD29_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD29_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD29` reader - Low-Power Analog Comparator Module Stop
pub type MSTPD29_R = crate::BitReader<MSTPD29_A>;
impl MSTPD29_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD29_A {
        match self.bits {
            false => MSTPD29_A::_0,
            true => MSTPD29_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD29_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD29_A::_1
    }
}
///Field `MSTPD29` writer - Low-Power Analog Comparator Module Stop
pub type MSTPD29_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD29_A>;
impl<'a, REG> MSTPD29_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD29_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD29_A::_1)
    }
}
/**Operational Amplifier Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPD31_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPD31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPD31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPD31` reader - Operational Amplifier Module Stop
pub type MSTPD31_R = crate::BitReader<MSTPD31_A>;
impl MSTPD31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPD31_A {
        match self.bits {
            false => MSTPD31_A::_0,
            true => MSTPD31_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPD31_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPD31_A::_1
    }
}
///Field `MSTPD31` writer - Operational Amplifier Module Stop
pub type MSTPD31_W<'a, REG> = crate::BitWriter<'a, REG, MSTPD31_A>;
impl<'a, REG> MSTPD31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD31_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPD31_A::_1)
    }
}
impl R {
    ///Bit 2 - Asynchronous General Purpose Timer 1 Module Stop
    #[inline(always)]
    pub fn mstpd2(&self) -> MSTPD2_R {
        MSTPD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Asynchronous General Purpose Timer 0 Module Stop
    #[inline(always)]
    pub fn mstpd3(&self) -> MSTPD3_R {
        MSTPD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - General PWM Timer 323 to 320 Module Stop
    #[inline(always)]
    pub fn mstpd5(&self) -> MSTPD5_R {
        MSTPD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - General PWM Timer 169 to 164 Module Stop
    #[inline(always)]
    pub fn mstpd6(&self) -> MSTPD6_R {
        MSTPD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - Port Output Enable for GPT Module Stop
    #[inline(always)]
    pub fn mstpd14(&self) -> MSTPD14_R {
        MSTPD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - 14-Bit A/D Converter Module Stop
    #[inline(always)]
    pub fn mstpd16(&self) -> MSTPD16_R {
        MSTPD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - 8-bit D/A Converter Module Stop
    #[inline(always)]
    pub fn mstpd19(&self) -> MSTPD19_R {
        MSTPD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - 12-Bit D/A Converter Module Stop
    #[inline(always)]
    pub fn mstpd20(&self) -> MSTPD20_R {
        MSTPD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 29 - Low-Power Analog Comparator Module Stop
    #[inline(always)]
    pub fn mstpd29(&self) -> MSTPD29_R {
        MSTPD29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Operational Amplifier Module Stop
    #[inline(always)]
    pub fn mstpd31(&self) -> MSTPD31_R {
        MSTPD31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Asynchronous General Purpose Timer 1 Module Stop
    #[inline(always)]
    pub fn mstpd2(&mut self) -> MSTPD2_W<MSTPCRD_SPEC> {
        MSTPD2_W::new(self, 2)
    }
    ///Bit 3 - Asynchronous General Purpose Timer 0 Module Stop
    #[inline(always)]
    pub fn mstpd3(&mut self) -> MSTPD3_W<MSTPCRD_SPEC> {
        MSTPD3_W::new(self, 3)
    }
    ///Bit 5 - General PWM Timer 323 to 320 Module Stop
    #[inline(always)]
    pub fn mstpd5(&mut self) -> MSTPD5_W<MSTPCRD_SPEC> {
        MSTPD5_W::new(self, 5)
    }
    ///Bit 6 - General PWM Timer 169 to 164 Module Stop
    #[inline(always)]
    pub fn mstpd6(&mut self) -> MSTPD6_W<MSTPCRD_SPEC> {
        MSTPD6_W::new(self, 6)
    }
    ///Bit 14 - Port Output Enable for GPT Module Stop
    #[inline(always)]
    pub fn mstpd14(&mut self) -> MSTPD14_W<MSTPCRD_SPEC> {
        MSTPD14_W::new(self, 14)
    }
    ///Bit 16 - 14-Bit A/D Converter Module Stop
    #[inline(always)]
    pub fn mstpd16(&mut self) -> MSTPD16_W<MSTPCRD_SPEC> {
        MSTPD16_W::new(self, 16)
    }
    ///Bit 19 - 8-bit D/A Converter Module Stop
    #[inline(always)]
    pub fn mstpd19(&mut self) -> MSTPD19_W<MSTPCRD_SPEC> {
        MSTPD19_W::new(self, 19)
    }
    ///Bit 20 - 12-Bit D/A Converter Module Stop
    #[inline(always)]
    pub fn mstpd20(&mut self) -> MSTPD20_W<MSTPCRD_SPEC> {
        MSTPD20_W::new(self, 20)
    }
    ///Bit 29 - Low-Power Analog Comparator Module Stop
    #[inline(always)]
    pub fn mstpd29(&mut self) -> MSTPD29_W<MSTPCRD_SPEC> {
        MSTPD29_W::new(self, 29)
    }
    ///Bit 31 - Operational Amplifier Module Stop
    #[inline(always)]
    pub fn mstpd31(&mut self) -> MSTPD31_W<MSTPCRD_SPEC> {
        MSTPD31_W::new(self, 31)
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
