///Register `ADCMPSR0` reader
pub type R = crate::R<ADCMPSR0_SPEC>;
///Register `ADCMPSR0` writer
pub type W = crate::W<ADCMPSR0_SPEC>;
/**Compare window A flag of AN000

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA00_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA00_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA00_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA00` reader - Compare window A flag of AN000

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA00_R = crate::BitReader<CMPSTCHA00_A>;
impl CMPSTCHA00_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA00_A {
        match self.bits {
            false => CMPSTCHA00_A::_0,
            true => CMPSTCHA00_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA00_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA00_A::_1
    }
}
///Field `CMPSTCHA00` writer - Compare window A flag of AN000
pub type CMPSTCHA00_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA00_A>;
impl<'a, REG> CMPSTCHA00_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA00_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA00_A::_1)
    }
}
/**Compare window A flag of AN001

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA01_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA01_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA01_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA01` reader - Compare window A flag of AN001

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA01_R = crate::BitReader<CMPSTCHA01_A>;
impl CMPSTCHA01_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA01_A {
        match self.bits {
            false => CMPSTCHA01_A::_0,
            true => CMPSTCHA01_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA01_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA01_A::_1
    }
}
///Field `CMPSTCHA01` writer - Compare window A flag of AN001
pub type CMPSTCHA01_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA01_A>;
impl<'a, REG> CMPSTCHA01_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA01_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA01_A::_1)
    }
}
/**Compare window A flag of AN002

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA02_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA02_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA02_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA02` reader - Compare window A flag of AN002

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA02_R = crate::BitReader<CMPSTCHA02_A>;
impl CMPSTCHA02_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA02_A {
        match self.bits {
            false => CMPSTCHA02_A::_0,
            true => CMPSTCHA02_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA02_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA02_A::_1
    }
}
///Field `CMPSTCHA02` writer - Compare window A flag of AN002
pub type CMPSTCHA02_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA02_A>;
impl<'a, REG> CMPSTCHA02_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA02_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA02_A::_1)
    }
}
/**Compare window A flag of AN003

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA03_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA03_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA03_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA03` reader - Compare window A flag of AN003

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA03_R = crate::BitReader<CMPSTCHA03_A>;
impl CMPSTCHA03_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA03_A {
        match self.bits {
            false => CMPSTCHA03_A::_0,
            true => CMPSTCHA03_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA03_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA03_A::_1
    }
}
///Field `CMPSTCHA03` writer - Compare window A flag of AN003
pub type CMPSTCHA03_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA03_A>;
impl<'a, REG> CMPSTCHA03_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA03_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA03_A::_1)
    }
}
/**Compare window A flag of AN005

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA05_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA05_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA05_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA05` reader - Compare window A flag of AN005

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA05_R = crate::BitReader<CMPSTCHA05_A>;
impl CMPSTCHA05_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA05_A {
        match self.bits {
            false => CMPSTCHA05_A::_0,
            true => CMPSTCHA05_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA05_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA05_A::_1
    }
}
///Field `CMPSTCHA05` writer - Compare window A flag of AN005
pub type CMPSTCHA05_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA05_A>;
impl<'a, REG> CMPSTCHA05_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA05_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA05_A::_1)
    }
}
/**Compare window A flag of AN006

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA06_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA06_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA06_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA06` reader - Compare window A flag of AN006

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA06_R = crate::BitReader<CMPSTCHA06_A>;
impl CMPSTCHA06_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA06_A {
        match self.bits {
            false => CMPSTCHA06_A::_0,
            true => CMPSTCHA06_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA06_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA06_A::_1
    }
}
///Field `CMPSTCHA06` writer - Compare window A flag of AN006
pub type CMPSTCHA06_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA06_A>;
impl<'a, REG> CMPSTCHA06_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA06_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA06_A::_1)
    }
}
/**Compare window A flag of AN007

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPSTCHA07_A {
    ///0: Comparison conditions are not met.
    _0 = 0,
    ///1: Comparison conditions are met.
    _1 = 1,
}
impl From<CMPSTCHA07_A> for bool {
    #[inline(always)]
    fn from(variant: CMPSTCHA07_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `CMPSTCHA07` reader - Compare window A flag of AN007

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type CMPSTCHA07_R = crate::BitReader<CMPSTCHA07_A>;
impl CMPSTCHA07_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMPSTCHA07_A {
        match self.bits {
            false => CMPSTCHA07_A::_0,
            true => CMPSTCHA07_A::_1,
        }
    }
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMPSTCHA07_A::_0
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMPSTCHA07_A::_1
    }
}
///Field `CMPSTCHA07` writer - Compare window A flag of AN007
pub type CMPSTCHA07_W<'a, REG> = crate::BitWriter0C<'a, REG, CMPSTCHA07_A>;
impl<'a, REG> CMPSTCHA07_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparison conditions are not met.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA07_A::_0)
    }
    ///Comparison conditions are met.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CMPSTCHA07_A::_1)
    }
}
impl R {
    ///Bit 0 - Compare window A flag of AN000
    #[inline(always)]
    pub fn cmpstcha00(&self) -> CMPSTCHA00_R {
        CMPSTCHA00_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare window A flag of AN001
    #[inline(always)]
    pub fn cmpstcha01(&self) -> CMPSTCHA01_R {
        CMPSTCHA01_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare window A flag of AN002
    #[inline(always)]
    pub fn cmpstcha02(&self) -> CMPSTCHA02_R {
        CMPSTCHA02_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare window A flag of AN003
    #[inline(always)]
    pub fn cmpstcha03(&self) -> CMPSTCHA03_R {
        CMPSTCHA03_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Compare window A flag of AN005
    #[inline(always)]
    pub fn cmpstcha05(&self) -> CMPSTCHA05_R {
        CMPSTCHA05_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Compare window A flag of AN006
    #[inline(always)]
    pub fn cmpstcha06(&self) -> CMPSTCHA06_R {
        CMPSTCHA06_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Compare window A flag of AN007
    #[inline(always)]
    pub fn cmpstcha07(&self) -> CMPSTCHA07_R {
        CMPSTCHA07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare window A flag of AN000
    #[inline(always)]
    pub fn cmpstcha00(&mut self) -> CMPSTCHA00_W<ADCMPSR0_SPEC> {
        CMPSTCHA00_W::new(self, 0)
    }
    ///Bit 1 - Compare window A flag of AN001
    #[inline(always)]
    pub fn cmpstcha01(&mut self) -> CMPSTCHA01_W<ADCMPSR0_SPEC> {
        CMPSTCHA01_W::new(self, 1)
    }
    ///Bit 2 - Compare window A flag of AN002
    #[inline(always)]
    pub fn cmpstcha02(&mut self) -> CMPSTCHA02_W<ADCMPSR0_SPEC> {
        CMPSTCHA02_W::new(self, 2)
    }
    ///Bit 3 - Compare window A flag of AN003
    #[inline(always)]
    pub fn cmpstcha03(&mut self) -> CMPSTCHA03_W<ADCMPSR0_SPEC> {
        CMPSTCHA03_W::new(self, 3)
    }
    ///Bit 5 - Compare window A flag of AN005
    #[inline(always)]
    pub fn cmpstcha05(&mut self) -> CMPSTCHA05_W<ADCMPSR0_SPEC> {
        CMPSTCHA05_W::new(self, 5)
    }
    ///Bit 6 - Compare window A flag of AN006
    #[inline(always)]
    pub fn cmpstcha06(&mut self) -> CMPSTCHA06_W<ADCMPSR0_SPEC> {
        CMPSTCHA06_W::new(self, 6)
    }
    ///Bit 7 - Compare window A flag of AN007
    #[inline(always)]
    pub fn cmpstcha07(&mut self) -> CMPSTCHA07_W<ADCMPSR0_SPEC> {
        CMPSTCHA07_W::new(self, 7)
    }
}
/**A/D Compare Function Window A Channel Status Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ADCMPSR0_SPEC;
impl crate::RegisterSpec for ADCMPSR0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`adcmpsr0::R`](R) reader structure
impl crate::Readable for ADCMPSR0_SPEC {}
///`write(|w| ..)` method takes [`adcmpsr0::W`](W) writer structure
impl crate::Writable for ADCMPSR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0xef;
}
///`reset()` method sets ADCMPSR0 to value 0
impl crate::Resettable for ADCMPSR0_SPEC {}
