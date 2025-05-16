///Register `SOSCCR` reader
pub type R = crate::R<SOSCCR_SPEC>;
///Register `SOSCCR` writer
pub type W = crate::W<SOSCCR_SPEC>;
/**Sub-Clock Oscillator Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSTP_A {
    ///0: Operate the sub-clock oscillator
    _0 = 0,
    ///1: Stop the sub-clock oscillator
    _1 = 1,
}
impl From<SOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: SOSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SOSTP` reader - Sub-Clock Oscillator Stop
pub type SOSTP_R = crate::BitReader<SOSTP_A>;
impl SOSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOSTP_A {
        match self.bits {
            false => SOSTP_A::_0,
            true => SOSTP_A::_1,
        }
    }
    ///Operate the sub-clock oscillator
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOSTP_A::_0
    }
    ///Stop the sub-clock oscillator
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOSTP_A::_1
    }
}
///Field `SOSTP` writer - Sub-Clock Oscillator Stop
pub type SOSTP_W<'a, REG> = crate::BitWriter<'a, REG, SOSTP_A>;
impl<'a, REG> SOSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Operate the sub-clock oscillator
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SOSTP_A::_0)
    }
    ///Stop the sub-clock oscillator
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - Sub-Clock Oscillator Stop
    #[inline(always)]
    pub fn sostp(&self) -> SOSTP_R {
        SOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sub-Clock Oscillator Stop
    #[inline(always)]
    pub fn sostp(&mut self) -> SOSTP_W<SOSCCR_SPEC> {
        SOSTP_W::new(self, 0)
    }
}
/**Sub-clock oscillator control register

You can [`read`](crate::Reg::read) this register and get [`sosccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SOSCCR_SPEC;
impl crate::RegisterSpec for SOSCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`sosccr::R`](R) reader structure
impl crate::Readable for SOSCCR_SPEC {}
///`write(|w| ..)` method takes [`sosccr::W`](W) writer structure
impl crate::Writable for SOSCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SOSCCR to value 0
impl crate::Resettable for SOSCCR_SPEC {}
