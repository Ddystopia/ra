///Register `WDTRCR` reader
pub type R = crate::R<WDTRCR_SPEC>;
///Register `WDTRCR` writer
pub type W = crate::W<WDTRCR_SPEC>;
/**Reset Interrupt Request Selection

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTIRQS_A {
    ///0: Non-maskable interrupt request or interrupt request output is enabled
    _0 = 0,
    ///1: Reset output is enabled.
    _1 = 1,
}
impl From<RSTIRQS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTIRQS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTIRQS` reader - Reset Interrupt Request Selection
pub type RSTIRQS_R = crate::BitReader<RSTIRQS_A>;
impl RSTIRQS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSTIRQS_A {
        match self.bits {
            false => RSTIRQS_A::_0,
            true => RSTIRQS_A::_1,
        }
    }
    ///Non-maskable interrupt request or interrupt request output is enabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTIRQS_A::_0
    }
    ///Reset output is enabled.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTIRQS_A::_1
    }
}
///Field `RSTIRQS` writer - Reset Interrupt Request Selection
pub type RSTIRQS_W<'a, REG> = crate::BitWriter<'a, REG, RSTIRQS_A>;
impl<'a, REG> RSTIRQS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non-maskable interrupt request or interrupt request output is enabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIRQS_A::_0)
    }
    ///Reset output is enabled.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIRQS_A::_1)
    }
}
impl R {
    ///Bit 7 - Reset Interrupt Request Selection
    #[inline(always)]
    pub fn rstirqs(&self) -> RSTIRQS_R {
        RSTIRQS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - Reset Interrupt Request Selection
    #[inline(always)]
    pub fn rstirqs(&mut self) -> RSTIRQS_W<WDTRCR_SPEC> {
        RSTIRQS_W::new(self, 7)
    }
}
/**WDT Reset Control Register

You can [`read`](crate::Reg::read) this register and get [`wdtrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDTRCR_SPEC;
impl crate::RegisterSpec for WDTRCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`wdtrcr::R`](R) reader structure
impl crate::Readable for WDTRCR_SPEC {}
///`write(|w| ..)` method takes [`wdtrcr::W`](W) writer structure
impl crate::Writable for WDTRCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDTRCR to value 0x80
impl crate::Resettable for WDTRCR_SPEC {
    const RESET_VALUE: u8 = 0x80;
}
