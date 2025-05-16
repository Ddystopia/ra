///Register `ADCMPSR1` reader
pub type R = crate::R<ADCMPSR1_SPEC>;
///Register `ADCMPSR1` writer
pub type W = crate::W<ADCMPSR1_SPEC>;
/**Compare window A flag of AN016

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA16_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA16_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA16` reader - Compare window A flag of AN016

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA16_R = crate::BitReader<CMPSTCHA16_A>;
impl CMPSTCHA16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA16_A {
        match self.bits {
            false => CMPSTCHA16_A::_0,
            true => CMPSTCHA16_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA16_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA16_A::_1
    }
}
///Field `CMPSTCHA16` writer - Compare window A flag of AN016
pub type CMPSTCHA16_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA16_A>;
impl<'a, REG> CMPSTCHA16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA16_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA16_A::_1)
    }
}
/**Compare window A flag of AN017

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA17_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA17_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA17` reader - Compare window A flag of AN017

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA17_R = crate::BitReader<CMPSTCHA17_A>;
impl CMPSTCHA17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA17_A {
        match self.bits {
            false => CMPSTCHA17_A::_0,
            true => CMPSTCHA17_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA17_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA17_A::_1
    }
}
///Field `CMPSTCHA17` writer - Compare window A flag of AN017
pub type CMPSTCHA17_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA17_A>;
impl<'a, REG> CMPSTCHA17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA17_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA17_A::_1)
    }
}
/**Compare window A flag of AN018

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA18_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA18_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA18` reader - Compare window A flag of AN018

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA18_R = crate::BitReader<CMPSTCHA18_A>;
impl CMPSTCHA18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA18_A {
        match self.bits {
            false => CMPSTCHA18_A::_0,
            true => CMPSTCHA18_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA18_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA18_A::_1
    }
}
///Field `CMPSTCHA18` writer - Compare window A flag of AN018
pub type CMPSTCHA18_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA18_A>;
impl<'a, REG> CMPSTCHA18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA18_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA18_A::_1)
    }
}
/**Compare window A flag of AN019

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA19_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA19_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA19` reader - Compare window A flag of AN019

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA19_R = crate::BitReader<CMPSTCHA19_A>;
impl CMPSTCHA19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA19_A {
        match self.bits {
            false => CMPSTCHA19_A::_0,
            true => CMPSTCHA19_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA19_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA19_A::_1
    }
}
///Field `CMPSTCHA19` writer - Compare window A flag of AN019
pub type CMPSTCHA19_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA19_A>;
impl<'a, REG> CMPSTCHA19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA19_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA19_A::_1)
    }
}
/**Compare window A flag of AN020

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA20_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA20_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA20` reader - Compare window A flag of AN020

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA20_R = crate::BitReader<CMPSTCHA20_A>;
impl CMPSTCHA20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA20_A {
        match self.bits {
            false => CMPSTCHA20_A::_0,
            true => CMPSTCHA20_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA20_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA20_A::_1
    }
}
///Field `CMPSTCHA20` writer - Compare window A flag of AN020
pub type CMPSTCHA20_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA20_A>;
impl<'a, REG> CMPSTCHA20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA20_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA20_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare window A flag of AN016
    #[inline(always)]
    pub fn cmpstcha16(&self) -> CMPSTCHA16_R {
        CMPSTCHA16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare window A flag of AN017
    #[inline(always)]
    pub fn cmpstcha17(&self) -> CMPSTCHA17_R {
        CMPSTCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare window A flag of AN018
    #[inline(always)]
    pub fn cmpstcha18(&self) -> CMPSTCHA18_R {
        CMPSTCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare window A flag of AN019
    #[inline(always)]
    pub fn cmpstcha19(&self) -> CMPSTCHA19_R {
        CMPSTCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare window A flag of AN020
    #[inline(always)]
    pub fn cmpstcha20(&self) -> CMPSTCHA20_R {
        CMPSTCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare window A flag of AN016
    #[inline(always)]
    pub fn cmpstcha16(&mut self) -> CMPSTCHA16_W<ADCMPSR1_SPEC> {
        CMPSTCHA16_W::new(self, 0)
    }
    ///Bit 1 - Compare window A flag of AN017
    #[inline(always)]
    pub fn cmpstcha17(&mut self) -> CMPSTCHA17_W<ADCMPSR1_SPEC> {
        CMPSTCHA17_W::new(self, 1)
    }
    ///Bit 2 - Compare window A flag of AN018
    #[inline(always)]
    pub fn cmpstcha18(&mut self) -> CMPSTCHA18_W<ADCMPSR1_SPEC> {
        CMPSTCHA18_W::new(self, 2)
    }
    ///Bit 3 - Compare window A flag of AN019
    #[inline(always)]
    pub fn cmpstcha19(&mut self) -> CMPSTCHA19_W<ADCMPSR1_SPEC> {
        CMPSTCHA19_W::new(self, 3)
    }
    ///Bit 4 - Compare window A flag of AN020
    #[inline(always)]
    pub fn cmpstcha20(&mut self) -> CMPSTCHA20_W<ADCMPSR1_SPEC> {
        CMPSTCHA20_W::new(self, 4)
    }
}
/**A/D Compare Function Window A Channel Status Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmpsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPSR1_SPEC;
impl crate::RegisterSpec for ADCMPSR1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpsr1::R`](R) reader structure
impl crate::Readable for ADCMPSR1_SPEC {}
///`write(|w| ..)` method takes [`adcmpsr1::W`](W) writer structure
impl crate::Writable for ADCMPSR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0x1f;
}
///`reset()` method sets ADCMPSR1 to value 0
impl crate::Resettable for ADCMPSR1_SPEC {}
