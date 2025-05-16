///Register `GTDLYCR2` reader
pub type R = crate::R<GTDLYCR2_SPEC>;
///Register `GTDLYCR2` writer
pub type W = crate::W<GTDLYCR2_SPEC>;
/**PWM Delay Generation Circuit bypass for channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS0_A {
    ///0: Bypass delay generation circuit of channel 0
    _0 = 0,
    ///1: Do not bypass delay generation circuit of channel 0.
    _1 = 1,
}
impl From<DLYBS0_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYBS0` reader - PWM Delay Generation Circuit bypass for channel 0
pub type DLYBS0_R = crate::BitReader<DLYBS0_A>;
impl DLYBS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYBS0_A {
        match self.bits {
            false => DLYBS0_A::_0,
            true => DLYBS0_A::_1,
        }
    }
    ///Bypass delay generation circuit of channel 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS0_A::_0
    }
    ///Do not bypass delay generation circuit of channel 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS0_A::_1
    }
}
///Field `DLYBS0` writer - PWM Delay Generation Circuit bypass for channel 0
pub type DLYBS0_W<'a, REG> = crate::BitWriter<'a, REG, DLYBS0_A>;
impl<'a, REG> DLYBS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass delay generation circuit of channel 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS0_A::_0)
    }
    ///Do not bypass delay generation circuit of channel 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS0_A::_1)
    }
}
/**PWM Delay Generation Circuit bypass for channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS1_A {
    ///0: Bypass delay generation circuit of channel 1
    _0 = 0,
    ///1: Do not bypass delay generation circuit of channel 1.
    _1 = 1,
}
impl From<DLYBS1_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYBS1` reader - PWM Delay Generation Circuit bypass for channel 1
pub type DLYBS1_R = crate::BitReader<DLYBS1_A>;
impl DLYBS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYBS1_A {
        match self.bits {
            false => DLYBS1_A::_0,
            true => DLYBS1_A::_1,
        }
    }
    ///Bypass delay generation circuit of channel 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS1_A::_0
    }
    ///Do not bypass delay generation circuit of channel 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS1_A::_1
    }
}
///Field `DLYBS1` writer - PWM Delay Generation Circuit bypass for channel 1
pub type DLYBS1_W<'a, REG> = crate::BitWriter<'a, REG, DLYBS1_A>;
impl<'a, REG> DLYBS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass delay generation circuit of channel 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS1_A::_0)
    }
    ///Do not bypass delay generation circuit of channel 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS1_A::_1)
    }
}
/**PWM Delay Generation Circuit bypass for channel 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS2_A {
    ///0: Bypass delay generation circuit of channel 2
    _0 = 0,
    ///1: Do not bypass delay generation circuit of channel 2.
    _1 = 1,
}
impl From<DLYBS2_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYBS2` reader - PWM Delay Generation Circuit bypass for channel 2
pub type DLYBS2_R = crate::BitReader<DLYBS2_A>;
impl DLYBS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYBS2_A {
        match self.bits {
            false => DLYBS2_A::_0,
            true => DLYBS2_A::_1,
        }
    }
    ///Bypass delay generation circuit of channel 2
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS2_A::_0
    }
    ///Do not bypass delay generation circuit of channel 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS2_A::_1
    }
}
///Field `DLYBS2` writer - PWM Delay Generation Circuit bypass for channel 2
pub type DLYBS2_W<'a, REG> = crate::BitWriter<'a, REG, DLYBS2_A>;
impl<'a, REG> DLYBS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass delay generation circuit of channel 2
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS2_A::_0)
    }
    ///Do not bypass delay generation circuit of channel 2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS2_A::_1)
    }
}
/**PWM Delay Generation Circuit bypass for channel 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYBS3_A {
    ///0: Bypass delay generation circuit of channel 3
    _0 = 0,
    ///1: Do not bypass delay generation circuit of channel 3.
    _1 = 1,
}
impl From<DLYBS3_A> for bool {
    #[inline(always)]
    fn from(variant: DLYBS3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYBS3` reader - PWM Delay Generation Circuit bypass for channel 3
pub type DLYBS3_R = crate::BitReader<DLYBS3_A>;
impl DLYBS3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYBS3_A {
        match self.bits {
            false => DLYBS3_A::_0,
            true => DLYBS3_A::_1,
        }
    }
    ///Bypass delay generation circuit of channel 3
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYBS3_A::_0
    }
    ///Do not bypass delay generation circuit of channel 3.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYBS3_A::_1
    }
}
///Field `DLYBS3` writer - PWM Delay Generation Circuit bypass for channel 3
pub type DLYBS3_W<'a, REG> = crate::BitWriter<'a, REG, DLYBS3_A>;
impl<'a, REG> DLYBS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bypass delay generation circuit of channel 3
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS3_A::_0)
    }
    ///Do not bypass delay generation circuit of channel 3.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYBS3_A::_1)
    }
}
/**PWM Delay Generation Circuit enable for channel 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN0_A {
    ///0: Enable delay generation circuit of channel 0
    _0 = 0,
    ///1: Disable delay generation circuit of channel 0.
    _1 = 1,
}
impl From<DLYEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYEN0` reader - PWM Delay Generation Circuit enable for channel 0
pub type DLYEN0_R = crate::BitReader<DLYEN0_A>;
impl DLYEN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYEN0_A {
        match self.bits {
            false => DLYEN0_A::_0,
            true => DLYEN0_A::_1,
        }
    }
    ///Enable delay generation circuit of channel 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN0_A::_0
    }
    ///Disable delay generation circuit of channel 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN0_A::_1
    }
}
///Field `DLYEN0` writer - PWM Delay Generation Circuit enable for channel 0
pub type DLYEN0_W<'a, REG> = crate::BitWriter<'a, REG, DLYEN0_A>;
impl<'a, REG> DLYEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable delay generation circuit of channel 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN0_A::_0)
    }
    ///Disable delay generation circuit of channel 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN0_A::_1)
    }
}
/**PWM Delay Generation Circuit enable for channel 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN1_A {
    ///0: Enable delay generation circuit of channel 1
    _0 = 0,
    ///1: Disable delay generation circuit of channel 1.
    _1 = 1,
}
impl From<DLYEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYEN1` reader - PWM Delay Generation Circuit enable for channel 1
pub type DLYEN1_R = crate::BitReader<DLYEN1_A>;
impl DLYEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYEN1_A {
        match self.bits {
            false => DLYEN1_A::_0,
            true => DLYEN1_A::_1,
        }
    }
    ///Enable delay generation circuit of channel 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN1_A::_0
    }
    ///Disable delay generation circuit of channel 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN1_A::_1
    }
}
///Field `DLYEN1` writer - PWM Delay Generation Circuit enable for channel 1
pub type DLYEN1_W<'a, REG> = crate::BitWriter<'a, REG, DLYEN1_A>;
impl<'a, REG> DLYEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable delay generation circuit of channel 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN1_A::_0)
    }
    ///Disable delay generation circuit of channel 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN1_A::_1)
    }
}
/**PWM Delay Generation Circuit enable for channel 2

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN2_A {
    ///0: Enable delay generation circuit of channel 2
    _0 = 0,
    ///1: Disable delay generation circuit of channel 2.
    _1 = 1,
}
impl From<DLYEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYEN2` reader - PWM Delay Generation Circuit enable for channel 2
pub type DLYEN2_R = crate::BitReader<DLYEN2_A>;
impl DLYEN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYEN2_A {
        match self.bits {
            false => DLYEN2_A::_0,
            true => DLYEN2_A::_1,
        }
    }
    ///Enable delay generation circuit of channel 2
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN2_A::_0
    }
    ///Disable delay generation circuit of channel 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN2_A::_1
    }
}
///Field `DLYEN2` writer - PWM Delay Generation Circuit enable for channel 2
pub type DLYEN2_W<'a, REG> = crate::BitWriter<'a, REG, DLYEN2_A>;
impl<'a, REG> DLYEN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable delay generation circuit of channel 2
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN2_A::_0)
    }
    ///Disable delay generation circuit of channel 2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN2_A::_1)
    }
}
/**PWM Delay Generation Circuit enable for channel 3

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYEN3_A {
    ///0: Enable delay generation circuit of channel 3
    _0 = 0,
    ///1: Disable delay generation circuit of channel 3
    _1 = 1,
}
impl From<DLYEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DLYEN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYEN3` reader - PWM Delay Generation Circuit enable for channel 3
pub type DLYEN3_R = crate::BitReader<DLYEN3_A>;
impl DLYEN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYEN3_A {
        match self.bits {
            false => DLYEN3_A::_0,
            true => DLYEN3_A::_1,
        }
    }
    ///Enable delay generation circuit of channel 3
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLYEN3_A::_0
    }
    ///Disable delay generation circuit of channel 3
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLYEN3_A::_1
    }
}
///Field `DLYEN3` writer - PWM Delay Generation Circuit enable for channel 3
pub type DLYEN3_W<'a, REG> = crate::BitWriter<'a, REG, DLYEN3_A>;
impl<'a, REG> DLYEN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable delay generation circuit of channel 3
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN3_A::_0)
    }
    ///Disable delay generation circuit of channel 3
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DLYEN3_A::_1)
    }
}
impl R {
    ///Bit 0 - PWM Delay Generation Circuit bypass for channel 0
    #[inline(always)]
    pub fn dlybs0(&self) -> DLYBS0_R {
        DLYBS0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PWM Delay Generation Circuit bypass for channel 1
    #[inline(always)]
    pub fn dlybs1(&self) -> DLYBS1_R {
        DLYBS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PWM Delay Generation Circuit bypass for channel 2
    #[inline(always)]
    pub fn dlybs2(&self) -> DLYBS2_R {
        DLYBS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PWM Delay Generation Circuit bypass for channel 3
    #[inline(always)]
    pub fn dlybs3(&self) -> DLYBS3_R {
        DLYBS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - PWM Delay Generation Circuit enable for channel 0
    #[inline(always)]
    pub fn dlyen0(&self) -> DLYEN0_R {
        DLYEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWM Delay Generation Circuit enable for channel 1
    #[inline(always)]
    pub fn dlyen1(&self) -> DLYEN1_R {
        DLYEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PWM Delay Generation Circuit enable for channel 2
    #[inline(always)]
    pub fn dlyen2(&self) -> DLYEN2_R {
        DLYEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PWM Delay Generation Circuit enable for channel 3
    #[inline(always)]
    pub fn dlyen3(&self) -> DLYEN3_R {
        DLYEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PWM Delay Generation Circuit bypass for channel 0
    #[inline(always)]
    pub fn dlybs0(&mut self) -> DLYBS0_W<GTDLYCR2_SPEC> {
        DLYBS0_W::new(self, 0)
    }
    ///Bit 1 - PWM Delay Generation Circuit bypass for channel 1
    #[inline(always)]
    pub fn dlybs1(&mut self) -> DLYBS1_W<GTDLYCR2_SPEC> {
        DLYBS1_W::new(self, 1)
    }
    ///Bit 2 - PWM Delay Generation Circuit bypass for channel 2
    #[inline(always)]
    pub fn dlybs2(&mut self) -> DLYBS2_W<GTDLYCR2_SPEC> {
        DLYBS2_W::new(self, 2)
    }
    ///Bit 3 - PWM Delay Generation Circuit bypass for channel 3
    #[inline(always)]
    pub fn dlybs3(&mut self) -> DLYBS3_W<GTDLYCR2_SPEC> {
        DLYBS3_W::new(self, 3)
    }
    ///Bit 8 - PWM Delay Generation Circuit enable for channel 0
    #[inline(always)]
    pub fn dlyen0(&mut self) -> DLYEN0_W<GTDLYCR2_SPEC> {
        DLYEN0_W::new(self, 8)
    }
    ///Bit 9 - PWM Delay Generation Circuit enable for channel 1
    #[inline(always)]
    pub fn dlyen1(&mut self) -> DLYEN1_W<GTDLYCR2_SPEC> {
        DLYEN1_W::new(self, 9)
    }
    ///Bit 10 - PWM Delay Generation Circuit enable for channel 2
    #[inline(always)]
    pub fn dlyen2(&mut self) -> DLYEN2_W<GTDLYCR2_SPEC> {
        DLYEN2_W::new(self, 10)
    }
    ///Bit 11 - PWM Delay Generation Circuit enable for channel 3
    #[inline(always)]
    pub fn dlyen3(&mut self) -> DLYEN3_W<GTDLYCR2_SPEC> {
        DLYEN3_W::new(self, 11)
    }
}
/**PWM Output Delay Control Register2

You can [`read`](crate::Reg::read) this register and get [`gtdlycr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdlycr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GTDLYCR2_SPEC;
impl crate::RegisterSpec for GTDLYCR2_SPEC {
    type Ux = u16;
}
///`read()` method returns [`gtdlycr2::R`](R) reader structure
impl crate::Readable for GTDLYCR2_SPEC {}
///`write(|w| ..)` method takes [`gtdlycr2::W`](W) writer structure
impl crate::Writable for GTDLYCR2_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GTDLYCR2 to value 0
impl crate::Resettable for GTDLYCR2_SPEC {}
