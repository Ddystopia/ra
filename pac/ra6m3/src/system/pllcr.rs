///Register `PLLCR` reader
pub type R = crate::R<PLLCR_SPEC>;
///Register `PLLCR` writer
pub type W = crate::W<PLLCR_SPEC>;
/**PLL Stop Control

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTP_A {
    ///0: Operate the PLL
    _0 = 0,
    ///1: Stop the PLL.
    _1 = 1,
}
impl From<PLLSTP_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSTP` reader - PLL Stop Control
pub type PLLSTP_R = crate::BitReader<PLLSTP_A>;
impl PLLSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLSTP_A {
        match self.bits {
            false => PLLSTP_A::_0,
            true => PLLSTP_A::_1,
        }
    }
    ///Operate the PLL
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTP_A::_0
    }
    ///Stop the PLL.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTP_A::_1
    }
}
///Field `PLLSTP` writer - PLL Stop Control
pub type PLLSTP_W<'a, REG> = crate::BitWriter<'a, REG, PLLSTP_A>;
impl<'a, REG> PLLSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate the PLL
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTP_A::_0)
    }
    ///Stop the PLL.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - PLL Stop Control
    #[inline(always)]
    pub fn pllstp(&self) -> PLLSTP_R {
        PLLSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PLL Stop Control
    #[inline(always)]
    pub fn pllstp(&mut self) -> PLLSTP_W<PLLCR_SPEC> {
        PLLSTP_W::new(self, 0)
    }
}
/**PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`pllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLLCR_SPEC;
impl crate::RegisterSpec for PLLCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`pllcr::R`](R) reader structure
impl crate::Readable for PLLCR_SPEC {}
///`write(|w| ..)` method takes [`pllcr::W`](W) writer structure
impl crate::Writable for PLLCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCR to value 0x01
impl crate::Resettable for PLLCR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
