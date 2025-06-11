///Register `ADADS1` reader
pub type R = crate::R<ADADS1_SPEC>;
///Register `ADADS1` writer
pub type W = crate::W<ADADS1_SPEC>;
/**A/D-Converted Value Addition/Average Channel AN016 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS16_A {
    ///0: AN016 is not selected.
    _0 = 0,
    ///1: AN016 is selected.
    _1 = 1,
}
impl From<ADS16_A> for bool {
    #[inline(always)]
    fn from(variant: ADS16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS16` reader - A/D-Converted Value Addition/Average Channel AN016 Select
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
    ///AN016 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS16_A::_0
    }
    ///AN016 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS16_A::_1
    }
}
///Field `ADS16` writer - A/D-Converted Value Addition/Average Channel AN016 Select
pub type ADS16_W<'a, REG> = crate::BitWriter<'a, REG, ADS16_A>;
impl<'a, REG> ADS16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN016 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS16_A::_0)
    }
    ///AN016 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS16_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN017 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS17_A {
    ///0: AN017 is not selected.
    _0 = 0,
    ///1: AN017 is selected.
    _1 = 1,
}
impl From<ADS17_A> for bool {
    #[inline(always)]
    fn from(variant: ADS17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS17` reader - A/D-Converted Value Addition/Average Channel AN017 Select
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
    ///AN017 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS17_A::_0
    }
    ///AN017 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS17_A::_1
    }
}
///Field `ADS17` writer - A/D-Converted Value Addition/Average Channel AN017 Select
pub type ADS17_W<'a, REG> = crate::BitWriter<'a, REG, ADS17_A>;
impl<'a, REG> ADS17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN017 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS17_A::_0)
    }
    ///AN017 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS17_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN018 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS18_A {
    ///0: AN018 is not selected.
    _0 = 0,
    ///1: AN018 is selected.
    _1 = 1,
}
impl From<ADS18_A> for bool {
    #[inline(always)]
    fn from(variant: ADS18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS18` reader - A/D-Converted Value Addition/Average Channel AN018 Select
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
    ///AN018 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS18_A::_0
    }
    ///AN018 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS18_A::_1
    }
}
///Field `ADS18` writer - A/D-Converted Value Addition/Average Channel AN018 Select
pub type ADS18_W<'a, REG> = crate::BitWriter<'a, REG, ADS18_A>;
impl<'a, REG> ADS18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN018 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS18_A::_0)
    }
    ///AN018 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS18_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN019 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS19_A {
    ///0: AN019 is not selected.
    _0 = 0,
    ///1: AN019 is selected.
    _1 = 1,
}
impl From<ADS19_A> for bool {
    #[inline(always)]
    fn from(variant: ADS19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS19` reader - A/D-Converted Value Addition/Average Channel AN019 Select
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
    ///AN019 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS19_A::_0
    }
    ///AN019 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS19_A::_1
    }
}
///Field `ADS19` writer - A/D-Converted Value Addition/Average Channel AN019 Select
pub type ADS19_W<'a, REG> = crate::BitWriter<'a, REG, ADS19_A>;
impl<'a, REG> ADS19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN019 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS19_A::_0)
    }
    ///AN019 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS19_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN020 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS20_A {
    ///0: AN020 is not selected.
    _0 = 0,
    ///1: AN020 is selected.
    _1 = 1,
}
impl From<ADS20_A> for bool {
    #[inline(always)]
    fn from(variant: ADS20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS20` reader - A/D-Converted Value Addition/Average Channel AN020 Select
pub type ADS20_R = crate::BitReader<ADS20_A>;
impl ADS20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS20_A {
        match self.bits {
            false => ADS20_A::_0,
            true => ADS20_A::_1,
        }
    }
    ///AN020 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS20_A::_0
    }
    ///AN020 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS20_A::_1
    }
}
///Field `ADS20` writer - A/D-Converted Value Addition/Average Channel AN020 Select
pub type ADS20_W<'a, REG> = crate::BitWriter<'a, REG, ADS20_A>;
impl<'a, REG> ADS20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN020 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS20_A::_0)
    }
    ///AN020 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS20_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN021 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS21_A {
    ///0: AN021 is not selected.
    _0 = 0,
    ///1: AN021 is selected.
    _1 = 1,
}
impl From<ADS21_A> for bool {
    #[inline(always)]
    fn from(variant: ADS21_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS21` reader - A/D-Converted Value Addition/Average Channel AN021 Select
pub type ADS21_R = crate::BitReader<ADS21_A>;
impl ADS21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS21_A {
        match self.bits {
            false => ADS21_A::_0,
            true => ADS21_A::_1,
        }
    }
    ///AN021 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS21_A::_0
    }
    ///AN021 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS21_A::_1
    }
}
///Field `ADS21` writer - A/D-Converted Value Addition/Average Channel AN021 Select
pub type ADS21_W<'a, REG> = crate::BitWriter<'a, REG, ADS21_A>;
impl<'a, REG> ADS21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN021 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS21_A::_0)
    }
    ///AN021 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS21_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN022 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS22_A {
    ///0: AN022 is not selected.
    _0 = 0,
    ///1: AN022 is selected.
    _1 = 1,
}
impl From<ADS22_A> for bool {
    #[inline(always)]
    fn from(variant: ADS22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS22` reader - A/D-Converted Value Addition/Average Channel AN022 Select
pub type ADS22_R = crate::BitReader<ADS22_A>;
impl ADS22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS22_A {
        match self.bits {
            false => ADS22_A::_0,
            true => ADS22_A::_1,
        }
    }
    ///AN022 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS22_A::_0
    }
    ///AN022 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS22_A::_1
    }
}
///Field `ADS22` writer - A/D-Converted Value Addition/Average Channel AN022 Select
pub type ADS22_W<'a, REG> = crate::BitWriter<'a, REG, ADS22_A>;
impl<'a, REG> ADS22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN022 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS22_A::_0)
    }
    ///AN022 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS22_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN023 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS23_A {
    ///0: AN023 is not selected.
    _0 = 0,
    ///1: AN023 is selected.
    _1 = 1,
}
impl From<ADS23_A> for bool {
    #[inline(always)]
    fn from(variant: ADS23_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS23` reader - A/D-Converted Value Addition/Average Channel AN023 Select
pub type ADS23_R = crate::BitReader<ADS23_A>;
impl ADS23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS23_A {
        match self.bits {
            false => ADS23_A::_0,
            true => ADS23_A::_1,
        }
    }
    ///AN023 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS23_A::_0
    }
    ///AN023 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS23_A::_1
    }
}
///Field `ADS23` writer - A/D-Converted Value Addition/Average Channel AN023 Select
pub type ADS23_W<'a, REG> = crate::BitWriter<'a, REG, ADS23_A>;
impl<'a, REG> ADS23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN023 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS23_A::_0)
    }
    ///AN023 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS23_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN024 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS24_A {
    ///0: AN024 is not selected.
    _0 = 0,
    ///1: AN024 is selected.
    _1 = 1,
}
impl From<ADS24_A> for bool {
    #[inline(always)]
    fn from(variant: ADS24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS24` reader - A/D-Converted Value Addition/Average Channel AN024 Select
pub type ADS24_R = crate::BitReader<ADS24_A>;
impl ADS24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS24_A {
        match self.bits {
            false => ADS24_A::_0,
            true => ADS24_A::_1,
        }
    }
    ///AN024 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS24_A::_0
    }
    ///AN024 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS24_A::_1
    }
}
///Field `ADS24` writer - A/D-Converted Value Addition/Average Channel AN024 Select
pub type ADS24_W<'a, REG> = crate::BitWriter<'a, REG, ADS24_A>;
impl<'a, REG> ADS24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN024 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS24_A::_0)
    }
    ///AN024 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS24_A::_1)
    }
}
/**A/D-Converted Value Addition/Average Channel AN025 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADS25_A {
    ///0: AN025 is not selected.
    _0 = 0,
    ///1: AN025 is selected.
    _1 = 1,
}
impl From<ADS25_A> for bool {
    #[inline(always)]
    fn from(variant: ADS25_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADS25` reader - A/D-Converted Value Addition/Average Channel AN025 Select
pub type ADS25_R = crate::BitReader<ADS25_A>;
impl ADS25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADS25_A {
        match self.bits {
            false => ADS25_A::_0,
            true => ADS25_A::_1,
        }
    }
    ///AN025 is not selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADS25_A::_0
    }
    ///AN025 is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADS25_A::_1
    }
}
///Field `ADS25` writer - A/D-Converted Value Addition/Average Channel AN025 Select
pub type ADS25_W<'a, REG> = crate::BitWriter<'a, REG, ADS25_A>;
impl<'a, REG> ADS25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN025 is not selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ADS25_A::_0)
    }
    ///AN025 is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ADS25_A::_1)
    }
}
impl R {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN016 Select
    #[inline(always)]
    pub fn ads16(&self) -> ADS16_R {
        ADS16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN017 Select
    #[inline(always)]
    pub fn ads17(&self) -> ADS17_R {
        ADS17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN018 Select
    #[inline(always)]
    pub fn ads18(&self) -> ADS18_R {
        ADS18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN019 Select
    #[inline(always)]
    pub fn ads19(&self) -> ADS19_R {
        ADS19_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A/D-Converted Value Addition/Average Channel AN020 Select
    #[inline(always)]
    pub fn ads20(&self) -> ADS20_R {
        ADS20_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel AN021 Select
    #[inline(always)]
    pub fn ads21(&self) -> ADS21_R {
        ADS21_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel AN022 Select
    #[inline(always)]
    pub fn ads22(&self) -> ADS22_R {
        ADS22_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel AN023 Select
    #[inline(always)]
    pub fn ads23(&self) -> ADS23_R {
        ADS23_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - A/D-Converted Value Addition/Average Channel AN024 Select
    #[inline(always)]
    pub fn ads24(&self) -> ADS24_R {
        ADS24_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - A/D-Converted Value Addition/Average Channel AN025 Select
    #[inline(always)]
    pub fn ads25(&self) -> ADS25_R {
        ADS25_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D-Converted Value Addition/Average Channel AN016 Select
    #[inline(always)]
    pub fn ads16(&mut self) -> ADS16_W<ADADS1_SPEC> {
        ADS16_W::new(self, 0)
    }
    ///Bit 1 - A/D-Converted Value Addition/Average Channel AN017 Select
    #[inline(always)]
    pub fn ads17(&mut self) -> ADS17_W<ADADS1_SPEC> {
        ADS17_W::new(self, 1)
    }
    ///Bit 2 - A/D-Converted Value Addition/Average Channel AN018 Select
    #[inline(always)]
    pub fn ads18(&mut self) -> ADS18_W<ADADS1_SPEC> {
        ADS18_W::new(self, 2)
    }
    ///Bit 3 - A/D-Converted Value Addition/Average Channel AN019 Select
    #[inline(always)]
    pub fn ads19(&mut self) -> ADS19_W<ADADS1_SPEC> {
        ADS19_W::new(self, 3)
    }
    ///Bit 4 - A/D-Converted Value Addition/Average Channel AN020 Select
    #[inline(always)]
    pub fn ads20(&mut self) -> ADS20_W<ADADS1_SPEC> {
        ADS20_W::new(self, 4)
    }
    ///Bit 5 - A/D-Converted Value Addition/Average Channel AN021 Select
    #[inline(always)]
    pub fn ads21(&mut self) -> ADS21_W<ADADS1_SPEC> {
        ADS21_W::new(self, 5)
    }
    ///Bit 6 - A/D-Converted Value Addition/Average Channel AN022 Select
    #[inline(always)]
    pub fn ads22(&mut self) -> ADS22_W<ADADS1_SPEC> {
        ADS22_W::new(self, 6)
    }
    ///Bit 7 - A/D-Converted Value Addition/Average Channel AN023 Select
    #[inline(always)]
    pub fn ads23(&mut self) -> ADS23_W<ADADS1_SPEC> {
        ADS23_W::new(self, 7)
    }
    ///Bit 8 - A/D-Converted Value Addition/Average Channel AN024 Select
    #[inline(always)]
    pub fn ads24(&mut self) -> ADS24_W<ADADS1_SPEC> {
        ADS24_W::new(self, 8)
    }
    ///Bit 9 - A/D-Converted Value Addition/Average Channel AN025 Select
    #[inline(always)]
    pub fn ads25(&mut self) -> ADS25_W<ADADS1_SPEC> {
        ADS25_W::new(self, 9)
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
