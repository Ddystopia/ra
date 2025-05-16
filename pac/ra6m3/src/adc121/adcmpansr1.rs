///Register `ADCMPANSR1` reader
pub type R = crate::R<ADCMPANSR1_SPEC>;
///Register `ADCMPANSR1` writer
pub type W = crate::W<ADCMPANSR1_SPEC>;
/**AN116 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA16_A {
    ///0: Excludes AN116 from the compare window A target range.
    _0 = 0,
    ///1: Includes AN116 from the compare window A target range.
    _1 = 1,
}
impl From<CMPCHA16_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA16` reader - AN116 Select
pub type CMPCHA16_R = crate::BitReader<CMPCHA16_A>;
impl CMPCHA16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA16_A {
        match self.bits {
            false => CMPCHA16_A::_0,
            true => CMPCHA16_A::_1,
        }
    }
    ///Excludes AN116 from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA16_A::_0
    }
    ///Includes AN116 from the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA16_A::_1
    }
}
///Field `CMPCHA16` writer - AN116 Select
pub type CMPCHA16_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA16_A>;
impl<'a, REG> CMPCHA16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes AN116 from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA16_A::_0)
    }
    ///Includes AN116 from the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA16_A::_1)
    }
}
/**AN117 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA17_A {
    ///0: Excludes AN117 from the compare window A target range.
    _0 = 0,
    ///1: Includes AN117 from the compare window A target range.
    _1 = 1,
}
impl From<CMPCHA17_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA17` reader - AN117 Select
pub type CMPCHA17_R = crate::BitReader<CMPCHA17_A>;
impl CMPCHA17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA17_A {
        match self.bits {
            false => CMPCHA17_A::_0,
            true => CMPCHA17_A::_1,
        }
    }
    ///Excludes AN117 from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA17_A::_0
    }
    ///Includes AN117 from the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA17_A::_1
    }
}
///Field `CMPCHA17` writer - AN117 Select
pub type CMPCHA17_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA17_A>;
impl<'a, REG> CMPCHA17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes AN117 from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA17_A::_0)
    }
    ///Includes AN117 from the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA17_A::_1)
    }
}
/**AN118 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA18_A {
    ///0: Excludes AN118 from the compare window A target range.
    _0 = 0,
    ///1: Includes AN118 from the compare window A target range.
    _1 = 1,
}
impl From<CMPCHA18_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA18` reader - AN118 Select
pub type CMPCHA18_R = crate::BitReader<CMPCHA18_A>;
impl CMPCHA18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA18_A {
        match self.bits {
            false => CMPCHA18_A::_0,
            true => CMPCHA18_A::_1,
        }
    }
    ///Excludes AN118 from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA18_A::_0
    }
    ///Includes AN118 from the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA18_A::_1
    }
}
///Field `CMPCHA18` writer - AN118 Select
pub type CMPCHA18_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA18_A>;
impl<'a, REG> CMPCHA18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes AN118 from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA18_A::_0)
    }
    ///Includes AN118 from the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA18_A::_1)
    }
}
/**AN119 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA19_A {
    ///0: Excludes AN119 from the compare window A target range.
    _0 = 0,
    ///1: Includes AN119 from the compare window A target range.
    _1 = 1,
}
impl From<CMPCHA19_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA19` reader - AN119 Select
pub type CMPCHA19_R = crate::BitReader<CMPCHA19_A>;
impl CMPCHA19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA19_A {
        match self.bits {
            false => CMPCHA19_A::_0,
            true => CMPCHA19_A::_1,
        }
    }
    ///Excludes AN119 from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA19_A::_0
    }
    ///Includes AN119 from the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA19_A::_1
    }
}
///Field `CMPCHA19` writer - AN119 Select
pub type CMPCHA19_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA19_A>;
impl<'a, REG> CMPCHA19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes AN119 from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA19_A::_0)
    }
    ///Includes AN119 from the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA19_A::_1)
    }
}
/**AN120 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA20_A {
    ///0: Excludes AN120 from the compare window A target range.
    _0 = 0,
    ///1: Includes AN120 from the compare window A target range.
    _1 = 1,
}
impl From<CMPCHA20_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA20` reader - AN120 Select
pub type CMPCHA20_R = crate::BitReader<CMPCHA20_A>;
impl CMPCHA20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA20_A {
        match self.bits {
            false => CMPCHA20_A::_0,
            true => CMPCHA20_A::_1,
        }
    }
    ///Excludes AN120 from the compare window A target range.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA20_A::_0
    }
    ///Includes AN120 from the compare window A target range.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA20_A::_1
    }
}
///Field `CMPCHA20` writer - AN120 Select
pub type CMPCHA20_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA20_A>;
impl<'a, REG> CMPCHA20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Excludes AN120 from the compare window A target range.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA20_A::_0)
    }
    ///Includes AN120 from the compare window A target range.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA20_A::_1)
    }
}
impl R {
    ///Bit 0 - AN116 Select
    #[inline(always)]
    pub fn cmpcha16(&self) -> CMPCHA16_R {
        CMPCHA16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN117 Select
    #[inline(always)]
    pub fn cmpcha17(&self) -> CMPCHA17_R {
        CMPCHA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN118 Select
    #[inline(always)]
    pub fn cmpcha18(&self) -> CMPCHA18_R {
        CMPCHA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN119 Select
    #[inline(always)]
    pub fn cmpcha19(&self) -> CMPCHA19_R {
        CMPCHA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AN120 Select
    #[inline(always)]
    pub fn cmpcha20(&self) -> CMPCHA20_R {
        CMPCHA20_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN116 Select
    #[inline(always)]
    pub fn cmpcha16(&mut self) -> CMPCHA16_W<ADCMPANSR1_SPEC> {
        CMPCHA16_W::new(self, 0)
    }
    ///Bit 1 - AN117 Select
    #[inline(always)]
    pub fn cmpcha17(&mut self) -> CMPCHA17_W<ADCMPANSR1_SPEC> {
        CMPCHA17_W::new(self, 1)
    }
    ///Bit 2 - AN118 Select
    #[inline(always)]
    pub fn cmpcha18(&mut self) -> CMPCHA18_W<ADCMPANSR1_SPEC> {
        CMPCHA18_W::new(self, 2)
    }
    ///Bit 3 - AN119 Select
    #[inline(always)]
    pub fn cmpcha19(&mut self) -> CMPCHA19_W<ADCMPANSR1_SPEC> {
        CMPCHA19_W::new(self, 3)
    }
    ///Bit 4 - AN120 Select
    #[inline(always)]
    pub fn cmpcha20(&mut self) -> CMPCHA20_W<ADCMPANSR1_SPEC> {
        CMPCHA20_W::new(self, 4)
    }
}
/**A/D Compare Function Window A Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmpansr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPANSR1_SPEC;
impl crate::RegisterSpec for ADCMPANSR1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpansr1::R`](R) reader structure
impl crate::Readable for ADCMPANSR1_SPEC {}
///`write(|w| ..)` method takes [`adcmpansr1::W`](W) writer structure
impl crate::Writable for ADCMPANSR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPANSR1 to value 0
impl crate::Resettable for ADCMPANSR1_SPEC {}
