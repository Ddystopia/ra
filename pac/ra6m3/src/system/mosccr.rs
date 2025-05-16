///Register `MOSCCR` reader
pub type R = crate::R<MOSCCR_SPEC>;
///Register `MOSCCR` writer
pub type W = crate::W<MOSCCR_SPEC>;
/**Main Clock Oscillator Stop

Value on reset: 1*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOSTP_A {
    ///0: Main clock oscillator is operating.
    _0 = 0,
    ///1: Main clock oscillator is stopped.
    _1 = 1,
}
impl From<MOSTP_A> for bool {
    #[inline(always)]
    fn from(variant: MOSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MOSTP` reader - Main Clock Oscillator Stop
pub type MOSTP_R = crate::BitReader<MOSTP_A>;
impl MOSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MOSTP_A {
        match self.bits {
            false => MOSTP_A::_0,
            true => MOSTP_A::_1,
        }
    }
    ///Main clock oscillator is operating.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSTP_A::_0
    }
    ///Main clock oscillator is stopped.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSTP_A::_1
    }
}
///Field `MOSTP` writer - Main Clock Oscillator Stop
pub type MOSTP_W<'a, REG> = crate::BitWriter<'a, REG, MOSTP_A>;
impl<'a, REG> MOSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Main clock oscillator is operating.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MOSTP_A::_0)
    }
    ///Main clock oscillator is stopped.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MOSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - Main Clock Oscillator Stop
    #[inline(always)]
    pub fn mostp(&self) -> MOSTP_R {
        MOSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Main Clock Oscillator Stop
    #[inline(always)]
    pub fn mostp(&mut self) -> MOSTP_W<MOSCCR_SPEC> {
        MOSTP_W::new(self, 0)
    }
}
/**Main Clock Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mosccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MOSCCR_SPEC;
impl crate::RegisterSpec for MOSCCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`mosccr::R`](R) reader structure
impl crate::Readable for MOSCCR_SPEC {}
///`write(|w| ..)` method takes [`mosccr::W`](W) writer structure
impl crate::Writable for MOSCCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOSCCR to value 0x01
impl crate::Resettable for MOSCCR_SPEC {
    const RESET_VALUE: u8 = 0x01;
}
