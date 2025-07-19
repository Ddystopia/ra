///Register `USBCKCR` reader
pub type R = crate::R<USBCKCR_SPEC>;
///Register `USBCKCR` writer
pub type W = crate::W<USBCKCR_SPEC>;
/**USB Clock Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCLKSEL_A {
    ///0: PLL(Value after reset)
    _0 = 0,
    ///1: HOCO
    _1 = 1,
}
impl From<USBCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: USBCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `USBCLKSEL` reader - USB Clock Source Select
pub type USBCLKSEL_R = crate::BitReader<USBCLKSEL_A>;
impl USBCLKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> USBCLKSEL_A {
        match self.bits {
            false => USBCLKSEL_A::_0,
            true => USBCLKSEL_A::_1,
        }
    }
    ///PLL(Value after reset)
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBCLKSEL_A::_0
    }
    ///HOCO
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBCLKSEL_A::_1
    }
}
///Field `USBCLKSEL` writer - USB Clock Source Select
pub type USBCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG, USBCLKSEL_A>;
impl<'a, REG> USBCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL(Value after reset)
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::_0)
    }
    ///HOCO
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCLKSEL_A::_1)
    }
}
impl R {
    ///Bit 0 - USB Clock Source Select
    #[inline(always)]
    pub fn usbclksel(&self) -> USBCLKSEL_R {
        USBCLKSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USB Clock Source Select
    #[inline(always)]
    pub fn usbclksel(&mut self) -> USBCLKSEL_W<'_, USBCKCR_SPEC> {
        USBCLKSEL_W::new(self, 0)
    }
}
/**USB Clock Control register

You can [`read`](crate::Reg::read) this register and get [`usbckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USBCKCR_SPEC;
impl crate::RegisterSpec for USBCKCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`usbckcr::R`](R) reader structure
impl crate::Readable for USBCKCR_SPEC {}
///`write(|w| ..)` method takes [`usbckcr::W`](W) writer structure
impl crate::Writable for USBCKCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USBCKCR to value 0
impl crate::Resettable for USBCKCR_SPEC {}
