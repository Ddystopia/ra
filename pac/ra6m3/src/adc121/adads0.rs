///Register `ADADS0` reader
pub type R = crate::R<ADADS0_SPEC>;
///Register `ADADS0` writer
pub type W = crate::W<ADADS0_SPEC>;
/**A/D-Converted Value Addition/Average Channel AN100 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS00_A {
    ///0: AN100 is not selected.
    _0 = 0,
    ///1: AN100 is selected.
    _1 = 1,
}
impl From<ADS00_A> for bool {
    #[inline(always)]
    fn from(variant: ADS00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS00` reader - A/D-Converted Value Addition/Average Channel AN100 Select
pub type ADS00_R = crate::BitReader<ADS00_A>;
impl ADS00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS00_A {
        match self.bits {
            false => ADS00_A::_0,
            true => ADS00_A::_1,
        }
    }
    ///AN100 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS00_A::_0
    }
    ///AN100 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS00_A::_1
    }
}
///Field `ADS00` writer - A/D-Converted Value Addition/Average Channel AN100 Select
pub type ADS00_W<'a, REG> = crate::BitWriter<'a, REG, ADS00_A>;
impl<'a, REG> ADS00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN100 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS00_A::_0)
    }
    ///AN100 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS00_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN101 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS01_A {
    ///0: AN101 is not selected.
    _0 = 0,
    ///1: AN101 is selected.
    _1 = 1,
}
impl From<ADS01_A> for bool {
    #[inline(always)]
    fn from(variant: ADS01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS01` reader - A/D-Converted Value Addition/Average Channel AN101 Select
pub type ADS01_R = crate::BitReader<ADS01_A>;
impl ADS01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS01_A {
        match self.bits {
            false => ADS01_A::_0,
            true => ADS01_A::_1,
        }
    }
    ///AN101 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS01_A::_0
    }
    ///AN101 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS01_A::_1
    }
}
///Field `ADS01` writer - A/D-Converted Value Addition/Average Channel AN101 Select
pub type ADS01_W<'a, REG> = crate::BitWriter<'a, REG, ADS01_A>;
impl<'a, REG> ADS01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN101 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS01_A::_0)
    }
    ///AN101 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS01_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN102 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS02_A {
    ///0: AN102 is not selected.
    _0 = 0,
    ///1: AN102 is selected.
    _1 = 1,
}
impl From<ADS02_A> for bool {
    #[inline(always)]
    fn from(variant: ADS02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS02` reader - A/D-Converted Value Addition/Average Channel AN102 Select
pub type ADS02_R = crate::BitReader<ADS02_A>;
impl ADS02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS02_A {
        match self.bits {
            false => ADS02_A::_0,
            true => ADS02_A::_1,
        }
    }
    ///AN102 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS02_A::_0
    }
    ///AN102 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS02_A::_1
    }
}
///Field `ADS02` writer - A/D-Converted Value Addition/Average Channel AN102 Select
pub type ADS02_W<'a, REG> = crate::BitWriter<'a, REG, ADS02_A>;
impl<'a, REG> ADS02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN102 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS02_A::_0)
    }
    ///AN102 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS02_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN103 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS03_A {
    ///0: AN103 is not selected.
    _0 = 0,
    ///1: AN103 is selected.
    _1 = 1,
}
impl From<ADS03_A> for bool {
    #[inline(always)]
    fn from(variant: ADS03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS03` reader - A/D-Converted Value Addition/Average Channel AN103 Select
pub type ADS03_R = crate::BitReader<ADS03_A>;
impl ADS03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS03_A {
        match self.bits {
            false => ADS03_A::_0,
            true => ADS03_A::_1,
        }
    }
    ///AN103 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS03_A::_0
    }
    ///AN103 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS03_A::_1
    }
}
///Field `ADS03` writer - A/D-Converted Value Addition/Average Channel AN103 Select
pub type ADS03_W<'a, REG> = crate::BitWriter<'a, REG, ADS03_A>;
impl<'a, REG> ADS03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN103 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS03_A::_0)
    }
    ///AN103 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS03_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN105 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS05_A {
    ///0: AN105 is not selected.
    _0 = 0,
    ///1: AN105 is selected.
    _1 = 1,
}
impl From<ADS05_A> for bool {
    #[inline(always)]
    fn from(variant: ADS05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS05` reader - A/D-Converted Value Addition/Average Channel AN105 Select
pub type ADS05_R = crate::BitReader<ADS05_A>;
impl ADS05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS05_A {
        match self.bits {
            false => ADS05_A::_0,
            true => ADS05_A::_1,
        }
    }
    ///AN105 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS05_A::_0
    }
    ///AN105 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS05_A::_1
    }
}
///Field `ADS05` writer - A/D-Converted Value Addition/Average Channel AN105 Select
pub type ADS05_W<'a, REG> = crate::BitWriter<'a, REG, ADS05_A>;
impl<'a, REG> ADS05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN105 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS05_A::_0)
    }
    ///AN105 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS05_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN106 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS06_A {
    ///0: AN106 is not selected.
    _0 = 0,
    ///1: AN106 is selected.
    _1 = 1,
}
impl From<ADS06_A> for bool {
    #[inline(always)]
    fn from(variant: ADS06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS06` reader - A/D-Converted Value Addition/Average Channel AN106 Select
pub type ADS06_R = crate::BitReader<ADS06_A>;
impl ADS06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS06_A {
        match self.bits {
            false => ADS06_A::_0,
            true => ADS06_A::_1,
        }
    }
    ///AN106 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS06_A::_0
    }
    ///AN106 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS06_A::_1
    }
}
///Field `ADS06` writer - A/D-Converted Value Addition/Average Channel AN106 Select
pub type ADS06_W<'a, REG> = crate::BitWriter<'a, REG, ADS06_A>;
impl<'a, REG> ADS06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN106 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS06_A::_0)
    }
    ///AN106 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS06_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN107 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS07_A {
    ///0: AN107 is not selected.
    _0 = 0,
    ///1: AN107 is selected.
    _1 = 1,
}
impl From<ADS07_A> for bool {
    #[inline(always)]
    fn from(variant: ADS07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS07` reader - A/D-Converted Value Addition/Average Channel AN107 Select
pub type ADS07_R = crate::BitReader<ADS07_A>;
impl ADS07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS07_A {
        match self.bits {
            false => ADS07_A::_0,
            true => ADS07_A::_1,
        }
    }
    ///AN107 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS07_A::_0
    }
    ///AN107 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS07_A::_1
    }
}
///Field `ADS07` writer - A/D-Converted Value Addition/Average Channel AN107 Select
pub type ADS07_W<'a, REG> = crate::BitWriter<'a, REG, ADS07_A>;
impl<'a, REG> ADS07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN107 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS07_A::_0)
    }
    ///AN107 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS07_A::_1)
    }
}
impl R {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN100 Select
    #[inline(always)]
    pub fn ads00(&self) -> ADS00_R {
        ADS00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN101 Select
    #[inline(always)]
    pub fn ads01(&self) -> ADS01_R {
        ADS01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN102 Select
    #[inline(always)]
    pub fn ads02(&self) -> ADS02_R {
        ADS02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN103 Select
    #[inline(always)]
    pub fn ads03(&self) -> ADS03_R {
        ADS03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel AN105 Select
    #[inline(always)]
    pub fn ads05(&self) -> ADS05_R {
        ADS05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel AN106 Select
    #[inline(always)]
    pub fn ads06(&self) -> ADS06_R {
        ADS06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel AN107 Select
    #[inline(always)]
    pub fn ads07(&self) -> ADS07_R {
        ADS07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN100 Select
    #[inline(always)]
    pub fn ads00(&mut self) -> ADS00_W<ADADS0_SPEC> {
        ADS00_W::new(self, 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN101 Select
    #[inline(always)]
    pub fn ads01(&mut self) -> ADS01_W<ADADS0_SPEC> {
        ADS01_W::new(self, 1)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN102 Select
    #[inline(always)]
    pub fn ads02(&mut self) -> ADS02_W<ADADS0_SPEC> {
        ADS02_W::new(self, 2)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN103 Select
    #[inline(always)]
    pub fn ads03(&mut self) -> ADS03_W<ADADS0_SPEC> {
        ADS03_W::new(self, 3)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel AN105 Select
    #[inline(always)]
    pub fn ads05(&mut self) -> ADS05_W<ADADS0_SPEC> {
        ADS05_W::new(self, 5)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel AN106 Select
    #[inline(always)]
    pub fn ads06(&mut self) -> ADS06_W<ADADS0_SPEC> {
        ADS06_W::new(self, 6)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel AN107 Select
    #[inline(always)]
    pub fn ads07(&mut self) -> ADS07_W<ADADS0_SPEC> {
        ADS07_W::new(self, 7)
    }
}
/**A/D-Converted Value Addition/Average Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adads0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADADS0_SPEC;
impl crate::RegisterSpec for ADADS0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adads0::R`](R) reader structure
impl crate::Readable for ADADS0_SPEC {}
///`write(|w| ..)` method takes [`adads0::W`](W) writer structure
impl crate::Writable for ADADS0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADADS0 to value 0
impl crate::Resettable for ADADS0_SPEC {}
