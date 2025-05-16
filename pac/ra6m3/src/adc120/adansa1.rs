///Register `ADANSA1` reader
pub type R = crate::R<ADANSA1_SPEC>;
///Register `ADANSA1` writer
pub type W = crate::W<ADANSA1_SPEC>;
/**AN016 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA16_A {
    ///0: AN016 is not subjected to conversion.
    _0 = 0,
    ///1: AN016 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA16` reader - AN016 Select
pub type ANSA16_R = crate::BitReader<ANSA16_A>;
impl ANSA16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA16_A {
        match self.bits {
            false => ANSA16_A::_0,
            true => ANSA16_A::_1,
        }
    }
    ///AN016 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA16_A::_0
    }
    ///AN016 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA16_A::_1
    }
}
///Field `ANSA16` writer - AN016 Select
pub type ANSA16_W<'a, REG> = crate::BitWriter<'a, REG, ANSA16_A>;
impl<'a, REG> ANSA16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN016 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA16_A::_0)
    }
    ///AN016 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA16_A::_1)
    }
}
/**AN017 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA17_A {
    ///0: AN017 is not subjected to conversion.
    _0 = 0,
    ///1: AN017 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA17` reader - AN017 Select
pub type ANSA17_R = crate::BitReader<ANSA17_A>;
impl ANSA17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA17_A {
        match self.bits {
            false => ANSA17_A::_0,
            true => ANSA17_A::_1,
        }
    }
    ///AN017 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA17_A::_0
    }
    ///AN017 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA17_A::_1
    }
}
///Field `ANSA17` writer - AN017 Select
pub type ANSA17_W<'a, REG> = crate::BitWriter<'a, REG, ANSA17_A>;
impl<'a, REG> ANSA17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN017 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA17_A::_0)
    }
    ///AN017 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA17_A::_1)
    }
}
/**AN018 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA18_A {
    ///0: AN018 is not subjected to conversion.
    _0 = 0,
    ///1: AN018 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA18` reader - AN018 Select
pub type ANSA18_R = crate::BitReader<ANSA18_A>;
impl ANSA18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA18_A {
        match self.bits {
            false => ANSA18_A::_0,
            true => ANSA18_A::_1,
        }
    }
    ///AN018 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA18_A::_0
    }
    ///AN018 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA18_A::_1
    }
}
///Field `ANSA18` writer - AN018 Select
pub type ANSA18_W<'a, REG> = crate::BitWriter<'a, REG, ANSA18_A>;
impl<'a, REG> ANSA18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN018 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA18_A::_0)
    }
    ///AN018 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA18_A::_1)
    }
}
/**AN019 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA19_A {
    ///0: AN019 is not subjected to conversion.
    _0 = 0,
    ///1: AN019 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA19_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA19` reader - AN019 Select
pub type ANSA19_R = crate::BitReader<ANSA19_A>;
impl ANSA19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA19_A {
        match self.bits {
            false => ANSA19_A::_0,
            true => ANSA19_A::_1,
        }
    }
    ///AN019 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA19_A::_0
    }
    ///AN019 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA19_A::_1
    }
}
///Field `ANSA19` writer - AN019 Select
pub type ANSA19_W<'a, REG> = crate::BitWriter<'a, REG, ANSA19_A>;
impl<'a, REG> ANSA19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN019 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA19_A::_0)
    }
    ///AN019 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA19_A::_1)
    }
}
/**AN020 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA20_A {
    ///0: AN020 is not subjected to conversion.
    _0 = 0,
    ///1: AN020 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA20_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA20` reader - AN020 Select
pub type ANSA20_R = crate::BitReader<ANSA20_A>;
impl ANSA20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA20_A {
        match self.bits {
            false => ANSA20_A::_0,
            true => ANSA20_A::_1,
        }
    }
    ///AN020 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA20_A::_0
    }
    ///AN020 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA20_A::_1
    }
}
///Field `ANSA20` writer - AN020 Select
pub type ANSA20_W<'a, REG> = crate::BitWriter<'a, REG, ANSA20_A>;
impl<'a, REG> ANSA20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN020 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA20_A::_0)
    }
    ///AN020 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA20_A::_1)
    }
}
impl R {
    ///Bit 0 - AN016 Select
    #[inline(always)]
    pub fn ansa16(&self) -> ANSA16_R {
        ANSA16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN017 Select
    #[inline(always)]
    pub fn ansa17(&self) -> ANSA17_R {
        ANSA17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN018 Select
    #[inline(always)]
    pub fn ansa18(&self) -> ANSA18_R {
        ANSA18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN019 Select
    #[inline(always)]
    pub fn ansa19(&self) -> ANSA19_R {
        ANSA19_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AN020 Select
    #[inline(always)]
    pub fn ansa20(&self) -> ANSA20_R {
        ANSA20_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN016 Select
    #[inline(always)]
    pub fn ansa16(&mut self) -> ANSA16_W<ADANSA1_SPEC> {
        ANSA16_W::new(self, 0)
    }
    ///Bit 1 - AN017 Select
    #[inline(always)]
    pub fn ansa17(&mut self) -> ANSA17_W<ADANSA1_SPEC> {
        ANSA17_W::new(self, 1)
    }
    ///Bit 2 - AN018 Select
    #[inline(always)]
    pub fn ansa18(&mut self) -> ANSA18_W<ADANSA1_SPEC> {
        ANSA18_W::new(self, 2)
    }
    ///Bit 3 - AN019 Select
    #[inline(always)]
    pub fn ansa19(&mut self) -> ANSA19_W<ADANSA1_SPEC> {
        ANSA19_W::new(self, 3)
    }
    ///Bit 4 - AN020 Select
    #[inline(always)]
    pub fn ansa20(&mut self) -> ANSA20_W<ADANSA1_SPEC> {
        ANSA20_W::new(self, 4)
    }
}
/**A/D Channel Select Register A1

You can [`read`](crate::Reg::read) this register and get [`adansa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADANSA1_SPEC;
impl crate::RegisterSpec for ADANSA1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adansa1::R`](R) reader structure
impl crate::Readable for ADANSA1_SPEC {}
///`write(|w| ..)` method takes [`adansa1::W`](W) writer structure
impl crate::Writable for ADANSA1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSA1 to value 0
impl crate::Resettable for ADANSA1_SPEC {}
