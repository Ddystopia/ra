///Register `DPSIEGR1` reader
pub type R = crate::R<DPSIEGR1_SPEC>;
///Register `DPSIEGR1` writer
pub type W = crate::W<DPSIEGR1_SPEC>;
/**IRQ8-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ8EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ8EG` reader - IRQ8-DS Pin Edge Select
pub type DIRQ8EG_R = crate::BitReader<DIRQ8EG_A>;
impl DIRQ8EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ8EG_A {
        match self.bits {
            false => DIRQ8EG_A::_0,
            true => DIRQ8EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8EG_A::_1
    }
}
///Field `DIRQ8EG` writer - IRQ8-DS Pin Edge Select
pub type DIRQ8EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ8EG_A>;
impl<'a, REG> DIRQ8EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ8EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ8EG_A::_1)
    }
}
/**IRQ9-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ9EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ9EG` reader - IRQ9-DS Pin Edge Select
pub type DIRQ9EG_R = crate::BitReader<DIRQ9EG_A>;
impl DIRQ9EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ9EG_A {
        match self.bits {
            false => DIRQ9EG_A::_0,
            true => DIRQ9EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9EG_A::_1
    }
}
///Field `DIRQ9EG` writer - IRQ9-DS Pin Edge Select
pub type DIRQ9EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ9EG_A>;
impl<'a, REG> DIRQ9EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ9EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ9EG_A::_1)
    }
}
/**IRQ10-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ10EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ10EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ10EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ10EG` reader - IRQ10-DS Pin Edge Select
pub type DIRQ10EG_R = crate::BitReader<DIRQ10EG_A>;
impl DIRQ10EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ10EG_A {
        match self.bits {
            false => DIRQ10EG_A::_0,
            true => DIRQ10EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ10EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ10EG_A::_1
    }
}
///Field `DIRQ10EG` writer - IRQ10-DS Pin Edge Select
pub type DIRQ10EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ10EG_A>;
impl<'a, REG> DIRQ10EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ10EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ10EG_A::_1)
    }
}
/**IRQ11-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ11EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ11EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ11EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ11EG` reader - IRQ11-DS Pin Edge Select
pub type DIRQ11EG_R = crate::BitReader<DIRQ11EG_A>;
impl DIRQ11EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ11EG_A {
        match self.bits {
            false => DIRQ11EG_A::_0,
            true => DIRQ11EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ11EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ11EG_A::_1
    }
}
///Field `DIRQ11EG` writer - IRQ11-DS Pin Edge Select
pub type DIRQ11EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ11EG_A>;
impl<'a, REG> DIRQ11EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ11EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ11EG_A::_1)
    }
}
/**IRQ12-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ12EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ12EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ12EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ12EG` reader - IRQ12-DS Pin Edge Select
pub type DIRQ12EG_R = crate::BitReader<DIRQ12EG_A>;
impl DIRQ12EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ12EG_A {
        match self.bits {
            false => DIRQ12EG_A::_0,
            true => DIRQ12EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ12EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ12EG_A::_1
    }
}
///Field `DIRQ12EG` writer - IRQ12-DS Pin Edge Select
pub type DIRQ12EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ12EG_A>;
impl<'a, REG> DIRQ12EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ12EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ12EG_A::_1)
    }
}
/**IRQ13-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ13EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ13EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ13EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ13EG` reader - IRQ13-DS Pin Edge Select
pub type DIRQ13EG_R = crate::BitReader<DIRQ13EG_A>;
impl DIRQ13EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ13EG_A {
        match self.bits {
            false => DIRQ13EG_A::_0,
            true => DIRQ13EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ13EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ13EG_A::_1
    }
}
///Field `DIRQ13EG` writer - IRQ13-DS Pin Edge Select
pub type DIRQ13EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ13EG_A>;
impl<'a, REG> DIRQ13EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ13EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ13EG_A::_1)
    }
}
/**IRQ14-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ14EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ14EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ14EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ14EG` reader - IRQ14-DS Pin Edge Select
pub type DIRQ14EG_R = crate::BitReader<DIRQ14EG_A>;
impl DIRQ14EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ14EG_A {
        match self.bits {
            false => DIRQ14EG_A::_0,
            true => DIRQ14EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ14EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ14EG_A::_1
    }
}
///Field `DIRQ14EG` writer - IRQ14-DS Pin Edge Select
pub type DIRQ14EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ14EG_A>;
impl<'a, REG> DIRQ14EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ14EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ14EG_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ8-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq8eg(&self) -> DIRQ8EG_R {
        DIRQ8EG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ9-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq9eg(&self) -> DIRQ9EG_R {
        DIRQ9EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ10-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq10eg(&self) -> DIRQ10EG_R {
        DIRQ10EG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ11-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq11eg(&self) -> DIRQ11EG_R {
        DIRQ11EG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ12-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq12eg(&self) -> DIRQ12EG_R {
        DIRQ12EG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ13-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq13eg(&self) -> DIRQ13EG_R {
        DIRQ13EG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ14-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq14eg(&self) -> DIRQ14EG_R {
        DIRQ14EG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ8-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq8eg(&mut self) -> DIRQ8EG_W<DPSIEGR1_SPEC> {
        DIRQ8EG_W::new(self, 0)
    }
    ///Bit 1 - IRQ9-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq9eg(&mut self) -> DIRQ9EG_W<DPSIEGR1_SPEC> {
        DIRQ9EG_W::new(self, 1)
    }
    ///Bit 2 - IRQ10-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq10eg(&mut self) -> DIRQ10EG_W<DPSIEGR1_SPEC> {
        DIRQ10EG_W::new(self, 2)
    }
    ///Bit 3 - IRQ11-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq11eg(&mut self) -> DIRQ11EG_W<DPSIEGR1_SPEC> {
        DIRQ11EG_W::new(self, 3)
    }
    ///Bit 4 - IRQ12-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq12eg(&mut self) -> DIRQ12EG_W<DPSIEGR1_SPEC> {
        DIRQ12EG_W::new(self, 4)
    }
    ///Bit 5 - IRQ13-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq13eg(&mut self) -> DIRQ13EG_W<DPSIEGR1_SPEC> {
        DIRQ13EG_W::new(self, 5)
    }
    ///Bit 6 - IRQ14-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq14eg(&mut self) -> DIRQ14EG_W<DPSIEGR1_SPEC> {
        DIRQ14EG_W::new(self, 6)
    }
}
/**Deep Standby Interrupt Edge Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsiegr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIEGR1_SPEC;
impl crate::RegisterSpec for DPSIEGR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsiegr1::R`](R) reader structure
impl crate::Readable for DPSIEGR1_SPEC {}
///`write(|w| ..)` method takes [`dpsiegr1::W`](W) writer structure
impl crate::Writable for DPSIEGR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIEGR1 to value 0
impl crate::Resettable for DPSIEGR1_SPEC {}
