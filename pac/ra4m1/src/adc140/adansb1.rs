///Register `ADANSB1` reader
pub type R = crate::R<ADANSB1_SPEC>;
///Register `ADANSB1` writer
pub type W = crate::W<ADANSB1_SPEC>;
/**AN016 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB16_A {
    ///0: AN016 is not subjected to conversion.
    _0 = 0,
    ///1: AN016 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB16_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB16_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB16` reader - AN016 Select
pub type ANSB16_R = crate::BitReader<ANSB16_A>;
impl ANSB16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB16_A {
        match self.bits {
            false => ANSB16_A::_0,
            true => ANSB16_A::_1,
        }
    }
    ///AN016 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB16_A::_0
    }
    ///AN016 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB16_A::_1
    }
}
///Field `ANSB16` writer - AN016 Select
pub type ANSB16_W<'a, REG> = crate::BitWriter<'a, REG, ANSB16_A>;
impl<'a, REG> ANSB16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN016 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB16_A::_0)
    }
    ///AN016 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB16_A::_1)
    }
}
/**AN017 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB17_A {
    ///0: AN017 is not subjected to conversion.
    _0 = 0,
    ///1: AN017 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB17_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB17_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB17` reader - AN017 Select
pub type ANSB17_R = crate::BitReader<ANSB17_A>;
impl ANSB17_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB17_A {
        match self.bits {
            false => ANSB17_A::_0,
            true => ANSB17_A::_1,
        }
    }
    ///AN017 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB17_A::_0
    }
    ///AN017 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB17_A::_1
    }
}
///Field `ANSB17` writer - AN017 Select
pub type ANSB17_W<'a, REG> = crate::BitWriter<'a, REG, ANSB17_A>;
impl<'a, REG> ANSB17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN017 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB17_A::_0)
    }
    ///AN017 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB17_A::_1)
    }
}
/**AN018 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB18_A {
    ///0: AN018 is not subjected to conversion.
    _0 = 0,
    ///1: AN018 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB18_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB18_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB18` reader - AN018 Select
pub type ANSB18_R = crate::BitReader<ANSB18_A>;
impl ANSB18_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB18_A {
        match self.bits {
            false => ANSB18_A::_0,
            true => ANSB18_A::_1,
        }
    }
    ///AN018 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB18_A::_0
    }
    ///AN018 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB18_A::_1
    }
}
///Field `ANSB18` writer - AN018 Select
pub type ANSB18_W<'a, REG> = crate::BitWriter<'a, REG, ANSB18_A>;
impl<'a, REG> ANSB18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN018 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB18_A::_0)
    }
    ///AN018 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB18_A::_1)
    }
}
/**AN019 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB19_A {
    ///0: AN019 is not subjected to conversion.
    _0 = 0,
    ///1: AN019 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB19_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB19_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB19` reader - AN019 Select
pub type ANSB19_R = crate::BitReader<ANSB19_A>;
impl ANSB19_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB19_A {
        match self.bits {
            false => ANSB19_A::_0,
            true => ANSB19_A::_1,
        }
    }
    ///AN019 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB19_A::_0
    }
    ///AN019 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB19_A::_1
    }
}
///Field `ANSB19` writer - AN019 Select
pub type ANSB19_W<'a, REG> = crate::BitWriter<'a, REG, ANSB19_A>;
impl<'a, REG> ANSB19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN019 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB19_A::_0)
    }
    ///AN019 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB19_A::_1)
    }
}
/**AN020 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB20_A {
    ///0: AN020 is not subjected to conversion.
    _0 = 0,
    ///1: AN020 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB20_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB20_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB20` reader - AN020 Select
pub type ANSB20_R = crate::BitReader<ANSB20_A>;
impl ANSB20_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB20_A {
        match self.bits {
            false => ANSB20_A::_0,
            true => ANSB20_A::_1,
        }
    }
    ///AN020 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB20_A::_0
    }
    ///AN020 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB20_A::_1
    }
}
///Field `ANSB20` writer - AN020 Select
pub type ANSB20_W<'a, REG> = crate::BitWriter<'a, REG, ANSB20_A>;
impl<'a, REG> ANSB20_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN020 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB20_A::_0)
    }
    ///AN020 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB20_A::_1)
    }
}
/**AN021 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB21_A {
    ///0: AN021 is not subjected to conversion.
    _0 = 0,
    ///1: AN021 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB21_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB21_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB21` reader - AN021 Select
pub type ANSB21_R = crate::BitReader<ANSB21_A>;
impl ANSB21_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB21_A {
        match self.bits {
            false => ANSB21_A::_0,
            true => ANSB21_A::_1,
        }
    }
    ///AN021 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB21_A::_0
    }
    ///AN021 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB21_A::_1
    }
}
///Field `ANSB21` writer - AN021 Select
pub type ANSB21_W<'a, REG> = crate::BitWriter<'a, REG, ANSB21_A>;
impl<'a, REG> ANSB21_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN021 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB21_A::_0)
    }
    ///AN021 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB21_A::_1)
    }
}
/**AN022 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB22_A {
    ///0: AN022 is not subjected to conversion.
    _0 = 0,
    ///1: AN022 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB22_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB22_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB22` reader - AN022 Select
pub type ANSB22_R = crate::BitReader<ANSB22_A>;
impl ANSB22_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB22_A {
        match self.bits {
            false => ANSB22_A::_0,
            true => ANSB22_A::_1,
        }
    }
    ///AN022 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB22_A::_0
    }
    ///AN022 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB22_A::_1
    }
}
///Field `ANSB22` writer - AN022 Select
pub type ANSB22_W<'a, REG> = crate::BitWriter<'a, REG, ANSB22_A>;
impl<'a, REG> ANSB22_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN022 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB22_A::_0)
    }
    ///AN022 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB22_A::_1)
    }
}
/**AN023 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB23_A {
    ///0: AN023 is not subjected to conversion.
    _0 = 0,
    ///1: AN023 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB23_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB23_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB23` reader - AN023 Select
pub type ANSB23_R = crate::BitReader<ANSB23_A>;
impl ANSB23_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB23_A {
        match self.bits {
            false => ANSB23_A::_0,
            true => ANSB23_A::_1,
        }
    }
    ///AN023 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB23_A::_0
    }
    ///AN023 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB23_A::_1
    }
}
///Field `ANSB23` writer - AN023 Select
pub type ANSB23_W<'a, REG> = crate::BitWriter<'a, REG, ANSB23_A>;
impl<'a, REG> ANSB23_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN023 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB23_A::_0)
    }
    ///AN023 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB23_A::_1)
    }
}
/**AN024 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB24_A {
    ///0: AN024 is not subjected to conversion.
    _0 = 0,
    ///1: AN024 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB24_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB24_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB24` reader - AN024 Select
pub type ANSB24_R = crate::BitReader<ANSB24_A>;
impl ANSB24_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB24_A {
        match self.bits {
            false => ANSB24_A::_0,
            true => ANSB24_A::_1,
        }
    }
    ///AN024 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB24_A::_0
    }
    ///AN024 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB24_A::_1
    }
}
///Field `ANSB24` writer - AN024 Select
pub type ANSB24_W<'a, REG> = crate::BitWriter<'a, REG, ANSB24_A>;
impl<'a, REG> ANSB24_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN024 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB24_A::_0)
    }
    ///AN024 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB24_A::_1)
    }
}
/**AN025 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB25_A {
    ///0: AN025 is not subjected to conversion.
    _0 = 0,
    ///1: AN025 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB25_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB25_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB25` reader - AN025 Select
pub type ANSB25_R = crate::BitReader<ANSB25_A>;
impl ANSB25_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB25_A {
        match self.bits {
            false => ANSB25_A::_0,
            true => ANSB25_A::_1,
        }
    }
    ///AN025 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB25_A::_0
    }
    ///AN025 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB25_A::_1
    }
}
///Field `ANSB25` writer - AN025 Select
pub type ANSB25_W<'a, REG> = crate::BitWriter<'a, REG, ANSB25_A>;
impl<'a, REG> ANSB25_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN025 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB25_A::_0)
    }
    ///AN025 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB25_A::_1)
    }
}
impl R {
    ///Bit 0 - AN016 Select
    #[inline(always)]
    pub fn ansb16(&self) -> ANSB16_R {
        ANSB16_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN017 Select
    #[inline(always)]
    pub fn ansb17(&self) -> ANSB17_R {
        ANSB17_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN018 Select
    #[inline(always)]
    pub fn ansb18(&self) -> ANSB18_R {
        ANSB18_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN019 Select
    #[inline(always)]
    pub fn ansb19(&self) -> ANSB19_R {
        ANSB19_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AN020 Select
    #[inline(always)]
    pub fn ansb20(&self) -> ANSB20_R {
        ANSB20_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AN021 Select
    #[inline(always)]
    pub fn ansb21(&self) -> ANSB21_R {
        ANSB21_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AN022 Select
    #[inline(always)]
    pub fn ansb22(&self) -> ANSB22_R {
        ANSB22_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AN023 Select
    #[inline(always)]
    pub fn ansb23(&self) -> ANSB23_R {
        ANSB23_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - AN024 Select
    #[inline(always)]
    pub fn ansb24(&self) -> ANSB24_R {
        ANSB24_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - AN025 Select
    #[inline(always)]
    pub fn ansb25(&self) -> ANSB25_R {
        ANSB25_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN016 Select
    #[inline(always)]
    pub fn ansb16(&mut self) -> ANSB16_W<ADANSB1_SPEC> {
        ANSB16_W::new(self, 0)
    }
    ///Bit 1 - AN017 Select
    #[inline(always)]
    pub fn ansb17(&mut self) -> ANSB17_W<ADANSB1_SPEC> {
        ANSB17_W::new(self, 1)
    }
    ///Bit 2 - AN018 Select
    #[inline(always)]
    pub fn ansb18(&mut self) -> ANSB18_W<ADANSB1_SPEC> {
        ANSB18_W::new(self, 2)
    }
    ///Bit 3 - AN019 Select
    #[inline(always)]
    pub fn ansb19(&mut self) -> ANSB19_W<ADANSB1_SPEC> {
        ANSB19_W::new(self, 3)
    }
    ///Bit 4 - AN020 Select
    #[inline(always)]
    pub fn ansb20(&mut self) -> ANSB20_W<ADANSB1_SPEC> {
        ANSB20_W::new(self, 4)
    }
    ///Bit 5 - AN021 Select
    #[inline(always)]
    pub fn ansb21(&mut self) -> ANSB21_W<ADANSB1_SPEC> {
        ANSB21_W::new(self, 5)
    }
    ///Bit 6 - AN022 Select
    #[inline(always)]
    pub fn ansb22(&mut self) -> ANSB22_W<ADANSB1_SPEC> {
        ANSB22_W::new(self, 6)
    }
    ///Bit 7 - AN023 Select
    #[inline(always)]
    pub fn ansb23(&mut self) -> ANSB23_W<ADANSB1_SPEC> {
        ANSB23_W::new(self, 7)
    }
    ///Bit 8 - AN024 Select
    #[inline(always)]
    pub fn ansb24(&mut self) -> ANSB24_W<ADANSB1_SPEC> {
        ANSB24_W::new(self, 8)
    }
    ///Bit 9 - AN025 Select
    #[inline(always)]
    pub fn ansb25(&mut self) -> ANSB25_W<ADANSB1_SPEC> {
        ANSB25_W::new(self, 9)
    }
}
/**A/D Channel Select Register B1

You can [`read`](crate::Reg::read) this register and get [`adansb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADANSB1_SPEC;
impl crate::RegisterSpec for ADANSB1_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adansb1::R`](R) reader structure
impl crate::Readable for ADANSB1_SPEC {}
///`write(|w| ..)` method takes [`adansb1::W`](W) writer structure
impl crate::Writable for ADANSB1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSB1 to value 0
impl crate::Resettable for ADANSB1_SPEC {}
