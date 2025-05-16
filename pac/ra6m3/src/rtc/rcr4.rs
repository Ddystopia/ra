///Register `RCR4` reader
pub type R = crate::R<RCR4_SPEC>;
///Register `RCR4` writer
pub type W = crate::W<RCR4_SPEC>;
/**Count Source Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCKSEL_A {
    ///0: Sub-clock oscillator is selected.
    _0 = 0,
    ///1: LOCO clock oscillator is selected.
    _1 = 1,
}
impl From<RCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RCKSEL` reader - Count Source Select
pub type RCKSEL_R = crate::BitReader<RCKSEL_A>;
impl RCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCKSEL_A {
        match self.bits {
            false => RCKSEL_A::_0,
            true => RCKSEL_A::_1,
        }
    }
    ///Sub-clock oscillator is selected.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCKSEL_A::_0
    }
    ///LOCO clock oscillator is selected.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCKSEL_A::_1
    }
}
///Field `RCKSEL` writer - Count Source Select
pub type RCKSEL_W<'a, REG> = crate::BitWriter<'a, REG, RCKSEL_A>;
impl<'a, REG> RCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sub-clock oscillator is selected.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RCKSEL_A::_0)
    }
    ///LOCO clock oscillator is selected.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RCKSEL_A::_1)
    }
}
impl R {
    ///Bit 0 - Count Source Select
    #[inline(always)]
    pub fn rcksel(&self) -> RCKSEL_R {
        RCKSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Count Source Select
    #[inline(always)]
    pub fn rcksel(&mut self) -> RCKSEL_W<RCR4_SPEC> {
        RCKSEL_W::new(self, 0)
    }
}
/**RTC Control Register 4

You can [`read`](crate::Reg::read) this register and get [`rcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RCR4_SPEC;
impl crate::RegisterSpec for RCR4_SPEC {
    type Ux = u8;
}
///`read()` method returns [`rcr4::R`](R) reader structure
impl crate::Readable for RCR4_SPEC {}
///`write(|w| ..)` method takes [`rcr4::W`](W) writer structure
impl crate::Writable for RCR4_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR4 to value 0
impl crate::Resettable for RCR4_SPEC {}
