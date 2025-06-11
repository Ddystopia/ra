///Register `VBTCMPCR` reader
pub type R = crate::R<VBTCMPCR_SPEC>;
///Register `VBTCMPCR` writer
pub type W = crate::W<VBTCMPCR_SPEC>;
/**VBATT pin low voltage detect circuit output enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBTCMPE_A {
    ///0: VBATT pin low voltage detect circuit output disabled
    _0 = 0,
    ///1: VBATT pin low voltage detect circuit output enabled
    _1 = 1,
}
impl From<VBTCMPE_A> for bool {
    #[inline(always)]
    fn from(variant: VBTCMPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VBTCMPE` reader - VBATT pin low voltage detect circuit output enable
pub type VBTCMPE_R = crate::BitReader<VBTCMPE_A>;
impl VBTCMPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBTCMPE_A {
        match self.bits {
            false => VBTCMPE_A::_0,
            true => VBTCMPE_A::_1,
        }
    }
    ///VBATT pin low voltage detect circuit output disabled
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VBTCMPE_A::_0
    }
    ///VBATT pin low voltage detect circuit output enabled
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VBTCMPE_A::_1
    }
}
///Field `VBTCMPE` writer - VBATT pin low voltage detect circuit output enable
pub type VBTCMPE_W<'a, REG> = crate::BitWriter<'a, REG, VBTCMPE_A>;
impl<'a, REG> VBTCMPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///VBATT pin low voltage detect circuit output disabled
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(VBTCMPE_A::_0)
    }
    ///VBATT pin low voltage detect circuit output enabled
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(VBTCMPE_A::_1)
    }
}
impl R {
    ///Bit 0 - VBATT pin low voltage detect circuit output enable
    #[inline(always)]
    pub fn vbtcmpe(&self) -> VBTCMPE_R {
        VBTCMPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VBATT pin low voltage detect circuit output enable
    #[inline(always)]
    pub fn vbtcmpe(&mut self) -> VBTCMPE_W<VBTCMPCR_SPEC> {
        VBTCMPE_W::new(self, 0)
    }
}
/**VBATT Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtcmpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcmpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTCMPCR_SPEC;
impl crate::RegisterSpec for VBTCMPCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtcmpcr::R`](R) reader structure
impl crate::Readable for VBTCMPCR_SPEC {}
///`write(|w| ..)` method takes [`vbtcmpcr::W`](W) writer structure
impl crate::Writable for VBTCMPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTCMPCR to value 0
impl crate::Resettable for VBTCMPCR_SPEC {}
