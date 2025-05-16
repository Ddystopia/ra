///Register `ADCMPANSR0` reader
pub type R = crate::R<ADCMPANSR0_SPEC>;
///Register `ADCMPANSR0` writer
pub type W = crate::W<ADCMPANSR0_SPEC>;
/**Compare Window A Channel AN000 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA00_A {
    ///0: Disable compare function for AN000
    _0 = 0,
    ///1: Enable compare function for AN000
    _1 = 1,
}
impl From<CMPCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA00_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA00` reader - Compare Window A Channel AN000 Select
pub type CMPCHA00_R = crate::BitReader<CMPCHA00_A>;
impl CMPCHA00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA00_A {
        match self.bits {
            false => CMPCHA00_A::_0,
            true => CMPCHA00_A::_1,
        }
    }
    ///Disable compare function for AN000
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA00_A::_0
    }
    ///Enable compare function for AN000
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA00_A::_1
    }
}
///Field `CMPCHA00` writer - Compare Window A Channel AN000 Select
pub type CMPCHA00_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA00_A>;
impl<'a, REG> CMPCHA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN000
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA00_A::_0)
    }
    ///Enable compare function for AN000
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA00_A::_1)
    }
}
/**Compare Window A Channel AN001 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA01_A {
    ///0: Disable compare function for AN001
    _0 = 0,
    ///1: Enable compare function for AN001
    _1 = 1,
}
impl From<CMPCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA01_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA01` reader - Compare Window A Channel AN001 Select
pub type CMPCHA01_R = crate::BitReader<CMPCHA01_A>;
impl CMPCHA01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA01_A {
        match self.bits {
            false => CMPCHA01_A::_0,
            true => CMPCHA01_A::_1,
        }
    }
    ///Disable compare function for AN001
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA01_A::_0
    }
    ///Enable compare function for AN001
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA01_A::_1
    }
}
///Field `CMPCHA01` writer - Compare Window A Channel AN001 Select
pub type CMPCHA01_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA01_A>;
impl<'a, REG> CMPCHA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN001
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA01_A::_0)
    }
    ///Enable compare function for AN001
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA01_A::_1)
    }
}
/**Compare Window A Channel AN002 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA02_A {
    ///0: Disable compare function for AN002
    _0 = 0,
    ///1: Enable compare function for AN002
    _1 = 1,
}
impl From<CMPCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA02_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA02` reader - Compare Window A Channel AN002 Select
pub type CMPCHA02_R = crate::BitReader<CMPCHA02_A>;
impl CMPCHA02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA02_A {
        match self.bits {
            false => CMPCHA02_A::_0,
            true => CMPCHA02_A::_1,
        }
    }
    ///Disable compare function for AN002
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA02_A::_0
    }
    ///Enable compare function for AN002
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA02_A::_1
    }
}
///Field `CMPCHA02` writer - Compare Window A Channel AN002 Select
pub type CMPCHA02_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA02_A>;
impl<'a, REG> CMPCHA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN002
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA02_A::_0)
    }
    ///Enable compare function for AN002
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA02_A::_1)
    }
}
/**Compare Window A Channel AN003 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA03_A {
    ///0: Disable compare function for AN003
    _0 = 0,
    ///1: Enable compare function for AN003
    _1 = 1,
}
impl From<CMPCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA03_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA03` reader - Compare Window A Channel AN003 Select
pub type CMPCHA03_R = crate::BitReader<CMPCHA03_A>;
impl CMPCHA03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA03_A {
        match self.bits {
            false => CMPCHA03_A::_0,
            true => CMPCHA03_A::_1,
        }
    }
    ///Disable compare function for AN003
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA03_A::_0
    }
    ///Enable compare function for AN003
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA03_A::_1
    }
}
///Field `CMPCHA03` writer - Compare Window A Channel AN003 Select
pub type CMPCHA03_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA03_A>;
impl<'a, REG> CMPCHA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN003
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA03_A::_0)
    }
    ///Enable compare function for AN003
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA03_A::_1)
    }
}
/**Compare Window A Channel AN004 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA04_A {
    ///0: Disable compare function for AN004
    _0 = 0,
    ///1: Enable compare function for AN004
    _1 = 1,
}
impl From<CMPCHA04_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA04_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA04` reader - Compare Window A Channel AN004 Select
pub type CMPCHA04_R = crate::BitReader<CMPCHA04_A>;
impl CMPCHA04_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA04_A {
        match self.bits {
            false => CMPCHA04_A::_0,
            true => CMPCHA04_A::_1,
        }
    }
    ///Disable compare function for AN004
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA04_A::_0
    }
    ///Enable compare function for AN004
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA04_A::_1
    }
}
///Field `CMPCHA04` writer - Compare Window A Channel AN004 Select
pub type CMPCHA04_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA04_A>;
impl<'a, REG> CMPCHA04_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN004
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA04_A::_0)
    }
    ///Enable compare function for AN004
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA04_A::_1)
    }
}
/**Compare Window A Channel AN005 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA05_A {
    ///0: Disable compare function for AN005
    _0 = 0,
    ///1: Enable compare function for AN005
    _1 = 1,
}
impl From<CMPCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA05_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA05` reader - Compare Window A Channel AN005 Select
pub type CMPCHA05_R = crate::BitReader<CMPCHA05_A>;
impl CMPCHA05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA05_A {
        match self.bits {
            false => CMPCHA05_A::_0,
            true => CMPCHA05_A::_1,
        }
    }
    ///Disable compare function for AN005
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA05_A::_0
    }
    ///Enable compare function for AN005
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA05_A::_1
    }
}
///Field `CMPCHA05` writer - Compare Window A Channel AN005 Select
pub type CMPCHA05_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA05_A>;
impl<'a, REG> CMPCHA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN005
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA05_A::_0)
    }
    ///Enable compare function for AN005
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA05_A::_1)
    }
}
/**Compare Window A Channel AN006 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA06_A {
    ///0: Disable compare function for AN006
    _0 = 0,
    ///1: Enable compare function for AN006
    _1 = 1,
}
impl From<CMPCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA06_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA06` reader - Compare Window A Channel AN006 Select
pub type CMPCHA06_R = crate::BitReader<CMPCHA06_A>;
impl CMPCHA06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA06_A {
        match self.bits {
            false => CMPCHA06_A::_0,
            true => CMPCHA06_A::_1,
        }
    }
    ///Disable compare function for AN006
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA06_A::_0
    }
    ///Enable compare function for AN006
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA06_A::_1
    }
}
///Field `CMPCHA06` writer - Compare Window A Channel AN006 Select
pub type CMPCHA06_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA06_A>;
impl<'a, REG> CMPCHA06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN006
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA06_A::_0)
    }
    ///Enable compare function for AN006
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA06_A::_1)
    }
}
/**Compare Window A Channel AN007 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPCHA07_A {
    ///0: Disable compare function for AN007
    _0 = 0,
    ///1: Enable compare function for AN007
    _1 = 1,
}
impl From<CMPCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPCHA07_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPCHA07` reader - Compare Window A Channel AN007 Select
pub type CMPCHA07_R = crate::BitReader<CMPCHA07_A>;
impl CMPCHA07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPCHA07_A {
        match self.bits {
            false => CMPCHA07_A::_0,
            true => CMPCHA07_A::_1,
        }
    }
    ///Disable compare function for AN007
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPCHA07_A::_0
    }
    ///Enable compare function for AN007
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPCHA07_A::_1
    }
}
///Field `CMPCHA07` writer - Compare Window A Channel AN007 Select
pub type CMPCHA07_W<'a, REG> = crate::BitWriter<'a, REG, CMPCHA07_A>;
impl<'a, REG> CMPCHA07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable compare function for AN007
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA07_A::_0)
    }
    ///Enable compare function for AN007
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPCHA07_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare Window A Channel AN000 Select
    #[inline(always)]
    pub fn cmpcha00(&self) -> CMPCHA00_R {
        CMPCHA00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare Window A Channel AN001 Select
    #[inline(always)]
    pub fn cmpcha01(&self) -> CMPCHA01_R {
        CMPCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare Window A Channel AN002 Select
    #[inline(always)]
    pub fn cmpcha02(&self) -> CMPCHA02_R {
        CMPCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare Window A Channel AN003 Select
    #[inline(always)]
    pub fn cmpcha03(&self) -> CMPCHA03_R {
        CMPCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Compare Window A Channel AN004 Select
    #[inline(always)]
    pub fn cmpcha04(&self) -> CMPCHA04_R {
        CMPCHA04_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Compare Window A Channel AN005 Select
    #[inline(always)]
    pub fn cmpcha05(&self) -> CMPCHA05_R {
        CMPCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare Window A Channel AN006 Select
    #[inline(always)]
    pub fn cmpcha06(&self) -> CMPCHA06_R {
        CMPCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare Window A Channel AN007 Select
    #[inline(always)]
    pub fn cmpcha07(&self) -> CMPCHA07_R {
        CMPCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare Window A Channel AN000 Select
    #[inline(always)]
    pub fn cmpcha00(&mut self) -> CMPCHA00_W<ADCMPANSR0_SPEC> {
        CMPCHA00_W::new(self, 0)
    }
    ///Bit 1 - Compare Window A Channel AN001 Select
    #[inline(always)]
    pub fn cmpcha01(&mut self) -> CMPCHA01_W<ADCMPANSR0_SPEC> {
        CMPCHA01_W::new(self, 1)
    }
    ///Bit 2 - Compare Window A Channel AN002 Select
    #[inline(always)]
    pub fn cmpcha02(&mut self) -> CMPCHA02_W<ADCMPANSR0_SPEC> {
        CMPCHA02_W::new(self, 2)
    }
    ///Bit 3 - Compare Window A Channel AN003 Select
    #[inline(always)]
    pub fn cmpcha03(&mut self) -> CMPCHA03_W<ADCMPANSR0_SPEC> {
        CMPCHA03_W::new(self, 3)
    }
    ///Bit 4 - Compare Window A Channel AN004 Select
    #[inline(always)]
    pub fn cmpcha04(&mut self) -> CMPCHA04_W<ADCMPANSR0_SPEC> {
        CMPCHA04_W::new(self, 4)
    }
    ///Bit 5 - Compare Window A Channel AN005 Select
    #[inline(always)]
    pub fn cmpcha05(&mut self) -> CMPCHA05_W<ADCMPANSR0_SPEC> {
        CMPCHA05_W::new(self, 5)
    }
    ///Bit 6 - Compare Window A Channel AN006 Select
    #[inline(always)]
    pub fn cmpcha06(&mut self) -> CMPCHA06_W<ADCMPANSR0_SPEC> {
        CMPCHA06_W::new(self, 6)
    }
    ///Bit 7 - Compare Window A Channel AN007 Select
    #[inline(always)]
    pub fn cmpcha07(&mut self) -> CMPCHA07_W<ADCMPANSR0_SPEC> {
        CMPCHA07_W::new(self, 7)
    }
}
/**A/D Compare Function Window A Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpansr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPANSR0_SPEC;
impl crate::RegisterSpec for ADCMPANSR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpansr0::R`](R) reader structure
impl crate::Readable for ADCMPANSR0_SPEC {}
///`write(|w| ..)` method takes [`adcmpansr0::W`](W) writer structure
impl crate::Writable for ADCMPANSR0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADCMPANSR0 to value 0
impl crate::Resettable for ADCMPANSR0_SPEC {}
