///Register `ADCMPLR1` reader
pub type R = crate::R<ADCMPLR1_SPEC>;
///Register `ADCMPLR1` writer
pub type W = crate::W<ADCMPLR1_SPEC>;
/**Comparison condition of AN016

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA16_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA16` reader - Comparison condition of AN016
pub type CMPLCHA16_R = crate::BitReader<CMPLCHA16_A>;
impl CMPLCHA16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA16_A {
        match self.bits {
            false => CMPLCHA16_A::_0,
            true => CMPLCHA16_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA16_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA16_A::_1
    }
}
///Field `CMPLCHA16` writer - Comparison condition of AN016
pub type CMPLCHA16_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA16_A>;
impl<'a, REG> CMPLCHA16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA16_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA16_A::_1)
    }
}
/**Comparison condition of AN017

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA17_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA17` reader - Comparison condition of AN017
pub type CMPLCHA17_R = crate::BitReader<CMPLCHA17_A>;
impl CMPLCHA17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA17_A {
        match self.bits {
            false => CMPLCHA17_A::_0,
            true => CMPLCHA17_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA17_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA17_A::_1
    }
}
///Field `CMPLCHA17` writer - Comparison condition of AN017
pub type CMPLCHA17_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA17_A>;
impl<'a, REG> CMPLCHA17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA17_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA17_A::_1)
    }
}
/**Comparison condition of AN018

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA18_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA18` reader - Comparison condition of AN018
pub type CMPLCHA18_R = crate::BitReader<CMPLCHA18_A>;
impl CMPLCHA18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA18_A {
        match self.bits {
            false => CMPLCHA18_A::_0,
            true => CMPLCHA18_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA18_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA18_A::_1
    }
}
///Field `CMPLCHA18` writer - Comparison condition of AN018
pub type CMPLCHA18_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA18_A>;
impl<'a, REG> CMPLCHA18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA18_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA18_A::_1)
    }
}
/**Comparison condition of AN019

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA19_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA19` reader - Comparison condition of AN019
pub type CMPLCHA19_R = crate::BitReader<CMPLCHA19_A>;
impl CMPLCHA19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA19_A {
        match self.bits {
            false => CMPLCHA19_A::_0,
            true => CMPLCHA19_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA19_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA19_A::_1
    }
}
///Field `CMPLCHA19` writer - Comparison condition of AN019
pub type CMPLCHA19_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA19_A>;
impl<'a, REG> CMPLCHA19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA19_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA19_A::_1)
    }
}
/**Comparison condition of AN020

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLCHA20_A {
    ///0: ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    _0 = 0,
    ///1: ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    _1 = 1,
}
impl From<CMPLCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLCHA20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPLCHA20` reader - Comparison condition of AN020
pub type CMPLCHA20_R = crate::BitReader<CMPLCHA20_A>;
impl CMPLCHA20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPLCHA20_A {
        match self.bits {
            false => CMPLCHA20_A::_0,
            true => CMPLCHA20_A::_1,
        }
    }
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPLCHA20_A::_0
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPLCHA20_A::_1
    }
}
///Field `CMPLCHA20` writer - Comparison condition of AN020
pub type CMPLCHA20_W<'a, REG> = crate::BitWriter<'a, REG, CMPLCHA20_A>;
impl<'a, REG> CMPLCHA20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADCMPDR0 value > A/D converted value (ADCMPCR.WCMPE=0) / A/D converted value < ADCMPDR0 value or, ADCMPDR1 value < A/D converted value (ADCMPCR.WCMPE=1)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA20_A::_0)
    }
    ///ADCMPDR0 value < A/D converted value (ADCMPCR.WCMPE=0) / A/DCMPDR0 value < A/D converted value < ADCMPDR1 value (ADCMPCR.WCMPE=1).
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPLCHA20_A::_1)
    }
}
impl R {
    ///Bit 0 - Comparison condition of AN016
    #[inline(always)]
    pub fn cmplcha16(&self) -> CMPLCHA16_R {
        CMPLCHA16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Comparison condition of AN017
    #[inline(always)]
    pub fn cmplcha17(&self) -> CMPLCHA17_R {
        CMPLCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Comparison condition of AN018
    #[inline(always)]
    pub fn cmplcha18(&self) -> CMPLCHA18_R {
        CMPLCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Comparison condition of AN019
    #[inline(always)]
    pub fn cmplcha19(&self) -> CMPLCHA19_R {
        CMPLCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Comparison condition of AN020
    #[inline(always)]
    pub fn cmplcha20(&self) -> CMPLCHA20_R {
        CMPLCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparison condition of AN016
    #[inline(always)]
    pub fn cmplcha16(&mut self) -> CMPLCHA16_W<ADCMPLR1_SPEC> {
        CMPLCHA16_W::new(self, 0)
    }
    ///Bit 1 - Comparison condition of AN017
    #[inline(always)]
    pub fn cmplcha17(&mut self) -> CMPLCHA17_W<ADCMPLR1_SPEC> {
        CMPLCHA17_W::new(self, 1)
    }
    ///Bit 2 - Comparison condition of AN018
    #[inline(always)]
    pub fn cmplcha18(&mut self) -> CMPLCHA18_W<ADCMPLR1_SPEC> {
        CMPLCHA18_W::new(self, 2)
    }
    ///Bit 3 - Comparison condition of AN019
    #[inline(always)]
    pub fn cmplcha19(&mut self) -> CMPLCHA19_W<ADCMPLR1_SPEC> {
        CMPLCHA19_W::new(self, 3)
    }
    ///Bit 4 - Comparison condition of AN020
    #[inline(always)]
    pub fn cmplcha20(&mut self) -> CMPLCHA20_W<ADCMPLR1_SPEC> {
        CMPLCHA20_W::new(self, 4)
    }
}
/**A/D Compare Function Window A Comparison Condition Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmplr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPLR1_SPEC;
impl crate::RegisterSpec for ADCMPLR1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmplr1::R`](R) reader structure
impl crate::Readable for ADCMPLR1_SPEC {}
///`write(|w| ..)` method takes [`adcmplr1::W`](W) writer structure
impl crate::Writable for ADCMPLR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPLR1 to value 0
impl crate::Resettable for ADCMPLR1_SPEC {}
