///Register `DPSIER3` reader
pub type R = crate::R<DPSIER3_SPEC>;
///Register `DPSIER3` writer
pub type W = crate::W<DPSIER3_SPEC>;
/**USBFS Suspend/Resume Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBFSIE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DUSBFSIE_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBFSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DUSBFSIE` reader - USBFS Suspend/Resume Deep Standby Cancel Signal Enable
pub type DUSBFSIE_R = crate::BitReader<DUSBFSIE_A>;
impl DUSBFSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DUSBFSIE_A {
        match self.bits {
            false => DUSBFSIE_A::_0,
            true => DUSBFSIE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBFSIE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBFSIE_A::_1
    }
}
///Field `DUSBFSIE` writer - USBFS Suspend/Resume Deep Standby Cancel Signal Enable
pub type DUSBFSIE_W<'a, REG> = crate::BitWriter<'a, REG, DUSBFSIE_A>;
impl<'a, REG> DUSBFSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBFSIE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBFSIE_A::_1)
    }
}
/**USBHS Suspend/Resume Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUSBHSIE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DUSBHSIE_A> for bool {
    #[inline(always)]
    fn from(variant: DUSBHSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DUSBHSIE` reader - USBHS Suspend/Resume Deep Standby Cancel Signal Enable
pub type DUSBHSIE_R = crate::BitReader<DUSBHSIE_A>;
impl DUSBHSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DUSBHSIE_A {
        match self.bits {
            false => DUSBHSIE_A::_0,
            true => DUSBHSIE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUSBHSIE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUSBHSIE_A::_1
    }
}
///Field `DUSBHSIE` writer - USBHS Suspend/Resume Deep Standby Cancel Signal Enable
pub type DUSBHSIE_W<'a, REG> = crate::BitWriter<'a, REG, DUSBHSIE_A>;
impl<'a, REG> DUSBHSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBHSIE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DUSBHSIE_A::_1)
    }
}
/**AGT1 Underflow Deep Standby Cancel Signal Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAGT1IE_A {
    ///0: Canceling deep software standby mode is disabled
    _0 = 0,
    ///1: Canceling deep software standby mode is enabled
    _1 = 1,
}
impl From<DAGT1IE_A> for bool {
    #[inline(always)]
    fn from(variant: DAGT1IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAGT1IE` reader - AGT1 Underflow Deep Standby Cancel Signal Enable
pub type DAGT1IE_R = crate::BitReader<DAGT1IE_A>;
impl DAGT1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAGT1IE_A {
        match self.bits {
            false => DAGT1IE_A::_0,
            true => DAGT1IE_A::_1,
        }
    }
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAGT1IE_A::_0
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAGT1IE_A::_1
    }
}
///Field `DAGT1IE` writer - AGT1 Underflow Deep Standby Cancel Signal Enable
pub type DAGT1IE_W<'a, REG> = crate::BitWriter<'a, REG, DAGT1IE_A>;
impl<'a, REG> DAGT1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Canceling deep software standby mode is disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAGT1IE_A::_0)
    }
    ///Canceling deep software standby mode is enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAGT1IE_A::_1)
    }
}
impl R {
    ///Bit 0 - USBFS Suspend/Resume Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbfsie(&self) -> DUSBFSIE_R {
        DUSBFSIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbhsie(&self) -> DUSBHSIE_R {
        DUSBHSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AGT1 Underflow Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dagt1ie(&self) -> DAGT1IE_R {
        DAGT1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USBFS Suspend/Resume Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbfsie(&mut self) -> DUSBFSIE_W<DPSIER3_SPEC> {
        DUSBFSIE_W::new(self, 0)
    }
    ///Bit 1 - USBHS Suspend/Resume Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dusbhsie(&mut self) -> DUSBHSIE_W<DPSIER3_SPEC> {
        DUSBHSIE_W::new(self, 1)
    }
    ///Bit 2 - AGT1 Underflow Deep Standby Cancel Signal Enable
    #[inline(always)]
    pub fn dagt1ie(&mut self) -> DAGT1IE_W<DPSIER3_SPEC> {
        DAGT1IE_W::new(self, 2)
    }
}
/**Deep Standby Interrupt Enable Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPSIER3_SPEC;
impl crate::RegisterSpec for DPSIER3_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dpsier3::R`](R) reader structure
impl crate::Readable for DPSIER3_SPEC {}
///`write(|w| ..)` method takes [`dpsier3::W`](W) writer structure
impl crate::Writable for DPSIER3_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DPSIER3 to value 0
impl crate::Resettable for DPSIER3_SPEC {}
