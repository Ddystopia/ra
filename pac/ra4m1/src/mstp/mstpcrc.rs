///Register `MSTPCRC` reader
pub type R = crate::R<MSTPCRC_SPEC>;
///Register `MSTPCRC` writer
pub type W = crate::W<MSTPCRC_SPEC>;
/**Clock Frequency Accuracy Measurement Circuit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC0_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC0_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC0` reader - Clock Frequency Accuracy Measurement Circuit Module Stop
pub type MSTPC0_R = crate::BitReader<MSTPC0_A>;
impl MSTPC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC0_A {
        match self.bits {
            false => MSTPC0_A::_0,
            true => MSTPC0_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC0_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC0_A::_1
    }
}
///Field `MSTPC0` writer - Clock Frequency Accuracy Measurement Circuit Module Stop
pub type MSTPC0_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC0_A>;
impl<'a, REG> MSTPC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC0_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC0_A::_1)
    }
}
/**Cyclic Redundancy Check Calculator Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC1_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC1` reader - Cyclic Redundancy Check Calculator Module Stop
pub type MSTPC1_R = crate::BitReader<MSTPC1_A>;
impl MSTPC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC1_A {
        match self.bits {
            false => MSTPC1_A::_0,
            true => MSTPC1_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC1_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC1_A::_1
    }
}
///Field `MSTPC1` writer - Cyclic Redundancy Check Calculator Module Stop
pub type MSTPC1_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC1_A>;
impl<'a, REG> MSTPC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC1_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC1_A::_1)
    }
}
/**Capacitive Touch Sensing Unit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC3_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC3_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC3` reader - Capacitive Touch Sensing Unit Module Stop
pub type MSTPC3_R = crate::BitReader<MSTPC3_A>;
impl MSTPC3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC3_A {
        match self.bits {
            false => MSTPC3_A::_0,
            true => MSTPC3_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC3_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC3_A::_1
    }
}
///Field `MSTPC3` writer - Capacitive Touch Sensing Unit Module Stop
pub type MSTPC3_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC3_A>;
impl<'a, REG> MSTPC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC3_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC3_A::_1)
    }
}
/**Segment LCD Controller Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC4_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC4_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC4` reader - Segment LCD Controller Module Stop
pub type MSTPC4_R = crate::BitReader<MSTPC4_A>;
impl MSTPC4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC4_A {
        match self.bits {
            false => MSTPC4_A::_0,
            true => MSTPC4_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC4_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC4_A::_1
    }
}
///Field `MSTPC4` writer - Segment LCD Controller Module Stop
pub type MSTPC4_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC4_A>;
impl<'a, REG> MSTPC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC4_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC4_A::_1)
    }
}
/**Synchronous Serial Interface 0 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC8_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC8_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC8_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC8` reader - Synchronous Serial Interface 0 Module Stop
pub type MSTPC8_R = crate::BitReader<MSTPC8_A>;
impl MSTPC8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC8_A {
        match self.bits {
            false => MSTPC8_A::_0,
            true => MSTPC8_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC8_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC8_A::_1
    }
}
///Field `MSTPC8` writer - Synchronous Serial Interface 0 Module Stop
pub type MSTPC8_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC8_A>;
impl<'a, REG> MSTPC8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC8_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC8_A::_1)
    }
}
/**Data Operation Circuit Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC13_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC13_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC13_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC13` reader - Data Operation Circuit Module Stop
pub type MSTPC13_R = crate::BitReader<MSTPC13_A>;
impl MSTPC13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC13_A {
        match self.bits {
            false => MSTPC13_A::_0,
            true => MSTPC13_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC13_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC13_A::_1
    }
}
///Field `MSTPC13` writer - Data Operation Circuit Module Stop
pub type MSTPC13_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC13_A>;
impl<'a, REG> MSTPC13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC13_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC13_A::_1)
    }
}
/**Event Link Controller Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC14_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC14_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC14_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC14` reader - Event Link Controller Module Stop
pub type MSTPC14_R = crate::BitReader<MSTPC14_A>;
impl MSTPC14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC14_A {
        match self.bits {
            false => MSTPC14_A::_0,
            true => MSTPC14_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC14_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC14_A::_1
    }
}
///Field `MSTPC14` writer - Event Link Controller Module Stop
pub type MSTPC14_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC14_A>;
impl<'a, REG> MSTPC14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC14_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC14_A::_1)
    }
}
/**SCE5 Module Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPC31_A {
    ///0: Cancel the module-stop state
    _0 = 0,
    ///1: Enter the module-stop state
    _1 = 1,
}
impl From<MSTPC31_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPC31_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTPC31` reader - SCE5 Module Stop
pub type MSTPC31_R = crate::BitReader<MSTPC31_A>;
impl MSTPC31_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTPC31_A {
        match self.bits {
            false => MSTPC31_A::_0,
            true => MSTPC31_A::_1,
        }
    }
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTPC31_A::_0
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTPC31_A::_1
    }
}
///Field `MSTPC31` writer - SCE5 Module Stop
pub type MSTPC31_W<'a, REG> = crate::BitWriter<'a, REG, MSTPC31_A>;
impl<'a, REG> MSTPC31_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Cancel the module-stop state
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC31_A::_0)
    }
    ///Enter the module-stop state
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTPC31_A::_1)
    }
}
impl R {
    ///Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop
    #[inline(always)]
    pub fn mstpc0(&self) -> MSTPC0_R {
        MSTPC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Cyclic Redundancy Check Calculator Module Stop
    #[inline(always)]
    pub fn mstpc1(&self) -> MSTPC1_R {
        MSTPC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Capacitive Touch Sensing Unit Module Stop
    #[inline(always)]
    pub fn mstpc3(&self) -> MSTPC3_R {
        MSTPC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Segment LCD Controller Module Stop
    #[inline(always)]
    pub fn mstpc4(&self) -> MSTPC4_R {
        MSTPC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Synchronous Serial Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpc8(&self) -> MSTPC8_R {
        MSTPC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 13 - Data Operation Circuit Module Stop
    #[inline(always)]
    pub fn mstpc13(&self) -> MSTPC13_R {
        MSTPC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Event Link Controller Module Stop
    #[inline(always)]
    pub fn mstpc14(&self) -> MSTPC14_R {
        MSTPC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 31 - SCE5 Module Stop
    #[inline(always)]
    pub fn mstpc31(&self) -> MSTPC31_R {
        MSTPC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop
    #[inline(always)]
    pub fn mstpc0(&mut self) -> MSTPC0_W<MSTPCRC_SPEC> {
        MSTPC0_W::new(self, 0)
    }
    ///Bit 1 - Cyclic Redundancy Check Calculator Module Stop
    #[inline(always)]
    pub fn mstpc1(&mut self) -> MSTPC1_W<MSTPCRC_SPEC> {
        MSTPC1_W::new(self, 1)
    }
    ///Bit 3 - Capacitive Touch Sensing Unit Module Stop
    #[inline(always)]
    pub fn mstpc3(&mut self) -> MSTPC3_W<MSTPCRC_SPEC> {
        MSTPC3_W::new(self, 3)
    }
    ///Bit 4 - Segment LCD Controller Module Stop
    #[inline(always)]
    pub fn mstpc4(&mut self) -> MSTPC4_W<MSTPCRC_SPEC> {
        MSTPC4_W::new(self, 4)
    }
    ///Bit 8 - Synchronous Serial Interface 0 Module Stop
    #[inline(always)]
    pub fn mstpc8(&mut self) -> MSTPC8_W<MSTPCRC_SPEC> {
        MSTPC8_W::new(self, 8)
    }
    ///Bit 13 - Data Operation Circuit Module Stop
    #[inline(always)]
    pub fn mstpc13(&mut self) -> MSTPC13_W<MSTPCRC_SPEC> {
        MSTPC13_W::new(self, 13)
    }
    ///Bit 14 - Event Link Controller Module Stop
    #[inline(always)]
    pub fn mstpc14(&mut self) -> MSTPC14_W<MSTPCRC_SPEC> {
        MSTPC14_W::new(self, 14)
    }
    ///Bit 31 - SCE5 Module Stop
    #[inline(always)]
    pub fn mstpc31(&mut self) -> MSTPC31_W<MSTPCRC_SPEC> {
        MSTPC31_W::new(self, 31)
    }
}
/**Module Stop Control Register C

You can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MSTPCRC_SPEC;
impl crate::RegisterSpec for MSTPCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mstpcrc::R`](R) reader structure
impl crate::Readable for MSTPCRC_SPEC {}
///`write(|w| ..)` method takes [`mstpcrc::W`](W) writer structure
impl crate::Writable for MSTPCRC_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSTPCRC to value 0xffff_ffff
impl crate::Resettable for MSTPCRC_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
