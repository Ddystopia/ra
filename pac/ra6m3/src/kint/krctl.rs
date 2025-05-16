///Register `KRCTL` reader
pub type R = crate::R<KRCTL_SPEC>;
///Register `KRCTL` writer
pub type W = crate::W<KRCTL_SPEC>;
/**Detection Edge Selection (KRF0 to KRF7)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KREG_A {
    ///0: Falling edge
    _0 = 0,
    ///1: Rising edge
    _1 = 1,
}
impl From<KREG_A> for bool {
    #[inline(always)]
    fn from(variant: KREG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `KREG` reader - Detection Edge Selection (KRF0 to KRF7)
pub type KREG_R = crate::BitReader<KREG_A>;
impl KREG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KREG_A {
        match self.bits {
            false => KREG_A::_0,
            true => KREG_A::_1,
        }
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KREG_A::_0
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KREG_A::_1
    }
}
///Field `KREG` writer - Detection Edge Selection (KRF0 to KRF7)
pub type KREG_W<'a, REG> = crate::BitWriter<'a, REG, KREG_A>;
impl<'a, REG> KREG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Falling edge
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KREG_A::_0)
    }
    ///Rising edge
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KREG_A::_1)
    }
}
/**Usage of Key Interrupt Flags(KR0 to KR7)

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KRMD_A {
    ///0: Do not use key interrupt flags
    _0 = 0,
    ///1: Use key interrupt flags.
    _1 = 1,
}
impl From<KRMD_A> for bool {
    #[inline(always)]
    fn from(variant: KRMD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `KRMD` reader - Usage of Key Interrupt Flags(KR0 to KR7)
pub type KRMD_R = crate::BitReader<KRMD_A>;
impl KRMD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> KRMD_A {
        match self.bits {
            false => KRMD_A::_0,
            true => KRMD_A::_1,
        }
    }
    ///Do not use key interrupt flags
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == KRMD_A::_0
    }
    ///Use key interrupt flags.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == KRMD_A::_1
    }
}
///Field `KRMD` writer - Usage of Key Interrupt Flags(KR0 to KR7)
pub type KRMD_W<'a, REG> = crate::BitWriter<'a, REG, KRMD_A>;
impl<'a, REG> KRMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Do not use key interrupt flags
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(KRMD_A::_0)
    }
    ///Use key interrupt flags.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(KRMD_A::_1)
    }
}
impl R {
    ///Bit 0 - Detection Edge Selection (KRF0 to KRF7)
    #[inline(always)]
    pub fn kreg(&self) -> KREG_R {
        KREG_R::new((self.bits & 1) != 0)
    }
    ///Bit 7 - Usage of Key Interrupt Flags(KR0 to KR7)
    #[inline(always)]
    pub fn krmd(&self) -> KRMD_R {
        KRMD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Detection Edge Selection (KRF0 to KRF7)
    #[inline(always)]
    pub fn kreg(&mut self) -> KREG_W<KRCTL_SPEC> {
        KREG_W::new(self, 0)
    }
    ///Bit 7 - Usage of Key Interrupt Flags(KR0 to KR7)
    #[inline(always)]
    pub fn krmd(&mut self) -> KRMD_W<KRCTL_SPEC> {
        KRMD_W::new(self, 7)
    }
}
/**KEY Return Control Register

You can [`read`](crate::Reg::read) this register and get [`krctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`krctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct KRCTL_SPEC;
impl crate::RegisterSpec for KRCTL_SPEC {
    type Ux = u8;
}
///`read()` method returns [`krctl::R`](R) reader structure
impl crate::Readable for KRCTL_SPEC {}
///`write(|w| ..)` method takes [`krctl::W`](W) writer structure
impl crate::Writable for KRCTL_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KRCTL to value 0
impl crate::Resettable for KRCTL_SPEC {}
