///Register `DPSIFR1` reader
pub type R = crate::R<DPSIFR1_SPEC>;
///Register `DPSIFR1` writer
pub type W = crate::W<DPSIFR1_SPEC>;
/**IRQ8-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ8F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ8F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ8F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ8F` reader - IRQ8-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ8F_R = crate::BitReader<DIRQ8F_A>;
impl DIRQ8F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ8F_A {
        match self.bits {
            false => DIRQ8F_A::_0,
            true => DIRQ8F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ8F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ8F_A::_1
    }
}
///Field `DIRQ8F` writer - IRQ8-DS Pin Deep Standby Cancel Flag
pub type DIRQ8F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ8F_A>;
impl<'a, REG> DIRQ8F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ8F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ8F_A::_1)
    }
}
/**IRQ9-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ9F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ9F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ9F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ9F` reader - IRQ9-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ9F_R = crate::BitReader<DIRQ9F_A>;
impl DIRQ9F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ9F_A {
        match self.bits {
            false => DIRQ9F_A::_0,
            true => DIRQ9F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ9F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ9F_A::_1
    }
}
///Field `DIRQ9F` writer - IRQ9-DS Pin Deep Standby Cancel Flag
pub type DIRQ9F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ9F_A>;
impl<'a, REG> DIRQ9F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ9F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ9F_A::_1)
    }
}
/**IRQ10-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ10F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ10F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ10F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ10F` reader - IRQ10-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ10F_R = crate::BitReader<DIRQ10F_A>;
impl DIRQ10F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ10F_A {
        match self.bits {
            false => DIRQ10F_A::_0,
            true => DIRQ10F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ10F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ10F_A::_1
    }
}
///Field `DIRQ10F` writer - IRQ10-DS Pin Deep Standby Cancel Flag
pub type DIRQ10F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ10F_A>;
impl<'a, REG> DIRQ10F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ10F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ10F_A::_1)
    }
}
/**IRQ11-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ11F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ11F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ11F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ11F` reader - IRQ11-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ11F_R = crate::BitReader<DIRQ11F_A>;
impl DIRQ11F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ11F_A {
        match self.bits {
            false => DIRQ11F_A::_0,
            true => DIRQ11F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ11F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ11F_A::_1
    }
}
///Field `DIRQ11F` writer - IRQ11-DS Pin Deep Standby Cancel Flag
pub type DIRQ11F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ11F_A>;
impl<'a, REG> DIRQ11F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ11F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ11F_A::_1)
    }
}
/**IRQ12-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ12F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ12F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ12F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ12F` reader - IRQ12-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ12F_R = crate::BitReader<DIRQ12F_A>;
impl DIRQ12F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ12F_A {
        match self.bits {
            false => DIRQ12F_A::_0,
            true => DIRQ12F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ12F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ12F_A::_1
    }
}
///Field `DIRQ12F` writer - IRQ12-DS Pin Deep Standby Cancel Flag
pub type DIRQ12F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ12F_A>;
impl<'a, REG> DIRQ12F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ12F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ12F_A::_1)
    }
}
/**IRQ13-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ13F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ13F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ13F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ13F` reader - IRQ13-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ13F_R = crate::BitReader<DIRQ13F_A>;
impl DIRQ13F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ13F_A {
        match self.bits {
            false => DIRQ13F_A::_0,
            true => DIRQ13F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ13F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ13F_A::_1
    }
}
///Field `DIRQ13F` writer - IRQ13-DS Pin Deep Standby Cancel Flag
pub type DIRQ13F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ13F_A>;
impl<'a, REG> DIRQ13F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ13F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ13F_A::_1)
    }
}
/**IRQ14-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ14F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ14F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ14F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ14F` reader - IRQ14-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ14F_R = crate::BitReader<DIRQ14F_A>;
impl DIRQ14F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ14F_A {
        match self.bits {
            false => DIRQ14F_A::_0,
            true => DIRQ14F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ14F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ14F_A::_1
    }
}
///Field `DIRQ14F` writer - IRQ14-DS Pin Deep Standby Cancel Flag
pub type DIRQ14F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ14F_A>;
impl<'a, REG> DIRQ14F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ14F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ14F_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ8-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq8f(&self) -> DIRQ8F_R {
        DIRQ8F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ9-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq9f(&self) -> DIRQ9F_R {
        DIRQ9F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ10-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq10f(&self) -> DIRQ10F_R {
        DIRQ10F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ11-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq11f(&self) -> DIRQ11F_R {
        DIRQ11F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ12-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq12f(&self) -> DIRQ12F_R {
        DIRQ12F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ13-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq13f(&self) -> DIRQ13F_R {
        DIRQ13F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ14-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq14f(&self) -> DIRQ14F_R {
        DIRQ14F_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ8-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq8f(&mut self) -> DIRQ8F_W<DPSIFR1_SPEC> {
        DIRQ8F_W::new(self, 0)
    }
    ///Bit 1 - IRQ9-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq9f(&mut self) -> DIRQ9F_W<DPSIFR1_SPEC> {
        DIRQ9F_W::new(self, 1)
    }
    ///Bit 2 - IRQ10-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq10f(&mut self) -> DIRQ10F_W<DPSIFR1_SPEC> {
        DIRQ10F_W::new(self, 2)
    }
    ///Bit 3 - IRQ11-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq11f(&mut self) -> DIRQ11F_W<DPSIFR1_SPEC> {
        DIRQ11F_W::new(self, 3)
    }
    ///Bit 4 - IRQ12-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq12f(&mut self) -> DIRQ12F_W<DPSIFR1_SPEC> {
        DIRQ12F_W::new(self, 4)
    }
    ///Bit 5 - IRQ13-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq13f(&mut self) -> DIRQ13F_W<DPSIFR1_SPEC> {
        DIRQ13F_W::new(self, 5)
    }
    ///Bit 6 - IRQ14-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq14f(&mut self) -> DIRQ14F_W<DPSIFR1_SPEC> {
        DIRQ14F_W::new(self, 6)
    }
}
/**Deep Standby Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsifr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIFR1_SPEC;
impl crate::RegisterSpec for DPSIFR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsifr1::R`](R) reader structure
impl crate::Readable for DPSIFR1_SPEC {}
///`write(|w| ..)` method takes [`dpsifr1::W`](W) writer structure
impl crate::Writable for DPSIFR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0x7f;
}
///`reset()` method sets DPSIFR1 to value 0
impl crate::Resettable for DPSIFR1_SPEC {}
