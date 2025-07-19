///Register `VBTWCTLR` reader
pub type R = crate::R<VBTWCTLR_SPEC>;
///Register `VBTWCTLR` writer
pub type W = crate::W<VBTWCTLR_SPEC>;
/**VBATT wakeup enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VWEN_A {
    ///0: Disable Wakeup function
    _0 = 0,
    ///1: Enable Wakeup function
    _1 = 1,
}
impl From<VWEN_A> for bool {
    #[inline(always)]
    fn from(variant: VWEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VWEN` reader - VBATT wakeup enable
pub type VWEN_R = crate::BitReader<VWEN_A>;
impl VWEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VWEN_A {
        match self.bits {
            false => VWEN_A::_0,
            true => VWEN_A::_1,
        }
    }
    ///Disable Wakeup function
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VWEN_A::_0
    }
    ///Enable Wakeup function
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VWEN_A::_1
    }
}
///Field `VWEN` writer - VBATT wakeup enable
pub type VWEN_W<'a, REG> = crate::BitWriter<'a, REG, VWEN_A>;
impl<'a, REG> VWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable Wakeup function
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VWEN_A::_0)
    }
    ///Enable Wakeup function
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VWEN_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATT wakeup enable
    #[inline(always)]
    pub fn vwen(&self) -> VWEN_R {
        VWEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATT wakeup enable
    #[inline(always)]
    pub fn vwen(&mut self) -> VWEN_W<'_, VBTWCTLR_SPEC> {
        VWEN_W::new(self, 0)
    }
}
/**VBATT Wakeup function Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtwctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTWCTLR_SPEC;
impl crate::RegisterSpec for VBTWCTLR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtwctlr::R`](R) reader structure
impl crate::Readable for VBTWCTLR_SPEC {}
///`write(|w| ..)` method takes [`vbtwctlr::W`](W) writer structure
impl crate::Writable for VBTWCTLR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTWCTLR to value 0
impl crate::Resettable for VBTWCTLR_SPEC {}
