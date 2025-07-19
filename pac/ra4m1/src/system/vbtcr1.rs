///Register `VBTCR1` reader
pub type R = crate::R<VBTCR1_SPEC>;
///Register `VBTCR1` writer
pub type W = crate::W<VBTCR1_SPEC>;
/**Battery Power supply Switch Stop

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPWSWSTP_A {
    ///0: Battery Power supply Switch Enable
    _0 = 0,
    ///1: Battery Power supply Switch stop
    _1 = 1,
}
impl From<BPWSWSTP_A> for bool {
    #[inline(always)]
    fn from(variant: BPWSWSTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BPWSWSTP` reader - Battery Power supply Switch Stop
pub type BPWSWSTP_R = crate::BitReader<BPWSWSTP_A>;
impl BPWSWSTP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BPWSWSTP_A {
        match self.bits {
            false => BPWSWSTP_A::_0,
            true => BPWSWSTP_A::_1,
        }
    }
    ///Battery Power supply Switch Enable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPWSWSTP_A::_0
    }
    ///Battery Power supply Switch stop
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPWSWSTP_A::_1
    }
}
///Field `BPWSWSTP` writer - Battery Power supply Switch Stop
pub type BPWSWSTP_W<'a, REG> = crate::BitWriter<'a, REG, BPWSWSTP_A>;
impl<'a, REG> BPWSWSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Battery Power supply Switch Enable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BPWSWSTP_A::_0)
    }
    ///Battery Power supply Switch stop
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BPWSWSTP_A::_1)
    }
}
impl R {
    ///Bit 0 - Battery Power supply Switch Stop
    #[inline(always)]
    pub fn bpwswstp(&self) -> BPWSWSTP_R {
        BPWSWSTP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Battery Power supply Switch Stop
    #[inline(always)]
    pub fn bpwswstp(&mut self) -> BPWSWSTP_W<'_, VBTCR1_SPEC> {
        BPWSWSTP_W::new(self, 0)
    }
}
/**VBATT Control Register1

You can [`read`](crate::Reg::read) this register and get [`vbtcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VBTCR1_SPEC;
impl crate::RegisterSpec for VBTCR1_SPEC {
    type Ux = u8;
}
///`read()` method returns [`vbtcr1::R`](R) reader structure
impl crate::Readable for VBTCR1_SPEC {}
///`write(|w| ..)` method takes [`vbtcr1::W`](W) writer structure
impl crate::Writable for VBTCR1_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VBTCR1 to value 0
impl crate::Resettable for VBTCR1_SPEC {}
