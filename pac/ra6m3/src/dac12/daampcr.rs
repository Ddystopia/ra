///Register `DAAMPCR` reader
pub type R = crate::R<DAAMPCR_SPEC>;
///Register `DAAMPCR` writer
pub type W = crate::W<DAAMPCR_SPEC>;
/**Amplifier Control 0

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAAMP0_A {
    ///0: Output amplifier of channel 0 is not used.
    _0 = 0,
    ///1: Output amplifier of channel 0 is used.
    _1 = 1,
}
impl From<DAAMP0_A> for bool {
    #[inline(always)]
    fn from(variant: DAAMP0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAAMP0` reader - Amplifier Control 0
pub type DAAMP0_R = crate::BitReader<DAAMP0_A>;
impl DAAMP0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAAMP0_A {
        match self.bits {
            false => DAAMP0_A::_0,
            true => DAAMP0_A::_1,
        }
    }
    ///Output amplifier of channel 0 is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAAMP0_A::_0
    }
    ///Output amplifier of channel 0 is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAAMP0_A::_1
    }
}
///Field `DAAMP0` writer - Amplifier Control 0
pub type DAAMP0_W<'a, REG> = crate::BitWriter<'a, REG, DAAMP0_A>;
impl<'a, REG> DAAMP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output amplifier of channel 0 is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAAMP0_A::_0)
    }
    ///Output amplifier of channel 0 is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAAMP0_A::_1)
    }
}
/**Amplifier Control 1

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAAMP1_A {
    ///0: Output amplifier of channel 1 is not used.
    _0 = 0,
    ///1: Output amplifier of channel 1 is used.
    _1 = 1,
}
impl From<DAAMP1_A> for bool {
    #[inline(always)]
    fn from(variant: DAAMP1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAAMP1` reader - Amplifier Control 1
pub type DAAMP1_R = crate::BitReader<DAAMP1_A>;
impl DAAMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAAMP1_A {
        match self.bits {
            false => DAAMP1_A::_0,
            true => DAAMP1_A::_1,
        }
    }
    ///Output amplifier of channel 1 is not used.
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAAMP1_A::_0
    }
    ///Output amplifier of channel 1 is used.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAAMP1_A::_1
    }
}
///Field `DAAMP1` writer - Amplifier Control 1
pub type DAAMP1_W<'a, REG> = crate::BitWriter<'a, REG, DAAMP1_A>;
impl<'a, REG> DAAMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output amplifier of channel 1 is not used.
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DAAMP1_A::_0)
    }
    ///Output amplifier of channel 1 is used.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DAAMP1_A::_1)
    }
}
impl R {
    ///Bit 6 - Amplifier Control 0
    #[inline(always)]
    pub fn daamp0(&self) -> DAAMP0_R {
        DAAMP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Amplifier Control 1
    #[inline(always)]
    pub fn daamp1(&self) -> DAAMP1_R {
        DAAMP1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 6 - Amplifier Control 0
    #[inline(always)]
    pub fn daamp0(&mut self) -> DAAMP0_W<DAAMPCR_SPEC> {
        DAAMP0_W::new(self, 6)
    }
    ///Bit 7 - Amplifier Control 1
    #[inline(always)]
    pub fn daamp1(&mut self) -> DAAMP1_W<DAAMPCR_SPEC> {
        DAAMP1_W::new(self, 7)
    }
}
/**D/A Output Amplifier Control Register

You can [`read`](crate::Reg::read) this register and get [`daampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAAMPCR_SPEC;
impl crate::RegisterSpec for DAAMPCR_SPEC {
    type Ux = u8;
}
///`read()` method returns [`daampcr::R`](R) reader structure
impl crate::Readable for DAAMPCR_SPEC {}
///`write(|w| ..)` method takes [`daampcr::W`](W) writer structure
impl crate::Writable for DAAMPCR_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAAMPCR to value 0
impl crate::Resettable for DAAMPCR_SPEC {}
