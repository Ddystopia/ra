///Register `ADANSB0` reader
pub type R = crate::R<ADANSB0_SPEC>;
///Register `ADANSB0` writer
pub type W = crate::W<ADANSB0_SPEC>;
/**AN100 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB00_A {
    ///0: AN100 is not subjected to conversion.
    _0 = 0,
    ///1: AN100 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB00_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB00` reader - AN100 Select
pub type ANSB00_R = crate::BitReader<ANSB00_A>;
impl ANSB00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB00_A {
        match self.bits {
            false => ANSB00_A::_0,
            true => ANSB00_A::_1,
        }
    }
    ///AN100 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB00_A::_0
    }
    ///AN100 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB00_A::_1
    }
}
///Field `ANSB00` writer - AN100 Select
pub type ANSB00_W<'a, REG> = crate::BitWriter<'a, REG, ANSB00_A>;
impl<'a, REG> ANSB00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN100 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB00_A::_0)
    }
    ///AN100 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB00_A::_1)
    }
}
/**AN101 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB01_A {
    ///0: AN101 is not subjected to conversion.
    _0 = 0,
    ///1: AN101 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB01_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB01` reader - AN101 Select
pub type ANSB01_R = crate::BitReader<ANSB01_A>;
impl ANSB01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB01_A {
        match self.bits {
            false => ANSB01_A::_0,
            true => ANSB01_A::_1,
        }
    }
    ///AN101 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB01_A::_0
    }
    ///AN101 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB01_A::_1
    }
}
///Field `ANSB01` writer - AN101 Select
pub type ANSB01_W<'a, REG> = crate::BitWriter<'a, REG, ANSB01_A>;
impl<'a, REG> ANSB01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN101 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB01_A::_0)
    }
    ///AN101 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB01_A::_1)
    }
}
/**AN102 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB02_A {
    ///0: AN102 is not subjected to conversion.
    _0 = 0,
    ///1: AN102 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB02_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB02` reader - AN102 Select
pub type ANSB02_R = crate::BitReader<ANSB02_A>;
impl ANSB02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB02_A {
        match self.bits {
            false => ANSB02_A::_0,
            true => ANSB02_A::_1,
        }
    }
    ///AN102 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB02_A::_0
    }
    ///AN102 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB02_A::_1
    }
}
///Field `ANSB02` writer - AN102 Select
pub type ANSB02_W<'a, REG> = crate::BitWriter<'a, REG, ANSB02_A>;
impl<'a, REG> ANSB02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN102 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB02_A::_0)
    }
    ///AN102 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB02_A::_1)
    }
}
/**AN103 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB03_A {
    ///0: AN103 is not subjected to conversion.
    _0 = 0,
    ///1: AN103 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB03_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB03` reader - AN103 Select
pub type ANSB03_R = crate::BitReader<ANSB03_A>;
impl ANSB03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB03_A {
        match self.bits {
            false => ANSB03_A::_0,
            true => ANSB03_A::_1,
        }
    }
    ///AN103 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB03_A::_0
    }
    ///AN103 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB03_A::_1
    }
}
///Field `ANSB03` writer - AN103 Select
pub type ANSB03_W<'a, REG> = crate::BitWriter<'a, REG, ANSB03_A>;
impl<'a, REG> ANSB03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN103 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB03_A::_0)
    }
    ///AN103 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB03_A::_1)
    }
}
/**AN105 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB05_A {
    ///0: AN105 is not subjected to conversion.
    _0 = 0,
    ///1: AN105 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB05_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB05` reader - AN105 Select
pub type ANSB05_R = crate::BitReader<ANSB05_A>;
impl ANSB05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB05_A {
        match self.bits {
            false => ANSB05_A::_0,
            true => ANSB05_A::_1,
        }
    }
    ///AN105 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB05_A::_0
    }
    ///AN105 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB05_A::_1
    }
}
///Field `ANSB05` writer - AN105 Select
pub type ANSB05_W<'a, REG> = crate::BitWriter<'a, REG, ANSB05_A>;
impl<'a, REG> ANSB05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN105 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB05_A::_0)
    }
    ///AN105 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB05_A::_1)
    }
}
/**AN106 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB06_A {
    ///0: AN106 is not subjected to conversion.
    _0 = 0,
    ///1: AN106 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB06_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB06` reader - AN106 Select
pub type ANSB06_R = crate::BitReader<ANSB06_A>;
impl ANSB06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB06_A {
        match self.bits {
            false => ANSB06_A::_0,
            true => ANSB06_A::_1,
        }
    }
    ///AN106 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB06_A::_0
    }
    ///AN106 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB06_A::_1
    }
}
///Field `ANSB06` writer - AN106 Select
pub type ANSB06_W<'a, REG> = crate::BitWriter<'a, REG, ANSB06_A>;
impl<'a, REG> ANSB06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN106 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB06_A::_0)
    }
    ///AN106 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB06_A::_1)
    }
}
/**AN107 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANSB07_A {
    ///0: AN107 is not subjected to conversion.
    _0 = 0,
    ///1: AN107 is subjected to conversion.
    _1 = 1,
}
impl From<ANSB07_A> for bool {
    #[inline(always)]
    fn from(variant: ANSB07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANSB07` reader - AN107 Select
pub type ANSB07_R = crate::BitReader<ANSB07_A>;
impl ANSB07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANSB07_A {
        match self.bits {
            false => ANSB07_A::_0,
            true => ANSB07_A::_1,
        }
    }
    ///AN107 is not subjected to conversion.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANSB07_A::_0
    }
    ///AN107 is subjected to conversion.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANSB07_A::_1
    }
}
///Field `ANSB07` writer - AN107 Select
pub type ANSB07_W<'a, REG> = crate::BitWriter<'a, REG, ANSB07_A>;
impl<'a, REG> ANSB07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AN107 is not subjected to conversion.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB07_A::_0)
    }
    ///AN107 is subjected to conversion.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ANSB07_A::_1)
    }
}
impl R {
    ///Bit 0 - AN100 Select
    #[inline(always)]
    pub fn ansb00(&self) -> ANSB00_R {
        ANSB00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AN101 Select
    #[inline(always)]
    pub fn ansb01(&self) -> ANSB01_R {
        ANSB01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AN102 Select
    #[inline(always)]
    pub fn ansb02(&self) -> ANSB02_R {
        ANSB02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AN103 Select
    #[inline(always)]
    pub fn ansb03(&self) -> ANSB03_R {
        ANSB03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - AN105 Select
    #[inline(always)]
    pub fn ansb05(&self) -> ANSB05_R {
        ANSB05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AN106 Select
    #[inline(always)]
    pub fn ansb06(&self) -> ANSB06_R {
        ANSB06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AN107 Select
    #[inline(always)]
    pub fn ansb07(&self) -> ANSB07_R {
        ANSB07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - AN100 Select
    #[inline(always)]
    pub fn ansb00(&mut self) -> ANSB00_W<ADANSB0_SPEC> {
        ANSB00_W::new(self, 0)
    }
    ///Bit 1 - AN101 Select
    #[inline(always)]
    pub fn ansb01(&mut self) -> ANSB01_W<ADANSB0_SPEC> {
        ANSB01_W::new(self, 1)
    }
    ///Bit 2 - AN102 Select
    #[inline(always)]
    pub fn ansb02(&mut self) -> ANSB02_W<ADANSB0_SPEC> {
        ANSB02_W::new(self, 2)
    }
    ///Bit 3 - AN103 Select
    #[inline(always)]
    pub fn ansb03(&mut self) -> ANSB03_W<ADANSB0_SPEC> {
        ANSB03_W::new(self, 3)
    }
    ///Bit 5 - AN105 Select
    #[inline(always)]
    pub fn ansb05(&mut self) -> ANSB05_W<ADANSB0_SPEC> {
        ANSB05_W::new(self, 5)
    }
    ///Bit 6 - AN106 Select
    #[inline(always)]
    pub fn ansb06(&mut self) -> ANSB06_W<ADANSB0_SPEC> {
        ANSB06_W::new(self, 6)
    }
    ///Bit 7 - AN107 Select
    #[inline(always)]
    pub fn ansb07(&mut self) -> ANSB07_W<ADANSB0_SPEC> {
        ANSB07_W::new(self, 7)
    }
}
/**A/D Channel Select Register B0

You can [`read`](crate::Reg::read) this register and get [`adansb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADANSB0_SPEC;
impl crate::RegisterSpec for ADANSB0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adansb0::R`](R) reader structure
impl crate::Readable for ADANSB0_SPEC {}
///`write(|w| ..)` method takes [`adansb0::W`](W) writer structure
impl crate::Writable for ADANSB0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADANSB0 to value 0
impl crate::Resettable for ADANSB0_SPEC {}
