///Register `DAASWCR` reader
pub type R = crate::R<DAASWCR_SPEC>;
///Register `DAASWCR` writer
pub type W = crate::W<DAASWCR_SPEC>;
/**D/A Amplifier Stabilization Wait 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAASW0_A {
    ///0: Amplifier stabilization wait off (output) for channel 0
    _0 = 0,
    ///1: Amplifier stabilization wait on (high-Z) for channel 0.
    _1 = 1,
}
impl From<DAASW0_A> for bool {
    #[inline(always)]
    fn from(variant: DAASW0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAASW0` reader - D/A Amplifier Stabilization Wait 0
pub type DAASW0_R = crate::BitReader<DAASW0_A>;
impl DAASW0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAASW0_A {
        match self.bits {
            false => DAASW0_A::_0,
            true => DAASW0_A::_1,
        }
    }
    ///Amplifier stabilization wait off (output) for channel 0
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAASW0_A::_0
    }
    ///Amplifier stabilization wait on (high-Z) for channel 0.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAASW0_A::_1
    }
}
///Field `DAASW0` writer - D/A Amplifier Stabilization Wait 0
pub type DAASW0_W<'a, REG> = crate::BitWriter<'a, REG, DAASW0_A>;
impl<'a, REG> DAASW0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Amplifier stabilization wait off (output) for channel 0
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAASW0_A::_0)
    }
    ///Amplifier stabilization wait on (high-Z) for channel 0.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAASW0_A::_1)
    }
}
/**D/A Amplifier Stabilization Wait 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAASW1_A {
    ///0: Amplifier stabilization wait off (output) for channel 1
    _0 = 0,
    ///1: Amplifier stabilization wait on (high-Z) for channel 1.
    _1 = 1,
}
impl From<DAASW1_A> for bool {
    #[inline(always)]
    fn from(variant: DAASW1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAASW1` reader - D/A Amplifier Stabilization Wait 1
pub type DAASW1_R = crate::BitReader<DAASW1_A>;
impl DAASW1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAASW1_A {
        match self.bits {
            false => DAASW1_A::_0,
            true => DAASW1_A::_1,
        }
    }
    ///Amplifier stabilization wait off (output) for channel 1
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAASW1_A::_0
    }
    ///Amplifier stabilization wait on (high-Z) for channel 1.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAASW1_A::_1
    }
}
///Field `DAASW1` writer - D/A Amplifier Stabilization Wait 1
pub type DAASW1_W<'a, REG> = crate::BitWriter<'a, REG, DAASW1_A>;
impl<'a, REG> DAASW1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Amplifier stabilization wait off (output) for channel 1
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAASW1_A::_0)
    }
    ///Amplifier stabilization wait on (high-Z) for channel 1.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAASW1_A::_1)
    }
}
impl R {
    ///Bit 6 - D/A Amplifier Stabilization Wait 0
    #[inline(always)]
    pub fn daasw0(&self) -> DAASW0_R {
        DAASW0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - D/A Amplifier Stabilization Wait 1
    #[inline(always)]
    pub fn daasw1(&self) -> DAASW1_R {
        DAASW1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - D/A Amplifier Stabilization Wait 0
    #[inline(always)]
    pub fn daasw0(&mut self) -> DAASW0_W<DAASWCR_SPEC> {
        DAASW0_W::new(self, 6)
    }
    ///Bit 7 - D/A Amplifier Stabilization Wait 1
    #[inline(always)]
    pub fn daasw1(&mut self) -> DAASW1_W<DAASWCR_SPEC> {
        DAASW1_W::new(self, 7)
    }
}
/**D/A Amplifier Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`daaswcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daaswcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAASWCR_SPEC;
impl crate::RegisterSpec for DAASWCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`daaswcr::R`](R) reader structure
impl crate::Readable for DAASWCR_SPEC {}
///`write(|w| ..)` method takes [`daaswcr::W`](W) writer structure
impl crate::Writable for DAASWCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAASWCR to value 0
impl crate::Resettable for DAASWCR_SPEC {}
