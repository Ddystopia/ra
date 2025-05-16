///Register `DPSIER1` reader
pub type R = crate::R<DPSIER1_SPEC>;
///Register `DPSIER1` writer
pub type W = crate::W<DPSIER1_SPEC>;
/**IRQ8-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ8E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ8E` reader - IRQ8-DS Pin Enable
pub type DIRQ8E_R = crate::BitReader<DIRQ8E_A>;
impl DIRQ8E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ8E_A {
        match self.bits {
            false => DIRQ8E_A::_0,
            true => DIRQ8E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8E_A::_1
    }
}
///Field `DIRQ8E` writer - IRQ8-DS Pin Enable
pub type DIRQ8E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ8E_A>;
impl<'a, REG> DIRQ8E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ8E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ8E_A::_1)
    }
}
/**IRQ9-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ9E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ9E` reader - IRQ9-DS Pin Enable
pub type DIRQ9E_R = crate::BitReader<DIRQ9E_A>;
impl DIRQ9E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ9E_A {
        match self.bits {
            false => DIRQ9E_A::_0,
            true => DIRQ9E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9E_A::_1
    }
}
///Field `DIRQ9E` writer - IRQ9-DS Pin Enable
pub type DIRQ9E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ9E_A>;
impl<'a, REG> DIRQ9E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ9E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ9E_A::_1)
    }
}
/**IRQ10-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ10E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ10E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ10E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ10E` reader - IRQ10-DS Pin Enable
pub type DIRQ10E_R = crate::BitReader<DIRQ10E_A>;
impl DIRQ10E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ10E_A {
        match self.bits {
            false => DIRQ10E_A::_0,
            true => DIRQ10E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ10E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ10E_A::_1
    }
}
///Field `DIRQ10E` writer - IRQ10-DS Pin Enable
pub type DIRQ10E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ10E_A>;
impl<'a, REG> DIRQ10E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ10E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ10E_A::_1)
    }
}
/**IRQ11-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ11E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ11E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ11E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ11E` reader - IRQ11-DS Pin Enable
pub type DIRQ11E_R = crate::BitReader<DIRQ11E_A>;
impl DIRQ11E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ11E_A {
        match self.bits {
            false => DIRQ11E_A::_0,
            true => DIRQ11E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ11E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ11E_A::_1
    }
}
///Field `DIRQ11E` writer - IRQ11-DS Pin Enable
pub type DIRQ11E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ11E_A>;
impl<'a, REG> DIRQ11E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ11E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ11E_A::_1)
    }
}
/**IRQ12-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ12E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ12E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ12E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ12E` reader - IRQ12-DS Pin Enable
pub type DIRQ12E_R = crate::BitReader<DIRQ12E_A>;
impl DIRQ12E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ12E_A {
        match self.bits {
            false => DIRQ12E_A::_0,
            true => DIRQ12E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ12E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ12E_A::_1
    }
}
///Field `DIRQ12E` writer - IRQ12-DS Pin Enable
pub type DIRQ12E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ12E_A>;
impl<'a, REG> DIRQ12E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ12E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ12E_A::_1)
    }
}
/**IRQ13-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ13E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ13E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ13E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ13E` reader - IRQ13-DS Pin Enable
pub type DIRQ13E_R = crate::BitReader<DIRQ13E_A>;
impl DIRQ13E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ13E_A {
        match self.bits {
            false => DIRQ13E_A::_0,
            true => DIRQ13E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ13E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ13E_A::_1
    }
}
///Field `DIRQ13E` writer - IRQ13-DS Pin Enable
pub type DIRQ13E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ13E_A>;
impl<'a, REG> DIRQ13E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ13E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ13E_A::_1)
    }
}
/**IRQ14-DS Pin Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ14E_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DIRQ14E_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ14E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRQ14E` reader - IRQ14-DS Pin Enable
pub type DIRQ14E_R = crate::BitReader<DIRQ14E_A>;
impl DIRQ14E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ14E_A {
        match self.bits {
            false => DIRQ14E_A::_0,
            true => DIRQ14E_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ14E_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ14E_A::_1
    }
}
///Field `DIRQ14E` writer - IRQ14-DS Pin Enable
pub type DIRQ14E_W<'a, REG> = crate::BitWriter<'a, REG, DIRQ14E_A>;
impl<'a, REG> DIRQ14E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ14E_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ14E_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ8-DS Pin Enable
    #[inline(always)]
    pub fn dirq8e(&self) -> DIRQ8E_R {
        DIRQ8E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ9-DS Pin Enable
    #[inline(always)]
    pub fn dirq9e(&self) -> DIRQ9E_R {
        DIRQ9E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ10-DS Pin Enable
    #[inline(always)]
    pub fn dirq10e(&self) -> DIRQ10E_R {
        DIRQ10E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ11-DS Pin Enable
    #[inline(always)]
    pub fn dirq11e(&self) -> DIRQ11E_R {
        DIRQ11E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ12-DS Pin Enable
    #[inline(always)]
    pub fn dirq12e(&self) -> DIRQ12E_R {
        DIRQ12E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ13-DS Pin Enable
    #[inline(always)]
    pub fn dirq13e(&self) -> DIRQ13E_R {
        DIRQ13E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ14-DS Pin Enable
    #[inline(always)]
    pub fn dirq14e(&self) -> DIRQ14E_R {
        DIRQ14E_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ8-DS Pin Enable
    #[inline(always)]
    pub fn dirq8e(&mut self) -> DIRQ8E_W<DPSIER1_SPEC> {
        DIRQ8E_W::new(self, 0)
    }
    ///Bit 1 - IRQ9-DS Pin Enable
    #[inline(always)]
    pub fn dirq9e(&mut self) -> DIRQ9E_W<DPSIER1_SPEC> {
        DIRQ9E_W::new(self, 1)
    }
    ///Bit 2 - IRQ10-DS Pin Enable
    #[inline(always)]
    pub fn dirq10e(&mut self) -> DIRQ10E_W<DPSIER1_SPEC> {
        DIRQ10E_W::new(self, 2)
    }
    ///Bit 3 - IRQ11-DS Pin Enable
    #[inline(always)]
    pub fn dirq11e(&mut self) -> DIRQ11E_W<DPSIER1_SPEC> {
        DIRQ11E_W::new(self, 3)
    }
    ///Bit 4 - IRQ12-DS Pin Enable
    #[inline(always)]
    pub fn dirq12e(&mut self) -> DIRQ12E_W<DPSIER1_SPEC> {
        DIRQ12E_W::new(self, 4)
    }
    ///Bit 5 - IRQ13-DS Pin Enable
    #[inline(always)]
    pub fn dirq13e(&mut self) -> DIRQ13E_W<DPSIER1_SPEC> {
        DIRQ13E_W::new(self, 5)
    }
    ///Bit 6 - IRQ14-DS Pin Enable
    #[inline(always)]
    pub fn dirq14e(&mut self) -> DIRQ14E_W<DPSIER1_SPEC> {
        DIRQ14E_W::new(self, 6)
    }
}
/**Deep Standby Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIER1_SPEC;
impl crate::RegisterSpec for DPSIER1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsier1::R`](R) reader structure
impl crate::Readable for DPSIER1_SPEC {}
///`write(|w| ..)` method takes [`dpsier1::W`](W) writer structure
impl crate::Writable for DPSIER1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER1 to value 0
impl crate::Resettable for DPSIER1_SPEC {}
