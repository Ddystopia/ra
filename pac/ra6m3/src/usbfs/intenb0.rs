///Register `INTENB0` reader
pub type R = crate::R<INTENB0_SPEC>;
///Register `INTENB0` writer
pub type W = crate::W<INTENB0_SPEC>;
/**Buffer Ready Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<BRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: BRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDYE` reader - Buffer Ready Interrupt Enable
pub type BRDYE_R = crate::BitReader<BRDYE_A>;
impl BRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRDYE_A {
        match self.bits {
            false => BRDYE_A::_0,
            true => BRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRDYE_A::_1
    }
}
///Field `BRDYE` writer - Buffer Ready Interrupt Enable
pub type BRDYE_W<'a, REG> = crate::BitWriter<'a, REG, BRDYE_A>;
impl<'a, REG> BRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BRDYE_A::_1)
    }
}
/**Buffer Not Ready Response Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRDYE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<NRDYE_A> for bool {
    #[inline(always)]
    fn from(variant: NRDYE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NRDYE` reader - Buffer Not Ready Response Interrupt Enable
pub type NRDYE_R = crate::BitReader<NRDYE_A>;
impl NRDYE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NRDYE_A {
        match self.bits {
            false => NRDYE_A::_0,
            true => NRDYE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NRDYE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NRDYE_A::_1
    }
}
///Field `NRDYE` writer - Buffer Not Ready Response Interrupt Enable
pub type NRDYE_W<'a, REG> = crate::BitWriter<'a, REG, NRDYE_A>;
impl<'a, REG> NRDYE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NRDYE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NRDYE_A::_1)
    }
}
/**Buffer Empty Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEMPE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<BEMPE_A> for bool {
    #[inline(always)]
    fn from(variant: BEMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BEMPE` reader - Buffer Empty Interrupt Enable
pub type BEMPE_R = crate::BitReader<BEMPE_A>;
impl BEMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BEMPE_A {
        match self.bits {
            false => BEMPE_A::_0,
            true => BEMPE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEMPE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEMPE_A::_1
    }
}
///Field `BEMPE` writer - Buffer Empty Interrupt Enable
pub type BEMPE_W<'a, REG> = crate::BitWriter<'a, REG, BEMPE_A>;
impl<'a, REG> BEMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BEMPE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BEMPE_A::_1)
    }
}
/**Control Transfer Stage Transition Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<CTRE_A> for bool {
    #[inline(always)]
    fn from(variant: CTRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTRE` reader - Control Transfer Stage Transition Interrupt Enable
pub type CTRE_R = crate::BitReader<CTRE_A>;
impl CTRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTRE_A {
        match self.bits {
            false => CTRE_A::_0,
            true => CTRE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTRE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTRE_A::_1
    }
}
///Field `CTRE` writer - Control Transfer Stage Transition Interrupt Enable
pub type CTRE_W<'a, REG> = crate::BitWriter<'a, REG, CTRE_A>;
impl<'a, REG> CTRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTRE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTRE_A::_1)
    }
}
/**Device State Transition Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVSE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<DVSE_A> for bool {
    #[inline(always)]
    fn from(variant: DVSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DVSE` reader - Device State Transition Interrupt Enable
pub type DVSE_R = crate::BitReader<DVSE_A>;
impl DVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DVSE_A {
        match self.bits {
            false => DVSE_A::_0,
            true => DVSE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVSE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVSE_A::_1
    }
}
///Field `DVSE` writer - Device State Transition Interrupt Enable
pub type DVSE_W<'a, REG> = crate::BitWriter<'a, REG, DVSE_A>;
impl<'a, REG> DVSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DVSE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DVSE_A::_1)
    }
}
/**Frame Number Update Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<SOFE_A> for bool {
    #[inline(always)]
    fn from(variant: SOFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOFE` reader - Frame Number Update Interrupt Enable
pub type SOFE_R = crate::BitReader<SOFE_A>;
impl SOFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOFE_A {
        match self.bits {
            false => SOFE_A::_0,
            true => SOFE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFE_A::_1
    }
}
///Field `SOFE` writer - Frame Number Update Interrupt Enable
pub type SOFE_W<'a, REG> = crate::BitWriter<'a, REG, SOFE_A>;
impl<'a, REG> SOFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOFE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOFE_A::_1)
    }
}
/**Resume Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSME_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<RSME_A> for bool {
    #[inline(always)]
    fn from(variant: RSME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSME` reader - Resume Interrupt Enable
pub type RSME_R = crate::BitReader<RSME_A>;
impl RSME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSME_A {
        match self.bits {
            false => RSME_A::_0,
            true => RSME_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSME_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSME_A::_1
    }
}
///Field `RSME` writer - Resume Interrupt Enable
pub type RSME_W<'a, REG> = crate::BitWriter<'a, REG, RSME_A>;
impl<'a, REG> RSME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSME_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSME_A::_1)
    }
}
/**VBUS Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBSE_A {
    ///0: Interrupt output disabled
    _0 = 0,
    ///1: Interrupt output enabled
    _1 = 1,
}
impl From<VBSE_A> for bool {
    #[inline(always)]
    fn from(variant: VBSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBSE` reader - VBUS Interrupt Enable
pub type VBSE_R = crate::BitReader<VBSE_A>;
impl VBSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBSE_A {
        match self.bits {
            false => VBSE_A::_0,
            true => VBSE_A::_1,
        }
    }
    ///Interrupt output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBSE_A::_0
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBSE_A::_1
    }
}
///Field `VBSE` writer - VBUS Interrupt Enable
pub type VBSE_W<'a, REG> = crate::BitWriter<'a, REG, VBSE_A>;
impl<'a, REG> VBSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBSE_A::_0)
    }
    ///Interrupt output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBSE_A::_1)
    }
}
impl R {
    ///Bit 8 - Buffer Ready Interrupt Enable
    #[inline(always)]
    pub fn brdye(&self) -> BRDYE_R {
        BRDYE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Buffer Not Ready Response Interrupt Enable
    #[inline(always)]
    pub fn nrdye(&self) -> NRDYE_R {
        NRDYE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer Empty Interrupt Enable
    #[inline(always)]
    pub fn bempe(&self) -> BEMPE_R {
        BEMPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Enable
    #[inline(always)]
    pub fn ctre(&self) -> CTRE_R {
        CTRE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Device State Transition Interrupt Enable
    #[inline(always)]
    pub fn dvse(&self) -> DVSE_R {
        DVSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Frame Number Update Interrupt Enable
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Resume Interrupt Enable
    #[inline(always)]
    pub fn rsme(&self) -> RSME_R {
        RSME_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VBUS Interrupt Enable
    #[inline(always)]
    pub fn vbse(&self) -> VBSE_R {
        VBSE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - Buffer Ready Interrupt Enable
    #[inline(always)]
    pub fn brdye(&mut self) -> BRDYE_W<INTENB0_SPEC> {
        BRDYE_W::new(self, 8)
    }
    ///Bit 9 - Buffer Not Ready Response Interrupt Enable
    #[inline(always)]
    pub fn nrdye(&mut self) -> NRDYE_W<INTENB0_SPEC> {
        NRDYE_W::new(self, 9)
    }
    ///Bit 10 - Buffer Empty Interrupt Enable
    #[inline(always)]
    pub fn bempe(&mut self) -> BEMPE_W<INTENB0_SPEC> {
        BEMPE_W::new(self, 10)
    }
    ///Bit 11 - Control Transfer Stage Transition Interrupt Enable
    #[inline(always)]
    pub fn ctre(&mut self) -> CTRE_W<INTENB0_SPEC> {
        CTRE_W::new(self, 11)
    }
    ///Bit 12 - Device State Transition Interrupt Enable
    #[inline(always)]
    pub fn dvse(&mut self) -> DVSE_W<INTENB0_SPEC> {
        DVSE_W::new(self, 12)
    }
    ///Bit 13 - Frame Number Update Interrupt Enable
    #[inline(always)]
    pub fn sofe(&mut self) -> SOFE_W<INTENB0_SPEC> {
        SOFE_W::new(self, 13)
    }
    ///Bit 14 - Resume Interrupt Enable
    #[inline(always)]
    pub fn rsme(&mut self) -> RSME_W<INTENB0_SPEC> {
        RSME_W::new(self, 14)
    }
    ///Bit 15 - VBUS Interrupt Enable
    #[inline(always)]
    pub fn vbse(&mut self) -> VBSE_W<INTENB0_SPEC> {
        VBSE_W::new(self, 15)
    }
}
/**Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`intenb0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTENB0_SPEC;
impl crate::RegisterSpec for INTENB0_SPEC {
    type Ux = u16;
}
///`read()` method returns [`intenb0::R`](R) reader structure
impl crate::Readable for INTENB0_SPEC {}
///`write(|w| ..)` method takes [`intenb0::W`](W) writer structure
impl crate::Writable for INTENB0_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTENB0 to value 0
impl crate::Resettable for INTENB0_SPEC {}
