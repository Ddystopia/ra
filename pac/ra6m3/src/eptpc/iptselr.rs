///Register `IPTSELR` reader
pub type R = crate::R<IPTSELR_SPEC>;
///Register `IPTSELR` writer
pub type W = crate::W<IPTSELR_SPEC>;
/**Pulse Output Timer 0 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL0_A {
    ///0: Pulse output timer 0 is not selected as a source of IPLS interrupt requests.
    _0 = 0,
    ///1: Pulse output timer 0 is selected as a source of IPLS interrupt requests.
    _1 = 1,
}
impl From<IPTSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IPTSEL0` reader - Pulse Output Timer 0 Select
pub type IPTSEL0_R = crate::BitReader<IPTSEL0_A>;
impl IPTSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPTSEL0_A {
        match self.bits {
            false => IPTSEL0_A::_0,
            true => IPTSEL0_A::_1,
        }
    }
    ///Pulse output timer 0 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL0_A::_0
    }
    ///Pulse output timer 0 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL0_A::_1
    }
}
///Field `IPTSEL0` writer - Pulse Output Timer 0 Select
pub type IPTSEL0_W<'a, REG> = crate::BitWriter<'a, REG, IPTSEL0_A>;
impl<'a, REG> IPTSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 0 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL0_A::_0)
    }
    ///Pulse output timer 0 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL0_A::_1)
    }
}
/**Pulse Output Timer 1 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL1_A {
    ///0: Pulse output timer 1 is not selected as a source of IPLS interrupt requests.
    _0 = 0,
    ///1: Pulse output timer 1 is selected as a source of IPLS interrupt requests.
    _1 = 1,
}
impl From<IPTSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IPTSEL1` reader - Pulse Output Timer 1 Select
pub type IPTSEL1_R = crate::BitReader<IPTSEL1_A>;
impl IPTSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPTSEL1_A {
        match self.bits {
            false => IPTSEL1_A::_0,
            true => IPTSEL1_A::_1,
        }
    }
    ///Pulse output timer 1 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL1_A::_0
    }
    ///Pulse output timer 1 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL1_A::_1
    }
}
///Field `IPTSEL1` writer - Pulse Output Timer 1 Select
pub type IPTSEL1_W<'a, REG> = crate::BitWriter<'a, REG, IPTSEL1_A>;
impl<'a, REG> IPTSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 1 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL1_A::_0)
    }
    ///Pulse output timer 1 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL1_A::_1)
    }
}
/**Pulse Output Timer 2 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL2_A {
    ///0: Pulse output timer 2 is not selected as a source of IPLS interrupt requests.
    _0 = 0,
    ///1: Pulse output timer 2 is selected as a source of IPLS interrupt requests.
    _1 = 1,
}
impl From<IPTSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IPTSEL2` reader - Pulse Output Timer 2 Select
pub type IPTSEL2_R = crate::BitReader<IPTSEL2_A>;
impl IPTSEL2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPTSEL2_A {
        match self.bits {
            false => IPTSEL2_A::_0,
            true => IPTSEL2_A::_1,
        }
    }
    ///Pulse output timer 2 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL2_A::_0
    }
    ///Pulse output timer 2 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL2_A::_1
    }
}
///Field `IPTSEL2` writer - Pulse Output Timer 2 Select
pub type IPTSEL2_W<'a, REG> = crate::BitWriter<'a, REG, IPTSEL2_A>;
impl<'a, REG> IPTSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 2 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL2_A::_0)
    }
    ///Pulse output timer 2 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL2_A::_1)
    }
}
/**Pulse Output Timer 3 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL3_A {
    ///0: Pulse output timer 3 is not selected as a source of IPLS interrupt requests.
    _0 = 0,
    ///1: Pulse output timer 3 is selected as a source of IPLS interrupt requests.
    _1 = 1,
}
impl From<IPTSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IPTSEL3` reader - Pulse Output Timer 3 Select
pub type IPTSEL3_R = crate::BitReader<IPTSEL3_A>;
impl IPTSEL3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPTSEL3_A {
        match self.bits {
            false => IPTSEL3_A::_0,
            true => IPTSEL3_A::_1,
        }
    }
    ///Pulse output timer 3 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL3_A::_0
    }
    ///Pulse output timer 3 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL3_A::_1
    }
}
///Field `IPTSEL3` writer - Pulse Output Timer 3 Select
pub type IPTSEL3_W<'a, REG> = crate::BitWriter<'a, REG, IPTSEL3_A>;
impl<'a, REG> IPTSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 3 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL3_A::_0)
    }
    ///Pulse output timer 3 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL3_A::_1)
    }
}
/**Pulse Output Timer 4 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL4_A {
    ///0: Pulse output timer 4 is not selected as a source of IPLS interrupt requests.
    _0 = 0,
    ///1: Pulse output timer 4 is selected as a source of IPLS interrupt requests.
    _1 = 1,
}
impl From<IPTSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IPTSEL4` reader - Pulse Output Timer 4 Select
pub type IPTSEL4_R = crate::BitReader<IPTSEL4_A>;
impl IPTSEL4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPTSEL4_A {
        match self.bits {
            false => IPTSEL4_A::_0,
            true => IPTSEL4_A::_1,
        }
    }
    ///Pulse output timer 4 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL4_A::_0
    }
    ///Pulse output timer 4 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL4_A::_1
    }
}
///Field `IPTSEL4` writer - Pulse Output Timer 4 Select
pub type IPTSEL4_W<'a, REG> = crate::BitWriter<'a, REG, IPTSEL4_A>;
impl<'a, REG> IPTSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 4 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL4_A::_0)
    }
    ///Pulse output timer 4 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL4_A::_1)
    }
}
/**Pulse Output Timer 5 Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL5_A {
    ///0: Pulse output timer 5 is not selected as a source of IPLS interrupt requests.
    _0 = 0,
    ///1: Pulse output timer 5 is selected as a source of IPLS interrupt requests.
    _1 = 1,
}
impl From<IPTSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IPTSEL5` reader - Pulse Output Timer 5 Select
pub type IPTSEL5_R = crate::BitReader<IPTSEL5_A>;
impl IPTSEL5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPTSEL5_A {
        match self.bits {
            false => IPTSEL5_A::_0,
            true => IPTSEL5_A::_1,
        }
    }
    ///Pulse output timer 5 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL5_A::_0
    }
    ///Pulse output timer 5 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL5_A::_1
    }
}
///Field `IPTSEL5` writer - Pulse Output Timer 5 Select
pub type IPTSEL5_W<'a, REG> = crate::BitWriter<'a, REG, IPTSEL5_A>;
impl<'a, REG> IPTSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 5 is not selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL5_A::_0)
    }
    ///Pulse output timer 5 is selected as a source of IPLS interrupt requests.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IPTSEL5_A::_1)
    }
}
impl R {
    ///Bit 0 - Pulse Output Timer 0 Select
    #[inline(always)]
    pub fn iptsel0(&self) -> IPTSEL0_R {
        IPTSEL0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Select
    #[inline(always)]
    pub fn iptsel1(&self) -> IPTSEL1_R {
        IPTSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pulse Output Timer 2 Select
    #[inline(always)]
    pub fn iptsel2(&self) -> IPTSEL2_R {
        IPTSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pulse Output Timer 3 Select
    #[inline(always)]
    pub fn iptsel3(&self) -> IPTSEL3_R {
        IPTSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pulse Output Timer 4 Select
    #[inline(always)]
    pub fn iptsel4(&self) -> IPTSEL4_R {
        IPTSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pulse Output Timer 5 Select
    #[inline(always)]
    pub fn iptsel5(&self) -> IPTSEL5_R {
        IPTSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pulse Output Timer 0 Select
    #[inline(always)]
    pub fn iptsel0(&mut self) -> IPTSEL0_W<IPTSELR_SPEC> {
        IPTSEL0_W::new(self, 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Select
    #[inline(always)]
    pub fn iptsel1(&mut self) -> IPTSEL1_W<IPTSELR_SPEC> {
        IPTSEL1_W::new(self, 1)
    }
    ///Bit 2 - Pulse Output Timer 2 Select
    #[inline(always)]
    pub fn iptsel2(&mut self) -> IPTSEL2_W<IPTSELR_SPEC> {
        IPTSEL2_W::new(self, 2)
    }
    ///Bit 3 - Pulse Output Timer 3 Select
    #[inline(always)]
    pub fn iptsel3(&mut self) -> IPTSEL3_W<IPTSELR_SPEC> {
        IPTSEL3_W::new(self, 3)
    }
    ///Bit 4 - Pulse Output Timer 4 Select
    #[inline(always)]
    pub fn iptsel4(&mut self) -> IPTSEL4_W<IPTSELR_SPEC> {
        IPTSEL4_W::new(self, 4)
    }
    ///Bit 5 - Pulse Output Timer 5 Select
    #[inline(always)]
    pub fn iptsel5(&mut self) -> IPTSEL5_W<IPTSELR_SPEC> {
        IPTSEL5_W::new(self, 5)
    }
}
/**IPLS Interrupt Request Timer Select Register

You can [`read`](crate::Reg::read) this register and get [`iptselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iptselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IPTSELR_SPEC;
impl crate::RegisterSpec for IPTSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`iptselr::R`](R) reader structure
impl crate::Readable for IPTSELR_SPEC {}
///`write(|w| ..)` method takes [`iptselr::W`](W) writer structure
impl crate::Writable for IPTSELR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IPTSELR to value 0
impl crate::Resettable for IPTSELR_SPEC {}
