///Register `OSTDCR` reader
pub type R = crate::R<OSTDCR_SPEC>;
///Register `OSTDCR` writer
pub type W = crate::W<OSTDCR_SPEC>;
/**Oscillation Stop Detection Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDIE_A {
    ///0: Disable oscillation stop detection interrupt (do not notify the POEG)
    _0 = 0,
    ///1: Enable oscillation stop detection interrupt (notify the POEG)
    _1 = 1,
}
impl From<OSTDIE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTDIE` reader - Oscillation Stop Detection Interrupt Enable
pub type OSTDIE_R = crate::BitReader<OSTDIE_A>;
impl OSTDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTDIE_A {
        match self.bits {
            false => OSTDIE_A::_0,
            true => OSTDIE_A::_1,
        }
    }
    ///Disable oscillation stop detection interrupt (do not notify the POEG)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDIE_A::_0
    }
    ///Enable oscillation stop detection interrupt (notify the POEG)
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDIE_A::_1
    }
}
///Field `OSTDIE` writer - Oscillation Stop Detection Interrupt Enable
pub type OSTDIE_W<'a, REG> = crate::BitWriter<'a, REG, OSTDIE_A>;
impl<'a, REG> OSTDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable oscillation stop detection interrupt (do not notify the POEG)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDIE_A::_0)
    }
    ///Enable oscillation stop detection interrupt (notify the POEG)
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDIE_A::_1)
    }
}
/**Oscillation Stop Detection Function Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSTDE_A {
    ///0: Disable oscillation stop detection function
    _0 = 0,
    ///1: Enable oscillation stop detection function
    _1 = 1,
}
impl From<OSTDE_A> for bool {
    #[inline(always)]
    fn from(variant: OSTDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OSTDE` reader - Oscillation Stop Detection Function Enable
pub type OSTDE_R = crate::BitReader<OSTDE_A>;
impl OSTDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSTDE_A {
        match self.bits {
            false => OSTDE_A::_0,
            true => OSTDE_A::_1,
        }
    }
    ///Disable oscillation stop detection function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSTDE_A::_0
    }
    ///Enable oscillation stop detection function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSTDE_A::_1
    }
}
///Field `OSTDE` writer - Oscillation Stop Detection Function Enable
pub type OSTDE_W<'a, REG> = crate::BitWriter<'a, REG, OSTDE_A>;
impl<'a, REG> OSTDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable oscillation stop detection function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDE_A::_0)
    }
    ///Enable oscillation stop detection function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OSTDE_A::_1)
    }
}
impl R {
    ///Bit 0 - Oscillation Stop Detection Interrupt Enable
    #[inline(always)]
    pub fn ostdie(&self) -> OSTDIE_R {
        OSTDIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Oscillation Stop Detection Function Enable
    #[inline(always)]
    pub fn ostde(&self) -> OSTDE_R {
        OSTDE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Oscillation Stop Detection Interrupt Enable
    #[inline(always)]
    pub fn ostdie(&mut self) -> OSTDIE_W<OSTDCR_SPEC> {
        OSTDIE_W::new(self, 0)
    }
    ///Bit 7 - Oscillation Stop Detection Function Enable
    #[inline(always)]
    pub fn ostde(&mut self) -> OSTDE_W<OSTDCR_SPEC> {
        OSTDE_W::new(self, 7)
    }
}
/**Oscillation Stop Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`ostdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OSTDCR_SPEC;
impl crate::RegisterSpec for OSTDCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`ostdcr::R`](R) reader structure
impl crate::Readable for OSTDCR_SPEC {}
///`write(|w| ..)` method takes [`ostdcr::W`](W) writer structure
impl crate::Writable for OSTDCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSTDCR to value 0
impl crate::Resettable for OSTDCR_SPEC {}
