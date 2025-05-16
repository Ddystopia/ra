///Register `DPSIER0` reader
pub type R = crate::R<DPSIER0_SPEC>;
///Register `DPSIER0` writer
pub type W = crate::W<DPSIER0_SPEC>;
/**IRQ0-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ0E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ0E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ0E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ0E` reader - IRQ0-DS Pin Enable
pub type DIRQ0E_R = crate::BitReader<DIRQ0E_A>;
impl DIRQ0E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ0E_A {
        match self.bits {
            false => DIRQ0E_A::_0,
            true => DIRQ0E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ0E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ0E_A::_1
    }
}
///Field `DIRQ0E` writer - IRQ0-DS Pin Enable
pub type DIRQ0E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ0E_A>;
impl<'a, REG> DIRQ0E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ0E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ0E_A::_1)
    }
}
/**IRQ1-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ1E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ1E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ1E` reader - IRQ1-DS Pin Enable
pub type DIRQ1E_R = crate::BitReader<DIRQ1E_A>;
impl DIRQ1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ1E_A {
        match self.bits {
            false => DIRQ1E_A::_0,
            true => DIRQ1E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ1E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ1E_A::_1
    }
}
///Field `DIRQ1E` writer - IRQ1-DS Pin Enable
pub type DIRQ1E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ1E_A>;
impl<'a, REG> DIRQ1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ1E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ1E_A::_1)
    }
}
/**IRQ2-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ2E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ2E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ2E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ2E` reader - IRQ2-DS Pin Enable
pub type DIRQ2E_R = crate::BitReader<DIRQ2E_A>;
impl DIRQ2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ2E_A {
        match self.bits {
            false => DIRQ2E_A::_0,
            true => DIRQ2E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ2E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ2E_A::_1
    }
}
///Field `DIRQ2E` writer - IRQ2-DS Pin Enable
pub type DIRQ2E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ2E_A>;
impl<'a, REG> DIRQ2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ2E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ2E_A::_1)
    }
}
/**IRQ3-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ3E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ3E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ3E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ3E` reader - IRQ3-DS Pin Enable
pub type DIRQ3E_R = crate::BitReader<DIRQ3E_A>;
impl DIRQ3E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ3E_A {
        match self.bits {
            false => DIRQ3E_A::_0,
            true => DIRQ3E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ3E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ3E_A::_1
    }
}
///Field `DIRQ3E` writer - IRQ3-DS Pin Enable
pub type DIRQ3E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ3E_A>;
impl<'a, REG> DIRQ3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ3E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ3E_A::_1)
    }
}
/**IRQ4-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ4E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ4E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ4E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ4E` reader - IRQ4-DS Pin Enable
pub type DIRQ4E_R = crate::BitReader<DIRQ4E_A>;
impl DIRQ4E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ4E_A {
        match self.bits {
            false => DIRQ4E_A::_0,
            true => DIRQ4E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ4E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ4E_A::_1
    }
}
///Field `DIRQ4E` writer - IRQ4-DS Pin Enable
pub type DIRQ4E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ4E_A>;
impl<'a, REG> DIRQ4E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ4E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ4E_A::_1)
    }
}
/**IRQ5-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ5E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ5E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ5E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ5E` reader - IRQ5-DS Pin Enable
pub type DIRQ5E_R = crate::BitReader<DIRQ5E_A>;
impl DIRQ5E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ5E_A {
        match self.bits {
            false => DIRQ5E_A::_0,
            true => DIRQ5E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ5E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ5E_A::_1
    }
}
///Field `DIRQ5E` writer - IRQ5-DS Pin Enable
pub type DIRQ5E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ5E_A>;
impl<'a, REG> DIRQ5E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ5E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ5E_A::_1)
    }
}
/**IRQ6-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ6E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ6E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ6E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ6E` reader - IRQ6-DS Pin Enable
pub type DIRQ6E_R = crate::BitReader<DIRQ6E_A>;
impl DIRQ6E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ6E_A {
        match self.bits {
            false => DIRQ6E_A::_0,
            true => DIRQ6E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ6E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ6E_A::_1
    }
}
///Field `DIRQ6E` writer - IRQ6-DS Pin Enable
pub type DIRQ6E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ6E_A>;
impl<'a, REG> DIRQ6E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ6E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ6E_A::_1)
    }
}
/**IRQ7-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ7E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ7E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ7E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ7E` reader - IRQ7-DS Pin Enable
pub type DIRQ7E_R = crate::BitReader<DIRQ7E_A>;
impl DIRQ7E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ7E_A {
        match self.bits {
            false => DIRQ7E_A::_0,
            true => DIRQ7E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ7E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ7E_A::_1
    }
}
///Field `DIRQ7E` writer - IRQ7-DS Pin Enable
pub type DIRQ7E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ7E_A>;
impl<'a, REG> DIRQ7E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ7E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ7E_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0-DS Pin Enable
    #[inline(always)]
    pub fn dirq0e(&self) -> DIRQ0E_R {
        DIRQ0E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1-DS Pin Enable
    #[inline(always)]
    pub fn dirq1e(&self) -> DIRQ1E_R {
        DIRQ1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2-DS Pin Enable
    #[inline(always)]
    pub fn dirq2e(&self) -> DIRQ2E_R {
        DIRQ2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3-DS Pin Enable
    #[inline(always)]
    pub fn dirq3e(&self) -> DIRQ3E_R {
        DIRQ3E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4-DS Pin Enable
    #[inline(always)]
    pub fn dirq4e(&self) -> DIRQ4E_R {
        DIRQ4E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5-DS Pin Enable
    #[inline(always)]
    pub fn dirq5e(&self) -> DIRQ5E_R {
        DIRQ5E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6-DS Pin Enable
    #[inline(always)]
    pub fn dirq6e(&self) -> DIRQ6E_R {
        DIRQ6E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7-DS Pin Enable
    #[inline(always)]
    pub fn dirq7e(&self) -> DIRQ7E_R {
        DIRQ7E_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ0-DS Pin Enable
    #[inline(always)]
    pub fn dirq0e(&mut self) -> DIRQ0E_W<DPSIER0_SPEC> {
        DIRQ0E_W::new(self, 0)
    }
    ///Bit 1 - IRQ1-DS Pin Enable
    #[inline(always)]
    pub fn dirq1e(&mut self) -> DIRQ1E_W<DPSIER0_SPEC> {
        DIRQ1E_W::new(self, 1)
    }
    ///Bit 2 - IRQ2-DS Pin Enable
    #[inline(always)]
    pub fn dirq2e(&mut self) -> DIRQ2E_W<DPSIER0_SPEC> {
        DIRQ2E_W::new(self, 2)
    }
    ///Bit 3 - IRQ3-DS Pin Enable
    #[inline(always)]
    pub fn dirq3e(&mut self) -> DIRQ3E_W<DPSIER0_SPEC> {
        DIRQ3E_W::new(self, 3)
    }
    ///Bit 4 - IRQ4-DS Pin Enable
    #[inline(always)]
    pub fn dirq4e(&mut self) -> DIRQ4E_W<DPSIER0_SPEC> {
        DIRQ4E_W::new(self, 4)
    }
    ///Bit 5 - IRQ5-DS Pin Enable
    #[inline(always)]
    pub fn dirq5e(&mut self) -> DIRQ5E_W<DPSIER0_SPEC> {
        DIRQ5E_W::new(self, 5)
    }
    ///Bit 6 - IRQ6-DS Pin Enable
    #[inline(always)]
    pub fn dirq6e(&mut self) -> DIRQ6E_W<DPSIER0_SPEC> {
        DIRQ6E_W::new(self, 6)
    }
    ///Bit 7 - IRQ7-DS Pin Enable
    #[inline(always)]
    pub fn dirq7e(&mut self) -> DIRQ7E_W<DPSIER0_SPEC> {
        DIRQ7E_W::new(self, 7)
    }
}
/**Deep Standby Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsier0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIER0_SPEC;
impl crate::RegisterSpec for DPSIER0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsier0::R`](R) reader structure
impl crate::Readable for DPSIER0_SPEC {}
///`write(|w| ..)` method takes [`dpsier0::W`](W) writer structure
impl crate::Writable for DPSIER0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER0 to value 0
impl crate::Resettable for DPSIER0_SPEC {}
