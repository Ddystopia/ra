///Register `DADPR` reader
pub type R = crate::R<DADPR_SPEC>;
///Register `DADPR` writer
pub type W = crate::W<DADPR_SPEC>;
/**DADRm Format Select

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSEL_A {
    ///0: Right justified format.
    _0 = 0,
    ///1: Left justified format.
    _1 = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DPSEL` reader - DADRm Format Select
pub type DPSEL_R = crate::BitReader<DPSEL_A>;
impl DPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::_0,
            true => DPSEL_A::_1,
        }
    }
    ///Right justified format.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSEL_A::_0
    }
    ///Left justified format.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSEL_A::_1
    }
}
///Field `DPSEL` writer - DADRm Format Select
pub type DPSEL_W<'a, REG> = crate::BitWriter<'a, REG, DPSEL_A>;
impl<'a, REG> DPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Right justified format.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DPSEL_A::_0)
    }
    ///Left justified format.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DPSEL_A::_1)
    }
}
impl R {
    ///Bit 7 - DADRm Format Select
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - DADRm Format Select
    #[inline(always)]
    pub fn dpsel(&mut self) -> DPSEL_W<DADPR_SPEC> {
        DPSEL_W::new(self, 7)
    }
}
/**DADRm Format Select Register

You can [`read`](crate::Reg::read) this register and get [`dadpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DADPR_SPEC;
impl crate::RegisterSpec for DADPR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dadpr::R`](R) reader structure
impl crate::Readable for DADPR_SPEC {}
///`write(|w| ..)` method takes [`dadpr::W`](W) writer structure
impl crate::Writable for DADPR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DADPR to value 0
impl crate::Resettable for DADPR_SPEC {}
