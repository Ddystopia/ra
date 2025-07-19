///Register `COMPOCR` reader
pub type R = crate::R<COMPOCR_SPEC>;
///Register `COMPOCR` writer
pub type W = crate::W<COMPOCR_SPEC>;
/**ACMPLP0 VCOUT Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0OE_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<C0OE_A> for bool {
    #[inline(always)]
    fn from(variant: C0OE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C0OE` reader - ACMPLP0 VCOUT Pin Output Enable
pub type C0OE_R = crate::BitReader<C0OE_A>;
impl C0OE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C0OE_A {
        match self.bits {
            false => C0OE_A::_0,
            true => C0OE_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0OE_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0OE_A::_1
    }
}
///Field `C0OE` writer - ACMPLP0 VCOUT Pin Output Enable
pub type C0OE_W<'a, REG> = crate::BitWriter<'a, REG, C0OE_A>;
impl<'a, REG> C0OE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0OE_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0OE_A::_1)
    }
}
/**ACMPLP0 VCOUT Output Polarity Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0OP_A {
    ///0: Non inverted
    _0 = 0,
    ///1: Inverted
    _1 = 1,
}
impl From<C0OP_A> for bool {
    #[inline(always)]
    fn from(variant: C0OP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C0OP` reader - ACMPLP0 VCOUT Output Polarity Selection
pub type C0OP_R = crate::BitReader<C0OP_A>;
impl C0OP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C0OP_A {
        match self.bits {
            false => C0OP_A::_0,
            true => C0OP_A::_1,
        }
    }
    ///Non inverted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0OP_A::_0
    }
    ///Inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0OP_A::_1
    }
}
///Field `C0OP` writer - ACMPLP0 VCOUT Output Polarity Selection
pub type C0OP_W<'a, REG> = crate::BitWriter<'a, REG, C0OP_A>;
impl<'a, REG> C0OP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non inverted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0OP_A::_0)
    }
    ///Inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0OP_A::_1)
    }
}
/**ACMPLP1 VCOUT Pin Output Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OE_A {
    ///0: Disabled
    _0 = 0,
    ///1: Enabled
    _1 = 1,
}
impl From<C1OE_A> for bool {
    #[inline(always)]
    fn from(variant: C1OE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1OE` reader - ACMPLP1 VCOUT Pin Output Enable
pub type C1OE_R = crate::BitReader<C1OE_A>;
impl C1OE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1OE_A {
        match self.bits {
            false => C1OE_A::_0,
            true => C1OE_A::_1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1OE_A::_0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1OE_A::_1
    }
}
///Field `C1OE` writer - ACMPLP1 VCOUT Pin Output Enable
pub type C1OE_W<'a, REG> = crate::BitWriter<'a, REG, C1OE_A>;
impl<'a, REG> C1OE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1OE_A::_0)
    }
    ///Enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1OE_A::_1)
    }
}
/**ACMPLP1 VCOUT Output Polarity Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1OP_A {
    ///0: Non inverted
    _0 = 0,
    ///1: Inverted
    _1 = 1,
}
impl From<C1OP_A> for bool {
    #[inline(always)]
    fn from(variant: C1OP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `C1OP` reader - ACMPLP1 VCOUT Output Polarity Selection
pub type C1OP_R = crate::BitReader<C1OP_A>;
impl C1OP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C1OP_A {
        match self.bits {
            false => C1OP_A::_0,
            true => C1OP_A::_1,
        }
    }
    ///Non inverted
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1OP_A::_0
    }
    ///Inverted
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1OP_A::_1
    }
}
///Field `C1OP` writer - ACMPLP1 VCOUT Output Polarity Selection
pub type C1OP_W<'a, REG> = crate::BitWriter<'a, REG, C1OP_A>;
impl<'a, REG> C1OP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non inverted
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1OP_A::_0)
    }
    ///Inverted
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1OP_A::_1)
    }
}
/**ACMPLP0/ACMPLP1 Speed Selection

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPDMD_A {
    ///0: Comparator low-speed mode
    _0 = 0,
    ///1: Comparator high-speed mode
    _1 = 1,
}
impl From<SPDMD_A> for bool {
    #[inline(always)]
    fn from(variant: SPDMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SPDMD` reader - ACMPLP0/ACMPLP1 Speed Selection
pub type SPDMD_R = crate::BitReader<SPDMD_A>;
impl SPDMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPDMD_A {
        match self.bits {
            false => SPDMD_A::_0,
            true => SPDMD_A::_1,
        }
    }
    ///Comparator low-speed mode
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPDMD_A::_0
    }
    ///Comparator high-speed mode
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPDMD_A::_1
    }
}
///Field `SPDMD` writer - ACMPLP0/ACMPLP1 Speed Selection
pub type SPDMD_W<'a, REG> = crate::BitWriter<'a, REG, SPDMD_A>;
impl<'a, REG> SPDMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator low-speed mode
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SPDMD_A::_0)
    }
    ///Comparator high-speed mode
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SPDMD_A::_1)
    }
}
impl R {
    ///Bit 1 - ACMPLP0 VCOUT Pin Output Enable
    #[inline(always)]
    pub fn c0oe(&self) -> C0OE_R {
        C0OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACMPLP0 VCOUT Output Polarity Selection
    #[inline(always)]
    pub fn c0op(&self) -> C0OP_R {
        C0OP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ACMPLP1 VCOUT Pin Output Enable
    #[inline(always)]
    pub fn c1oe(&self) -> C1OE_R {
        C1OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ACMPLP1 VCOUT Output Polarity Selection
    #[inline(always)]
    pub fn c1op(&self) -> C1OP_R {
        C1OP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ACMPLP0/ACMPLP1 Speed Selection
    #[inline(always)]
    pub fn spdmd(&self) -> SPDMD_R {
        SPDMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - ACMPLP0 VCOUT Pin Output Enable
    #[inline(always)]
    pub fn c0oe(&mut self) -> C0OE_W<'_, COMPOCR_SPEC> {
        C0OE_W::new(self, 1)
    }
    ///Bit 2 - ACMPLP0 VCOUT Output Polarity Selection
    #[inline(always)]
    pub fn c0op(&mut self) -> C0OP_W<'_, COMPOCR_SPEC> {
        C0OP_W::new(self, 2)
    }
    ///Bit 5 - ACMPLP1 VCOUT Pin Output Enable
    #[inline(always)]
    pub fn c1oe(&mut self) -> C1OE_W<'_, COMPOCR_SPEC> {
        C1OE_W::new(self, 5)
    }
    ///Bit 6 - ACMPLP1 VCOUT Output Polarity Selection
    #[inline(always)]
    pub fn c1op(&mut self) -> C1OP_W<'_, COMPOCR_SPEC> {
        C1OP_W::new(self, 6)
    }
    ///Bit 7 - ACMPLP0/ACMPLP1 Speed Selection
    #[inline(always)]
    pub fn spdmd(&mut self) -> SPDMD_W<'_, COMPOCR_SPEC> {
        SPDMD_W::new(self, 7)
    }
}
/**ACMPLP Output Control Register

You can [`read`](crate::Reg::read) this register and get [`compocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMPOCR_SPEC;
impl crate::RegisterSpec for COMPOCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`compocr::R`](R) reader structure
impl crate::Readable for COMPOCR_SPEC {}
///`write(|w| ..)` method takes [`compocr::W`](W) writer structure
impl crate::Writable for COMPOCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMPOCR to value 0
impl crate::Resettable for COMPOCR_SPEC {}
