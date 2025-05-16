///Register `ADADS1` reader
pub type R = crate::R<ADADS1_SPEC>;
///Register `ADADS1` writer
pub type W = crate::W<ADADS1_SPEC>;
/**A/D-Converted Value Addition/Average Channel AN116 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS16_A {
    ///0: AN116 is not selected.
    _0 = 0,
    ///1: AN116 is selected.
    _1 = 1,
}
impl From<ADS16_A> for bool {
    #[inline(always)]
    fn from(variant: ADS16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS16` reader - A/D-Converted Value Addition/Average Channel AN116 Select
pub type ADS16_R = crate::BitReader<ADS16_A>;
impl ADS16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS16_A {
        match self.bits {
            false => ADS16_A::_0,
            true => ADS16_A::_1,
        }
    }
    ///AN116 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS16_A::_0
    }
    ///AN116 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS16_A::_1
    }
}
///Field `ADS16` writer - A/D-Converted Value Addition/Average Channel AN116 Select
pub type ADS16_W<'a, REG> = crate::BitWriter<'a, REG, ADS16_A>;
impl<'a, REG> ADS16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN116 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS16_A::_0)
    }
    ///AN116 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS16_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN117 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS17_A {
    ///0: AN117 is not selected.
    _0 = 0,
    ///1: AN117 is selected.
    _1 = 1,
}
impl From<ADS17_A> for bool {
    #[inline(always)]
    fn from(variant: ADS17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS17` reader - A/D-Converted Value Addition/Average Channel AN117 Select
pub type ADS17_R = crate::BitReader<ADS17_A>;
impl ADS17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS17_A {
        match self.bits {
            false => ADS17_A::_0,
            true => ADS17_A::_1,
        }
    }
    ///AN117 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS17_A::_0
    }
    ///AN117 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS17_A::_1
    }
}
///Field `ADS17` writer - A/D-Converted Value Addition/Average Channel AN117 Select
pub type ADS17_W<'a, REG> = crate::BitWriter<'a, REG, ADS17_A>;
impl<'a, REG> ADS17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN117 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS17_A::_0)
    }
    ///AN117 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS17_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN118 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS18_A {
    ///0: AN118 is not selected.
    _0 = 0,
    ///1: AN118 is selected.
    _1 = 1,
}
impl From<ADS18_A> for bool {
    #[inline(always)]
    fn from(variant: ADS18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS18` reader - A/D-Converted Value Addition/Average Channel AN118 Select
pub type ADS18_R = crate::BitReader<ADS18_A>;
impl ADS18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS18_A {
        match self.bits {
            false => ADS18_A::_0,
            true => ADS18_A::_1,
        }
    }
    ///AN118 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS18_A::_0
    }
    ///AN118 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS18_A::_1
    }
}
///Field `ADS18` writer - A/D-Converted Value Addition/Average Channel AN118 Select
pub type ADS18_W<'a, REG> = crate::BitWriter<'a, REG, ADS18_A>;
impl<'a, REG> ADS18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN118 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS18_A::_0)
    }
    ///AN118 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS18_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN119 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS19_A {
    ///0: AN119 is not selected.
    _0 = 0,
    ///1: AN119 is selected.
    _1 = 1,
}
impl From<ADS19_A> for bool {
    #[inline(always)]
    fn from(variant: ADS19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS19` reader - A/D-Converted Value Addition/Average Channel AN119 Select
pub type ADS19_R = crate::BitReader<ADS19_A>;
impl ADS19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS19_A {
        match self.bits {
            false => ADS19_A::_0,
            true => ADS19_A::_1,
        }
    }
    ///AN119 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS19_A::_0
    }
    ///AN119 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS19_A::_1
    }
}
///Field `ADS19` writer - A/D-Converted Value Addition/Average Channel AN119 Select
pub type ADS19_W<'a, REG> = crate::BitWriter<'a, REG, ADS19_A>;
impl<'a, REG> ADS19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN119 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS19_A::_0)
    }
    ///AN119 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS19_A::_1)
    }
}
impl R {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN116 Select
    #[inline(always)]
    pub fn ads16(&self) -> ADS16_R {
        ADS16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN117 Select
    #[inline(always)]
    pub fn ads17(&self) -> ADS17_R {
        ADS17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN118 Select
    #[inline(always)]
    pub fn ads18(&self) -> ADS18_R {
        ADS18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN119 Select
    #[inline(always)]
    pub fn ads19(&self) -> ADS19_R {
        ADS19_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN116 Select
    #[inline(always)]
    pub fn ads16(&mut self) -> ADS16_W<ADADS1_SPEC> {
        ADS16_W::new(self, 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN117 Select
    #[inline(always)]
    pub fn ads17(&mut self) -> ADS17_W<ADADS1_SPEC> {
        ADS17_W::new(self, 1)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN118 Select
    #[inline(always)]
    pub fn ads18(&mut self) -> ADS18_W<ADADS1_SPEC> {
        ADS18_W::new(self, 2)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN119 Select
    #[inline(always)]
    pub fn ads19(&mut self) -> ADS19_W<ADADS1_SPEC> {
        ADS19_W::new(self, 3)
    }
}
/**A/D-Converted Value Addition/Average Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adads1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADADS1_SPEC;
impl crate::RegisterSpec for ADADS1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adads1::R`](R) reader structure
impl crate::Readable for ADADS1_SPEC {}
///`write(|w| ..)` method takes [`adads1::W`](W) writer structure
impl crate::Writable for ADADS1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADADS1 to value 0
impl crate::Resettable for ADADS1_SPEC {}
