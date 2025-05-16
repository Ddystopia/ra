///Register `DMINT` reader
pub type R = crate::R<DMINT_SPEC>;
///Register `DMINT` writer
pub type W = crate::W<DMINT_SPEC>;
/**Destination Address Extended Repeat Area Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DARIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<DARIE_A> for bool {
    #[inline(always)]
    fn from(variant: DARIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DARIE` reader - Destination Address Extended Repeat Area Overflow Interrupt Enable
pub type DARIE_R = crate::BitReader<DARIE_A>;
impl DARIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DARIE_A {
        match self.bits {
            false => DARIE_A::_0,
            true => DARIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DARIE_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DARIE_A::_1
    }
}
///Field `DARIE` writer - Destination Address Extended Repeat Area Overflow Interrupt Enable
pub type DARIE_W<'a, REG> = crate::BitWriter<'a, REG, DARIE_A>;
impl<'a, REG> DARIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DARIE_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DARIE_A::_1)
    }
}
/**Source Address Extended Repeat Area Overflow Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SARIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<SARIE_A> for bool {
    #[inline(always)]
    fn from(variant: SARIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SARIE` reader - Source Address Extended Repeat Area Overflow Interrupt Enable
pub type SARIE_R = crate::BitReader<SARIE_A>;
impl SARIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SARIE_A {
        match self.bits {
            false => SARIE_A::_0,
            true => SARIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SARIE_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SARIE_A::_1
    }
}
///Field `SARIE` writer - Source Address Extended Repeat Area Overflow Interrupt Enable
pub type SARIE_W<'a, REG> = crate::BitWriter<'a, REG, SARIE_A>;
impl<'a, REG> SARIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(SARIE_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(SARIE_A::_1)
    }
}
/**Repeat Size End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPTIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<RPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RPTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPTIE` reader - Repeat Size End Interrupt Enable
pub type RPTIE_R = crate::BitReader<RPTIE_A>;
impl RPTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RPTIE_A {
        match self.bits {
            false => RPTIE_A::_0,
            true => RPTIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPTIE_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPTIE_A::_1
    }
}
///Field `RPTIE` writer - Repeat Size End Interrupt Enable
pub type RPTIE_W<'a, REG> = crate::BitWriter<'a, REG, RPTIE_A>;
impl<'a, REG> RPTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(RPTIE_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(RPTIE_A::_1)
    }
}
/**Transfer Escape End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<ESIE_A> for bool {
    #[inline(always)]
    fn from(variant: ESIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ESIE` reader - Transfer Escape End Interrupt Enable
pub type ESIE_R = crate::BitReader<ESIE_A>;
impl ESIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ESIE_A {
        match self.bits {
            false => ESIE_A::_0,
            true => ESIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESIE_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESIE_A::_1
    }
}
///Field `ESIE` writer - Transfer Escape End Interrupt Enable
pub type ESIE_W<'a, REG> = crate::BitWriter<'a, REG, ESIE_A>;
impl<'a, REG> ESIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ESIE_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ESIE_A::_1)
    }
}
/**Transfer End Interrupt Enable

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTIE_A {
    ///0: Disable
    _0 = 0,
    ///1: Enable.
    _1 = 1,
}
impl From<DTIE_A> for bool {
    #[inline(always)]
    fn from(variant: DTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DTIE` reader - Transfer End Interrupt Enable
pub type DTIE_R = crate::BitReader<DTIE_A>;
impl DTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTIE_A {
        match self.bits {
            false => DTIE_A::_0,
            true => DTIE_A::_1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTIE_A::_0
    }
    ///Enable.
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTIE_A::_1
    }
}
///Field `DTIE` writer - Transfer End Interrupt Enable
pub type DTIE_W<'a, REG> = crate::BitWriter<'a, REG, DTIE_A>;
impl<'a, REG> DTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DTIE_A::_0)
    }
    ///Enable.
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DTIE_A::_1)
    }
}
impl R {
    ///Bit 0 - Destination Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn darie(&self) -> DARIE_R {
        DARIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Source Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn sarie(&self) -> SARIE_R {
        SARIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Repeat Size End Interrupt Enable
    #[inline(always)]
    pub fn rptie(&self) -> RPTIE_R {
        RPTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer Escape End Interrupt Enable
    #[inline(always)]
    pub fn esie(&self) -> ESIE_R {
        ESIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transfer End Interrupt Enable
    #[inline(always)]
    pub fn dtie(&self) -> DTIE_R {
        DTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Destination Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn darie(&mut self) -> DARIE_W<DMINT_SPEC> {
        DARIE_W::new(self, 0)
    }
    ///Bit 1 - Source Address Extended Repeat Area Overflow Interrupt Enable
    #[inline(always)]
    pub fn sarie(&mut self) -> SARIE_W<DMINT_SPEC> {
        SARIE_W::new(self, 1)
    }
    ///Bit 2 - Repeat Size End Interrupt Enable
    #[inline(always)]
    pub fn rptie(&mut self) -> RPTIE_W<DMINT_SPEC> {
        RPTIE_W::new(self, 2)
    }
    ///Bit 3 - Transfer Escape End Interrupt Enable
    #[inline(always)]
    pub fn esie(&mut self) -> ESIE_W<DMINT_SPEC> {
        ESIE_W::new(self, 3)
    }
    ///Bit 4 - Transfer End Interrupt Enable
    #[inline(always)]
    pub fn dtie(&mut self) -> DTIE_W<DMINT_SPEC> {
        DTIE_W::new(self, 4)
    }
}
/**DMA Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`dmint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMINT_SPEC;
impl crate::RegisterSpec for DMINT_SPEC {
    type Ux = u8;
}
///`read()` method returns [`dmint::R`](R) reader structure
impl crate::Readable for DMINT_SPEC {}
///`write(|w| ..)` method takes [`dmint::W`](W) writer structure
impl crate::Writable for DMINT_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMINT to value 0
impl crate::Resettable for DMINT_SPEC {}
