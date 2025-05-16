///Register `ADANSA0` reader
pub type R = crate::R<ADANSA0_SPEC>;
///Register `ADANSA0` writer
pub type W = crate::W<ADANSA0_SPEC>;
/**AN100 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA00_A {
    ///0: AN100 is not subjected to conversion.
    _0 = 0,
    ///1: AN100 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA00` reader - AN100 Select
pub type ANSA00_R = crate::BitReader<ANSA00_A>;
impl ANSA00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA00_A {
        match self.bits {
            false => ANSA00_A::_0,
            true => ANSA00_A::_1,
        }
    }
    ///AN100 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA00_A::_0
    }
    ///AN100 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA00_A::_1
    }
}
///Field `ANSA00` writer - AN100 Select
pub type ANSA00_W<'a, REG> = crate::BitWriter<'a, REG, ANSA00_A>;
impl<'a, REG> ANSA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN100 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA00_A::_0)
    }
    ///AN100 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA00_A::_1)
    }
}
/**AN101 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA01_A {
    ///0: AN101 is not subjected to conversion.
    _0 = 0,
    ///1: AN101 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA01` reader - AN101 Select
pub type ANSA01_R = crate::BitReader<ANSA01_A>;
impl ANSA01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA01_A {
        match self.bits {
            false => ANSA01_A::_0,
            true => ANSA01_A::_1,
        }
    }
    ///AN101 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA01_A::_0
    }
    ///AN101 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA01_A::_1
    }
}
///Field `ANSA01` writer - AN101 Select
pub type ANSA01_W<'a, REG> = crate::BitWriter<'a, REG, ANSA01_A>;
impl<'a, REG> ANSA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN101 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA01_A::_0)
    }
    ///AN101 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA01_A::_1)
    }
}
/**AN102 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA02_A {
    ///0: AN102 is not subjected to conversion.
    _0 = 0,
    ///1: AN102 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA02` reader - AN102 Select
pub type ANSA02_R = crate::BitReader<ANSA02_A>;
impl ANSA02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA02_A {
        match self.bits {
            false => ANSA02_A::_0,
            true => ANSA02_A::_1,
        }
    }
    ///AN102 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA02_A::_0
    }
    ///AN102 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA02_A::_1
    }
}
///Field `ANSA02` writer - AN102 Select
pub type ANSA02_W<'a, REG> = crate::BitWriter<'a, REG, ANSA02_A>;
impl<'a, REG> ANSA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN102 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA02_A::_0)
    }
    ///AN102 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA02_A::_1)
    }
}
/**AN103 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA03_A {
    ///0: AN103 is not subjected to conversion.
    _0 = 0,
    ///1: AN103 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA03` reader - AN103 Select
pub type ANSA03_R = crate::BitReader<ANSA03_A>;
impl ANSA03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA03_A {
        match self.bits {
            false => ANSA03_A::_0,
            true => ANSA03_A::_1,
        }
    }
    ///AN103 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA03_A::_0
    }
    ///AN103 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA03_A::_1
    }
}
///Field `ANSA03` writer - AN103 Select
pub type ANSA03_W<'a, REG> = crate::BitWriter<'a, REG, ANSA03_A>;
impl<'a, REG> ANSA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN103 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA03_A::_0)
    }
    ///AN103 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA03_A::_1)
    }
}
/**AN105 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA05_A {
    ///0: AN105 is not subjected to conversion.
    _0 = 0,
    ///1: AN105 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA05` reader - AN105 Select
pub type ANSA05_R = crate::BitReader<ANSA05_A>;
impl ANSA05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA05_A {
        match self.bits {
            false => ANSA05_A::_0,
            true => ANSA05_A::_1,
        }
    }
    ///AN105 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA05_A::_0
    }
    ///AN105 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA05_A::_1
    }
}
///Field `ANSA05` writer - AN105 Select
pub type ANSA05_W<'a, REG> = crate::BitWriter<'a, REG, ANSA05_A>;
impl<'a, REG> ANSA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN105 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA05_A::_0)
    }
    ///AN105 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA05_A::_1)
    }
}
/**AN106 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA06_A {
    ///0: AN106 is not subjected to conversion.
    _0 = 0,
    ///1: AN106 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA06` reader - AN106 Select
pub type ANSA06_R = crate::BitReader<ANSA06_A>;
impl ANSA06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA06_A {
        match self.bits {
            false => ANSA06_A::_0,
            true => ANSA06_A::_1,
        }
    }
    ///AN106 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA06_A::_0
    }
    ///AN106 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA06_A::_1
    }
}
///Field `ANSA06` writer - AN106 Select
pub type ANSA06_W<'a, REG> = crate::BitWriter<'a, REG, ANSA06_A>;
impl<'a, REG> ANSA06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN106 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA06_A::_0)
    }
    ///AN106 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA06_A::_1)
    }
}
/**AN107 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSA07_A {
    ///0: AN107 is not subjected to conversion.
    _0 = 0,
    ///1: AN107 is subjected to conversion.
    _1 = 1,
}
impl From<ANSA07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSA07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSA07` reader - AN107 Select
pub type ANSA07_R = crate::BitReader<ANSA07_A>;
impl ANSA07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSA07_A {
        match self.bits {
            false => ANSA07_A::_0,
            true => ANSA07_A::_1,
        }
    }
    ///AN107 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSA07_A::_0
    }
    ///AN107 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSA07_A::_1
    }
}
///Field `ANSA07` writer - AN107 Select
pub type ANSA07_W<'a, REG> = crate::BitWriter<'a, REG, ANSA07_A>;
impl<'a, REG> ANSA07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN107 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA07_A::_0)
    }
    ///AN107 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSA07_A::_1)
    }
}
impl R {
    ///Bit 0 - AN100 Select
    #[inline(always)]
    pub fn ansa00(&self) -> ANSA00_R {
        ANSA00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN101 Select
    #[inline(always)]
    pub fn ansa01(&self) -> ANSA01_R {
        ANSA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN102 Select
    #[inline(always)]
    pub fn ansa02(&self) -> ANSA02_R {
        ANSA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN103 Select
    #[inline(always)]
    pub fn ansa03(&self) -> ANSA03_R {
        ANSA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - AN105 Select
    #[inline(always)]
    pub fn ansa05(&self) -> ANSA05_R {
        ANSA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AN106 Select
    #[inline(always)]
    pub fn ansa06(&self) -> ANSA06_R {
        ANSA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AN107 Select
    #[inline(always)]
    pub fn ansa07(&self) -> ANSA07_R {
        ANSA07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN100 Select
    #[inline(always)]
    pub fn ansa00(&mut self) -> ANSA00_W<ADANSA0_SPEC> {
        ANSA00_W::new(self, 0)
    }
    ///Bit 1 - AN101 Select
    #[inline(always)]
    pub fn ansa01(&mut self) -> ANSA01_W<ADANSA0_SPEC> {
        ANSA01_W::new(self, 1)
    }
    ///Bit 2 - AN102 Select
    #[inline(always)]
    pub fn ansa02(&mut self) -> ANSA02_W<ADANSA0_SPEC> {
        ANSA02_W::new(self, 2)
    }
    ///Bit 3 - AN103 Select
    #[inline(always)]
    pub fn ansa03(&mut self) -> ANSA03_W<ADANSA0_SPEC> {
        ANSA03_W::new(self, 3)
    }
    ///Bit 5 - AN105 Select
    #[inline(always)]
    pub fn ansa05(&mut self) -> ANSA05_W<ADANSA0_SPEC> {
        ANSA05_W::new(self, 5)
    }
    ///Bit 6 - AN106 Select
    #[inline(always)]
    pub fn ansa06(&mut self) -> ANSA06_W<ADANSA0_SPEC> {
        ANSA06_W::new(self, 6)
    }
    ///Bit 7 - AN107 Select
    #[inline(always)]
    pub fn ansa07(&mut self) -> ANSA07_W<ADANSA0_SPEC> {
        ANSA07_W::new(self, 7)
    }
}
/**A/D Channel Select Register A0

You can [`read`](crate::Reg::read) this register and get [`adansa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADANSA0_SPEC;
impl crate::RegisterSpec for ADANSA0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adansa0::R`](R) reader structure
impl crate::Readable for ADANSA0_SPEC {}
///`write(|w| ..)` method takes [`adansa0::W`](W) writer structure
impl crate::Writable for ADANSA0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSA0 to value 0
impl crate::Resettable for ADANSA0_SPEC {}
