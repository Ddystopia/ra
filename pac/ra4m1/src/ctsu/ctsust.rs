///Register `CTSUST` reader
pub type R = crate::R<CTSUST_SPEC>;
///Register `CTSUST` writer
pub type W = crate::W<CTSUST_SPEC>;
/**CTSU Measurement Status Counter

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUSTC_A {
    ///0: Status 0
    _000 = 0,
    ///1: Status 1
    _001 = 1,
    ///2: Status 2
    _010 = 2,
    ///3: Status 3
    _011 = 3,
    ///4: Status 4
    _100 = 4,
    ///5: Status 5
    _101 = 5,
}
impl From<CTSUSTC_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUSTC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUSTC_A {
    type Ux = u8;
}
impl crate::IsEnum for CTSUSTC_A {}
///Field `CTSUSTC` reader - CTSU Measurement Status Counter
pub type CTSUSTC_R = crate::FieldReader<CTSUSTC_A>;
impl CTSUSTC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUSTC_A> {
        match self.bits {
            0 => Some(CTSUSTC_A::_000),
            1 => Some(CTSUSTC_A::_001),
            2 => Some(CTSUSTC_A::_010),
            3 => Some(CTSUSTC_A::_011),
            4 => Some(CTSUSTC_A::_100),
            5 => Some(CTSUSTC_A::_101),
            _ => None,
        }
    }
    ///Status 0
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CTSUSTC_A::_000
    }
    ///Status 1
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CTSUSTC_A::_001
    }
    ///Status 2
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CTSUSTC_A::_010
    }
    ///Status 3
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CTSUSTC_A::_011
    }
    ///Status 4
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CTSUSTC_A::_100
    }
    ///Status 5
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CTSUSTC_A::_101
    }
}
/**CTSU Data Transfer Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUDTSR_A {
    ///0: Measurement result has been read
    _0 = 0,
    ///1: Measurement result has not been read
    _1 = 1,
}
impl From<CTSUDTSR_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUDTSR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUDTSR` reader - CTSU Data Transfer Status Flag
pub type CTSUDTSR_R = crate::BitReader<CTSUDTSR_A>;
impl CTSUDTSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUDTSR_A {
        match self.bits {
            false => CTSUDTSR_A::_0,
            true => CTSUDTSR_A::_1,
        }
    }
    ///Measurement result has been read
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUDTSR_A::_0
    }
    ///Measurement result has not been read
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUDTSR_A::_1
    }
}
/**CTSU Sensor Counter Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSOVF_A {
    ///0: No overflow
    _0 = 0,
    ///1: An overflow
    _1 = 1,
}
impl From<CTSUSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSOVF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUSOVF` reader - CTSU Sensor Counter Overflow Flag
pub type CTSUSOVF_R = crate::BitReader<CTSUSOVF_A>;
impl CTSUSOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSOVF_A {
        match self.bits {
            false => CTSUSOVF_A::_0,
            true => CTSUSOVF_A::_1,
        }
    }
    ///No overflow
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSOVF_A::_0
    }
    ///An overflow
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSOVF_A::_1
    }
}
///Field `CTSUSOVF` writer - CTSU Sensor Counter Overflow Flag
pub type CTSUSOVF_W<'a, REG> = crate::BitWriter<'a, REG, CTSUSOVF_A>;
impl<'a, REG> CTSUSOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overflow
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSOVF_A::_0)
    }
    ///An overflow
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSOVF_A::_1)
    }
}
/**CTSU Reference Counter Overflow Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUROVF_A {
    ///0: No overflow
    _0 = 0,
    ///1: An overflow
    _1 = 1,
}
impl From<CTSUROVF_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUROVF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUROVF` reader - CTSU Reference Counter Overflow Flag
pub type CTSUROVF_R = crate::BitReader<CTSUROVF_A>;
impl CTSUROVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUROVF_A {
        match self.bits {
            false => CTSUROVF_A::_0,
            true => CTSUROVF_A::_1,
        }
    }
    ///No overflow
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUROVF_A::_0
    }
    ///An overflow
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUROVF_A::_1
    }
}
///Field `CTSUROVF` writer - CTSU Reference Counter Overflow Flag
pub type CTSUROVF_W<'a, REG> = crate::BitWriter<'a, REG, CTSUROVF_A>;
impl<'a, REG> CTSUROVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overflow
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUROVF_A::_0)
    }
    ///An overflow
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUROVF_A::_1)
    }
}
/**CTSU Mutual Capacitance Status Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUPS_A {
    ///0: First measurement
    _0 = 0,
    ///1: Second measurement
    _1 = 1,
}
impl From<CTSUPS_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUPS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSUPS` reader - CTSU Mutual Capacitance Status Flag
pub type CTSUPS_R = crate::BitReader<CTSUPS_A>;
impl CTSUPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSUPS_A {
        match self.bits {
            false => CTSUPS_A::_0,
            true => CTSUPS_A::_1,
        }
    }
    ///First measurement
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUPS_A::_0
    }
    ///Second measurement
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUPS_A::_1
    }
}
impl R {
    ///Bits 0:2 - CTSU Measurement Status Counter
    #[inline(always)]
    pub fn ctsustc(&self) -> CTSUSTC_R {
        CTSUSTC_R::new(self.bits & 7)
    }
    ///Bit 4 - CTSU Data Transfer Status Flag
    #[inline(always)]
    pub fn ctsudtsr(&self) -> CTSUDTSR_R {
        CTSUDTSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CTSU Sensor Counter Overflow Flag
    #[inline(always)]
    pub fn ctsusovf(&self) -> CTSUSOVF_R {
        CTSUSOVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CTSU Reference Counter Overflow Flag
    #[inline(always)]
    pub fn ctsurovf(&self) -> CTSUROVF_R {
        CTSUROVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CTSU Mutual Capacitance Status Flag
    #[inline(always)]
    pub fn ctsups(&self) -> CTSUPS_R {
        CTSUPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - CTSU Sensor Counter Overflow Flag
    #[inline(always)]
    pub fn ctsusovf(&mut self) -> CTSUSOVF_W<CTSUST_SPEC> {
        CTSUSOVF_W::new(self, 5)
    }
    ///Bit 6 - CTSU Reference Counter Overflow Flag
    #[inline(always)]
    pub fn ctsurovf(&mut self) -> CTSUROVF_W<CTSUST_SPEC> {
        CTSUROVF_W::new(self, 6)
    }
}
/**CTSU Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsust::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTSUST_SPEC;
impl crate::RegisterSpec for CTSUST_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ctsust::R`](R) reader structure
impl crate::Readable for CTSUST_SPEC {}
///`write(|w| ..)` method takes [`ctsust::W`](W) writer structure
impl crate::Writable for CTSUST_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTSUST to value 0
impl crate::Resettable for CTSUST_SPEC {}
