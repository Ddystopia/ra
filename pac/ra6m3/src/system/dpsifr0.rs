///Register `DPSIFR0` reader
pub type R = crate::R<DPSIFR0_SPEC>;
///Register `DPSIFR0` writer
pub type W = crate::W<DPSIFR0_SPEC>;
/**IRQ0-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ0F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ0F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ0F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ0F` reader - IRQ0-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ0F_R = crate::BitReader<DIRQ0F_A>;
impl DIRQ0F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ0F_A {
        match self.bits {
            false => DIRQ0F_A::_0,
            true => DIRQ0F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ0F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ0F_A::_1
    }
}
///Field `DIRQ0F` writer - IRQ0-DS Pin Deep Standby Cancel Flag
pub type DIRQ0F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ0F_A>;
impl<'a, REG> DIRQ0F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ0F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ0F_A::_1)
    }
}
/**IRQ1-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ1F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ1F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ1F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ1F` reader - IRQ1-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ1F_R = crate::BitReader<DIRQ1F_A>;
impl DIRQ1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ1F_A {
        match self.bits {
            false => DIRQ1F_A::_0,
            true => DIRQ1F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ1F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ1F_A::_1
    }
}
///Field `DIRQ1F` writer - IRQ1-DS Pin Deep Standby Cancel Flag
pub type DIRQ1F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ1F_A>;
impl<'a, REG> DIRQ1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ1F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ1F_A::_1)
    }
}
/**IRQ2-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ2F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ2F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ2F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ2F` reader - IRQ2-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ2F_R = crate::BitReader<DIRQ2F_A>;
impl DIRQ2F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ2F_A {
        match self.bits {
            false => DIRQ2F_A::_0,
            true => DIRQ2F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ2F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ2F_A::_1
    }
}
///Field `DIRQ2F` writer - IRQ2-DS Pin Deep Standby Cancel Flag
pub type DIRQ2F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ2F_A>;
impl<'a, REG> DIRQ2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ2F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ2F_A::_1)
    }
}
/**IRQ3-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ3F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ3F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ3F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ3F` reader - IRQ3-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ3F_R = crate::BitReader<DIRQ3F_A>;
impl DIRQ3F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ3F_A {
        match self.bits {
            false => DIRQ3F_A::_0,
            true => DIRQ3F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ3F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ3F_A::_1
    }
}
///Field `DIRQ3F` writer - IRQ3-DS Pin Deep Standby Cancel Flag
pub type DIRQ3F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ3F_A>;
impl<'a, REG> DIRQ3F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ3F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ3F_A::_1)
    }
}
/**IRQ4-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ4F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ4F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ4F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ4F` reader - IRQ4-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ4F_R = crate::BitReader<DIRQ4F_A>;
impl DIRQ4F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ4F_A {
        match self.bits {
            false => DIRQ4F_A::_0,
            true => DIRQ4F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ4F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ4F_A::_1
    }
}
///Field `DIRQ4F` writer - IRQ4-DS Pin Deep Standby Cancel Flag
pub type DIRQ4F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ4F_A>;
impl<'a, REG> DIRQ4F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ4F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ4F_A::_1)
    }
}
/**IRQ5-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ5F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ5F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ5F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ5F` reader - IRQ5-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ5F_R = crate::BitReader<DIRQ5F_A>;
impl DIRQ5F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ5F_A {
        match self.bits {
            false => DIRQ5F_A::_0,
            true => DIRQ5F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ5F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ5F_A::_1
    }
}
///Field `DIRQ5F` writer - IRQ5-DS Pin Deep Standby Cancel Flag
pub type DIRQ5F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ5F_A>;
impl<'a, REG> DIRQ5F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ5F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ5F_A::_1)
    }
}
/**IRQ6-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ6F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ6F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ6F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ6F` reader - IRQ6-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ6F_R = crate::BitReader<DIRQ6F_A>;
impl DIRQ6F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ6F_A {
        match self.bits {
            false => DIRQ6F_A::_0,
            true => DIRQ6F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ6F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ6F_A::_1
    }
}
///Field `DIRQ6F` writer - IRQ6-DS Pin Deep Standby Cancel Flag
pub type DIRQ6F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ6F_A>;
impl<'a, REG> DIRQ6F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ6F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ6F_A::_1)
    }
}
/**IRQ7-DS Pin Deep Standby Cancel Flag

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ7F_A {
    ///0: The cancel request is not generated
    _0 = 0,
    ///1: The cancel request is generated
    _1 = 1,
}
impl From<DIRQ7F_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ7F_A) -> Self {
        variant as u8 != 0
    }
}
/**Field `DIRQ7F` reader - IRQ7-DS Pin Deep Standby Cancel Flag

<div class="warning">The field is <b>modified</b> in some way after a read operation.</div>*/
pub type DIRQ7F_R = crate::BitReader<DIRQ7F_A>;
impl DIRQ7F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRQ7F_A {
        match self.bits {
            false => DIRQ7F_A::_0,
            true => DIRQ7F_A::_1,
        }
    }
    ///The cancel request is not generated
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIRQ7F_A::_0
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIRQ7F_A::_1
    }
}
///Field `DIRQ7F` writer - IRQ7-DS Pin Deep Standby Cancel Flag
pub type DIRQ7F_W<'a, REG> = crate::BitWriter0C<'a, REG, DIRQ7F_A>;
impl<'a, REG> DIRQ7F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The cancel request is not generated
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ7F_A::_0)
    }
    ///The cancel request is generated
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRQ7F_A::_1)
    }
}
impl R {
    ///Bit 0 - IRQ0-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq0f(&self) -> DIRQ0F_R {
        DIRQ0F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IRQ1-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq1f(&self) -> DIRQ1F_R {
        DIRQ1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IRQ2-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq2f(&self) -> DIRQ2F_R {
        DIRQ2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IRQ3-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq3f(&self) -> DIRQ3F_R {
        DIRQ3F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IRQ4-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq4f(&self) -> DIRQ4F_R {
        DIRQ4F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IRQ5-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq5f(&self) -> DIRQ5F_R {
        DIRQ5F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IRQ6-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq6f(&self) -> DIRQ6F_R {
        DIRQ6F_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IRQ7-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq7f(&self) -> DIRQ7F_R {
        DIRQ7F_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IRQ0-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq0f(&mut self) -> DIRQ0F_W<DPSIFR0_SPEC> {
        DIRQ0F_W::new(self, 0)
    }
    ///Bit 1 - IRQ1-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq1f(&mut self) -> DIRQ1F_W<DPSIFR0_SPEC> {
        DIRQ1F_W::new(self, 1)
    }
    ///Bit 2 - IRQ2-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq2f(&mut self) -> DIRQ2F_W<DPSIFR0_SPEC> {
        DIRQ2F_W::new(self, 2)
    }
    ///Bit 3 - IRQ3-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq3f(&mut self) -> DIRQ3F_W<DPSIFR0_SPEC> {
        DIRQ3F_W::new(self, 3)
    }
    ///Bit 4 - IRQ4-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq4f(&mut self) -> DIRQ4F_W<DPSIFR0_SPEC> {
        DIRQ4F_W::new(self, 4)
    }
    ///Bit 5 - IRQ5-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq5f(&mut self) -> DIRQ5F_W<DPSIFR0_SPEC> {
        DIRQ5F_W::new(self, 5)
    }
    ///Bit 6 - IRQ6-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq6f(&mut self) -> DIRQ6F_W<DPSIFR0_SPEC> {
        DIRQ6F_W::new(self, 6)
    }
    ///Bit 7 - IRQ7-DS Pin Deep Standby Cancel Flag
    #[inline(always)]
    pub fn dirq7f(&mut self) -> DIRQ7F_W<DPSIFR0_SPEC> {
        DIRQ7F_W::new(self, 7)
    }
}
/**Deep Standby Interrupt Flag Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsifr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIFR0_SPEC;
impl crate::RegisterSpec for DPSIFR0_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsifr0::R`](R) reader structure
impl crate::Readable for DPSIFR0_SPEC {}
///`write(|w| ..)` method takes [`dpsifr0::W`](W) writer structure
impl crate::Writable for DPSIFR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0xff;
}
///`reset()` method sets DPSIFR0 to value 0
impl crate::Resettable for DPSIFR0_SPEC {}
