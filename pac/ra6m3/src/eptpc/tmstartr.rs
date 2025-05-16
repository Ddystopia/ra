///Register `TMSTARTR` reader
pub type R = crate::R<TMSTARTR_SPEC>;
///Register `TMSTARTR` writer
pub type W = crate::W<TMSTARTR_SPEC>;
/**Pulse Output Timer 0 Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0_A {
    ///0: Stops pulse output timer 0.
    _0 = 0,
    ///1: Starts pulse output timer 0.
    _1 = 1,
}
impl From<EN0_A> for bool {
    #[inline(always)]
    fn from(variant: EN0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN0` reader - Pulse Output Timer 0 Start
pub type EN0_R = crate::BitReader<EN0_A>;
impl EN0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN0_A {
        match self.bits {
            false => EN0_A::_0,
            true => EN0_A::_1,
        }
    }
    ///Stops pulse output timer 0.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN0_A::_0
    }
    ///Starts pulse output timer 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN0_A::_1
    }
}
///Field `EN0` writer - Pulse Output Timer 0 Start
pub type EN0_W<'a, REG> = crate::BitWriter<'a, REG, EN0_A>;
impl<'a, REG> EN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops pulse output timer 0.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN0_A::_0)
    }
    ///Starts pulse output timer 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN0_A::_1)
    }
}
/**Pulse Output Timer 1 Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    ///0: Stops pulse output timer 1.
    _0 = 0,
    ///1: Starts pulse output timer 1.
    _1 = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN1` reader - Pulse Output Timer 1 Start
pub type EN1_R = crate::BitReader<EN1_A>;
impl EN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::_0,
            true => EN1_A::_1,
        }
    }
    ///Stops pulse output timer 1.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN1_A::_0
    }
    ///Starts pulse output timer 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN1_A::_1
    }
}
///Field `EN1` writer - Pulse Output Timer 1 Start
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1_A>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops pulse output timer 1.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN1_A::_0)
    }
    ///Starts pulse output timer 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN1_A::_1)
    }
}
/**Pulse Output Timer 2 Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN2_A {
    ///0: Stops pulse output timer 2.
    _0 = 0,
    ///1: Starts pulse output timer 2.
    _1 = 1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN2` reader - Pulse Output Timer 2 Start
pub type EN2_R = crate::BitReader<EN2_A>;
impl EN2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::_0,
            true => EN2_A::_1,
        }
    }
    ///Stops pulse output timer 2.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN2_A::_0
    }
    ///Starts pulse output timer 2.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN2_A::_1
    }
}
///Field `EN2` writer - Pulse Output Timer 2 Start
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG, EN2_A>;
impl<'a, REG> EN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops pulse output timer 2.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN2_A::_0)
    }
    ///Starts pulse output timer 2.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN2_A::_1)
    }
}
/**Pulse Output Timer 3 Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN3_A {
    ///0: Stops pulse output timer 3.
    _0 = 0,
    ///1: Starts pulse output timer 3.
    _1 = 1,
}
impl From<EN3_A> for bool {
    #[inline(always)]
    fn from(variant: EN3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN3` reader - Pulse Output Timer 3 Start
pub type EN3_R = crate::BitReader<EN3_A>;
impl EN3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN3_A {
        match self.bits {
            false => EN3_A::_0,
            true => EN3_A::_1,
        }
    }
    ///Stops pulse output timer 3.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN3_A::_0
    }
    ///Starts pulse output timer 3.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN3_A::_1
    }
}
///Field `EN3` writer - Pulse Output Timer 3 Start
pub type EN3_W<'a, REG> = crate::BitWriter<'a, REG, EN3_A>;
impl<'a, REG> EN3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops pulse output timer 3.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN3_A::_0)
    }
    ///Starts pulse output timer 3.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN3_A::_1)
    }
}
/**Pulse Output Timer 4 Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN4_A {
    ///0: Stops pulse output timer 4.
    _0 = 0,
    ///1: Starts pulse output timer 4.
    _1 = 1,
}
impl From<EN4_A> for bool {
    #[inline(always)]
    fn from(variant: EN4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN4` reader - Pulse Output Timer 4 Start
pub type EN4_R = crate::BitReader<EN4_A>;
impl EN4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN4_A {
        match self.bits {
            false => EN4_A::_0,
            true => EN4_A::_1,
        }
    }
    ///Stops pulse output timer 4.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN4_A::_0
    }
    ///Starts pulse output timer 4.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN4_A::_1
    }
}
///Field `EN4` writer - Pulse Output Timer 4 Start
pub type EN4_W<'a, REG> = crate::BitWriter<'a, REG, EN4_A>;
impl<'a, REG> EN4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops pulse output timer 4.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN4_A::_0)
    }
    ///Starts pulse output timer 4.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN4_A::_1)
    }
}
/**Pulse Output Timer 5 Start

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN5_A {
    ///0: Stops pulse output timer 5.
    _0 = 0,
    ///1: Starts pulse output timer 5.
    _1 = 1,
}
impl From<EN5_A> for bool {
    #[inline(always)]
    fn from(variant: EN5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN5` reader - Pulse Output Timer 5 Start
pub type EN5_R = crate::BitReader<EN5_A>;
impl EN5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN5_A {
        match self.bits {
            false => EN5_A::_0,
            true => EN5_A::_1,
        }
    }
    ///Stops pulse output timer 5.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN5_A::_0
    }
    ///Starts pulse output timer 5.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN5_A::_1
    }
}
///Field `EN5` writer - Pulse Output Timer 5 Start
pub type EN5_W<'a, REG> = crate::BitWriter<'a, REG, EN5_A>;
impl<'a, REG> EN5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stops pulse output timer 5.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EN5_A::_0)
    }
    ///Starts pulse output timer 5.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EN5_A::_1)
    }
}
impl R {
    ///Bit 0 - Pulse Output Timer 0 Start
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Start
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pulse Output Timer 2 Start
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pulse Output Timer 3 Start
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pulse Output Timer 4 Start
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pulse Output Timer 5 Start
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pulse Output Timer 0 Start
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W<TMSTARTR_SPEC> {
        EN0_W::new(self, 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Start
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<TMSTARTR_SPEC> {
        EN1_W::new(self, 1)
    }
    ///Bit 2 - Pulse Output Timer 2 Start
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<TMSTARTR_SPEC> {
        EN2_W::new(self, 2)
    }
    ///Bit 3 - Pulse Output Timer 3 Start
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W<TMSTARTR_SPEC> {
        EN3_W::new(self, 3)
    }
    ///Bit 4 - Pulse Output Timer 4 Start
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W<TMSTARTR_SPEC> {
        EN4_W::new(self, 4)
    }
    ///Bit 5 - Pulse Output Timer 5 Start
    #[inline(always)]
    pub fn en5(&mut self) -> EN5_W<TMSTARTR_SPEC> {
        EN5_W::new(self, 5)
    }
}
/**Timer Start Register

You can [`read`](crate::Reg::read) this register and get [`tmstartr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmstartr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TMSTARTR_SPEC;
impl crate::RegisterSpec for TMSTARTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tmstartr::R`](R) reader structure
impl crate::Readable for TMSTARTR_SPEC {}
///`write(|w| ..)` method takes [`tmstartr::W`](W) writer structure
impl crate::Writable for TMSTARTR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TMSTARTR to value 0
impl crate::Resettable for TMSTARTR_SPEC {}
