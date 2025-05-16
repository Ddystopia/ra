///Register `BCKCR` reader
pub type R = crate::R<BCKCR_SPEC>;
///Register `BCKCR` writer
pub type W = crate::W<BCKCR_SPEC>;
/**BCLK Pin Output Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCLKDIV_A {
    ///0: BCLK
    _0 = 0,
    ///1: BCLK/2
    _1 = 1,
}
impl From<BCLKDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BCLKDIV_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BCLKDIV` reader - BCLK Pin Output Select
pub type BCLKDIV_R = crate::BitReader<BCLKDIV_A>;
impl BCLKDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BCLKDIV_A {
        match self.bits {
            false => BCLKDIV_A::_0,
            true => BCLKDIV_A::_1,
        }
    }
    ///BCLK
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCLKDIV_A::_0
    }
    ///BCLK/2
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCLKDIV_A::_1
    }
}
///Field `BCLKDIV` writer - BCLK Pin Output Select
pub type BCLKDIV_W<'a, REG> = crate::BitWriter<'a, REG, BCLKDIV_A>;
impl<'a, REG> BCLKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///BCLK
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BCLKDIV_A::_0)
    }
    ///BCLK/2
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BCLKDIV_A::_1)
    }
}
impl R {
    ///Bit 0 - BCLK Pin Output Select
    #[inline(always)]
    pub fn bclkdiv(&self) -> BCLKDIV_R {
        BCLKDIV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BCLK Pin Output Select
    #[inline(always)]
    pub fn bclkdiv(&mut self) -> BCLKDIV_W<BCKCR_SPEC> {
        BCLKDIV_W::new(self, 0)
    }
}
/**External Bus Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`bckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BCKCR_SPEC;
impl crate::RegisterSpec for BCKCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`bckcr::R`](R) reader structure
impl crate::Readable for BCKCR_SPEC {}
///`write(|w| ..)` method takes [`bckcr::W`](W) writer structure
impl crate::Writable for BCKCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCKCR to value 0
impl crate::Resettable for BCKCR_SPEC {}
