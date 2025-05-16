///Register `DPSIEGR0` reader
pub type R = crate::R<DPSIEGR0_SPEC>;
///Register `DPSIEGR0` writer
pub type W = crate::W<DPSIEGR0_SPEC>;
/**IRQ0-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ0EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ0EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ0EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ0EG` reader - IRQ0-DS Pin Edge Select
pub type DIRQ0EG_R = crate::BitReader<DIRQ0EG_A>;
impl DIRQ0EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ0EG_A {
        match self.bits {
            false => DIRQ0EG_A::_0,
            true => DIRQ0EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ0EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ0EG_A::_1
    }
}
///Field `DIRQ0EG` writer - IRQ0-DS Pin Edge Select
pub type DIRQ0EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ0EG_A>;
impl<'a, REG> DIRQ0EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ0EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ0EG_A::_1)
    }
}
/**IRQ1-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ1EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ1EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ1EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ1EG` reader - IRQ1-DS Pin Edge Select
pub type DIRQ1EG_R = crate::BitReader<DIRQ1EG_A>;
impl DIRQ1EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ1EG_A {
        match self.bits {
            false => DIRQ1EG_A::_0,
            true => DIRQ1EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ1EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ1EG_A::_1
    }
}
///Field `DIRQ1EG` writer - IRQ1-DS Pin Edge Select
pub type DIRQ1EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ1EG_A>;
impl<'a, REG> DIRQ1EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ1EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ1EG_A::_1)
    }
}
/**IRQ2-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ2EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ2EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ2EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ2EG` reader - IRQ2-DS Pin Edge Select
pub type DIRQ2EG_R = crate::BitReader<DIRQ2EG_A>;
impl DIRQ2EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ2EG_A {
        match self.bits {
            false => DIRQ2EG_A::_0,
            true => DIRQ2EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ2EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ2EG_A::_1
    }
}
///Field `DIRQ2EG` writer - IRQ2-DS Pin Edge Select
pub type DIRQ2EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ2EG_A>;
impl<'a, REG> DIRQ2EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ2EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ2EG_A::_1)
    }
}
/**IRQ3-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ3EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ3EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ3EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ3EG` reader - IRQ3-DS Pin Edge Select
pub type DIRQ3EG_R = crate::BitReader<DIRQ3EG_A>;
impl DIRQ3EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ3EG_A {
        match self.bits {
            false => DIRQ3EG_A::_0,
            true => DIRQ3EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ3EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ3EG_A::_1
    }
}
///Field `DIRQ3EG` writer - IRQ3-DS Pin Edge Select
pub type DIRQ3EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ3EG_A>;
impl<'a, REG> DIRQ3EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ3EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ3EG_A::_1)
    }
}
/**IRQ4-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ4EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ4EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ4EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ4EG` reader - IRQ4-DS Pin Edge Select
pub type DIRQ4EG_R = crate::BitReader<DIRQ4EG_A>;
impl DIRQ4EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ4EG_A {
        match self.bits {
            false => DIRQ4EG_A::_0,
            true => DIRQ4EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ4EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ4EG_A::_1
    }
}
///Field `DIRQ4EG` writer - IRQ4-DS Pin Edge Select
pub type DIRQ4EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ4EG_A>;
impl<'a, REG> DIRQ4EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ4EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ4EG_A::_1)
    }
}
/**IRQ5-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ5EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ5EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ5EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ5EG` reader - IRQ5-DS Pin Edge Select
pub type DIRQ5EG_R = crate::BitReader<DIRQ5EG_A>;
impl DIRQ5EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ5EG_A {
        match self.bits {
            false => DIRQ5EG_A::_0,
            true => DIRQ5EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ5EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ5EG_A::_1
    }
}
///Field `DIRQ5EG` writer - IRQ5-DS Pin Edge Select
pub type DIRQ5EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ5EG_A>;
impl<'a, REG> DIRQ5EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ5EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ5EG_A::_1)
    }
}
/**IRQ6-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ6EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ6EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ6EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ6EG` reader - IRQ6-DS Pin Edge Select
pub type DIRQ6EG_R = crate::BitReader<DIRQ6EG_A>;
impl DIRQ6EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ6EG_A {
        match self.bits {
            false => DIRQ6EG_A::_0,
            true => DIRQ6EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ6EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ6EG_A::_1
    }
}
///Field `DIRQ6EG` writer - IRQ6-DS Pin Edge Select
pub type DIRQ6EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ6EG_A>;
impl<'a, REG> DIRQ6EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ6EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ6EG_A::_1)
    }
}
/**IRQ7-DS Pin Edge Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ7EG_A {
    ///0: A cancel request is generated at a falling edge
    _0 = 0,
    ///1: A cancel request is generated at a rising edge
    _1 = 1,
}
impl From<DIRQ7EG_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ7EG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ7EG` reader - IRQ7-DS Pin Edge Select
pub type DIRQ7EG_R = crate::BitReader<DIRQ7EG_A>;
impl DIRQ7EG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ7EG_A {
        match self.bits {
            false => DIRQ7EG_A::_0,
            true => DIRQ7EG_A::_1,
        }
    }
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ7EG_A::_0
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ7EG_A::_1
    }
}
///Field `DIRQ7EG` writer - IRQ7-DS Pin Edge Select
pub type DIRQ7EG_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ7EG_A>;
impl<'a, REG> DIRQ7EG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A cancel request is generated at a falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ7EG_A::_0)
    }
    ///A cancel request is generated at a rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ7EG_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq0eg(&self) -> DIRQ0EG_R {
        DIRQ0EG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq1eg(&self) -> DIRQ1EG_R {
        DIRQ1EG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq2eg(&self) -> DIRQ2EG_R {
        DIRQ2EG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq3eg(&self) -> DIRQ3EG_R {
        DIRQ3EG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq4eg(&self) -> DIRQ4EG_R {
        DIRQ4EG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq5eg(&self) -> DIRQ5EG_R {
        DIRQ5EG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq6eg(&self) -> DIRQ6EG_R {
        DIRQ6EG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq7eg(&self) -> DIRQ7EG_R {
        DIRQ7EG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ0-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq0eg(&mut self) -> DIRQ0EG_W<DPSIEGR0_SPEC> {
        DIRQ0EG_W::new(self, 0)
    }
    ///Bit 1 - IRQ1-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq1eg(&mut self) -> DIRQ1EG_W<DPSIEGR0_SPEC> {
        DIRQ1EG_W::new(self, 1)
    }
    ///Bit 2 - IRQ2-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq2eg(&mut self) -> DIRQ2EG_W<DPSIEGR0_SPEC> {
        DIRQ2EG_W::new(self, 2)
    }
    ///Bit 3 - IRQ3-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq3eg(&mut self) -> DIRQ3EG_W<DPSIEGR0_SPEC> {
        DIRQ3EG_W::new(self, 3)
    }
    ///Bit 4 - IRQ4-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq4eg(&mut self) -> DIRQ4EG_W<DPSIEGR0_SPEC> {
        DIRQ4EG_W::new(self, 4)
    }
    ///Bit 5 - IRQ5-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq5eg(&mut self) -> DIRQ5EG_W<DPSIEGR0_SPEC> {
        DIRQ5EG_W::new(self, 5)
    }
    ///Bit 6 - IRQ6-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq6eg(&mut self) -> DIRQ6EG_W<DPSIEGR0_SPEC> {
        DIRQ6EG_W::new(self, 6)
    }
    ///Bit 7 - IRQ7-DS Pin Edge Select
    #[inline(always)]
    pub fn dirq7eg(&mut self) -> DIRQ7EG_W<DPSIEGR0_SPEC> {
        DIRQ7EG_W::new(self, 7)
    }
}
/**Deep Standby Interrupt Edge Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsiegr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIEGR0_SPEC;
impl crate::RegisterSpec for DPSIEGR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsiegr0::R`](R) reader structure
impl crate::Readable for DPSIEGR0_SPEC {}
///`write(|w| ..)` method takes [`dpsiegr0::W`](W) writer structure
impl crate::Writable for DPSIEGR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIEGR0 to value 0
impl crate::Resettable for DPSIEGR0_SPEC {}
