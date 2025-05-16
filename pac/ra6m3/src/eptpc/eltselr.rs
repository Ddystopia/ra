///Register `ELTSELR` reader
pub type R = crate::R<ELTSELR_SPEC>;
///Register `ELTSELR` writer
pub type W = crate::W<ELTSELR_SPEC>;
/**Pulse Output Timer 0 Event Generation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS0_A {
    ///0: Pulse output timer 0 is used for the generation of event signals for the ELC.
    _0 = 0,
    ///1: Pulse output timer 0 is not used for the generation of event signals for the ELC.
    _1 = 1,
}
impl From<ELTDIS0_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELTDIS0` reader - Pulse Output Timer 0 Event Generation Disable
pub type ELTDIS0_R = crate::BitReader<ELTDIS0_A>;
impl ELTDIS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELTDIS0_A {
        match self.bits {
            false => ELTDIS0_A::_0,
            true => ELTDIS0_A::_1,
        }
    }
    ///Pulse output timer 0 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS0_A::_0
    }
    ///Pulse output timer 0 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS0_A::_1
    }
}
///Field `ELTDIS0` writer - Pulse Output Timer 0 Event Generation Disable
pub type ELTDIS0_W<'a, REG> = crate::BitWriter<'a, REG, ELTDIS0_A>;
impl<'a, REG> ELTDIS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 0 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS0_A::_0)
    }
    ///Pulse output timer 0 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS0_A::_1)
    }
}
/**Pulse Output Timer 1 Event Generation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS1_A {
    ///0: Pulse output timer 1 is used for the generation of event signals for the ELC.
    _0 = 0,
    ///1: Pulse output timer 1 is not used for the generation of event signals for the ELC.
    _1 = 1,
}
impl From<ELTDIS1_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELTDIS1` reader - Pulse Output Timer 1 Event Generation Disable
pub type ELTDIS1_R = crate::BitReader<ELTDIS1_A>;
impl ELTDIS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELTDIS1_A {
        match self.bits {
            false => ELTDIS1_A::_0,
            true => ELTDIS1_A::_1,
        }
    }
    ///Pulse output timer 1 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS1_A::_0
    }
    ///Pulse output timer 1 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS1_A::_1
    }
}
///Field `ELTDIS1` writer - Pulse Output Timer 1 Event Generation Disable
pub type ELTDIS1_W<'a, REG> = crate::BitWriter<'a, REG, ELTDIS1_A>;
impl<'a, REG> ELTDIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 1 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS1_A::_0)
    }
    ///Pulse output timer 1 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS1_A::_1)
    }
}
/**Pulse Output Timer 2 Event Generation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS2_A {
    ///0: Pulse output timer 2 is used for the generation of event signals for the ELC.
    _0 = 0,
    ///1: Pulse output timer 2 is not used for the generation of event signals for the ELC.
    _1 = 1,
}
impl From<ELTDIS2_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELTDIS2` reader - Pulse Output Timer 2 Event Generation Disable
pub type ELTDIS2_R = crate::BitReader<ELTDIS2_A>;
impl ELTDIS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELTDIS2_A {
        match self.bits {
            false => ELTDIS2_A::_0,
            true => ELTDIS2_A::_1,
        }
    }
    ///Pulse output timer 2 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS2_A::_0
    }
    ///Pulse output timer 2 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS2_A::_1
    }
}
///Field `ELTDIS2` writer - Pulse Output Timer 2 Event Generation Disable
pub type ELTDIS2_W<'a, REG> = crate::BitWriter<'a, REG, ELTDIS2_A>;
impl<'a, REG> ELTDIS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 2 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS2_A::_0)
    }
    ///Pulse output timer 2 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS2_A::_1)
    }
}
/**Pulse Output Timer 3 Event Generation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS3_A {
    ///0: Pulse output timer 3 is used for the generation of event signals for the ELC.
    _0 = 0,
    ///1: Pulse output timer 3 is not used for the generation of event signals for the ELC.
    _1 = 1,
}
impl From<ELTDIS3_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELTDIS3` reader - Pulse Output Timer 3 Event Generation Disable
pub type ELTDIS3_R = crate::BitReader<ELTDIS3_A>;
impl ELTDIS3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELTDIS3_A {
        match self.bits {
            false => ELTDIS3_A::_0,
            true => ELTDIS3_A::_1,
        }
    }
    ///Pulse output timer 3 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS3_A::_0
    }
    ///Pulse output timer 3 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS3_A::_1
    }
}
///Field `ELTDIS3` writer - Pulse Output Timer 3 Event Generation Disable
pub type ELTDIS3_W<'a, REG> = crate::BitWriter<'a, REG, ELTDIS3_A>;
impl<'a, REG> ELTDIS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 3 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS3_A::_0)
    }
    ///Pulse output timer 3 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS3_A::_1)
    }
}
/**Pulse Output Timer 4 Event Generation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS4_A {
    ///0: Pulse output timer 4 is used for the generation of event signals for the ELC.
    _0 = 0,
    ///1: Pulse output timer 4 is not used for the generation of event signals for the ELC.
    _1 = 1,
}
impl From<ELTDIS4_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELTDIS4` reader - Pulse Output Timer 4 Event Generation Disable
pub type ELTDIS4_R = crate::BitReader<ELTDIS4_A>;
impl ELTDIS4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELTDIS4_A {
        match self.bits {
            false => ELTDIS4_A::_0,
            true => ELTDIS4_A::_1,
        }
    }
    ///Pulse output timer 4 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS4_A::_0
    }
    ///Pulse output timer 4 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS4_A::_1
    }
}
///Field `ELTDIS4` writer - Pulse Output Timer 4 Event Generation Disable
pub type ELTDIS4_W<'a, REG> = crate::BitWriter<'a, REG, ELTDIS4_A>;
impl<'a, REG> ELTDIS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 4 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS4_A::_0)
    }
    ///Pulse output timer 4 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS4_A::_1)
    }
}
/**Pulse Output Timer 5 Event Generation Disable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELTDIS5_A {
    ///0: Pulse output timer 5 is used for the generation of event signals for the ELC.
    _0 = 0,
    ///1: Pulse output timer 5 is not used for the generation of event signals for the ELC.
    _1 = 1,
}
impl From<ELTDIS5_A> for bool {
    #[inline(always)]
    fn from(variant: ELTDIS5_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELTDIS5` reader - Pulse Output Timer 5 Event Generation Disable
pub type ELTDIS5_R = crate::BitReader<ELTDIS5_A>;
impl ELTDIS5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELTDIS5_A {
        match self.bits {
            false => ELTDIS5_A::_0,
            true => ELTDIS5_A::_1,
        }
    }
    ///Pulse output timer 5 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELTDIS5_A::_0
    }
    ///Pulse output timer 5 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELTDIS5_A::_1
    }
}
///Field `ELTDIS5` writer - Pulse Output Timer 5 Event Generation Disable
pub type ELTDIS5_W<'a, REG> = crate::BitWriter<'a, REG, ELTDIS5_A>;
impl<'a, REG> ELTDIS5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pulse output timer 5 is used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS5_A::_0)
    }
    ///Pulse output timer 5 is not used for the generation of event signals for the ELC.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ELTDIS5_A::_1)
    }
}
impl R {
    ///Bit 0 - Pulse Output Timer 0 Event Generation Disable
    #[inline(always)]
    pub fn eltdis0(&self) -> ELTDIS0_R {
        ELTDIS0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Event Generation Disable
    #[inline(always)]
    pub fn eltdis1(&self) -> ELTDIS1_R {
        ELTDIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pulse Output Timer 2 Event Generation Disable
    #[inline(always)]
    pub fn eltdis2(&self) -> ELTDIS2_R {
        ELTDIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pulse Output Timer 3 Event Generation Disable
    #[inline(always)]
    pub fn eltdis3(&self) -> ELTDIS3_R {
        ELTDIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pulse Output Timer 4 Event Generation Disable
    #[inline(always)]
    pub fn eltdis4(&self) -> ELTDIS4_R {
        ELTDIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pulse Output Timer 5 Event Generation Disable
    #[inline(always)]
    pub fn eltdis5(&self) -> ELTDIS5_R {
        ELTDIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pulse Output Timer 0 Event Generation Disable
    #[inline(always)]
    pub fn eltdis0(&mut self) -> ELTDIS0_W<ELTSELR_SPEC> {
        ELTDIS0_W::new(self, 0)
    }
    ///Bit 1 - Pulse Output Timer 1 Event Generation Disable
    #[inline(always)]
    pub fn eltdis1(&mut self) -> ELTDIS1_W<ELTSELR_SPEC> {
        ELTDIS1_W::new(self, 1)
    }
    ///Bit 2 - Pulse Output Timer 2 Event Generation Disable
    #[inline(always)]
    pub fn eltdis2(&mut self) -> ELTDIS2_W<ELTSELR_SPEC> {
        ELTDIS2_W::new(self, 2)
    }
    ///Bit 3 - Pulse Output Timer 3 Event Generation Disable
    #[inline(always)]
    pub fn eltdis3(&mut self) -> ELTDIS3_W<ELTSELR_SPEC> {
        ELTDIS3_W::new(self, 3)
    }
    ///Bit 4 - Pulse Output Timer 4 Event Generation Disable
    #[inline(always)]
    pub fn eltdis4(&mut self) -> ELTDIS4_W<ELTSELR_SPEC> {
        ELTDIS4_W::new(self, 4)
    }
    ///Bit 5 - Pulse Output Timer 5 Event Generation Disable
    #[inline(always)]
    pub fn eltdis5(&mut self) -> ELTDIS5_W<ELTSELR_SPEC> {
        ELTDIS5_W::new(self, 5)
    }
}
/**ELC Output Timer Select Register

You can [`read`](crate::Reg::read) this register and get [`eltselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eltselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ELTSELR_SPEC;
impl crate::RegisterSpec for ELTSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`eltselr::R`](R) reader structure
impl crate::Readable for ELTSELR_SPEC {}
///`write(|w| ..)` method takes [`eltselr::W`](W) writer structure
impl crate::Writable for ELTSELR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ELTSELR to value 0
impl crate::Resettable for ELTSELR_SPEC {}
